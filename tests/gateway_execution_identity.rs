use aegis::{
    audit::AuditRecordMetadata,
    gateway::{
        ExecutionIdentityContext, ExecutionIdentitySource, Gateway, GatewayEntrypointContext,
        GatewayStatus, PendingReference, ResponseDecision, ResponseMetadata, SupportedTools,
        ToolCallResponse,
    },
    policy::{PendingApprovalDecision, PolicyDecision, PolicyDenial},
};

#[test]
fn allowed_decision_carries_execution_identity_reference() {
    let result = process_request(PolicyDecision::Allow, Some(execution_identity_context()));

    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert_execution_identity(
        result
            .audit_record
            .details
            .execution_identity_context
            .as_ref(),
    );
}

#[test]
fn denied_decision_carries_execution_identity_reference() {
    let result = process_request(
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some("policy_denied".to_string()),
            safe_message: "Policy denied this request.".to_string(),
        }),
        Some(execution_identity_context()),
    );

    assert_eq!(result.response.status(), &GatewayStatus::Denied);
    assert_eq!(result.response.decision, Some(ResponseDecision::Deny));
    assert_execution_identity(
        result
            .audit_record
            .details
            .execution_identity_context
            .as_ref(),
    );
}

#[test]
fn pending_decision_carries_execution_identity_reference() {
    let result = process_request(
        PolicyDecision::PendingApproval(PendingApprovalDecision {
            pending_reference: pending_reference(),
            reason_code: Some("approval_required".to_string()),
            safe_message: Some("Approval is required.".to_string()),
        }),
        Some(execution_identity_context()),
    );

    assert_eq!(result.response.status(), &GatewayStatus::Pending);
    assert_eq!(
        result.response.decision,
        Some(ResponseDecision::PendingApproval)
    );
    assert_execution_identity(
        result
            .audit_record
            .details
            .execution_identity_context
            .as_ref(),
    );
}

#[test]
fn entrypoint_without_execution_identity_context_remains_unchanged() {
    let result = process_request(PolicyDecision::Allow, None);

    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert!(result.execution_identity_context.is_none());
    assert!(result
        .audit_record
        .details
        .execution_identity_context
        .is_none());
}

#[test]
fn execution_identity_context_does_not_generate_execution_ids() {
    let result = process_request(PolicyDecision::Allow, Some(execution_identity_context()));
    let context = result
        .execution_identity_context
        .as_ref()
        .unwrap_or_else(|| panic!("execution identity context should be present"));

    assert_eq!(context.execution_identity.as_str(), "exec_identity_001");
    assert_eq!(
        result.audit_record.details.execution_identity_context,
        result.execution_identity_context
    );
}

#[test]
fn execution_identity_context_does_not_hash_binding_fields() {
    let context = execution_identity_context();

    assert_eq!(context.binding.orchestrator_id.as_str(), "orchestrator_001");
    assert_eq!(context.binding.workflow_id.as_str(), "workflow_001");
    assert_eq!(context.binding.tool_call_id.as_str(), "tool_call_001");
    assert_eq!(context.binding.policy_bundle_version.as_str(), "0.2.0");
    assert_eq!(context.binding.nonce_ref.as_str(), "nonce_ref_001");
}

#[test]
fn binding_fields_are_required_when_execution_identity_context_is_supplied() {
    for field in [
        "orchestrator_id",
        "workflow_id",
        "tool_call_id",
        "policy_bundle_version",
        "nonce_ref",
    ] {
        let mut value = execution_identity_value();
        value["binding"]
            .as_object_mut()
            .unwrap_or_else(|| panic!("binding should be a JSON object"))
            .remove(field);

        assert!(
            serde_json::from_value::<ExecutionIdentityContext>(value).is_err(),
            "{field} should be required"
        );
    }
}

#[test]
fn policy_bundle_version_is_part_of_execution_identity_binding() {
    let context = execution_identity_context();

    assert_eq!(context.binding.policy_bundle_version.as_str(), "0.2.0");
}

fn assert_execution_identity(context: Option<&ExecutionIdentityContext>) {
    let context = context.unwrap_or_else(|| panic!("execution identity context should be present"));

    assert_eq!(context.execution_identity.as_str(), "exec_identity_001");
    assert_eq!(context.binding.orchestrator_id.as_str(), "orchestrator_001");
    assert_eq!(context.binding.workflow_id.as_str(), "workflow_001");
    assert_eq!(context.binding.tool_call_id.as_str(), "tool_call_001");
    assert_eq!(context.binding.policy_bundle_version.as_str(), "0.2.0");
    assert_eq!(context.binding.nonce_ref.as_str(), "nonce_ref_001");
    assert_eq!(context.source, ExecutionIdentitySource::CallerSupplied);
}

fn process_request(
    policy_decision: PolicyDecision,
    execution_identity_context: Option<ExecutionIdentityContext>,
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
            execution_identity_context,
            approval_context: None,
        },
    )
}

fn execution_identity_context() -> ExecutionIdentityContext {
    serde_json::from_value(execution_identity_value())
        .unwrap_or_else(|error| panic!("execution identity context should parse: {error}"))
}

fn execution_identity_value() -> serde_json::Value {
    serde_json::json!({
        "execution_identity": "exec_identity_001",
        "binding": {
            "orchestrator_id": "orchestrator_001",
            "workflow_id": "workflow_001",
            "tool_call_id": "tool_call_001",
            "policy_bundle_version": "0.2.0",
            "nonce_ref": "nonce_ref_001"
        },
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
