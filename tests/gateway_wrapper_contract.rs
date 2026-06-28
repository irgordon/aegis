use aegis::{
    audit::AuditRecordMetadata,
    gateway::{
        Gateway, GatewayEntrypointContext, GatewayStatus, PendingReference, ResponseDecision,
        ResponseMetadata, SupportedTools, ToolCallResponse, WrapperExecutionContext,
        WrapperExecutionMode,
    },
    policy::{PendingApprovalDecision, PolicyDecision, PolicyDenial},
};

#[test]
fn allowed_decision_carries_wrapper_configuration_reference() {
    let result = process_request(PolicyDecision::Allow, Some(wrapper_context()));

    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert_wrapper_context(result.audit_record.details.wrapper_context.as_ref());
}

#[test]
fn denied_decision_carries_wrapper_configuration_reference() {
    let result = process_request(
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some("policy_denied".to_string()),
            safe_message: "Policy denied this request.".to_string(),
        }),
        Some(wrapper_context()),
    );

    assert_eq!(result.response.status(), &GatewayStatus::Denied);
    assert_eq!(result.response.decision, Some(ResponseDecision::Deny));
    assert_wrapper_context(result.audit_record.details.wrapper_context.as_ref());
}

#[test]
fn pending_decision_carries_wrapper_configuration_reference() {
    let result = process_request(
        PolicyDecision::PendingApproval(PendingApprovalDecision {
            pending_reference: pending_reference(),
            reason_code: Some("approval_required".to_string()),
            safe_message: Some("Approval is required.".to_string()),
        }),
        Some(wrapper_context()),
    );

    assert_eq!(result.response.status(), &GatewayStatus::Pending);
    assert_eq!(
        result.response.decision,
        Some(ResponseDecision::PendingApproval)
    );
    assert_wrapper_context(result.audit_record.details.wrapper_context.as_ref());
}

#[test]
fn entrypoint_without_wrapper_context_remains_unchanged() {
    let result = process_request(PolicyDecision::Allow, None);

    assert_eq!(result.response.status(), &GatewayStatus::Allowed);
    assert!(result.wrapper_context.is_none());
    assert!(result.audit_record.details.wrapper_context.is_none());
}

#[test]
fn wrapper_configuration_context_does_not_execute_wrappers() {
    let result = process_request(PolicyDecision::Allow, Some(wrapper_context()));

    assert!(result.response.result.is_none());
    assert_eq!(
        result.audit_record.details.wrapper_context,
        result.wrapper_context
    );
}

#[test]
fn wrapper_configuration_context_does_not_require_secret_values() {
    let context = wrapper_context();
    let serialized = serde_json::to_value(context)
        .unwrap_or_else(|error| panic!("wrapper context should serialize: {error}"));

    assert!(serialized.get("secret").is_none());
    assert!(serialized.get("token").is_none());
    assert!(serialized.get("password").is_none());
    assert!(serialized.get("private_key").is_none());
    assert!(serialized.get("raw_credentials").is_none());
}

#[test]
fn wrapper_execution_mode_is_bounded_by_enum() {
    let modes = [
        WrapperExecutionMode::ObserveOnly,
        WrapperExecutionMode::Enforce,
        WrapperExecutionMode::DryRun,
    ];

    assert_eq!(modes.len(), 3);
}

#[test]
fn external_system_schema_version_is_present_when_context_is_supplied() {
    let context = wrapper_context();

    assert_eq!(
        context.external_system_schema_version.as_str(),
        "metrics-api-v1"
    );
}

fn assert_wrapper_context(context: Option<&WrapperExecutionContext>) {
    let context = context.unwrap_or_else(|| panic!("wrapper context should be present"));

    assert_eq!(context.config.wrapper_name.as_str(), "credential_scope");
    assert_eq!(context.config.wrapper_version.as_str(), "1.0.0");
    assert_eq!(context.config.target_system.as_str(), "metrics");
    assert_eq!(
        context
            .config
            .config_digest
            .as_ref()
            .map(|digest| digest.as_str()),
        Some("sha256:wrapper-config")
    );
    assert_eq!(context.redaction_profile.as_str(), "standard-redaction");
    assert_eq!(context.execution_mode, WrapperExecutionMode::Enforce);
    assert!(context.credential_injection_required);
}

fn process_request(
    policy_decision: PolicyDecision,
    wrapper_context: Option<WrapperExecutionContext>,
) -> aegis::gateway::GatewayEntrypointResult {
    Gateway::process_entrypoint_request(
        &read_fixture("schemas/examples/valid/ToolCallRequest.json"),
        GatewayEntrypointContext {
            supported_tools: SupportedTools::from_names(["metrics.read"]),
            policy_decision,
            response_metadata: response_metadata(),
            audit_metadata: audit_metadata(),
            idempotency_context: None,
            wrapper_context,
            execution_identity_context: None,
            approval_context: None,
        },
    )
}

fn wrapper_context() -> WrapperExecutionContext {
    serde_json::from_value(serde_json::json!({
        "config": {
            "wrapper_name": "credential_scope",
            "wrapper_version": "1.0.0",
            "target_system": "metrics",
            "config_reference": "wrappers/credential_scope",
            "config_digest": "sha256:wrapper-config"
        },
        "external_system_schema_version": "metrics-api-v1",
        "redaction_profile": "standard-redaction",
        "execution_mode": "enforce",
        "credential_injection_required": true
    }))
    .unwrap_or_else(|error| panic!("wrapper context should parse: {error}"))
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
