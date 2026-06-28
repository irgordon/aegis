use aegis::{
    audit::AuditRecordMetadata,
    gateway::{
        ApprovalContext, ApprovalContextSource, ApprovalStatus, ApprovalTokenState, Gateway,
        GatewayEntrypointContext, GatewayStatus, PendingReference, ResponseDecision,
        ResponseMetadata, SupportedTools, ToolCallResponse,
    },
    policy::{PendingApprovalDecision, PolicyDecision, PolicyDenial},
};

#[test]
fn allowed_decision_carries_approval_context_reference() {
    let result = process_request(PolicyDecision::Allow, Some(approval_context()));

    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert_approval_context(result.audit_record.details.approval_context.as_ref());
}

#[test]
fn denied_decision_carries_approval_context_reference() {
    let result = process_request(
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some("policy_denied".to_string()),
            safe_message: "Policy denied this request.".to_string(),
        }),
        Some(approval_context()),
    );

    assert_eq!(result.response.status(), &GatewayStatus::Denied);
    assert_eq!(result.response.decision, Some(ResponseDecision::Deny));
    assert_approval_context(result.audit_record.details.approval_context.as_ref());
}

#[test]
fn pending_decision_carries_approval_context_reference() {
    let result = process_request(
        PolicyDecision::PendingApproval(PendingApprovalDecision {
            pending_reference: pending_reference(),
            reason_code: Some("approval_required".to_string()),
            safe_message: Some("Approval is required.".to_string()),
        }),
        Some(approval_context()),
    );

    assert_eq!(result.response.status(), &GatewayStatus::Pending);
    assert_eq!(
        result.response.decision,
        Some(ResponseDecision::PendingApproval)
    );
    assert_approval_context(result.audit_record.details.approval_context.as_ref());
}

#[test]
fn entrypoint_without_approval_context_remains_unchanged() {
    let result = process_request(PolicyDecision::Allow, None);

    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert!(result.approval_context.is_none());
    assert!(result.audit_record.details.approval_context.is_none());
}

#[test]
fn approval_context_does_not_generate_approval_tokens() {
    let result = process_request(PolicyDecision::Allow, Some(approval_context()));
    let context = result
        .approval_context
        .as_ref()
        .unwrap_or_else(|| panic!("approval context should be present"));

    assert_eq!(
        context.approval_token_ref.as_str(),
        "approval_token_ref_001"
    );
    assert_eq!(
        result.audit_record.details.approval_context,
        result.approval_context
    );
}

#[test]
fn approval_context_does_not_read_system_time() {
    let result = process_request(PolicyDecision::Allow, Some(approval_context()));
    let context = result
        .audit_record
        .details
        .approval_context
        .as_ref()
        .unwrap_or_else(|| panic!("approval context should be present"));

    assert_eq!(
        context
            .expiration
            .expires_at
            .as_ref()
            .map(|timestamp| timestamp.as_str()),
        Some("2026-06-28T00:10:00Z")
    );
    assert_eq!(
        context.expiration.ttl.as_ref().map(|ttl| ttl.as_str()),
        Some("PT10M")
    );
}

#[test]
fn approval_context_binds_to_execution_id() {
    let context = approval_context();

    assert_eq!(context.binding.execution_id.as_str(), "exec_001");
}

#[test]
fn approval_context_binds_to_tool_call_hash() {
    let context = approval_context();

    assert_eq!(context.binding.tool_call_hash.as_str(), "tool_hash_001");
}

#[test]
fn approval_status_is_bounded_by_enum() {
    let statuses = [
        ApprovalStatus::Pending,
        ApprovalStatus::Approved,
        ApprovalStatus::Denied,
    ];

    assert_eq!(statuses.len(), 3);
    assert!(approval_context_with_status("unknown").is_err());
}

#[test]
fn terminal_token_states_are_representable() {
    let states = [
        ApprovalTokenState::Revoked,
        ApprovalTokenState::Expired,
        ApprovalTokenState::Used,
        ApprovalTokenState::ContextMismatch,
    ];

    assert_eq!(states.len(), 4);
}

#[test]
fn approval_requirement_reference_is_present_when_context_is_supplied() {
    let context = approval_context();

    assert_eq!(
        context.approval_requirement_ref.as_str(),
        "approval_requirement_001"
    );
}

fn assert_approval_context(context: Option<&ApprovalContext>) {
    let context = context.unwrap_or_else(|| panic!("approval context should be present"));

    assert_eq!(
        context.approval_token_ref.as_str(),
        "approval_token_ref_001"
    );
    assert_eq!(context.binding.execution_id.as_str(), "exec_001");
    assert_eq!(context.binding.tool_call_hash.as_str(), "tool_hash_001");
    assert_eq!(context.status, ApprovalStatus::Pending);
    assert_eq!(context.token_state, ApprovalTokenState::Active);
    assert_eq!(
        context.approval_requirement_ref.as_str(),
        "approval_requirement_001"
    );
    assert_eq!(
        context.approver_role_ref.as_ref().map(|role| role.as_str()),
        Some("security_reviewer")
    );
    assert_eq!(context.source, ApprovalContextSource::CallerSupplied);
}

fn process_request(
    policy_decision: PolicyDecision,
    approval_context: Option<ApprovalContext>,
) -> aegis::gateway::GatewayEntrypointResult {
    Gateway::process_entrypoint_request(
        &read_fixture("schemas/examples/valid/ToolCallRequest.json"),
        GatewayEntrypointContext {
            supported_tools: SupportedTools::from_names(["metrics.read"]),
            policy_decision,
            response_metadata: response_metadata(),
            audit_metadata: audit_metadata(),
            idempotency_context: None,
            wrapper_context: None,
            execution_identity_context: None,
            approval_context,
        },
    )
}

fn approval_context() -> ApprovalContext {
    serde_json::from_value(approval_context_value())
        .unwrap_or_else(|error| panic!("approval context should parse: {error}"))
}

fn approval_context_with_status(status: &str) -> Result<ApprovalContext, serde_json::Error> {
    let mut value = approval_context_value();
    value["status"] = serde_json::json!(status);

    serde_json::from_value(value)
}

fn approval_context_value() -> serde_json::Value {
    serde_json::json!({
        "approval_token_ref": "approval_token_ref_001",
        "binding": {
            "execution_id": "exec_001",
            "tool_call_hash": "tool_hash_001"
        },
        "status": "pending",
        "token_state": "active",
        "expiration": {
            "ttl": "PT10M",
            "expires_at": "2026-06-28T00:10:00Z"
        },
        "approval_requirement_ref": "approval_requirement_001",
        "approver_ref": "approver_001",
        "approver_role_ref": "security_reviewer",
        "break_glass_ref": "break_glass_policy_001",
        "source": "caller_supplied"
    })
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
