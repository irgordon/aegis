use serde::{Deserialize, Serialize};

use super::NonEmptyString;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperConfigRef {
    pub wrapper_name: NonEmptyString,
    pub wrapper_version: NonEmptyString,
    pub target_system: NonEmptyString,
    pub config_reference: Option<NonEmptyString>,
    pub config_digest: Option<NonEmptyString>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RedactionProfileRef(pub NonEmptyString);

impl RedactionProfileRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemSchemaVersion(pub NonEmptyString);

impl ExternalSystemSchemaVersion {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrapperExecutionMode {
    ObserveOnly,
    Enforce,
    DryRun,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperExecutionContext {
    pub config: WrapperConfigRef,
    pub external_system_schema_version: ExternalSystemSchemaVersion,
    pub redaction_profile: RedactionProfileRef,
    pub execution_mode: WrapperExecutionMode,
    pub credential_injection_required: bool,
}
