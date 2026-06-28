use serde::{Deserialize, Serialize};

use super::{NonEmptyString, ToolCallRequest};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IdempotencyKey(pub NonEmptyString);

impl IdempotencyKey {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Read,
    Mutation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdempotencyContext {
    pub key: IdempotencyKey,
    pub execution_id: NonEmptyString,
    pub tool_call_hash: NonEmptyString,
    pub target_system: NonEmptyString,
    pub operation_type: OperationType,
    pub policy_bundle_version: NonEmptyString,
}

impl ToolCallRequest {
    pub fn carries_mutation_risk(&self) -> bool {
        self.tool
            .capability_class
            .as_ref()
            .is_some_and(super::CapabilityClass::is_mutation_capable)
    }
}

impl super::CapabilityClass {
    pub fn is_mutation_capable(&self) -> bool {
        matches!(self, Self::L1 | Self::L2 | Self::L3)
    }
}
