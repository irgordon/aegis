use serde::{Deserialize, Serialize};

use crate::gateway::{
    CapabilityClass, GatewayStatus, NonEmptyString, PolicyProvenance, ResponseDecision,
    SchemaVersion, Timestamp, ToolCallRequest, ToolCallResponse,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRecord {
    pub schema_version: SchemaVersion,
    pub audit_record_id: NonEmptyString,
    pub timestamp: Timestamp,
    pub event_type: AuditEventType,
    pub execution_id: NonEmptyString,
    pub run_id: Option<NonEmptyString>,
    pub task_id: Option<NonEmptyString>,
    pub action_id: Option<NonEmptyString>,
    pub tool_name: Option<NonEmptyString>,
    pub actor_identity: Option<NonEmptyString>,
    pub status: AuditStatus,
    pub reason_code: Option<String>,
    pub policy_provenance: PolicyProvenance,
    pub component: NonEmptyString,
    pub details: AuditRecordDetails,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventType {
    RequestReceived,
    ValidationResult,
    PolicyDecision,
    ApprovalDecision,
    WrapperResult,
    ExecutionResult,
    StateTransition,
    ReplayAttempt,
    PolicyActivation,
    PolicyRollback,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditStatus {
    Allowed,
    Denied,
    Pending,
    Failed,
    Canceled,
    Replayed,
    Recorded,
}

impl From<&GatewayStatus> for AuditStatus {
    fn from(status: &GatewayStatus) -> Self {
        match status {
            GatewayStatus::Allowed => Self::Allowed,
            GatewayStatus::Denied => Self::Denied,
            GatewayStatus::Pending => Self::Pending,
            GatewayStatus::Failed => Self::Failed,
            GatewayStatus::Canceled => Self::Canceled,
            GatewayStatus::Replayed => Self::Replayed,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRecordDetails {
    pub request_id: NonEmptyString,
    pub decision: Option<ResponseDecision>,
    pub capability_class: Option<CapabilityClass>,
}

impl AuditRecordDetails {
    pub fn from_gateway_decision(request: &ToolCallRequest, response: &ToolCallResponse) -> Self {
        Self {
            request_id: request.request_id.clone(),
            decision: response.decision.clone(),
            capability_class: request.tool.capability_class.clone(),
        }
    }
}
