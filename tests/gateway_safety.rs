use aegis::{
    audit::{AuditRecordMetadata, AuditStatus},
    gateway::{
        Gateway, GatewayStatus, ResponseDecision, ResponseMetadata, SupportedTools,
        ToolCallRequest, ToolCallResponse,
    },
};

#[test]
fn unsupported_tool_returns_denied_response() {
    let request = load_valid_request();
    let evidence = unsupported_tool_evidence(&request);

    assert_eq!(evidence.response.status(), &GatewayStatus::Denied);
    assert_eq!(evidence.response.decision, Some(ResponseDecision::Deny));
    assert_eq!(
        evidence.response.reason_code.as_deref(),
        Some("unsupported_tool")
    );
    assert_eq!(
        evidence.response.safe_message(),
        Some("Tool is not supported by this gateway.")
    );
}

#[test]
fn unsupported_tool_denial_builds_audit_record() {
    let request = load_valid_request();
    let evidence = unsupported_tool_evidence(&request);

    assert_eq!(evidence.audit_record.status, AuditStatus::Denied);
    assert_eq!(
        evidence.audit_record.reason_code.as_deref(),
        Some("unsupported_tool")
    );
    assert_eq!(
        evidence
            .audit_record
            .tool_name
            .as_ref()
            .map(|name| name.as_str()),
        Some("metrics.read")
    );
}

#[test]
fn supported_tool_does_not_return_denial() {
    let request = load_valid_request();
    let supported_tools = SupportedTools::from_names(["metrics.read"]);

    let evidence = Gateway::deny_unsupported_tool(
        &request,
        &supported_tools,
        response_metadata(),
        audit_metadata(),
    );

    assert!(evidence.is_none());
}

#[test]
fn supported_tool_allowlist_is_explicit() {
    let supported_tools = SupportedTools::from_names(["metrics.read"]);

    assert!(supported_tools.contains("metrics.read"));
    assert!(!supported_tools.contains("email.send"));
}

fn unsupported_tool_evidence(request: &ToolCallRequest) -> aegis::gateway::GatewayDecisionEvidence {
    let supported_tools = SupportedTools::from_names(["email.send"]);

    Gateway::deny_unsupported_tool(
        request,
        &supported_tools,
        response_metadata(),
        audit_metadata(),
    )
    .unwrap_or_else(|| panic!("unsupported tool should produce denial evidence"))
}

fn load_valid_request() -> ToolCallRequest {
    parse_request_fixture("schemas/examples/valid/ToolCallRequest.json")
        .unwrap_or_else(|error| panic!("valid ToolCallRequest fixture should parse: {error}"))
}

fn load_valid_response() -> ToolCallResponse {
    parse_response_fixture("schemas/examples/valid/ToolCallResponse.json")
        .unwrap_or_else(|error| panic!("valid ToolCallResponse fixture should parse: {error}"))
}

fn parse_request_fixture(path: &str) -> serde_json::Result<ToolCallRequest> {
    serde_json::from_str(&read_fixture(path))
}

fn parse_response_fixture(path: &str) -> serde_json::Result<ToolCallResponse> {
    serde_json::from_str(&read_fixture(path))
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
