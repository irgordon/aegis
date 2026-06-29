use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::gateway::{NonEmptyString, PolicyProvenance, Timestamp};

const GATEWAY_POLICY_FILE: &str = "gateway_policy.yaml";
const RISK_MATRIX_FILE: &str = "risk_matrix.yaml";
const MANIFEST_FILE: &str = "manifest.yaml";
const SIGNATURES_DIR: &str = "signatures";
const CHECKSUMS_DIR: &str = "checksums";
const CHECKSUM_MANIFEST_FILE: &str = "SHA256SUMS";
const SHA256_HEX_LENGTH: usize = 64;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PolicyBundleRef(pub NonEmptyString);

impl PolicyBundleRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PolicyVersion(pub NonEmptyString);

impl PolicyVersion {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RiskMatrixVersion(pub NonEmptyString);

impl RiskMatrixVersion {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BundleDigestRef(pub NonEmptyString);

impl BundleDigestRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SignatureRef(pub NonEmptyString);

impl SignatureRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChecksumRef(pub NonEmptyString);

impl ChecksumRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumAlgorithm {
    Sha256,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChecksumDigest(pub NonEmptyString);

impl ChecksumDigest {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyBundleVerificationStatus {
    SignatureCryptographicVerificationNotImplemented,
    Rejected,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureMetadataStatus {
    SignatureMetadataPresent,
    SignatureMetadataMissing,
    SignatureCryptographicVerificationNotImplemented,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureVerificationStatus {
    SignatureCryptographicVerificationNotImplemented,
    SignatureMetadataMissing,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumMetadataStatus {
    ChecksumMetadataPresent,
    ChecksumMetadataMissing,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumVerificationStatus {
    Verified,
    MetadataMissing,
    EntryMissing,
    Mismatch,
    MalformedMetadata,
    FileReadFailed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumVerificationFailure {
    MetadataMissing,
    EntryMissing,
    DigestMismatch,
    MalformedMetadata,
    FileReadFailed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyBundleChecksumEntry {
    pub file_path: String,
    pub algorithm: ChecksumAlgorithm,
    pub expected_digest: Option<ChecksumDigest>,
    pub actual_digest: Option<ChecksumDigest>,
    pub verification_status: ChecksumVerificationStatus,
    pub failure_reason: Option<ChecksumVerificationFailure>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyBundleManifest {
    pub bundle: PolicyBundleRef,
    pub policy_version: PolicyVersion,
    pub risk_matrix_version: RiskMatrixVersion,
    pub policy_hash: BundleDigestRef,
    pub target_environment: NonEmptyString,
    pub signer_identity: Option<NonEmptyString>,
    pub created_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyBundleVerification {
    pub bundle: Option<PolicyBundleRef>,
    pub policy_version: Option<PolicyVersion>,
    pub risk_matrix_version: Option<RiskMatrixVersion>,
    pub policy_hash: Option<BundleDigestRef>,
    pub manifest_path: String,
    pub gateway_policy_path: String,
    pub risk_matrix_path: String,
    pub signature_metadata_status: SignatureMetadataStatus,
    pub signature_verification_status: SignatureVerificationStatus,
    pub checksum_metadata_status: ChecksumMetadataStatus,
    pub checksum_verification_status: ChecksumVerificationStatus,
    pub checksum_entries: Vec<PolicyBundleChecksumEntry>,
    pub verification_status: PolicyBundleVerificationStatus,
    pub failure_reason: Option<String>,
}

impl PolicyBundleVerification {
    pub fn is_verified(&self) -> bool {
        self.verification_status
            == PolicyBundleVerificationStatus::SignatureCryptographicVerificationNotImplemented
    }

    pub fn policy_provenance(&self) -> PolicyProvenance {
        let bundle_id = self
            .bundle
            .as_ref()
            .map(PolicyBundleRef::as_str)
            .unwrap_or("unverified_policy_bundle");
        let version = self
            .policy_version
            .as_ref()
            .map(PolicyVersion::as_str)
            .unwrap_or("unverified");
        let policy_hash = self
            .policy_hash
            .as_ref()
            .map(BundleDigestRef::as_str)
            .unwrap_or("unverified");

        policy_provenance_from_parts(bundle_id, version, policy_hash)
    }
}

pub type PolicyBundleLoadResult<T> = Result<T, Box<PolicyBundleVerification>>;

pub fn load_policy_bundle(
    path: impl AsRef<Path>,
) -> PolicyBundleLoadResult<PolicyBundleVerification> {
    PolicyBundleLoader::new(path.as_ref()).load()
}

struct PolicyBundleLoader {
    root: PathBuf,
}

impl PolicyBundleLoader {
    fn new(root: &Path) -> Self {
        Self {
            root: root.to_path_buf(),
        }
    }

    fn load(&self) -> PolicyBundleLoadResult<PolicyBundleVerification> {
        self.verify_required_paths()?;
        let manifest = self.read_manifest()?;
        let risk_matrix_version = self.read_risk_matrix_version(&manifest)?;
        self.verify_version_binding(&manifest, &risk_matrix_version)?;
        let checksum_entries = self.verify_checksums(&manifest)?;
        self.verify_signature_metadata(&manifest)?;

        Ok(self.verified_metadata(manifest, checksum_entries))
    }

    fn verify_required_paths(&self) -> PolicyBundleLoadResult<()> {
        for path in [
            self.manifest_path(),
            self.gateway_policy_path(),
            self.risk_matrix_path(),
        ] {
            if !path.is_file() {
                return Err(self.rejected(format!("required file missing: {}", path.display())));
            }
        }

        for path in [self.signatures_path(), self.checksums_path()] {
            if !path.is_dir() {
                return Err(
                    self.rejected(format!("required directory missing: {}", path.display()))
                );
            }
        }

        Ok(())
    }

    fn read_manifest(&self) -> PolicyBundleLoadResult<PolicyBundleManifest> {
        let metadata = self.read_metadata_file(&self.manifest_path())?;
        let bundle_id =
            required_non_empty(&metadata, "bundle_id").map_err(|reason| self.rejected(reason))?;
        let policy_version = required_non_empty(&metadata, "policy_version")
            .map_err(|reason| self.rejected(reason))?;
        let risk_matrix_version = required_non_empty(&metadata, "risk_matrix_version")
            .map_err(|reason| self.rejected(reason))?;
        let policy_hash =
            required_non_empty(&metadata, "policy_hash").map_err(|reason| self.rejected(reason))?;
        let target_environment = required_non_empty(&metadata, "target_environment")
            .map_err(|reason| self.rejected(reason))?;
        let signer_identity = optional_non_empty(&metadata, "signer_identity")
            .map_err(|reason| self.rejected(reason))?;
        let created_at =
            optional_timestamp(&metadata, "created_at").map_err(|reason| self.rejected(reason))?;
        let manifest = PolicyBundleManifest {
            bundle: PolicyBundleRef(bundle_id),
            policy_version: PolicyVersion(policy_version),
            risk_matrix_version: RiskMatrixVersion(risk_matrix_version),
            policy_hash: BundleDigestRef(policy_hash),
            target_environment,
            signer_identity,
            created_at,
        };

        Ok(manifest)
    }

    fn read_risk_matrix_version(
        &self,
        manifest: &PolicyBundleManifest,
    ) -> PolicyBundleLoadResult<RiskMatrixVersion> {
        let metadata = self
            .read_metadata_file(&self.risk_matrix_path())
            .map_err(|_| self.rejected("risk matrix metadata could not be read"))?;

        required_non_empty(&metadata, "risk_matrix_version")
            .map(RiskMatrixVersion)
            .map_err(|reason| self.rejected_with_manifest(reason, manifest))
    }

    fn verify_version_binding(
        &self,
        manifest: &PolicyBundleManifest,
        risk_matrix_version: &RiskMatrixVersion,
    ) -> PolicyBundleLoadResult<()> {
        if manifest.risk_matrix_version != *risk_matrix_version {
            return Err(self.rejected_with_manifest(
                "manifest risk matrix version does not match risk matrix version".to_string(),
                manifest,
            ));
        }

        Ok(())
    }

    fn verify_checksums(
        &self,
        manifest: &PolicyBundleManifest,
    ) -> PolicyBundleLoadResult<Vec<PolicyBundleChecksumEntry>> {
        let checksum_manifest = self.checksum_manifest_path();
        if !checksum_manifest.is_file() {
            return Err(self.rejected_with_manifest_and_checksums(
                format!("checksum manifest missing: {}", checksum_manifest.display()),
                manifest,
                ChecksumVerificationStatus::MetadataMissing,
                Vec::new(),
            ));
        }

        let checksums = self.read_checksum_manifest(manifest)?;
        let mut entries = Vec::new();

        for file_name in required_bundle_file_names() {
            let Some(expected_digest) = checksums.get(file_name).cloned() else {
                entries.push(missing_checksum_entry(file_name));
                return Err(self.rejected_with_manifest_and_checksums(
                    format!("checksum entry missing: {file_name}"),
                    manifest,
                    ChecksumVerificationStatus::EntryMissing,
                    entries,
                ));
            };

            let file_path = self.root.join(file_name);
            let Ok(actual_digest) = sha256_file(&file_path) else {
                entries.push(file_read_failed_checksum_entry(file_name, expected_digest));
                return Err(self.rejected_with_manifest_and_checksums(
                    format!("checksum file read failed: {}", file_path.display()),
                    manifest,
                    ChecksumVerificationStatus::FileReadFailed,
                    entries,
                ));
            };

            let entry =
                verified_or_mismatched_checksum_entry(file_name, expected_digest, actual_digest);
            if entry.verification_status != ChecksumVerificationStatus::Verified {
                entries.push(entry);
                return Err(self.rejected_with_manifest_and_checksums(
                    format!("checksum mismatch: {file_name}"),
                    manifest,
                    ChecksumVerificationStatus::Mismatch,
                    entries,
                ));
            }

            entries.push(entry);
        }

        Ok(entries)
    }

    fn verify_signature_metadata(
        &self,
        manifest: &PolicyBundleManifest,
    ) -> PolicyBundleLoadResult<()> {
        for file_name in required_metadata_file_names("sig") {
            let path = self.signatures_path().join(file_name);
            if !path.is_file() {
                return Err(self.rejected_with_manifest(
                    format!("signature metadata missing: {}", path.display()),
                    manifest,
                ));
            }
        }

        Ok(())
    }

    fn verified_metadata(
        &self,
        manifest: PolicyBundleManifest,
        checksum_entries: Vec<PolicyBundleChecksumEntry>,
    ) -> PolicyBundleVerification {
        PolicyBundleVerification {
            bundle: Some(manifest.bundle),
            policy_version: Some(manifest.policy_version),
            risk_matrix_version: Some(manifest.risk_matrix_version),
            policy_hash: Some(manifest.policy_hash),
            manifest_path: path_string(self.manifest_path()),
            gateway_policy_path: path_string(self.gateway_policy_path()),
            risk_matrix_path: path_string(self.risk_matrix_path()),
            signature_metadata_status: SignatureMetadataStatus::SignatureMetadataPresent,
            signature_verification_status:
                SignatureVerificationStatus::SignatureCryptographicVerificationNotImplemented,
            checksum_metadata_status: ChecksumMetadataStatus::ChecksumMetadataPresent,
            checksum_verification_status: ChecksumVerificationStatus::Verified,
            checksum_entries,
            verification_status:
                PolicyBundleVerificationStatus::SignatureCryptographicVerificationNotImplemented,
            failure_reason: None,
        }
    }

    fn rejected(&self, reason: impl Into<String>) -> Box<PolicyBundleVerification> {
        Box::new(PolicyBundleVerification {
            bundle: None,
            policy_version: None,
            risk_matrix_version: None,
            policy_hash: None,
            manifest_path: path_string(self.manifest_path()),
            gateway_policy_path: path_string(self.gateway_policy_path()),
            risk_matrix_path: path_string(self.risk_matrix_path()),
            signature_metadata_status: SignatureMetadataStatus::SignatureMetadataMissing,
            signature_verification_status:
                SignatureVerificationStatus::SignatureCryptographicVerificationNotImplemented,
            checksum_metadata_status: ChecksumMetadataStatus::ChecksumMetadataMissing,
            checksum_verification_status: ChecksumVerificationStatus::MetadataMissing,
            checksum_entries: Vec::new(),
            verification_status: PolicyBundleVerificationStatus::Rejected,
            failure_reason: Some(reason.into()),
        })
    }

    fn rejected_with_manifest(
        &self,
        reason: impl Into<String>,
        manifest: &PolicyBundleManifest,
    ) -> Box<PolicyBundleVerification> {
        Box::new(PolicyBundleVerification {
            bundle: Some(manifest.bundle.clone()),
            policy_version: Some(manifest.policy_version.clone()),
            risk_matrix_version: Some(manifest.risk_matrix_version.clone()),
            policy_hash: Some(manifest.policy_hash.clone()),
            manifest_path: path_string(self.manifest_path()),
            gateway_policy_path: path_string(self.gateway_policy_path()),
            risk_matrix_path: path_string(self.risk_matrix_path()),
            signature_metadata_status: SignatureMetadataStatus::SignatureMetadataMissing,
            signature_verification_status:
                SignatureVerificationStatus::SignatureCryptographicVerificationNotImplemented,
            checksum_metadata_status: ChecksumMetadataStatus::ChecksumMetadataMissing,
            checksum_verification_status: ChecksumVerificationStatus::MetadataMissing,
            checksum_entries: Vec::new(),
            verification_status: PolicyBundleVerificationStatus::Rejected,
            failure_reason: Some(reason.into()),
        })
    }

    fn rejected_with_manifest_and_checksums(
        &self,
        reason: impl Into<String>,
        manifest: &PolicyBundleManifest,
        checksum_verification_status: ChecksumVerificationStatus,
        checksum_entries: Vec<PolicyBundleChecksumEntry>,
    ) -> Box<PolicyBundleVerification> {
        Box::new(PolicyBundleVerification {
            bundle: Some(manifest.bundle.clone()),
            policy_version: Some(manifest.policy_version.clone()),
            risk_matrix_version: Some(manifest.risk_matrix_version.clone()),
            policy_hash: Some(manifest.policy_hash.clone()),
            manifest_path: path_string(self.manifest_path()),
            gateway_policy_path: path_string(self.gateway_policy_path()),
            risk_matrix_path: path_string(self.risk_matrix_path()),
            signature_metadata_status: SignatureMetadataStatus::SignatureMetadataMissing,
            signature_verification_status:
                SignatureVerificationStatus::SignatureCryptographicVerificationNotImplemented,
            checksum_metadata_status: match checksum_verification_status {
                ChecksumVerificationStatus::MetadataMissing
                | ChecksumVerificationStatus::MalformedMetadata => {
                    ChecksumMetadataStatus::ChecksumMetadataMissing
                }
                _ => ChecksumMetadataStatus::ChecksumMetadataPresent,
            },
            checksum_verification_status,
            checksum_entries,
            verification_status: PolicyBundleVerificationStatus::Rejected,
            failure_reason: Some(reason.into()),
        })
    }

    fn read_checksum_manifest(
        &self,
        manifest: &PolicyBundleManifest,
    ) -> PolicyBundleLoadResult<BTreeMap<String, ChecksumDigest>> {
        let content = fs::read_to_string(self.checksum_manifest_path()).map_err(|error| {
            self.rejected_with_manifest_and_checksums(
                format!("checksum manifest read failed: {error}"),
                manifest,
                ChecksumVerificationStatus::FileReadFailed,
                Vec::new(),
            )
        })?;

        parse_sha256sums(&content).map_err(|reason| {
            self.rejected_with_manifest_and_checksums(
                format!("checksum manifest malformed: {reason}"),
                manifest,
                ChecksumVerificationStatus::MalformedMetadata,
                Vec::new(),
            )
        })
    }

    fn read_metadata_file(&self, path: &Path) -> PolicyBundleLoadResult<BTreeMap<String, String>> {
        fs::read_to_string(path)
            .map_err(|error| self.rejected(format!("failed to read {}: {error}", path.display())))
            .and_then(|content| {
                parse_flat_yaml_metadata(&content).map_err(|reason| {
                    self.rejected(format!("failed to parse {}: {reason}", path.display()))
                })
            })
    }

    fn manifest_path(&self) -> PathBuf {
        self.root.join(MANIFEST_FILE)
    }

    fn gateway_policy_path(&self) -> PathBuf {
        self.root.join(GATEWAY_POLICY_FILE)
    }

    fn risk_matrix_path(&self) -> PathBuf {
        self.root.join(RISK_MATRIX_FILE)
    }

    fn signatures_path(&self) -> PathBuf {
        self.root.join(SIGNATURES_DIR)
    }

    fn checksums_path(&self) -> PathBuf {
        self.root.join(CHECKSUMS_DIR)
    }

    fn checksum_manifest_path(&self) -> PathBuf {
        self.checksums_path().join(CHECKSUM_MANIFEST_FILE)
    }
}

fn required_metadata_file_names(extension: &str) -> [String; 3] {
    required_bundle_file_names().map(|file_name| format!("{file_name}.{extension}"))
}

fn required_bundle_file_names() -> [&'static str; 3] {
    [MANIFEST_FILE, GATEWAY_POLICY_FILE, RISK_MATRIX_FILE]
}

fn parse_sha256sums(content: &str) -> Result<BTreeMap<String, ChecksumDigest>, String> {
    let mut checksums = BTreeMap::new();

    for line in content.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (digest, file_path) = split_sha256sum_line(line)?;
        let digest = non_empty_string(digest)
            .map_err(|_| format!("checksum digest is empty: {file_path}"))?;
        checksums.insert(file_path.to_string(), ChecksumDigest(digest));
    }

    Ok(checksums)
}

fn split_sha256sum_line(line: &str) -> Result<(&str, &str), String> {
    let mut parts = line.split_whitespace();
    let digest = parts
        .next()
        .ok_or_else(|| "checksum digest missing".to_string())?;
    let file_path = parts
        .next()
        .ok_or_else(|| "checksum file path missing".to_string())?;

    if parts.next().is_some() {
        return Err("checksum line has too many fields".to_string());
    }

    if !is_sha256_hex_digest(digest) {
        return Err(format!("checksum digest is not SHA-256 hex: {digest}"));
    }

    Ok((digest, file_path))
}

fn is_sha256_hex_digest(value: &str) -> bool {
    value.len() == SHA256_HEX_LENGTH && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn missing_checksum_entry(file_name: &str) -> PolicyBundleChecksumEntry {
    PolicyBundleChecksumEntry {
        file_path: file_name.to_string(),
        algorithm: ChecksumAlgorithm::Sha256,
        expected_digest: None,
        actual_digest: None,
        verification_status: ChecksumVerificationStatus::EntryMissing,
        failure_reason: Some(ChecksumVerificationFailure::EntryMissing),
    }
}

fn file_read_failed_checksum_entry(
    file_name: &str,
    expected_digest: ChecksumDigest,
) -> PolicyBundleChecksumEntry {
    PolicyBundleChecksumEntry {
        file_path: file_name.to_string(),
        algorithm: ChecksumAlgorithm::Sha256,
        expected_digest: Some(expected_digest),
        actual_digest: None,
        verification_status: ChecksumVerificationStatus::FileReadFailed,
        failure_reason: Some(ChecksumVerificationFailure::FileReadFailed),
    }
}

fn verified_or_mismatched_checksum_entry(
    file_name: &str,
    expected_digest: ChecksumDigest,
    actual_digest: ChecksumDigest,
) -> PolicyBundleChecksumEntry {
    let matches = expected_digest == actual_digest;

    PolicyBundleChecksumEntry {
        file_path: file_name.to_string(),
        algorithm: ChecksumAlgorithm::Sha256,
        expected_digest: Some(expected_digest),
        actual_digest: Some(actual_digest),
        verification_status: if matches {
            ChecksumVerificationStatus::Verified
        } else {
            ChecksumVerificationStatus::Mismatch
        },
        failure_reason: if matches {
            None
        } else {
            Some(ChecksumVerificationFailure::DigestMismatch)
        },
    }
}

fn sha256_file(path: &Path) -> Result<ChecksumDigest, std::io::Error> {
    let content = fs::read(path)?;
    let digest = sha256_hex(&content);

    non_empty_string(&digest)
        .map(ChecksumDigest)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

fn parse_flat_yaml_metadata(content: &str) -> Result<BTreeMap<String, String>, String> {
    let mut metadata = BTreeMap::new();

    for line in content.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = line
            .split_once(':')
            .ok_or_else(|| format!("metadata line is not key-value: {line}"))?;
        metadata.insert(key.trim().to_string(), clean_yaml_scalar(value));
    }

    Ok(metadata)
}

fn clean_yaml_scalar(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

fn required_non_empty(
    metadata: &BTreeMap<String, String>,
    key: &str,
) -> Result<NonEmptyString, String> {
    metadata
        .get(key)
        .ok_or_else(|| format!("required metadata missing: {key}"))
        .and_then(|value| non_empty_string(value).map_err(|_| format!("metadata is empty: {key}")))
}

fn optional_non_empty(
    metadata: &BTreeMap<String, String>,
    key: &str,
) -> Result<Option<NonEmptyString>, String> {
    metadata
        .get(key)
        .map(|value| non_empty_string(value).map_err(|_| format!("metadata is empty: {key}")))
        .transpose()
}

fn optional_timestamp(
    metadata: &BTreeMap<String, String>,
    key: &str,
) -> Result<Option<Timestamp>, String> {
    metadata
        .get(key)
        .map(|value| timestamp(value).map_err(|_| format!("metadata timestamp is invalid: {key}")))
        .transpose()
}

fn non_empty_string(value: &str) -> Result<NonEmptyString, serde_json::Error> {
    serde_json::from_value(serde_json::Value::String(value.to_string()))
}

fn timestamp(value: &str) -> Result<Timestamp, serde_json::Error> {
    serde_json::from_value(serde_json::Value::String(value.to_string()))
}

fn sha256_hex(input: &[u8]) -> String {
    const H0: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut state = H0;
    let padded = sha256_padded_message(input);

    for chunk in padded.chunks_exact(64) {
        let schedule = sha256_message_schedule(chunk);
        let mut working = state;

        for index in 0..64 {
            let t1 = working[7]
                .wrapping_add(big_sigma1(working[4]))
                .wrapping_add(ch(working[4], working[5], working[6]))
                .wrapping_add(K[index])
                .wrapping_add(schedule[index]);
            let t2 = big_sigma0(working[0]).wrapping_add(maj(working[0], working[1], working[2]));

            working[7] = working[6];
            working[6] = working[5];
            working[5] = working[4];
            working[4] = working[3].wrapping_add(t1);
            working[3] = working[2];
            working[2] = working[1];
            working[1] = working[0];
            working[0] = t1.wrapping_add(t2);
        }

        for index in 0..8 {
            state[index] = state[index].wrapping_add(working[index]);
        }
    }

    state
        .iter()
        .map(|word| format!("{word:08x}"))
        .collect::<String>()
}

fn sha256_padded_message(input: &[u8]) -> Vec<u8> {
    let bit_len = (input.len() as u64) * 8;
    let mut padded = input.to_vec();
    padded.push(0x80);

    while (padded.len() % 64) != 56 {
        padded.push(0);
    }

    padded.extend_from_slice(&bit_len.to_be_bytes());
    padded
}

fn sha256_message_schedule(chunk: &[u8]) -> [u32; 64] {
    let mut schedule = [0_u32; 64];

    for (index, word) in schedule.iter_mut().enumerate().take(16) {
        let start = index * 4;
        *word = u32::from_be_bytes([
            chunk[start],
            chunk[start + 1],
            chunk[start + 2],
            chunk[start + 3],
        ]);
    }

    for index in 16..64 {
        schedule[index] = small_sigma1(schedule[index - 2])
            .wrapping_add(schedule[index - 7])
            .wrapping_add(small_sigma0(schedule[index - 15]))
            .wrapping_add(schedule[index - 16]);
    }

    schedule
}

fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn big_sigma0(value: u32) -> u32 {
    value.rotate_right(2) ^ value.rotate_right(13) ^ value.rotate_right(22)
}

fn big_sigma1(value: u32) -> u32 {
    value.rotate_right(6) ^ value.rotate_right(11) ^ value.rotate_right(25)
}

fn small_sigma0(value: u32) -> u32 {
    value.rotate_right(7) ^ value.rotate_right(18) ^ (value >> 3)
}

fn small_sigma1(value: u32) -> u32 {
    value.rotate_right(17) ^ value.rotate_right(19) ^ (value >> 10)
}

fn policy_provenance_from_parts(
    bundle_id: &str,
    version: &str,
    policy_hash: &str,
) -> PolicyProvenance {
    serde_json::from_value(serde_json::json!({
        "bundle_id": bundle_id,
        "version": version,
        "policy_hash": policy_hash,
        "environment": "local",
        "signer_identity": "local-policy-bundle-loader",
        "activated_at": "2026-06-28T00:00:00Z"
    }))
    .unwrap_or_else(|error| panic!("static policy provenance should parse: {error}"))
}

fn path_string(path: PathBuf) -> String {
    path.to_string_lossy().into_owned()
}
