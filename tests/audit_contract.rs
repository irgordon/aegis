use aegis::{
    audit::{AuditEventType, AuditRecordBuilder, AuditRecordMetadata, AuditStatus},
    gateway::{
        Gateway, GatewayStatus, PendingReference, ResponseDecision, ResponseMetadata,
        ToolCallRequest, ToolCallResponse,
    },
    policy::{PendingApprovalDecision, PolicyDecision, PolicyDenial},
};

#[test]
fn allowed_response_produces_audit_record() {
    let request = load_valid_request();
    let response =
        Gateway::map_policy_decision(&request, PolicyDecision::Allow, response_metadata());

    let record =
        AuditRecordBuilder::build_gateway_decision_record(&request, &response, audit_metadata());

    assert_eq!(record.event_type, AuditEventType::PolicyDecision);
    assert_eq!(record.status, AuditStatus::Allowed);
    assert_eq!(record.details.decision, Some(ResponseDecision::Allow));
}

#[test]
fn denied_response_produces_audit_record() {
    let request = load_valid_request();
    let response = Gateway::map_policy_decision(
        &request,
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some("unknown_tool".to_string()),
            safe_message: "Tool is not authorized.".to_string(),
        }),
        response_metadata(),
    );

    let record =
        AuditRecordBuilder::build_gateway_decision_record(&request, &response, audit_metadata());

    assert_eq!(record.status, AuditStatus::Denied);
    assert_eq!(record.reason_code.as_deref(), Some("unknown_tool"));
    assert_eq!(record.details.decision, Some(ResponseDecision::Deny));
}

#[test]
fn pending_response_produces_audit_record() {
    let request = load_valid_request();
    let response = Gateway::map_policy_decision(
        &request,
        PolicyDecision::PendingApproval(PendingApprovalDecision {
            pending_reference: pending_reference(),
            reason_code: Some("approval_required".to_string()),
            safe_message: Some("Approval is required.".to_string()),
        }),
        response_metadata(),
    );

    let record =
        AuditRecordBuilder::build_gateway_decision_record(&request, &response, audit_metadata());

    assert_eq!(record.status, AuditStatus::Pending);
    assert_eq!(
        record.details.decision,
        Some(ResponseDecision::PendingApproval)
    );
}

#[test]
fn audit_record_uses_caller_supplied_metadata() {
    let request = load_valid_request();
    let response =
        Gateway::map_policy_decision(&request, PolicyDecision::Allow, response_metadata());

    let record =
        AuditRecordBuilder::build_gateway_decision_record(&request, &response, audit_metadata());

    assert_eq!(record.audit_record_id, response.audit_record_id);
    assert_eq!(record.timestamp, response.completed_at);
    assert_eq!(record.component.as_str(), "gateway");
}

#[test]
fn audit_record_construction_uses_loaded_models_without_io() {
    let request = load_valid_request();
    let response =
        Gateway::map_policy_decision(&request, PolicyDecision::Allow, response_metadata());

    let record =
        AuditRecordBuilder::build_gateway_decision_record(&request, &response, audit_metadata());

    assert_eq!(record.execution_id, response.execution_id);
    assert_eq!(
        record.run_id.as_ref().map(|id| id.as_str()),
        Some("run_001")
    );
}

#[test]
fn audit_record_fields_remain_bounded_by_enums() {
    let statuses = [
        AuditStatus::Allowed,
        AuditStatus::Denied,
        AuditStatus::Pending,
        AuditStatus::Failed,
        AuditStatus::Canceled,
        AuditStatus::Replayed,
        AuditStatus::Recorded,
    ];

    assert_eq!(statuses.len(), 7);
    assert_eq!(
        AuditStatus::from(&GatewayStatus::Allowed),
        AuditStatus::Allowed
    );
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
