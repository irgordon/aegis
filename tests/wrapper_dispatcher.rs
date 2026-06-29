use std::collections::BTreeMap;

use aegis::{
    auth::{CredentialRequirement, ExecutionAuthorization},
    gateway::{
        ToolCallRequest, ToolCallResponse, WrapperDispatchError, WrapperDispatcher,
        WrapperExecutionContext, WrapperExecutionError, WrapperExecutionMode,
        WrapperExecutionOutput, WrapperExecutionStatus, WrapperExecutor,
    },
};
use serde_json::Value;

#[test]
fn dispatcher_routes_to_matching_wrapper() {
    let request = request();
    let context = wrapper_context(WrapperExecutionMode::Enforce);
    let authorization = authorization(&request, &context);
    let wrapper = TestWrapper::new("health.check", "1.0.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_or_else(|error| panic!("matching wrapper should execute: {error:?}"));

    assert_eq!(result.status, WrapperExecutionStatus::Executed);
    assert_eq!(result.context.config.wrapper_name.as_str(), "health.check");
    assert_eq!(
        result
            .result
            .as_ref()
            .and_then(|value| value.get("wrapper"))
            .and_then(Value::as_str),
        Some("health.check")
    );
}

#[test]
fn missing_wrapper_fails_closed() {
    let dispatcher = WrapperDispatcher::new([]);
    let request = request();
    let context = wrapper_context(WrapperExecutionMode::Enforce);
    let authorization = authorization(&request, &context);
    let error = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_err();

    assert_eq!(error.reason_code(), "wrapper_missing");
    assert!(error.safe_message().contains("health.check"));
}

#[test]
fn incompatible_wrapper_version_fails_closed() {
    let request = request();
    let context = wrapper_context(WrapperExecutionMode::Enforce);
    let authorization = authorization(&request, &context);
    let wrapper = TestWrapper::new("health.check", "0.9.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let error = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_err();

    assert_eq!(error.reason_code(), "wrapper_version_incompatible");
    assert!(matches!(
        error,
        WrapperDispatchError::IncompatibleWrapperVersion { .. }
    ));
}

#[test]
fn wrapper_execution_error_fails_closed() {
    let wrapper = FailingWrapper;
    let request = request();
    let context = wrapper_context(WrapperExecutionMode::Enforce);
    let authorization = authorization(&request, &context);
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let error = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_err();

    assert_eq!(error.reason_code(), "wrapper_failed");
    assert_eq!(error.safe_message(), "Wrapper execution failed.");
}

#[test]
fn dispatcher_maps_execution_mode_to_bounded_status() {
    assert_dispatch_status(
        WrapperExecutionMode::ObserveOnly,
        WrapperExecutionStatus::Observed,
    );
    assert_dispatch_status(
        WrapperExecutionMode::Enforce,
        WrapperExecutionStatus::Executed,
    );
    assert_dispatch_status(WrapperExecutionMode::DryRun, WrapperExecutionStatus::DryRun);
}

#[test]
fn dispatcher_does_not_decide_policy_or_expose_credentials() {
    let request = request();
    let context = wrapper_context(WrapperExecutionMode::Enforce);
    let authorization = authorization(&request, &context);
    let wrapper = TestWrapper::new("health.check", "1.0.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_or_else(|error| panic!("matching wrapper should execute: {error:?}"));
    let serialized = serde_json::to_value(result)
        .unwrap_or_else(|error| panic!("wrapper result should serialize: {error}"));

    assert!(serialized.get("policy_decision").is_none());
    assert!(serialized.to_string().find("raw_credentials").is_none());
}

fn assert_dispatch_status(mode: WrapperExecutionMode, status: WrapperExecutionStatus) {
    let request = request();
    let context = wrapper_context(mode);
    let authorization = authorization(&request, &context);
    let wrapper = TestWrapper::new("health.check", "1.0.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_or_else(|error| panic!("matching wrapper should execute: {error:?}"));

    assert_eq!(result.status, status);
}

struct TestWrapper {
    name: &'static str,
    version: &'static str,
}

impl TestWrapper {
    fn new(name: &'static str, version: &'static str) -> Self {
        Self { name, version }
    }
}

impl WrapperExecutor for TestWrapper {
    fn wrapper_name(&self) -> &str {
        self.name
    }

    fn wrapper_version(&self) -> &str {
        self.version
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement::none()
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
        _credential_injection: Option<&aegis::auth::CredentialInjectionResult>,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Ok(WrapperExecutionOutput {
            result: Some(BTreeMap::from([(
                "wrapper".to_string(),
                Value::String(self.name.to_string()),
            )])),
        })
    }
}

struct FailingWrapper;

impl WrapperExecutor for FailingWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement::none()
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
        _credential_injection: Option<&aegis::auth::CredentialInjectionResult>,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Err(WrapperExecutionError {
            reason_code: Some("wrapper_failed".to_string()),
            safe_message: "Wrapper execution failed.".to_string(),
        })
    }
}

fn request() -> ToolCallRequest {
    serde_json::from_str(
        &std::fs::read_to_string("schemas/examples/valid/HealthCheckRequest.json")
            .unwrap_or_else(|error| panic!("request fixture should be readable: {error}")),
    )
    .unwrap_or_else(|error| panic!("request fixture should parse: {error}"))
}

fn wrapper_context(mode: WrapperExecutionMode) -> WrapperExecutionContext {
    let mode = match mode {
        WrapperExecutionMode::ObserveOnly => "observe_only",
        WrapperExecutionMode::Enforce => "enforce",
        WrapperExecutionMode::DryRun => "dry_run",
    };

    serde_json::from_value(serde_json::json!({
        "config": {
            "wrapper_name": "health.check",
            "wrapper_version": "1.0.0",
            "target_system": "local",
            "config_reference": "builtins/health.check",
            "config_digest": "builtin:health.check@1.0.0"
        },
        "external_system_schema_version": "aegis-local-v1",
        "redaction_profile": "no-secrets",
        "execution_mode": mode,
        "credential_injection_required": false
    }))
    .unwrap_or_else(|error| panic!("wrapper context should parse: {error}"))
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
