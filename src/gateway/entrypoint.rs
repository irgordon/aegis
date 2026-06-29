use crate::{
    audit::{AuditRecord, AuditRecordBuilder, AuditRecordMetadata, GatewayAuditContexts},
    policy::{PolicyAdapterError, PolicyDecision, PolicyDecisionAdapter, PolicyDenial},
};

use super::{
    Gateway, GatewayStatus, GatewayValidationOutcome, ResponseMetadata, SupportedTools,
    ToolCallResponse,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GatewayEntrypointContext {
    pub supported_tools: SupportedTools,
    pub policy_decision: PolicyDecision,
    pub response_metadata: ResponseMetadata,
    pub audit_metadata: AuditRecordMetadata,
    pub idempotency_context: Option<super::IdempotencyContext>,
    pub wrapper_context: Option<super::WrapperExecutionContext>,
    pub execution_identity_context: Option<super::ExecutionIdentityContext>,
    pub approval_context: Option<super::ApprovalContext>,
    pub policy_bundle_verification: Option<crate::policy::PolicyBundleVerification>,
}

pub struct GatewayPolicyAdapterContext<'a> {
    pub supported_tools: SupportedTools,
    pub policy_adapter: &'a dyn PolicyDecisionAdapter,
    pub response_metadata: ResponseMetadata,
    pub audit_metadata: AuditRecordMetadata,
    pub idempotency_context: Option<super::IdempotencyContext>,
    pub wrapper_context: Option<super::WrapperExecutionContext>,
    pub execution_identity_context: Option<super::ExecutionIdentityContext>,
    pub approval_context: Option<super::ApprovalContext>,
    pub policy_bundle_verification: Option<crate::policy::PolicyBundleVerification>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GatewayEntrypointSummary {
    MalformedRequestDenied,
    UnsupportedToolDenied,
    PolicyDecisionMapped,
    PolicyAdapterFailedClosed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GatewayEntrypointResult {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
    pub summary: GatewayEntrypointSummary,
    pub idempotency_context: Option<super::IdempotencyContext>,
    pub wrapper_context: Option<super::WrapperExecutionContext>,
    pub execution_identity_context: Option<super::ExecutionIdentityContext>,
    pub approval_context: Option<super::ApprovalContext>,
    pub policy_bundle_verification: Option<crate::policy::PolicyBundleVerification>,
}

struct GatewayDecisionMappingContext {
    response_metadata: ResponseMetadata,
    audit_metadata: AuditRecordMetadata,
    supplied_idempotency_context: Option<super::IdempotencyContext>,
    supplied_wrapper_context: Option<super::WrapperExecutionContext>,
    supplied_execution_identity_context: Option<super::ExecutionIdentityContext>,
    supplied_approval_context: Option<super::ApprovalContext>,
    supplied_policy_bundle_verification: Option<crate::policy::PolicyBundleVerification>,
    summary: GatewayEntrypointSummary,
}

impl Gateway {
    pub fn process_entrypoint_request(
        input: &str,
        context: GatewayEntrypointContext,
    ) -> GatewayEntrypointResult {
        match validate_entrypoint_request(input, &context) {
            GatewayValidationOutcome::Accepted(request) => map_supported_request(*request, context),
            GatewayValidationOutcome::Denied(evidence) => denied_entrypoint_result(*evidence),
        }
    }

    pub fn process_entrypoint_request_with_policy_adapter(
        input: &str,
        context: GatewayPolicyAdapterContext<'_>,
    ) -> GatewayEntrypointResult {
        match validate_adapter_entrypoint_request(input, &context) {
            GatewayValidationOutcome::Accepted(request) => {
                map_adapter_supported_request(*request, context)
            }
            GatewayValidationOutcome::Denied(evidence) => denied_entrypoint_result(*evidence),
        }
    }
}

fn validate_entrypoint_request(
    input: &str,
    context: &GatewayEntrypointContext,
) -> GatewayValidationOutcome {
    Gateway::validate_request_json(
        input,
        &context.supported_tools,
        context.response_metadata.clone(),
        context.audit_metadata.clone(),
    )
}

fn validate_adapter_entrypoint_request(
    input: &str,
    context: &GatewayPolicyAdapterContext<'_>,
) -> GatewayValidationOutcome {
    Gateway::validate_request_json(
        input,
        &context.supported_tools,
        context.response_metadata.clone(),
        context.audit_metadata.clone(),
    )
}

fn map_supported_request(
    request: super::ToolCallRequest,
    context: GatewayEntrypointContext,
) -> GatewayEntrypointResult {
    map_policy_decision_result(
        request,
        context.policy_decision,
        GatewayDecisionMappingContext {
            response_metadata: context.response_metadata,
            audit_metadata: context.audit_metadata,
            supplied_idempotency_context: context.idempotency_context,
            supplied_wrapper_context: context.wrapper_context,
            supplied_execution_identity_context: context.execution_identity_context,
            supplied_approval_context: context.approval_context,
            supplied_policy_bundle_verification: context.policy_bundle_verification,
            summary: GatewayEntrypointSummary::PolicyDecisionMapped,
        },
    )
}

fn map_adapter_supported_request(
    request: super::ToolCallRequest,
    context: GatewayPolicyAdapterContext<'_>,
) -> GatewayEntrypointResult {
    let (decision, summary) = adapter_decision(&request, context.policy_adapter);

    map_policy_decision_result(
        request,
        decision,
        GatewayDecisionMappingContext {
            response_metadata: context.response_metadata,
            audit_metadata: context.audit_metadata,
            supplied_idempotency_context: context.idempotency_context,
            supplied_wrapper_context: context.wrapper_context,
            supplied_execution_identity_context: context.execution_identity_context,
            supplied_approval_context: context.approval_context,
            supplied_policy_bundle_verification: context.policy_bundle_verification,
            summary,
        },
    )
}

fn map_policy_decision_result(
    request: super::ToolCallRequest,
    policy_decision: PolicyDecision,
    context: GatewayDecisionMappingContext,
) -> GatewayEntrypointResult {
    let idempotency_context =
        idempotency_context_for_request(&request, &context.supplied_idempotency_context);
    let response =
        Gateway::map_policy_decision(&request, policy_decision, context.response_metadata);
    let audit_record = AuditRecordBuilder::build_gateway_decision_record_with_contexts(
        &request,
        &response,
        context.audit_metadata,
        GatewayAuditContexts {
            idempotency_context: idempotency_context.clone(),
            wrapper_context: context.supplied_wrapper_context.clone(),
            wrapper_execution_evidence: None,
            execution_identity_context: context.supplied_execution_identity_context.clone(),
            approval_context: context.supplied_approval_context.clone(),
            policy_bundle_verification: context.supplied_policy_bundle_verification.clone(),
            policy_evaluation: None,
            execution_authorization: None,
            credential_boundary: None,
            credential_injection: None,
            execution_lifecycle: None,
            error_report: None,
        },
    );

    GatewayEntrypointResult {
        response,
        audit_record,
        summary: context.summary,
        idempotency_context,
        wrapper_context: context.supplied_wrapper_context,
        execution_identity_context: context.supplied_execution_identity_context,
        approval_context: context.supplied_approval_context,
        policy_bundle_verification: context.supplied_policy_bundle_verification,
    }
}

fn denied_entrypoint_result(evidence: super::GatewayDecisionEvidence) -> GatewayEntrypointResult {
    let summary = denied_summary(&evidence.response);

    GatewayEntrypointResult {
        response: evidence.response,
        audit_record: evidence.audit_record,
        summary,
        idempotency_context: None,
        wrapper_context: None,
        execution_identity_context: None,
        approval_context: None,
        policy_bundle_verification: None,
    }
}

fn idempotency_context_for_request(
    request: &super::ToolCallRequest,
    supplied_idempotency_context: &Option<super::IdempotencyContext>,
) -> Option<super::IdempotencyContext> {
    if request.carries_mutation_risk() {
        return supplied_idempotency_context.clone();
    }

    None
}

fn adapter_decision(
    request: &super::ToolCallRequest,
    adapter: &dyn PolicyDecisionAdapter,
) -> (PolicyDecision, GatewayEntrypointSummary) {
    match adapter.decide(request) {
        Ok(decision) => (decision, GatewayEntrypointSummary::PolicyDecisionMapped),
        Err(error) => (
            fail_closed_policy_decision(error),
            GatewayEntrypointSummary::PolicyAdapterFailedClosed,
        ),
    }
}

fn fail_closed_policy_decision(error: PolicyAdapterError) -> PolicyDecision {
    PolicyDecision::Deny(PolicyDenial {
        reason_code: Some(
            error
                .reason_code
                .unwrap_or_else(|| "policy_adapter_error".to_string()),
        ),
        safe_message: error.safe_message,
    })
}

fn denied_summary(response: &ToolCallResponse) -> GatewayEntrypointSummary {
    if response.request_id.is_none() {
        return GatewayEntrypointSummary::MalformedRequestDenied;
    }

    if response.status == GatewayStatus::Denied {
        return GatewayEntrypointSummary::UnsupportedToolDenied;
    }

    GatewayEntrypointSummary::PolicyDecisionMapped
}
