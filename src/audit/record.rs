use serde::{Deserialize, Serialize};

use crate::{
    auth::{CredentialBoundary, CredentialInjectionResult, ExecutionAuthorization},
    error::AuditErrorReport,
    gateway::{
        ApprovalContext, CapabilityClass, ExecutionIdentityContext, GatewayStatus,
        IdempotencyContext, NonEmptyString, PolicyProvenance, ResponseDecision, SchemaVersion,
        Timestamp, ToolCallRequest, ToolCallResponse, WrapperExecutionContext,
        WrapperExecutionEvidence,
    },
    policy::{PolicyBundleVerification, PolicyEvaluation},
    state::ExecutionLifecycle,
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
    pub request_id: Option<NonEmptyString>,
    pub decision: Option<ResponseDecision>,
    pub capability_class: Option<CapabilityClass>,
    pub idempotency_context: Option<IdempotencyContext>,
    pub wrapper_context: Option<WrapperExecutionContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapper_execution_evidence: Option<WrapperExecutionEvidence>,
    pub execution_identity_context: Option<ExecutionIdentityContext>,
    pub approval_context: Option<ApprovalContext>,
    pub policy_bundle_verification: Option<PolicyBundleVerification>,
    pub policy_evaluation: Option<PolicyEvaluation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_authorization: Option<ExecutionAuthorization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_boundary: Option<CredentialBoundary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_injection: Option<CredentialInjectionResult>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_lifecycle: Option<ExecutionLifecycle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_report: Option<AuditErrorReport>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AuditRecordDetailContexts {
    pub idempotency_context: Option<IdempotencyContext>,
    pub wrapper_context: Option<WrapperExecutionContext>,
    pub wrapper_execution_evidence: Option<WrapperExecutionEvidence>,
    pub execution_identity_context: Option<ExecutionIdentityContext>,
    pub approval_context: Option<ApprovalContext>,
    pub policy_bundle_verification: Option<PolicyBundleVerification>,
    pub policy_evaluation: Option<PolicyEvaluation>,
    pub execution_authorization: Option<ExecutionAuthorization>,
    pub credential_boundary: Option<CredentialBoundary>,
    pub credential_injection: Option<CredentialInjectionResult>,
    pub execution_lifecycle: Option<ExecutionLifecycle>,
    pub error_report: Option<AuditErrorReport>,
}

impl AuditRecordDetails {
    pub fn from_gateway_decision(request: &ToolCallRequest, response: &ToolCallResponse) -> Self {
        Self::from_gateway_decision_with_idempotency(request, response, None)
    }

    pub fn from_gateway_decision_with_idempotency(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        idempotency_context: Option<IdempotencyContext>,
    ) -> Self {
        Self::from_gateway_decision_with_contexts(
            request,
            response,
            AuditRecordDetailContexts {
                idempotency_context,
                ..AuditRecordDetailContexts::default()
            },
        )
    }

    pub fn from_gateway_decision_with_contexts(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        contexts: AuditRecordDetailContexts,
    ) -> Self {
        Self {
            request_id: Some(request.request_id.clone()),
            decision: response.decision.clone(),
            capability_class: request.tool.capability_class.clone(),
            idempotency_context: contexts.idempotency_context,
            wrapper_context: contexts.wrapper_context,
            wrapper_execution_evidence: contexts.wrapper_execution_evidence,
            execution_identity_context: contexts.execution_identity_context,
            approval_context: contexts.approval_context,
            policy_bundle_verification: contexts.policy_bundle_verification,
            policy_evaluation: contexts.policy_evaluation,
            execution_authorization: contexts.execution_authorization,
            credential_boundary: contexts.credential_boundary,
            credential_injection: contexts.credential_injection,
            execution_lifecycle: contexts.execution_lifecycle,
            error_report: contexts.error_report,
        }
    }

    pub fn from_response(response: &ToolCallResponse) -> Self {
        Self {
            request_id: response.request_id.clone(),
            decision: response.decision.clone(),
            capability_class: None,
            idempotency_context: None,
            wrapper_context: None,
            wrapper_execution_evidence: None,
            execution_identity_context: None,
            approval_context: None,
            policy_bundle_verification: None,
            policy_evaluation: None,
            execution_authorization: None,
            credential_boundary: None,
            credential_injection: None,
            execution_lifecycle: None,
            error_report: None,
        }
    }
}
