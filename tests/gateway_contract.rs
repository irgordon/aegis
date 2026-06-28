use aegis::{
    gateway::{
        ActorType, CapabilityClass, Gateway, GatewayStatus, PendingReference, ResponseDecision,
        ResponseMetadata, ToolCallRequest, ToolCallResponse,
    },
    policy::{PendingApprovalDecision, PolicyDecision, PolicyDenial},
};

#[test]
fn valid_request_fixture_loads() {
    let request = load_valid_request();

    assert_eq!(request.request_id(), "req_001");
    assert_eq!(request.tool_name(), "metrics.read");
    assert_eq!(request.actor.actor_type, ActorType::Agent);
    assert_eq!(request.tool.capability_class, Some(CapabilityClass::L0));
}

#[test]
fn valid_response_fixture_loads() {
    let response = load_valid_response();

    assert_eq!(response.status(), &GatewayStatus::Allowed);
    assert_eq!(response.request_id(), Some("req_001"));
    assert_eq!(response.safe_message(), Some("Execution allowed."));
}

#[test]
fn invalid_request_fixture_fails() {
    let error = parse_request_fixture("schemas/examples/invalid/ToolCallRequest.json");

    assert!(error.is_err());
}

#[test]
fn invalid_response_fixture_fails() {
    let error = parse_response_fixture("schemas/examples/invalid/ToolCallResponse.json");

    assert!(error.is_err());
}

#[test]
fn status_values_are_bounded_by_enum() {
    let statuses = [
        GatewayStatus::Allowed,
        GatewayStatus::Denied,
        GatewayStatus::Pending,
        GatewayStatus::Failed,
        GatewayStatus::Canceled,
        GatewayStatus::Replayed,
    ];

    assert_eq!(statuses.len(), 6);
}

#[test]
fn request_actor_and_capability_classes_are_bounded() {
    let request = load_valid_request();

    assert_eq!(request.actor.actor_type, ActorType::Agent);
    assert_eq!(request.tool.capability_class, Some(CapabilityClass::L0));
}

#[test]
fn invalid_actor_type_fails() {
    let mut fixture = load_request_fixture_value();
    fixture["actor"]["actor_type"] = serde_json::json!("robot");

    let error = serde_json::from_value::<ToolCallRequest>(fixture);

    assert!(error.is_err());
}

#[test]
fn invalid_capability_class_fails() {
    let mut fixture = load_request_fixture_value();
    fixture["tool"]["capability_class"] = serde_json::json!("L4");

    let error = serde_json::from_value::<ToolCallRequest>(fixture);

    assert!(error.is_err());
}

#[test]
fn allowed_policy_decision_maps_to_allowed_response() {
    let request = load_valid_request();
    let response =
        Gateway::map_policy_decision(&request, PolicyDecision::Allow, response_metadata());

    assert_eq!(response.request_id(), Some("req_001"));
    assert_eq!(response.status(), &GatewayStatus::Allowed);
    assert_eq!(response.decision, Some(ResponseDecision::Allow));
    assert!(response.result.is_none());
    assert!(response.pending_reference.is_none());
}

#[test]
fn denied_policy_decision_maps_to_denied_response() {
    let request = load_valid_request();
    let response = Gateway::map_policy_decision(
        &request,
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some("unknown_tool".to_string()),
            safe_message: "Tool is not authorized.".to_string(),
        }),
        response_metadata(),
    );

    assert_eq!(response.status(), &GatewayStatus::Denied);
    assert_eq!(response.decision, Some(ResponseDecision::Deny));
    assert_eq!(response.reason_code.as_deref(), Some("unknown_tool"));
    assert_eq!(response.safe_message(), Some("Tool is not authorized."));
}

#[test]
fn pending_policy_decision_maps_to_pending_response() {
    let request = load_valid_request();
    let pending_reference = pending_reference();
    let response = Gateway::map_policy_decision(
        &request,
        PolicyDecision::PendingApproval(PendingApprovalDecision {
            pending_reference: pending_reference.clone(),
            reason_code: Some("approval_required".to_string()),
            safe_message: Some("Approval is required.".to_string()),
        }),
        response_metadata(),
    );

    assert_eq!(response.status(), &GatewayStatus::Pending);
    assert_eq!(response.decision, Some(ResponseDecision::PendingApproval));
    assert_eq!(response.pending_reference, Some(pending_reference));
    assert_eq!(response.reason_code.as_deref(), Some("approval_required"));
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

fn load_request_fixture_value() -> serde_json::Value {
    serde_json::from_str(&read_fixture("schemas/examples/valid/ToolCallRequest.json"))
        .unwrap_or_else(|error| panic!("valid ToolCallRequest fixture should be JSON: {error}"))
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

fn read_fixture(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
