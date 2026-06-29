use std::{collections::BTreeMap, fs, path::Path};

use aegis::{
    auth::{
        CredentialBoundaryStatus, CredentialClass, CredentialRequirement, ExecutionAuthorization,
    },
    error::{ErrorCode, ErrorLocation},
    gateway::{
        GatewayStatus, ToolCallRequest, ToolCallResponse, WrapperDispatcher,
        WrapperExecutionContext, WrapperExecutionError, WrapperExecutionOutput, WrapperExecutor,
    },
    runtime::local::{
        process_local_gateway_request, process_local_gateway_request_with_wrapper_registry,
    },
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn health_check_requires_no_credentials() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let boundary = output
        .credential_boundary
        .expect("health check should record credential boundary evidence");

    assert!(!boundary.credential_required);
    assert_eq!(boundary.credential_class, Some(CredentialClass::None));
    assert_eq!(
        boundary.credential_boundary_status,
        CredentialBoundaryStatus::Satisfied
    );
}

#[test]
fn wrapper_requiring_credentials_fails_without_authorization() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&LocalRuntimeCredentialWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert_eq!(
        output.response.reason_code.as_deref(),
        Some("credentials_required_without_authorization")
    );
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.code),
        Some(&ErrorCode::CredentialsRequiredWithoutAuthorization)
    );
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.location),
        Some(&ErrorLocation::CredentialBoundary)
    );
    assert!(output.wrapper_execution.is_none());
}

#[test]
fn wrapper_requiring_local_runtime_succeeds_when_authorized() {
    let request = health_request();
    let context = wrapper_context("health.check", "1.0.0");
    let mut authorization = authorization(&request, &context);
    authorization.authorized_credential_class = CredentialClass::LocalRuntime;
    let dispatcher =
        WrapperDispatcher::new([&LocalRuntimeCredentialWrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_or_else(|error| {
            panic!("authorized local runtime wrapper should dispatch: {error:?}")
        });

    assert_eq!(
        result.credential_boundary.credential_boundary_status,
        CredentialBoundaryStatus::Satisfied
    );
    assert!(result.credential_boundary.credential_required);
    assert_eq!(
        result.credential_boundary.credential_class,
        Some(CredentialClass::LocalRuntime)
    );
}

#[test]
fn credential_mismatch_fails_closed() {
    let request = health_request();
    let context = wrapper_context("health.check", "1.0.0");
    let mut authorization = authorization(&request, &context);
    authorization.authorized_credential_class = CredentialClass::LocalRuntime;
    let dispatcher =
        WrapperDispatcher::new([&InvalidCredentialClassWrapper as &dyn WrapperExecutor]);
    let error = dispatcher
        .dispatch(&request, &context, &authorization)
        .expect_err("credential class mismatch should fail closed");

    assert_eq!(error.reason_code(), "credential_class_mismatch");
}

#[test]
fn missing_credential_class_fails_closed() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&MissingCredentialClassWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert_eq!(
        output.response.reason_code.as_deref(),
        Some("credential_class_missing")
    );
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.code),
        Some(&ErrorCode::CredentialClassMissing)
    );
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.location),
        Some(&ErrorLocation::CredentialBoundary)
    );
}

#[test]
fn credential_class_appears_in_audit_evidence() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let boundary = output
        .audit_record
        .details
        .credential_boundary
        .expect("audit should include credential boundary evidence");

    assert_eq!(boundary.credential_class, Some(CredentialClass::None));
    assert_eq!(
        boundary.credential_boundary_status,
        CredentialBoundaryStatus::Satisfied
    );
}

#[test]
fn credential_class_appears_in_runtime_output() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));

    assert_eq!(
        output
            .credential_boundary
            .as_ref()
            .and_then(|boundary| boundary.credential_class.clone()),
        Some(CredentialClass::None)
    );
}

#[test]
fn credential_boundary_output_contains_no_secrets() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let serialized = serde_json::to_string(&output)
        .unwrap_or_else(|error| panic!("runtime output should serialize: {error}"));

    assert!(!serialized.contains("password"));
    assert!(!serialized.contains("api_key"));
    assert!(!serialized.contains("bearer"));
    assert!(!serialized.contains("credential_value"));
    assert!(!serialized.contains("vault"));
}

#[test]
fn lifecycle_remains_deterministic_with_credential_boundary() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let states: Vec<_> = output
        .execution_lifecycle
        .transitions
        .iter()
        .map(|transition| transition.execution_state.clone())
        .collect();

    assert_eq!(states.len(), 6);
    assert!(output.credential_boundary.is_some());
}

#[test]
fn health_check_still_executes_after_credential_boundary() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));

    assert_eq!(output.response.status, GatewayStatus::Allowed);
    assert_eq!(
        output.response.result.as_ref().and_then(result_wrapper),
        Some("health.check")
    );
}

struct LocalRuntimeCredentialWrapper;

impl WrapperExecutor for LocalRuntimeCredentialWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement::local_runtime()
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Ok(WrapperExecutionOutput {
            result: Some(BTreeMap::from([(
                "wrapper".to_string(),
                Value::String("health.check".to_string()),
            )])),
        })
    }
}

struct InvalidCredentialClassWrapper;

impl WrapperExecutor for InvalidCredentialClassWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement {
            requires_credentials: true,
            credential_class: Some(CredentialClass::None),
        }
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        panic!("wrapper should not execute when credential class is incompatible")
    }
}

struct MissingCredentialClassWrapper;

impl WrapperExecutor for MissingCredentialClassWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement {
            requires_credentials: false,
            credential_class: None,
        }
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        panic!("wrapper should not execute when credential class is missing")
    }
}

fn authorization(
    request: &ToolCallRequest,
    context: &WrapperExecutionContext,
) -> ExecutionAuthorization {
    ExecutionAuthorization::policy_allow(request, &response(), context)
        .unwrap_or_else(|error| panic!("authorization should be valid: {error:?}"))
}

fn response() -> ToolCallResponse {
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

fn health_request() -> ToolCallRequest {
    serde_json::from_str(&health_request_json())
        .unwrap_or_else(|error| panic!("health request should parse: {error}"))
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

fn result_wrapper(value: &BTreeMap<String, Value>) -> Option<&str> {
    value.get("wrapper").and_then(Value::as_str)
}

fn health_request_json() -> String {
    fs::read_to_string("schemas/examples/valid/HealthCheckRequest.json")
        .unwrap_or_else(|error| panic!("health request fixture should be readable: {error}"))
}
