use std::fs;
use std::path::Path;

const WORKFLOW_PATH: &str = ".github/workflows/draft-artifacts.yml";
const ARTIFACT_README_PATH: &str = "docs/releases/artifact-readme-v0.4.1.md";

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
fn draft_artifact_workflow_uploads_only_workflow_artifacts() {
    let workflow = read_workflow();

    assert!(workflow.contains("actions/upload-artifact@v4"));
    assert!(workflow.contains("Upload draft workflow artifact"));
    assert!(workflow.contains("dist/${{ matrix.archive_name }}"));
    assert!(workflow.contains("dist/SHA256SUMS"));
}

#[test]
fn artifact_readme_contains_required_warnings() {
    let readme = read_artifact_readme();
    let required = [
        "AEGIS v0.4.1 Developer Preview Artifact",
        "unsigned and not notarized",
        "local-only, pre-alpha, and developer-oriented",
        "not production-ready or enterprise-hardened",
        "archive, not an installer",
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

fn workflow_path() -> std::path::PathBuf {
    repo_path(WORKFLOW_PATH)
}

fn repo_path(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}
