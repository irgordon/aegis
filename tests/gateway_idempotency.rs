use aegis::{
    audit::AuditRecordMetadata,
    gateway::{
        CapabilityClass, Gateway, GatewayEntrypointContext, GatewayEntrypointSummary,
        GatewayStatus, IdempotencyContext, OperationType, ResponseMetadata, SupportedTools,
        ToolCallResponse,
    },
    policy::PolicyDecision,
};

#[test]
fn l1_request_carries_supplied_idempotency_context() {
    assert_mutation_capability_carries_idempotency_context(CapabilityClass::L1);
}

#[test]
fn l2_request_carries_supplied_idempotency_context() {
    assert_mutation_capability_carries_idempotency_context(CapabilityClass::L2);
}

#[test]
fn l3_request_carries_supplied_idempotency_context() {
    assert_mutation_capability_carries_idempotency_context(CapabilityClass::L3);
}

#[test]
fn l0_request_does_not_attach_idempotency_context() {
    let result = process_request_with_capability(CapabilityClass::L0);

    assert!(result.idempotency_context.is_none());
    assert!(result.audit_record.details.idempotency_context.is_none());
}

#[test]
fn mutation_request_without_context_does_not_generate_idempotency_key() {
    let input = request_with_capability(CapabilityClass::L1);
    let result = Gateway::process_entrypoint_request(
        &input,
        GatewayEntrypointContext {
            supported_tools: SupportedTools::from_names(["metrics.read"]),
            policy_decision: PolicyDecision::Allow,
            response_metadata: response_metadata(),
            audit_metadata: audit_metadata(),
            idempotency_context: None,
            wrapper_context: None,
        },
    );

    assert!(result.idempotency_context.is_none());
    assert!(result.audit_record.details.idempotency_context.is_none());
}

#[test]
fn idempotency_context_binds_required_execution_fields() {
    let context = idempotency_context();

    assert_eq!(context.key.as_str(), "idem_001");
    assert_eq!(context.execution_id.as_str(), "exec_001");
    assert_eq!(context.tool_call_hash.as_str(), "tool_hash_001");
    assert_eq!(context.target_system.as_str(), "metrics");
    assert_eq!(context.operation_type, OperationType::Mutation);
    assert_eq!(context.policy_bundle_version.as_str(), "0.2.0");
}

fn assert_mutation_capability_carries_idempotency_context(capability_class: CapabilityClass) {
    let result = process_request_with_capability(capability_class);
    let result_context = result
        .idempotency_context
        .as_ref()
        .unwrap_or_else(|| panic!("mutation-capable request should carry idempotency context"));
    let audit_context = result
        .audit_record
        .details
        .idempotency_context
        .as_ref()
        .unwrap_or_else(|| panic!("audit record should include idempotency context"));

    assert_eq!(
        result.summary,
        GatewayEntrypointSummary::PolicyDecisionMapped
    );
    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert_eq!(result_context, audit_context);
    assert_eq!(audit_context.key.as_str(), "idem_001");
}

fn process_request_with_capability(
    capability_class: CapabilityClass,
) -> aegis::gateway::GatewayEntrypointResult {
    let input = request_with_capability(capability_class);

    Gateway::process_entrypoint_request(
        &input,
        GatewayEntrypointContext {
            supported_tools: SupportedTools::from_names(["metrics.read"]),
            policy_decision: PolicyDecision::Allow,
            response_metadata: response_metadata(),
            audit_metadata: audit_metadata(),
            idempotency_context: Some(idempotency_context()),
            wrapper_context: None,
        },
    )
}

fn request_with_capability(capability_class: CapabilityClass) -> String {
    let mut fixture: serde_json::Value =
        serde_json::from_str(&read_fixture("schemas/examples/valid/ToolCallRequest.json"))
            .unwrap_or_else(|error| {
                panic!("valid ToolCallRequest fixture should be JSON: {error}")
            });

    fixture["tool"]["capability_class"] = serde_json::json!(capability_class);
    serde_json::to_string(&fixture)
        .unwrap_or_else(|error| panic!("request fixture should serialize: {error}"))
}

fn idempotency_context() -> IdempotencyContext {
    serde_json::from_value(serde_json::json!({
        "key": "idem_001",
        "execution_id": "exec_001",
        "tool_call_hash": "tool_hash_001",
        "target_system": "metrics",
        "operation_type": "mutation",
        "policy_bundle_version": "0.2.0"
    }))
    .unwrap_or_else(|error| panic!("idempotency context should parse: {error}"))
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
