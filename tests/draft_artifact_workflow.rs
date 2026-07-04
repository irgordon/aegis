use std::fs;
use std::path::Path;

const WORKFLOW_PATH: &str = ".github/workflows/draft-artifacts.yml";
const ARTIFACT_README_PATH: &str = "docs/releases/artifact-readme-v0.4.1.md";
const ARTIFACT_FIXTURE_PATH: &str = "examples/artifact/health-check-request.json";

#[test]
fn draft_artifact_workflow_exists() {
    assert!(
        workflow_path().exists(),
        "draft artifact workflow must exist"
    );
}

#[test]
fn draft_artifact_workflow_is_manual_only() {
    let workflow = read_workflow();

    assert!(workflow.contains("name: Draft Release Artifacts"));
    assert!(workflow.contains("workflow_dispatch:"));
    assert!(!workflow.contains("\n  push:"));
    assert!(!workflow.contains("\n  pull_request:"));
    assert!(!workflow.contains("tags:"));
}

#[test]
fn draft_artifact_workflow_does_not_publish_releases() {
    let workflow = read_workflow();
    let blocked = [
        "softprops/action-gh-release",
        "actions/create-release",
        "ncipollo/release-action",
        "gh release",
        "gh api repos",
        "git tag",
        "git push --tags",
    ];

    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_artifact_workflow_does_not_sign_or_create_installers() {
    let workflow = read_workflow();
    let blocked = [
        "codesign",
        "notarytool",
        "security import",
        "signtool",
        "gpg --sign",
        ".dmg",
        ".pkg",
        ".msi",
        ".AppImage",
        ".deb",
        ".rpm",
    ];

    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_artifact_workflow_references_expected_artifacts() {
    let workflow = read_workflow();

    assert!(workflow.contains("v0.4.1"));
    assert!(workflow.contains("aegis-v0.4.1-macos-arm64.tar.gz"));
    assert!(workflow.contains("aegis-v0.4.1-macos-x64.tar.gz"));
    assert!(workflow.contains("SHA256SUMS"));
    assert!(workflow.contains("shasum -a 256"));
}

#[test]
fn draft_artifact_workflow_remaps_release_build_paths() {
    let workflow = read_workflow();
    let required = [
        "Configure draft release path remapping",
        "RUSTFLAGS=--remap-path-prefix=${GITHUB_WORKSPACE}=.",
        "--remap-path-prefix=${HOME}=~",
        "--remap-path-prefix=${RUNNER_TEMP}=.",
        "CARGO_PROFILE_RELEASE_DEBUG=false",
        "CARGO_PROFILE_RELEASE_STRIP=debuginfo",
    ];

    assert_present(&workflow, &required);
    let remapping_step = workflow
        .find("Configure draft release path remapping")
        .expect("path remapping step should exist");
    let gateway_build = workflow
        .find("Build gateway binary")
        .expect("gateway release build step should exist");
    let desktop_build = workflow
        .find("Build desktop binary")
        .expect("desktop release build step should exist");

    assert!(
        remapping_step < gateway_build,
        "draft release path remapping must be configured before gateway release build"
    );
    assert!(
        remapping_step < desktop_build,
        "draft release path remapping must be configured before desktop release build"
    );
}

#[test]
fn draft_artifact_workflow_generates_combined_checksum_manifest() {
    let workflow = read_workflow();
    let required = [
        "combined-draft-artifacts:",
        "needs: macos-draft-artifacts",
        "actions/download-artifact@v4",
        "merge-multiple: true",
        "Generate combined SHA256SUMS",
        "find . -maxdepth 1 -name 'aegis-v0.4.1-*.tar.gz' -print | sort",
        "shasum -a 256 -c SHA256SUMS",
        "Upload combined draft workflow artifact",
        "name: draft-artifacts-v0.4.1",
    ];

    assert_present(&workflow, &required);
}

#[test]
fn draft_artifact_workflow_checksums_final_archives_only() {
    let workflow = read_workflow();
    let blocked = [
        "shasum -a 256 \"${archive}\" > SHA256SUMS",
        "dist/SHA256SUMS",
        "dist/combined/SHA256SUMS.sig",
        "gpg --sign",
        "gpg --detach-sign",
    ];

    assert_absent(&workflow, &blocked);
    assert!(workflow.contains("dist/combined/aegis-v0.4.1-*.tar.gz"));
    assert!(workflow.contains("dist/combined/SHA256SUMS"));
}

#[test]
fn draft_artifact_workflow_uploads_only_workflow_artifacts() {
    let workflow = read_workflow();

    assert!(workflow.contains("actions/upload-artifact@v4"));
    assert!(workflow.contains("Upload draft workflow artifact"));
    assert!(workflow.contains("Upload combined draft workflow artifact"));
    assert!(workflow.contains("dist/${{ matrix.archive_name }}"));
    assert!(workflow.contains("dist/combined/SHA256SUMS"));
}

#[test]
fn draft_artifact_workflow_stages_local_policy_bundle() {
    let workflow = read_workflow();
    let required = [
        "policy-bundles/local-dev",
        "examples/policy-bundles/local-dev/manifest.yaml",
        "examples/policy-bundles/local-dev/gateway_policy.yaml",
        "examples/policy-bundles/local-dev/risk_matrix.yaml",
        "examples/policy-bundles/local-dev/checksums/SHA256SUMS",
        "examples/policy-bundles/local-dev/signatures/public.pem",
        "examples/policy-bundles/local-dev/signatures/SHA256SUMS.sig",
        "verified local development policy bundle",
    ];

    assert_present(&workflow, &required);
}

#[test]
fn draft_artifact_workflow_stages_health_check_smoke_fixture() {
    let workflow = read_workflow();
    let required = [
        "mkdir -p \"${staging}/bin\" \"${staging}/desktop\" \"${staging}/examples\"",
        "examples/artifact/health-check-request.json",
        "${staging}/examples/health-check-request.json",
        "It includes examples/health-check-request.json for an artifact-only gateway smoke test.",
        "./bin/aegis-gateway --bundle policy-bundles/local-dev examples/health-check-request.json",
    ];
    let blocked = [
        "cp -R examples",
        "cp -r examples",
        "schemas/examples/valid/SandboxNoteWriteRequest.json",
        "sandbox.note.write",
        "approval",
        "replay",
    ];

    assert_present(&workflow, &required);
    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_artifact_workflow_does_not_stage_private_policy_material() {
    let workflow = read_workflow().to_lowercase();
    let blocked = [
        "private.pem",
        "private.key",
        "secret",
        "regenerate",
        "scripts/sign",
    ];

    assert_absent(&workflow, &blocked);
}

#[test]
fn artifact_health_check_fixture_is_safe_and_read_only() {
    let fixture = read_artifact_fixture();
    let request: serde_json::Value =
        serde_json::from_str(&fixture).expect("artifact fixture should be valid JSON");

    assert_eq!(request["tool"]["name"], "health.check");
    assert_eq!(request["tool"]["capability_class"], "L0");
    assert_eq!(request["params"], serde_json::json!({}));

    let blocked = [
        "sandbox.note.write",
        "approval",
        "approve",
        "replay",
        "credential",
        "password",
        "token",
        "secret",
        "private_key",
    ];

    assert_absent(&fixture.to_lowercase(), &blocked);
}

#[test]
fn artifact_readme_contains_required_warnings() {
    let readme = read_artifact_readme();
    let required = [
        "AEGIS v0.4.1 Developer Preview Artifact",
        "unsigned, not notarized",
        "archive-based, and developer-oriented",
        "not production-ready or enterprise-hardened",
        "archive, not an installer",
        "safe `health.check` request fixture",
        "First Five Minutes",
        "./bin/aegis-gateway --bundle policy-bundles/local-dev examples/health-check-request.json",
        "request was validated",
        "policy was verified",
        "execution was authorized",
        "audit evidence was produced",
        "state evidence was produced",
        "Validate the SHA-256 checksum before use",
        "Do not treat this artifact as a trusted production distribution",
    ];

    assert_present(&readme, &required);
}

fn assert_absent(content: &str, blocked: &[&str]) {
    for value in blocked {
        assert!(
            !content.contains(value),
            "draft workflow must not contain `{value}`"
        );
    }
}

fn assert_present(content: &str, required: &[&str]) {
    for value in required {
        assert!(
            content.contains(value),
            "artifact README must contain `{value}`"
        );
    }
}

fn read_workflow() -> String {
    fs::read_to_string(workflow_path()).expect("draft artifact workflow should be readable")
}

fn read_artifact_readme() -> String {
    fs::read_to_string(repo_path(ARTIFACT_README_PATH)).expect("artifact README should be readable")
}

fn read_artifact_fixture() -> String {
    fs::read_to_string(repo_path(ARTIFACT_FIXTURE_PATH))
        .expect("artifact health-check fixture should be readable")
}

fn workflow_path() -> std::path::PathBuf {
    repo_path(WORKFLOW_PATH)
}

fn repo_path(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}
