use serde::{Deserialize, Serialize};

use super::{NonEmptyString, Timestamp};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApprovalTokenRef(pub NonEmptyString);

impl ApprovalTokenRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApprovalRequirementRef(pub NonEmptyString);

impl ApprovalRequirementRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ApprovalTtl(pub NonEmptyString);

impl ApprovalTtl {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalTokenState {
    Active,
    Revoked,
    Expired,
    Used,
    ContextMismatch,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalContextSource {
    CallerSupplied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalExpiration {
    pub ttl: Option<ApprovalTtl>,
    pub expires_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalBinding {
    pub execution_id: NonEmptyString,
    pub tool_call_hash: NonEmptyString,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApprovalContext {
    pub approval_token_ref: ApprovalTokenRef,
    pub binding: ApprovalBinding,
    pub status: ApprovalStatus,
    pub token_state: ApprovalTokenState,
    pub expiration: ApprovalExpiration,
    pub approval_requirement_ref: ApprovalRequirementRef,
    pub approver_ref: Option<NonEmptyString>,
    pub approver_role_ref: Option<NonEmptyString>,
    pub break_glass_ref: Option<NonEmptyString>,
    pub source: ApprovalContextSource,
}
