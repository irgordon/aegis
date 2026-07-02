use std::fs;
use std::path::Path;

const WORKFLOW_PATH: &str = ".github/workflows/draft-github-release.yml";
const RELEASE_BODY_PATH: &str = "docs/releases/v0.4.1-draft-body.md";

#[test]
fn draft_github_release_workflow_exists() {
    assert!(
        workflow_path().exists(),
        "draft GitHub Release workflow must exist"
    );
}

#[test]
fn draft_github_release_workflow_is_manual_only() {
    let workflow = read_workflow();

    assert!(workflow.contains("name: Draft GitHub Release"));
    assert!(workflow.contains("workflow_dispatch:"));
    assert!(!workflow.contains("\n  push:"));
    assert!(!workflow.contains("\n  pull_request:"));
    assert!(!workflow.contains("\n  schedule:"));
    assert!(!workflow.contains("\n  release:"));
    assert!(!workflow.contains("inputs:"));
}

#[test]
fn draft_github_release_workflow_uses_fixed_release_target() {
    let workflow = read_workflow();
    let required = [
        "AEGIS_RELEASE_VERSION: v0.4.1",
        "AEGIS_RELEASE_TITLE: AEGIS v0.4.1 Developer Preview",
        "aegis-v0.4.1-macos-arm64.tar.gz",
        "aegis-v0.4.1-macos-x64.tar.gz",
        "SHA256SUMS",
    ];

    assert_present(&workflow, &required);
}

#[test]
fn draft_github_release_workflow_requires_existing_tag() {
    let workflow = read_workflow();
    let required = [
        "Require workflow ref to match v0.4.1 tag",
        "git rev-list -n 1 \"${AEGIS_RELEASE_VERSION}\"",
        "git rev-parse HEAD",
        "Run this workflow with --ref ${AEGIS_RELEASE_VERSION} after the maintainer-created tag exists.",
        "Require existing v0.4.1 tag",
        "git ls-remote --exit-code --tags origin \"refs/tags/${AEGIS_RELEASE_VERSION}\"",
        "gh release create \"${AEGIS_RELEASE_VERSION}\" \"${release_assets[@]}\"",
        "--verify-tag",
    ];

    assert_present(&workflow, &required);
}

