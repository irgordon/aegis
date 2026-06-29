use std::{fs, path::Path};

use aegis::{
    auth::{
        AuthorizationError, AuthorizationStatus, CredentialClassRef, ExecutionAuthority,
        ExecutionAuthorization, ExecutionScope,
    },
    error::{ErrorCode, ErrorLocation},
    gateway::{CapabilityClass, GatewayStatus, ToolCallRequest, WrapperExecutionContext},
    runtime::local::{
        process_local_gateway_request, process_local_gateway_request_with_wrapper_registry,
    },
    state::ExecutionState,
    wrappers::HealthCheckWrapper,
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn allowed_request_produces_execution_authorization() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let authorization = output
        .execution_authorization
        .expect("allowed request should produce authorization");

    assert_eq!(
        authorization.authorization_status,
        AuthorizationStatus::Authorized
    );
    assert_eq!(
        authorization.authority_source,
        ExecutionAuthority::PolicyAllow
    );
    assert_eq!(
        authorization.credential_class_ref,
        CredentialClassRef::NoCredentialRequired
    );
    assert_eq!(
        authorization.execution_scope,
        Some(ExecutionScope::LocalGatewayHealth)
    );
    assert_eq!(authorization.binding.wrapper_name.as_str(), "health.check");
    assert_eq!(authorization.binding.wrapper_version.as_str(), "1.0.0");
    assert_eq!(authorization.binding.tool_name.as_str(), "health.check");
    assert_eq!(authorization.binding.capability_class, CapabilityClass::L0);
}

#[test]
fn denied_request_produces_no_authorization() {
    let output = process_local_gateway_request(
        &request_with_tool_and_capability("email.send", "L1"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert!(output.execution_authorization.is_none());
    assert!(output
        .audit_record
        .details
        .execution_authorization
        .is_none());
}

#[test]
fn pending_request_produces_no_authorization() {
    let output = process_local_gateway_request(
        &request_with_tool_and_capability("deploy.prod", "L2"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_eq!(output.response.status, GatewayStatus::Pending);
    assert!(output.execution_authorization.is_none());
    assert!(output
        .audit_record
        .details
        .execution_authorization
        .is_none());
}

#[test]
fn wrapper_mismatch_fails_closed_with_structured_error() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&HealthCheckWrapper],
        Some(wrapper_context("metrics.read", "1.0.0")),
    );

    assert_authorization_failure(output, "authorization_wrapper_mismatch");
}

#[test]
fn wrapper_version_mismatch_fails_closed_with_structured_error() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&HealthCheckWrapper],
        Some(wrapper_context("health.check", "2.0.0")),
    );

    assert_authorization_failure(output, "authorization_version_mismatch");
}

#[test]
fn capability_mismatch_fails_authorization_validation() {
    let request = health_request();
    let context = wrapper_context("health.check", "1.0.0");
    let response = response();
    let mut authorization = ExecutionAuthorization::policy_allow(&request, &response, &context)
        .unwrap_or_else(|error| panic!("authorization should be valid: {error:?}"));
    authorization.binding.capability_class = CapabilityClass::L1;

    let error = authorization
        .validate_for(&request, &context)
        .expect_err("capability mismatch should fail closed");

    assert!(matches!(
        error,
        AuthorizationError::CapabilityMismatch { .. }
    ));
    assert_eq!(error.reason_code(), "authorization_capability_mismatch");
}

#[test]
fn invalid_execution_scope_fails_closed() {
    let output = process_local_gateway_request(
        &request_with_tool_and_capability("metrics.read", "L0"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_authorization_failure(output, "authorization_scope_invalid");
}

#[test]
fn broad_execution_scope_is_not_representable() {
    let value = serde_json::json!("global");
    let result: Result<ExecutionScope, _> = serde_json::from_value(value);

    assert!(result.is_err());
}

#[test]
fn audit_contains_authorization_evidence() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let authorization = output
        .audit_record
        .details
        .execution_authorization
        .expect("audit should include authorization evidence");

    assert_eq!(
        authorization.authorization_status,
        AuthorizationStatus::Authorized
    );
    assert_eq!(authorization.binding.wrapper_name.as_str(), "health.check");
    assert_eq!(
        authorization.execution_scope,
        Some(ExecutionScope::LocalGatewayHealth)
    );
}

#[test]
fn lifecycle_contains_authorized_state_for_allowed_execution() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let states: Vec<_> = output
        .execution_lifecycle
        .transitions
        .iter()
        .map(|transition| transition.execution_state.clone())
        .collect();

    assert!(states.contains(&ExecutionState::Authorized));
}

