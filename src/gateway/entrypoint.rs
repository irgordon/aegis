use crate::{
    audit::{AuditRecord, AuditRecordBuilder, AuditRecordMetadata},
    policy::PolicyDecision,
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
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GatewayEntrypointSummary {
    MalformedRequestDenied,
    UnsupportedToolDenied,
    PolicyDecisionMapped,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GatewayEntrypointResult {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
    pub summary: GatewayEntrypointSummary,
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

fn map_supported_request(
    request: super::ToolCallRequest,
    context: GatewayEntrypointContext,
) -> GatewayEntrypointResult {
    let response =
        Gateway::map_policy_decision(&request, context.policy_decision, context.response_metadata);
    let audit_record = AuditRecordBuilder::build_gateway_decision_record(
        &request,
        &response,
        context.audit_metadata,
    );

    GatewayEntrypointResult {
        response,
        audit_record,
        summary: GatewayEntrypointSummary::PolicyDecisionMapped,
    }
}

fn denied_entrypoint_result(evidence: super::GatewayDecisionEvidence) -> GatewayEntrypointResult {
    let summary = denied_summary(&evidence.response);

    GatewayEntrypointResult {
        response: evidence.response,
        audit_record: evidence.audit_record,
        summary,
    }
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
