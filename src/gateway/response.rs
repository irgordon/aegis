use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{NonEmptyString, SchemaVersion, Timestamp};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GatewayStatus {
    Allowed,
    Denied,
    Pending,
    Failed,
    Canceled,
    Replayed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseDecision {
    Allow,
    Deny,
    PendingApproval,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GatewayError {
    GatewayExecutionUnavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCallResponse {
    pub schema_version: SchemaVersion,
    pub execution_id: NonEmptyString,
    pub request_id: Option<NonEmptyString>,
    pub status: GatewayStatus,
    pub decision: Option<ResponseDecision>,
    pub result: Option<BTreeMap<String, Value>>,
    pub reason_code: Option<String>,
    pub safe_message: Option<String>,
    pub pending_reference: Option<PendingReference>,
    pub replay_reference: Option<ReplayReference>,
    pub policy_provenance: PolicyProvenance,
    pub audit_record_id: NonEmptyString,
    pub completed_at: Timestamp,
}

impl ToolCallResponse {
    pub fn status(&self) -> &GatewayStatus {
        &self.status
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_ref().map(NonEmptyString::as_str)
    }

    pub fn safe_message(&self) -> Option<&str> {
        self.safe_message.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseMetadata {
    pub execution_id: NonEmptyString,
    pub policy_provenance: PolicyProvenance,
    pub audit_record_id: NonEmptyString,
    pub completed_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingReference {
    pub approval_id: NonEmptyString,
    pub expires_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplayReference {
    pub replay_token: NonEmptyString,
    pub attempt_number: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyProvenance {
    pub bundle_id: NonEmptyString,
    pub version: NonEmptyString,
    pub policy_hash: NonEmptyString,
    pub environment: NonEmptyString,
    pub signer_identity: Option<String>,
    pub activated_at: Option<Timestamp>,
}
