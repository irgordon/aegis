use serde::{Deserialize, Serialize};

use super::NonEmptyString;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionIdentity(pub NonEmptyString);

impl ExecutionIdentity {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionNonceRef(pub NonEmptyString);

impl ExecutionNonceRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionIdentitySource {
    CallerSupplied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionIdentityBinding {
    pub orchestrator_id: NonEmptyString,
    pub workflow_id: NonEmptyString,
    pub tool_call_id: NonEmptyString,
    pub policy_bundle_version: NonEmptyString,
    pub nonce_ref: ExecutionNonceRef,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionIdentityContext {
    pub execution_identity: ExecutionIdentity,
    pub binding: ExecutionIdentityBinding,
    pub source: ExecutionIdentitySource,
}
