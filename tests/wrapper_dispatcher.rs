use std::collections::BTreeMap;

use aegis::gateway::{
    ToolCallRequest, WrapperDispatchError, WrapperDispatcher, WrapperExecutionContext,
    WrapperExecutionError, WrapperExecutionMode, WrapperExecutionOutput, WrapperExecutionStatus,
    WrapperExecutor,
};
use serde_json::Value;

#[test]
fn dispatcher_routes_to_matching_wrapper() {
    let wrapper = TestWrapper::new("credential_scope", "1.0.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request(), &wrapper_context(WrapperExecutionMode::Enforce))
        .unwrap_or_else(|error| panic!("matching wrapper should execute: {error:?}"));

    assert_eq!(result.status, WrapperExecutionStatus::Executed);
    assert_eq!(
        result.context.config.wrapper_name.as_str(),
        "credential_scope"
    );
    assert_eq!(
        result
            .result
            .as_ref()
            .and_then(|value| value.get("wrapper"))
            .and_then(Value::as_str),
        Some("credential_scope")
    );
}

#[test]
fn missing_wrapper_fails_closed() {
    let dispatcher = WrapperDispatcher::new([]);
    let error = dispatcher
        .dispatch(&request(), &wrapper_context(WrapperExecutionMode::Enforce))
        .unwrap_err();

    assert_eq!(error.reason_code(), "wrapper_missing");
    assert!(error.safe_message().contains("credential_scope"));
}

#[test]
fn incompatible_wrapper_version_fails_closed() {
    let wrapper = TestWrapper::new("credential_scope", "0.9.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let error = dispatcher
        .dispatch(&request(), &wrapper_context(WrapperExecutionMode::Enforce))
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
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let error = dispatcher
        .dispatch(&request(), &wrapper_context(WrapperExecutionMode::Enforce))
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
fn dispatcher_does_not_decide_policy_or_inject_credentials() {
    let wrapper = TestWrapper::new("credential_scope", "1.0.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request(), &wrapper_context(WrapperExecutionMode::Enforce))
        .unwrap_or_else(|error| panic!("matching wrapper should execute: {error:?}"));
    let serialized = serde_json::to_value(result)
        .unwrap_or_else(|error| panic!("wrapper result should serialize: {error}"));

    assert!(serialized.get("policy_decision").is_none());
    assert!(serialized.to_string().find("raw_credentials").is_none());
}

fn assert_dispatch_status(mode: WrapperExecutionMode, status: WrapperExecutionStatus) {
    let wrapper = TestWrapper::new("credential_scope", "1.0.0");
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);
    let result = dispatcher
        .dispatch(&request(), &wrapper_context(mode))
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

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
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
        "credential_scope"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Err(WrapperExecutionError {
            reason_code: Some("wrapper_failed".to_string()),
            safe_message: "Wrapper execution failed.".to_string(),
        })
    }
}

fn request() -> ToolCallRequest {
    serde_json::from_str(
        &std::fs::read_to_string("schemas/examples/valid/ToolCallRequest.json")
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
            "wrapper_name": "credential_scope",
            "wrapper_version": "1.0.0",
            "target_system": "metrics",
            "config_reference": "wrappers/credential_scope",
            "config_digest": "sha256:wrapper-config"
        },
        "external_system_schema_version": "metrics-api-v1",
        "redaction_profile": "standard-redaction",
        "execution_mode": mode,
        "credential_injection_required": true
    }))
    .unwrap_or_else(|error| panic!("wrapper context should parse: {error}"))
}
