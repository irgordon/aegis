use std::{
    fs,
    path::{Path, PathBuf},
};

use aegis::policy::{
    load_policy_bundle, ChecksumMetadataStatus, PolicyBundleVerificationStatus,
    SignatureMetadataStatus,
};

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn complete_local_development_bundle_loads_successfully() {
    let verification = load_policy_bundle(LOCAL_DEV_BUNDLE)
        .unwrap_or_else(|error| panic!("local development bundle should verify: {error:?}"));

    assert!(verification.is_verified());
    assert_eq!(
        verification.bundle.as_ref().map(|bundle| bundle.as_str()),
        Some("local-dev")
    );
    assert_eq!(
        verification
            .policy_version
            .as_ref()
            .map(|version| version.as_str()),
        Some("0.1.0-local")
    );
    assert_eq!(
        verification
            .risk_matrix_version
            .as_ref()
            .map(|version| version.as_str()),
        Some("risk-0.1.0-local")
    );
    assert_eq!(
        verification.verification_status,
        PolicyBundleVerificationStatus::SignatureCryptographicVerificationNotImplemented
    );
    assert_eq!(
        verification.signature_metadata_status,
        SignatureMetadataStatus::SignatureMetadataPresent
    );
    assert_eq!(
        verification.checksum_metadata_status,
        ChecksumMetadataStatus::ChecksumMetadataPresent
    );
}

#[test]
fn missing_manifest_fails_closed() {
    let bundle = mutable_bundle("missing_manifest");
    fs::remove_file(bundle.join("manifest.yaml"))
        .unwrap_or_else(|error| panic!("manifest fixture should be removable: {error}"));

    assert_rejected(bundle);
}

#[test]
fn missing_gateway_policy_fails_closed() {
    let bundle = mutable_bundle("missing_gateway_policy");
    fs::remove_file(bundle.join("gateway_policy.yaml"))
        .unwrap_or_else(|error| panic!("gateway policy fixture should be removable: {error}"));

    assert_rejected(bundle);
}

#[test]
fn missing_risk_matrix_fails_closed() {
    let bundle = mutable_bundle("missing_risk_matrix");
    fs::remove_file(bundle.join("risk_matrix.yaml"))
        .unwrap_or_else(|error| panic!("risk matrix fixture should be removable: {error}"));

    assert_rejected(bundle);
}

#[test]
fn missing_signatures_metadata_fails_closed() {
    let bundle = mutable_bundle("missing_signatures_metadata");
    fs::remove_dir_all(bundle.join("signatures"))
        .unwrap_or_else(|error| panic!("signature metadata fixture should be removable: {error}"));

    assert_rejected(bundle);
}

#[test]
fn missing_checksums_metadata_fails_closed() {
    let bundle = mutable_bundle("missing_checksums_metadata");
    fs::remove_dir_all(bundle.join("checksums"))
        .unwrap_or_else(|error| panic!("checksum metadata fixture should be removable: {error}"));

    assert_rejected(bundle);
}

#[test]
fn manifest_risk_matrix_version_mismatch_fails_closed() {
    let bundle = mutable_bundle("risk_matrix_version_mismatch");
    fs::write(
        bundle.join("risk_matrix.yaml"),
        "risk_matrix_version: risk-mismatch\n",
    )
    .unwrap_or_else(|error| panic!("risk matrix fixture should be writable: {error}"));

    assert_rejected(bundle);
}

#[test]
fn loader_does_not_evaluate_real_policy_decisions_yet() {
    let policy = fs::read_to_string(Path::new(LOCAL_DEV_BUNDLE).join("gateway_policy.yaml"))
        .unwrap_or_else(|error| panic!("gateway policy fixture should be readable: {error}"));
    let verification = load_policy_bundle(LOCAL_DEV_BUNDLE)
        .unwrap_or_else(|error| panic!("local development bundle should verify: {error:?}"));

    assert!(policy.contains("default_decision: deny"));
    assert!(verification.is_verified());
}

fn assert_rejected(bundle: PathBuf) {
    let verification = load_policy_bundle(bundle)
        .expect_err("invalid policy bundle should fail closed with verification evidence");

    assert_eq!(
        verification.verification_status,
        PolicyBundleVerificationStatus::Rejected
    );
    assert!(verification.failure_reason.is_some());
}

fn mutable_bundle(case_name: &str) -> PathBuf {
    let target = Path::new("target")
        .join("policy-bundle-tests")
        .join(case_name);

    if target.exists() {
        fs::remove_dir_all(&target)
            .unwrap_or_else(|error| panic!("old mutable fixture should be removable: {error}"));
    }

    copy_dir(Path::new(LOCAL_DEV_BUNDLE), &target);
    target
}

fn copy_dir(source: &Path, target: &Path) {
    fs::create_dir_all(target)
        .unwrap_or_else(|error| panic!("target fixture directory should be creatable: {error}"));

    for entry in fs::read_dir(source)
        .unwrap_or_else(|error| panic!("source fixture directory should be readable: {error}"))
    {
        let entry =
            entry.unwrap_or_else(|error| panic!("fixture entry should be readable: {error}"));
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &target_path);
        } else {
            fs::copy(&source_path, &target_path)
                .unwrap_or_else(|error| panic!("fixture file should copy: {error}"));
        }
    }
}
