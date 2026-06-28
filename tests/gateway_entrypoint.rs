use aegis::{
    audit::{AuditEventType, AuditRecordMetadata, AuditStatus},
    gateway::{
        Gateway, GatewayEntrypointContext, GatewayEntrypointResult, GatewayEntrypointSummary,
        GatewayStatus, PendingReference, ResponseDecision, ResponseMetadata, SupportedTools,
        ToolCallResponse,
    },
    policy::{PendingApprovalDecision, PolicyDecision, PolicyDenial},
};

#[test]
fn malformed_json_returns_denied_response_and_audit_record() {
    let input = read_fixture("schemas/examples/invalid/ToolCallRequest.json");
    let result = process_request(input, PolicyDecision::Allow, ["metrics.read"]);

    assert_eq!(
        result.summary,
        GatewayEntrypointSummary::MalformedRequestDenied
    );
    assert_eq!(result.response.status(), &GatewayStatus::Denied);
    assert_eq!(
        result.response.reason_code.as_deref(),
        Some("malformed_request")
    );
    assert_eq!(
        result.audit_record.event_type,
        AuditEventType::ValidationResult
    );
    assert_eq!(result.audit_record.status, AuditStatus::Denied);
}

#[test]
fn unsupported_tool_returns_denied_response_and_audit_record() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let result = process_request(input, PolicyDecision::Allow, ["email.send"]);

    assert_eq!(
        result.summary,
        GatewayEntrypointSummary::UnsupportedToolDenied
    );
    assert_eq!(result.response.status(), &GatewayStatus::Denied);
    assert_eq!(
        result.response.reason_code.as_deref(),
        Some("unsupported_tool")
    );
    assert_eq!(
        result.audit_record.event_type,
        AuditEventType::PolicyDecision
    );
    assert_eq!(result.audit_record.status, AuditStatus::Denied);
}

#[test]
fn supported_request_with_allow_decision_returns_allowed_evidence() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let result = process_request(input, PolicyDecision::Allow, ["metrics.read"]);

    assert_policy_decision_result(&result, GatewayStatus::Allowed, ResponseDecision::Allow);
    assert!(result.response.result.is_none());
}

#[test]
fn supported_request_with_deny_decision_returns_denied_evidence() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let decision = PolicyDecision::Deny(PolicyDenial {
        reason_code: Some("policy_denied".to_string()),
        safe_message: "Policy denied this request.".to_string(),
    });

    let result = process_request(input, decision, ["metrics.read"]);

    assert_policy_decision_result(&result, GatewayStatus::Denied, ResponseDecision::Deny);
    assert_eq!(
        result.response.reason_code.as_deref(),
        Some("policy_denied")
    );
    assert_eq!(
        result.response.safe_message(),
        Some("Policy denied this request.")
    );
}

#[test]
fn supported_request_with_pending_decision_returns_pending_evidence() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let pending_reference = pending_reference();
    let decision = PolicyDecision::PendingApproval(PendingApprovalDecision {
        pending_reference: pending_reference.clone(),
        reason_code: Some("approval_required".to_string()),
        safe_message: Some("Approval is required.".to_string()),
    });

    let result = process_request(input, decision, ["metrics.read"]);

    assert_policy_decision_result(
        &result,
        GatewayStatus::Pending,
        ResponseDecision::PendingApproval,
    );
    assert_eq!(result.response.pending_reference, Some(pending_reference));
}

#[test]
fn entrypoint_uses_only_caller_supplied_context() {
    let input = read_fixture("schemas/examples/valid/ToolCallRequest.json");
    let result = process_request(input, PolicyDecision::Allow, ["metrics.read"]);

    assert_eq!(result.response.audit_record_id.as_str(), "audit_001");
    assert_eq!(result.audit_record.component.as_str(), "gateway");
    assert_eq!(result.audit_record.timestamp, result.response.completed_at);
    assert!(result.response.result.is_none());
}

fn assert_policy_decision_result(
    result: &GatewayEntrypointResult,
    status: GatewayStatus,
    decision: ResponseDecision,
) {
    assert_eq!(
        result.summary,
        GatewayEntrypointSummary::PolicyDecisionMapped
    );
    assert_eq!(result.response.status(), &status);
    assert_eq!(result.response.decision, Some(decision.clone()));
    assert_eq!(
        result.audit_record.event_type,
        AuditEventType::PolicyDecision
    );
    assert_eq!(result.audit_record.status, AuditStatus::from(&status));
    assert_eq!(result.audit_record.details.decision, Some(decision));
}

fn process_request<I, S>(
    input: String,
    policy_decision: PolicyDecision,
    supported_tools: I,
) -> GatewayEntrypointResult
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    Gateway::process_entrypoint_request(
        &input,
        GatewayEntrypointContext {
            supported_tools: SupportedTools::from_names(supported_tools),
            policy_decision,
            response_metadata: response_metadata(),
            audit_metadata: audit_metadata(),
            idempotency_context: None,
            wrapper_context: None,
            execution_identity_context: None,
        },
    )
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

fn pending_reference() -> PendingReference {
    serde_json::from_value(serde_json::json!({
        "approval_id": "approval_001",
        "expires_at": "2026-06-28T00:10:00Z"
    }))
    .unwrap_or_else(|error| panic!("pending reference fixture should parse: {error}"))
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