#[test]
fn draft_github_release_workflow_does_not_create_or_move_tags() {
    let workflow = read_workflow();
    let blocked = [
        "git tag",
        "git push",
        "git push --tags",
        "gh api repos",
        "git update-ref",
        "gh release create \"${AEGIS_RELEASE_VERSION}\" --target",
        "gh release edit \"${AEGIS_RELEASE_VERSION}\" --target",
    ];

    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_github_release_workflow_creates_draft_prerelease_only() {
    let workflow = read_workflow();
    let required = [
        "Create or update draft GitHub Release",
        "gh release create",
        "gh release edit",
        "--draft",
        "--prerelease",
        "--latest=false",
        "--title \"${AEGIS_RELEASE_TITLE}\"",
        "--notes-file docs/releases/v0.4.1-draft-body.md",
    ];
    let blocked = ["--draft=false", "--prerelease=false"];

    assert_present(&workflow, &required);
    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_github_release_workflow_refuses_published_release_overwrite() {
    let workflow = read_workflow();
    let required = [
        "gh release view \"${AEGIS_RELEASE_VERSION}\" --json isDraft --jq '.isDraft'",
        "Existing ${AEGIS_RELEASE_VERSION} release is not a draft; refusing to modify it.",
        "exit 1",
        "gh release delete-asset \"${AEGIS_RELEASE_VERSION}\" \"${asset}\" --yes",
    ];

    assert_present(&workflow, &required);
}

#[test]
fn draft_github_release_workflow_validates_assets_before_release_creation() {
    let workflow = read_workflow();
    let validation = workflow
        .find("Validate release asset set")
        .expect("release asset validation step should exist");
    let tag_check = workflow
        .find("Require existing v0.4.1 tag")
        .expect("existing tag check step should exist");
    let release_step = workflow
        .find("Create or update draft GitHub Release")
        .expect("draft release step should exist");

    assert!(
        validation < release_step,
        "release assets must be validated before draft release creation"
    );
    assert!(
        tag_check < release_step,
        "existing tag must be verified before draft release creation"
    );
}

#[test]
fn draft_github_release_workflow_uploads_only_expected_release_assets() {
    let workflow = read_workflow();
    let required = [
        "dist/release-assets/aegis-v0.4.1-macos-arm64.tar.gz",
        "dist/release-assets/aegis-v0.4.1-macos-x64.tar.gz",
        "dist/release-assets/SHA256SUMS",
        "gh release upload \"${AEGIS_RELEASE_VERSION}\" \"${release_assets[@]}\"",
        "diff -u \"${RUNNER_TEMP}/expected-release-assets.txt\" \"${RUNNER_TEMP}/actual-release-assets.txt\"",
    ];
    let blocked = [
        "dist/release-assets/SHA256SUMS.sig",
        "dist/release-assets/*.dmg",
        "dist/release-assets/*.pkg",
        "dist/release-assets/*.msi",
        "dist/release-assets/*.AppImage",
        "dist/release-assets/*.deb",
        "dist/release-assets/*.rpm",
    ];

    assert_present(&workflow, &required);
    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_github_release_workflow_verifies_combined_checksums() {
    let workflow = read_workflow();
    let required = [
        "Generate and verify combined SHA256SUMS",
        "find . -maxdepth 1 -name 'aegis-v0.4.1-*.tar.gz' -print | sort",
        "shasum -a 256 -c SHA256SUMS",
        "grep -F \"  ${archive}\" SHA256SUMS",
    ];

    assert_present(&workflow, &required);
}

#[test]
fn draft_github_release_workflow_uses_minimal_permissions() {
    let workflow = read_workflow();
    let required = [
        "permissions:\n  contents: read",
        "draft-github-release:",
        "permissions:\n      contents: write",
    ];
    let blocked = [
        "id-token: write",
        "packages: write",
        "deployments: write",
        "issues: write",
        "pull-requests: write",
        "security-events: write",
    ];

    assert_present(&workflow, &required);
    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_github_release_workflow_does_not_sign_notarize_or_create_installers() {
    let workflow = read_workflow();
    let blocked = [
        "codesign",
        "notarytool",
        "security import",
        "signtool",
        "gpg --sign",
        "gpg --detach-sign",
        "tauri build",
        ".dmg",
        ".pkg",
        ".msi",
        ".AppImage",
        ".deb",
        ".rpm",
        "auto-update",
    ];

    assert_absent(&workflow, &blocked);
}

#[test]
fn draft_github_release_body_contains_required_warnings() {
    let body = read_release_body();
    let required = [
        "AEGIS v0.4.1 Developer Preview",
        "draft GitHub Release for maintainer review",
        "developer preview",
        "unsigned and not notarized",
        "local-only, pre-alpha, and developer-oriented",
        "not production-ready or enterprise-hardened",
        "archive-based",
        "There is no installer",
        "There is no auto-update",
        "Validate SHA-256 checksums before use",
        "shasum -a 256 -c SHA256SUMS",
    ];

    assert_present(&body, &required);
}

fn assert_absent(content: &str, blocked: &[&str]) {
    for value in blocked {
        assert!(
            !content.contains(value),
            "draft GitHub Release workflow must not contain `{value}`"
        );
    }
}

fn assert_present(content: &str, required: &[&str]) {
    for value in required {
        assert!(
            content.contains(value),
            "draft GitHub Release workflow must contain `{value}`"
        );
    }
}

fn read_workflow() -> String {
    fs::read_to_string(workflow_path()).expect("draft GitHub Release workflow should be readable")
}

fn read_release_body() -> String {
    fs::read_to_string(repo_path(RELEASE_BODY_PATH))
        .expect("draft GitHub Release body should be readable")
}

fn workflow_path() -> std::path::PathBuf {
    repo_path(WORKFLOW_PATH)
}

fn repo_path(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}
