mod request;
mod response;
mod schema;
mod support;

pub use request::{
    ActorType, CapabilityClass, OrchestratorReference, RequestActor, RequestedTool, ToolCallRequest,
};
pub use response::{
    GatewayError, GatewayStatus, PendingReference, PolicyProvenance, ReplayReference,
    ResponseDecision, ResponseMetadata, ToolCallResponse,
};
pub use schema::{NonEmptyString, SchemaVersion, Timestamp};
pub use support::{GatewayDecisionEvidence, SupportedTools};

use crate::policy::PolicyDecision;

pub struct Gateway;

impl Gateway {
    pub fn map_policy_decision(
        request: &ToolCallRequest,
        decision: PolicyDecision,
        metadata: ResponseMetadata,
    ) -> ToolCallResponse {
        match decision {
            PolicyDecision::Allow => map_allowed_response(request, metadata),
            PolicyDecision::Deny(denial) => map_denied_response(request, denial, metadata),
            PolicyDecision::PendingApproval(pending) => {
                map_pending_response(request, pending, metadata)
            }
        }
    }
}

pub fn entrypoint_status() -> &'static str {
    "AEGIS Gateway MVP scaffold is present; governed execution is not implemented."
}

fn map_allowed_response(request: &ToolCallRequest, metadata: ResponseMetadata) -> ToolCallResponse {
    build_response(
        request,
        GatewayStatus::Allowed,
        Some(ResponseDecision::Allow),
        None,
        None,
        None,
        metadata,
    )
}

fn map_denied_response(
    request: &ToolCallRequest,
    denial: crate::policy::PolicyDenial,
    metadata: ResponseMetadata,
) -> ToolCallResponse {
    build_response(
        request,
        GatewayStatus::Denied,
        Some(ResponseDecision::Deny),
        denial.reason_code,
        Some(denial.safe_message),
        None,
        metadata,
    )
}

fn map_pending_response(
    request: &ToolCallRequest,
    pending: crate::policy::PendingApprovalDecision,
    metadata: ResponseMetadata,
) -> ToolCallResponse {
    build_response(
        request,
        GatewayStatus::Pending,
        Some(ResponseDecision::PendingApproval),
        pending.reason_code,
        pending.safe_message,
        Some(pending.pending_reference),
        metadata,
    )
}

fn build_response(
    request: &ToolCallRequest,
    status: GatewayStatus,
    decision: Option<ResponseDecision>,
    reason_code: Option<String>,
    safe_message: Option<String>,
    pending_reference: Option<PendingReference>,
    metadata: ResponseMetadata,
) -> ToolCallResponse {
    ToolCallResponse {
        schema_version: SchemaVersion::V1,
        execution_id: metadata.execution_id,
        request_id: Some(request.request_id.clone()),
        status,
        decision,
        result: None,
        reason_code,
        safe_message,
        pending_reference,
        replay_reference: None,
        policy_provenance: metadata.policy_provenance,
        audit_record_id: metadata.audit_record_id,
        completed_at: metadata.completed_at,
    }
}
