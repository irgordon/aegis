use std::{
    fs,
    path::{Path, PathBuf},
};

use aegis::policy::{
    load_policy_bundle, ChecksumMetadataStatus, ChecksumVerificationStatus,
    PolicyBundleVerificationStatus, SignatureMetadataStatus, SignatureVerificationStatus,
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
        verification.signature_verification_status,
        SignatureVerificationStatus::SignatureCryptographicVerificationNotImplemented
    );
    assert_eq!(
        verification.checksum_metadata_status,
        ChecksumMetadataStatus::ChecksumMetadataPresent
    );
    assert_eq!(
        verification.checksum_verification_status,
        ChecksumVerificationStatus::Verified
    );
    assert_eq!(verification.checksum_entries.len(), 3);
    assert!(verification
        .checksum_entries
        .iter()
        .all(|entry| entry.verification_status == ChecksumVerificationStatus::Verified));
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
    fs::remove_file(bundle.join("checksums").join("SHA256SUMS"))
        .unwrap_or_else(|error| panic!("checksum manifest fixture should be removable: {error}"));

    let verification = rejected_verification(bundle);

    assert_eq!(
        verification.checksum_verification_status,
        ChecksumVerificationStatus::MetadataMissing
    );
}

#[test]
fn missing_manifest_checksum_entry_fails_closed() {
    let bundle = mutable_bundle("missing_manifest_checksum_entry");
    remove_checksum_entry(&bundle, "manifest.yaml");

    assert_checksum_rejected(bundle, ChecksumVerificationStatus::EntryMissing);
}

#[test]
fn missing_gateway_policy_checksum_entry_fails_closed() {
    let bundle = mutable_bundle("missing_gateway_policy_checksum_entry");
    remove_checksum_entry(&bundle, "gateway_policy.yaml");

    assert_checksum_rejected(bundle, ChecksumVerificationStatus::EntryMissing);
}

#[test]
fn missing_risk_matrix_checksum_entry_fails_closed() {
    let bundle = mutable_bundle("missing_risk_matrix_checksum_entry");
    remove_checksum_entry(&bundle, "risk_matrix.yaml");

    assert_checksum_rejected(bundle, ChecksumVerificationStatus::EntryMissing);
}

#[test]
fn manifest_checksum_mismatch_fails_closed() {
    let bundle = mutable_bundle("manifest_checksum_mismatch");
    fs::write(
        bundle.join("manifest.yaml"),
        "bundle_id: local-dev\npolicy_version: tampered\nrisk_matrix_version: risk-0.1.0-local\npolicy_hash: sha256:local-dev-policy\ntarget_environment: local\n",
    )
    .unwrap_or_else(|error| panic!("manifest fixture should be writable: {error}"));

    assert_checksum_rejected(bundle, ChecksumVerificationStatus::Mismatch);
}

#[test]
fn gateway_policy_checksum_mismatch_fails_closed() {
    let bundle = mutable_bundle("gateway_policy_checksum_mismatch");
    fs::write(
        bundle.join("gateway_policy.yaml"),
        "policy_version: 0.1.0-local\ndefault_decision: allow\n",
    )
    .unwrap_or_else(|error| panic!("gateway policy fixture should be writable: {error}"));

    assert_checksum_rejected(bundle, ChecksumVerificationStatus::Mismatch);
}

#[test]
fn risk_matrix_checksum_mismatch_fails_closed() {
    let bundle = mutable_bundle("risk_matrix_checksum_mismatch");
    fs::write(
        bundle.join("risk_matrix.yaml"),
        "risk_matrix_version: risk-0.1.0-local\nnote: tampered\n",
    )
    .unwrap_or_else(|error| panic!("risk matrix fixture should be writable: {error}"));

    assert_checksum_rejected(bundle, ChecksumVerificationStatus::Mismatch);
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
    let verification = rejected_verification(bundle);

    assert_eq!(
        verification.verification_status,
        PolicyBundleVerificationStatus::Rejected
    );
    assert!(verification.failure_reason.is_some());
}

fn assert_checksum_rejected(bundle: PathBuf, status: ChecksumVerificationStatus) {
    let verification = rejected_verification(bundle);

    assert_eq!(verification.checksum_verification_status, status);
    assert_eq!(
        verification.verification_status,
        PolicyBundleVerificationStatus::Rejected
    );
    assert!(verification.failure_reason.is_some());
}

fn rejected_verification(bundle: PathBuf) -> aegis::policy::PolicyBundleVerification {
    *load_policy_bundle(bundle)
        .expect_err("invalid policy bundle should fail closed with verification evidence")
}

fn remove_checksum_entry(bundle: &Path, file_name: &str) {
    let checksum_path = bundle.join("checksums").join("SHA256SUMS");
    let content = fs::read_to_string(&checksum_path)
        .unwrap_or_else(|error| panic!("checksum manifest should be readable: {error}"));
    let filtered = content
        .lines()
        .filter(|line| !line.ends_with(file_name))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(checksum_path, format!("{filtered}\n"))
        .unwrap_or_else(|error| panic!("checksum manifest should be writable: {error}"));
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
