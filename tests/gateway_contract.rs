use aegis::gateway::{
    ActorType, CapabilityClass, GatewayStatus, ToolCallRequest, ToolCallResponse,
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

fn read_fixture(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
