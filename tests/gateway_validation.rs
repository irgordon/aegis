use aegis::{
    audit::{AuditEventType, AuditRecordMetadata, AuditStatus},
    gateway::{
        Gateway, GatewayStatus, GatewayValidationOutcome, ResponseDecision, ResponseMetadata,
        SupportedTools, ToolCallResponse,
    },
};

#[test]
fn valid_supported_request_is_accepted() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let supported_tools = SupportedTools::from_names(["metrics.read"]);

    let outcome = Gateway::validate_request_json(
        &input,
        &supported_tools,
        response_metadata(),
        audit_metadata(),
    );

    let GatewayValidationOutcome::Accepted(request) = outcome else {
        panic!("supported request should be accepted");
    };

    assert_eq!(request.request_id(), "req_001");
    assert_eq!(request.tool_name(), "metrics.read");
}

#[test]
fn malformed_request_is_denied_fail_closed() {
    let input = read_fixture("schemas/examples/invalid/ToolCallRequest.json");
    let supported_tools = SupportedTools::from_names(["metrics.read"]);

    let evidence = denied_validation_outcome(&input, &supported_tools);

    assert_eq!(evidence.response.status(), &GatewayStatus::Denied);
    assert_eq!(evidence.response.decision, Some(ResponseDecision::Deny));
    assert_eq!(
        evidence.response.reason_code.as_deref(),
        Some("malformed_request")
    );
    assert_eq!(evidence.response.request_id(), None);
}

#[test]
fn malformed_request_denial_builds_audit_record() {
    let input = read_fixture("schemas/examples/invalid/ToolCallRequest.json");
    let supported_tools = SupportedTools::from_names(["metrics.read"]);

    let evidence = denied_validation_outcome(&input, &supported_tools);

    assert_eq!(
        evidence.audit_record.event_type,
        AuditEventType::ValidationResult
    );
    assert_eq!(evidence.audit_record.status, AuditStatus::Denied);
    assert_eq!(
        evidence.audit_record.reason_code.as_deref(),
        Some("malformed_request")
    );
    assert!(evidence.audit_record.details.request_id.is_none());
}

#[test]
fn unsupported_valid_request_is_denied_with_audit_record() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let supported_tools = SupportedTools::from_names(["email.send"]);

    let evidence = denied_validation_outcome(&input, &supported_tools);

    assert_eq!(
        evidence.response.reason_code.as_deref(),
        Some("unsupported_tool")
    );
    assert_eq!(evidence.audit_record.status, AuditStatus::Denied);
    assert_eq!(
        evidence
            .audit_record
            .details
            .request_id
            .as_ref()
            .map(|request_id| request_id.as_str()),
        Some("req_001")
    );
}

#[test]
fn validation_denial_uses_caller_supplied_metadata() {
    let input = read_fixture("schemas/examples/invalid/ToolCallRequest.json");
    let supported_tools = SupportedTools::from_names(["metrics.read"]);

    let evidence = denied_validation_outcome(&input, &supported_tools);

    assert_eq!(evidence.response.audit_record_id.as_str(), "audit_001");
    assert_eq!(evidence.audit_record.component.as_str(), "gateway");
    assert_eq!(
        evidence.audit_record.timestamp,
        evidence.response.completed_at
    );
}

fn denied_validation_outcome(
    input: &str,
    supported_tools: &SupportedTools,
) -> aegis::gateway::GatewayDecisionEvidence {
    match Gateway::validate_request_json(
        input,
        supported_tools,
        response_metadata(),
        audit_metadata(),
    ) {
        GatewayValidationOutcome::Denied(evidence) => *evidence,
        GatewayValidationOutcome::Accepted(_) => panic!("request should be denied"),
    }
}

fn load_valid_response() -> ToolCallResponse {
    serde_json::from_str(&read_fixture(
        "schemas/examples/valid/ToolCallResponse.json",
    ))
    .unwrap_or_else(|error| panic!("valid ToolCallResponse fixture should parse: {error}"))
}

fn response_metadata() -> ResponseMetadata {
    let fixture = load_valid_response();

    ResponseMetadata {
        execution_id: fixture.execution_id,
        policy_provenance: fixture.policy_provenance,
        audit_record_id: fixture.audit_record_id,
        completed_at: fixture.completed_at,
    }
}

fn audit_metadata() -> AuditRecordMetadata {
    serde_json::from_value(serde_json::json!({
        "component": "gateway"
    }))
    .unwrap_or_else(|error| panic!("audit metadata fixture should parse: {error}"))
}

fn read_fixture(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
