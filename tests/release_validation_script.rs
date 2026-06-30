use std::{fs, os::unix::fs::PermissionsExt, path::Path};

const RELEASE_SCRIPT: &str = "scripts/validate-v0.4.0-release.sh";

#[test]
fn release_validation_script_exists_and_is_executable() {
    let metadata = fs::metadata(RELEASE_SCRIPT).expect("release validation script should exist");
    let mode = metadata.permissions().mode();

    assert!(Path::new(RELEASE_SCRIPT).is_file());
    assert_ne!(
        mode & 0o111,
        0,
        "release validation script must be executable"
    );
}

#[test]
fn release_validation_script_uses_fail_fast_bash() {
    let script = read_script();

    assert!(script.starts_with("#!/usr/bin/env bash"));
    assert!(script.contains("set -euo pipefail"));
    assert!(script.contains("trap report_failure ERR"));
    assert!(script.contains("trap cleanup EXIT"));
}

#[test]
fn release_validation_script_references_expected_release_phases() {
    let script = read_script();
    let phases = [
        "Repository validation",
        "Rust workspace validation",
        "Desktop validation",
        "Desktop UI validation",
        "Gateway health-check smoke test",
        "Sandbox mutation smoke test",
        "Recovery inspection",
        "Recovery planning",
        "Desktop launch check",
        "Repository cleanliness",
    ];

    for phase in phases {
        assert!(
            script.contains(phase),
            "release validation script should include phase: {phase}"
        );
    }
}

#[test]
fn release_validation_script_calls_existing_validation_commands() {
    let script = read_script();
    let commands = [
        "python3 scripts/verify.py",
        "cargo fmt --check",
        "cargo clippy --all-targets --all-features -- -D warnings",
        "cargo test",
        "cargo fmt --manifest-path src-tauri/Cargo.toml --check",
        "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings",
        "cargo test --manifest-path src-tauri/Cargo.toml",
        "cargo check --manifest-path src-tauri/Cargo.toml",
        "cargo test --test ui_scaffold",
        "cargo run --quiet --bin aegis-gateway",
        "cargo run --manifest-path src-tauri/Cargo.toml",
        "git diff --check",
        "git status --short --branch",
    ];

    for command in commands {
        assert!(
            script.contains(command),
            "release validation script should call: {command}"
        );
    }
}

#[test]
fn release_validation_script_does_not_publish_or_package_release() {
    let script = read_script().to_lowercase();
    let forbidden = [
        "git tag",
        "git push --tags",
        "gh release",
        "cargo tauri build",
        "codesign",
        "notarytool",
        "auto-update",
        "installer",
    ];

    for term in forbidden {
        assert!(
            !script.contains(term),
            "release validation script must not publish or package releases: {term}"
        );
    }
}

fn read_script() -> String {
    fs::read_to_string(RELEASE_SCRIPT).expect("release validation script should be readable")
}
