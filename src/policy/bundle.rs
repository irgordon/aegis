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
pub enum ChecksumMetadataStatus {
    ChecksumMetadataPresent,
    ChecksumMetadataMissing,
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
    pub checksum_metadata_status: ChecksumMetadataStatus,
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
        self.verify_checksum_metadata(&manifest)?;
        self.verify_signature_metadata(&manifest)?;

        Ok(self.verified_metadata(manifest))
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

    fn verify_checksum_metadata(
        &self,
        manifest: &PolicyBundleManifest,
    ) -> PolicyBundleLoadResult<()> {
        for file_name in required_metadata_file_names("sha256") {
            let path = self.checksums_path().join(file_name);
            if !path.is_file() {
                return Err(self.rejected_with_manifest(
                    format!("checksum metadata missing: {}", path.display()),
                    manifest,
                ));
            }
        }

        Ok(())
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

    fn verified_metadata(&self, manifest: PolicyBundleManifest) -> PolicyBundleVerification {
        PolicyBundleVerification {
            bundle: Some(manifest.bundle),
            policy_version: Some(manifest.policy_version),
            risk_matrix_version: Some(manifest.risk_matrix_version),
            policy_hash: Some(manifest.policy_hash),
            manifest_path: path_string(self.manifest_path()),
            gateway_policy_path: path_string(self.gateway_policy_path()),
            risk_matrix_path: path_string(self.risk_matrix_path()),
            signature_metadata_status: SignatureMetadataStatus::SignatureMetadataPresent,
            checksum_metadata_status: ChecksumMetadataStatus::ChecksumMetadataPresent,
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
            checksum_metadata_status: ChecksumMetadataStatus::ChecksumMetadataMissing,
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
            checksum_metadata_status: ChecksumMetadataStatus::ChecksumMetadataMissing,
            verification_status: PolicyBundleVerificationStatus::Rejected,
            failure_reason: Some(reason.into()),
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
}

fn required_metadata_file_names(extension: &str) -> [String; 3] {
    [
        format!("{MANIFEST_FILE}.{extension}"),
        format!("{GATEWAY_POLICY_FILE}.{extension}"),
        format!("{RISK_MATRIX_FILE}.{extension}"),
    ]
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