#[test]
fn authorization_output_does_not_contain_secret_material() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let serialized = serde_json::to_string(&output.execution_authorization)
        .unwrap_or_else(|error| panic!("authorization should serialize: {error}"));

    assert!(!serialized.contains("password"));
    assert!(!serialized.contains("token"));
    assert!(!serialized.contains("secret"));
    assert!(!serialized.contains("credential_value"));
}

fn assert_authorization_failure(output: aegis::runtime::local::LocalRuntimeOutput, code: &str) {
    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert_eq!(output.response.reason_code.as_deref(), Some(code));
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.location),
        Some(&ErrorLocation::ExecutionAuthorization)
    );
    assert_eq!(
        output
            .error_report
            .as_ref()
            .map(|report| report.code.clone()),
        Some(expected_error_code(code))
    );
    assert_eq!(
        output
            .execution_authorization
            .as_ref()
            .map(|authorization| &authorization.authorization_status),
        Some(&AuthorizationStatus::Denied)
    );
    assert!(output.wrapper_execution.is_none());
    assert!(output
        .audit_record
        .details
        .execution_authorization
        .is_some());
}

fn expected_error_code(code: &str) -> ErrorCode {
    match code {
        "authorization_wrapper_mismatch" => ErrorCode::AuthorizationWrapperMismatch,
        "authorization_version_mismatch" => ErrorCode::AuthorizationVersionMismatch,
        "authorization_scope_invalid" => ErrorCode::AuthorizationScopeInvalid,
        _ => panic!("unexpected authorization error code: {code}"),
    }
}

fn health_request() -> ToolCallRequest {
    serde_json::from_str(&health_request_json())
        .unwrap_or_else(|error| panic!("health request should parse: {error}"))
}

fn response() -> aegis::gateway::ToolCallResponse {
    serde_json::from_value(serde_json::json!({
        "schema_version": "1.0",
        "execution_id": "local_exec_001",
        "request_id": "req_health_001",
        "status": "allowed",
        "decision": "allow",
        "result": null,
        "reason_code": null,
        "safe_message": null,
        "pending_reference": null,
        "replay_reference": null,
        "policy_provenance": {
            "bundle_id": "local-dev",
            "version": "0.1.0-local",
            "policy_hash": "sha256:local",
            "environment": "local",
            "signer_identity": "local",
            "activated_at": "2026-06-28T00:00:00Z"
        },
        "audit_record_id": "local_audit_001",
        "completed_at": "2026-06-28T00:00:00Z"
    }))
    .unwrap_or_else(|error| panic!("response should parse: {error}"))
}

fn wrapper_context(wrapper_name: &str, wrapper_version: &str) -> WrapperExecutionContext {
    serde_json::from_value(serde_json::json!({
        "config": {
            "wrapper_name": wrapper_name,
            "wrapper_version": wrapper_version,
            "target_system": "local",
            "config_reference": format!("builtins/{wrapper_name}"),
            "config_digest": format!("builtin:{wrapper_name}@{wrapper_version}")
        },
        "external_system_schema_version": "aegis-local-v1",
        "redaction_profile": "no-secrets",
        "execution_mode": "enforce",
        "credential_injection_required": false
    }))
    .unwrap_or_else(|error| panic!("wrapper context should parse: {error}"))
}

fn request_with_tool_and_capability(tool_name: &str, capability_class: &str) -> String {
    let mut request: Value = serde_json::from_str(&health_request_json())
        .unwrap_or_else(|error| panic!("health request fixture should parse: {error}"));
    request["tool"]["name"] = Value::String(tool_name.to_string());
    request["tool"]["capability_class"] = Value::String(capability_class.to_string());
    serde_json::to_string(&request)
        .unwrap_or_else(|error| panic!("modified request should serialize: {error}"))
}

fn health_request_json() -> String {
    fs::read_to_string("schemas/examples/valid/HealthCheckRequest.json")
        .unwrap_or_else(|error| panic!("health request fixture should be readable: {error}"))
}
