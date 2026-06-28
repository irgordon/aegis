use crate::audit::{AuditRecordBuilder, AuditRecordMetadata};

use super::{
    Gateway, GatewayDecisionEvidence, GatewayStatus, ResponseDecision, ResponseMetadata,
    SchemaVersion, SupportedTools, ToolCallRequest, ToolCallResponse,
};

const MALFORMED_REQUEST_REASON: &str = "malformed_request";
const MALFORMED_REQUEST_MESSAGE: &str = "Request could not be validated.";

#[derive(Debug, Clone, PartialEq)]
pub enum GatewayValidationOutcome {
    Accepted(Box<ToolCallRequest>),
    Denied(Box<GatewayDecisionEvidence>),
}

impl Gateway {
    pub fn validate_request_json(
        input: &str,
        supported_tools: &SupportedTools,
        response_metadata: ResponseMetadata,
        audit_metadata: AuditRecordMetadata,
    ) -> GatewayValidationOutcome {
        match deserialize_request(input) {
            Ok(request) => validate_supported_request(
                request,
                supported_tools,
                response_metadata,
                audit_metadata,
            ),
            Err(_) => deny_malformed_request(response_metadata, audit_metadata),
        }
    }
}

fn deserialize_request(input: &str) -> serde_json::Result<ToolCallRequest> {
    serde_json::from_str(input)
}

fn validate_supported_request(
    request: ToolCallRequest,
    supported_tools: &SupportedTools,
    response_metadata: ResponseMetadata,
    audit_metadata: AuditRecordMetadata,
) -> GatewayValidationOutcome {
    match Gateway::deny_unsupported_tool(
        &request,
        supported_tools,
        response_metadata,
        audit_metadata,
    ) {
        Some(evidence) => GatewayValidationOutcome::Denied(Box::new(evidence)),
        None => GatewayValidationOutcome::Accepted(Box::new(request)),
    }
}

fn deny_malformed_request(
    response_metadata: ResponseMetadata,
    audit_metadata: AuditRecordMetadata,
) -> GatewayValidationOutcome {
    let response = malformed_request_response(response_metadata);
    let audit_record =
        AuditRecordBuilder::build_gateway_validation_denial_record(&response, audit_metadata);

    GatewayValidationOutcome::Denied(Box::new(GatewayDecisionEvidence {
        response,
        audit_record,
    }))
}

fn malformed_request_response(metadata: ResponseMetadata) -> ToolCallResponse {
    ToolCallResponse {
        schema_version: SchemaVersion::V1,
        execution_id: metadata.execution_id,
        request_id: None,
        status: GatewayStatus::Denied,
        decision: Some(ResponseDecision::Deny),
        result: None,
        reason_code: Some(MALFORMED_REQUEST_REASON.to_string()),
        safe_message: Some(MALFORMED_REQUEST_MESSAGE.to_string()),
        pending_reference: None,
        replay_reference: None,
        policy_provenance: metadata.policy_provenance,
        audit_record_id: metadata.audit_record_id,
        completed_at: metadata.completed_at,
    }
}
