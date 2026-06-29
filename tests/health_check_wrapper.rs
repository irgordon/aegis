use std::{collections::BTreeMap, fs, io::Write, path::Path};

use aegis::{
    gateway::{
        ToolCallRequest, WrapperExecutionContext, WrapperExecutionError, WrapperExecutionMode,
        WrapperExecutionOutput, WrapperExecutor,
    },
    runtime::local::{
        process_local_gateway_request, process_local_gateway_request_with_wrapper_registry,
    },
    wrappers::HealthCheckWrapper,
};
use serde_json::{json, Value};

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn health_check_executes_when_policy_allows() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));

    assert_eq!(
        output.response.status,
        aegis::gateway::GatewayStatus::Allowed
    );
    assert_eq!(
        output.response.result.as_ref().and_then(result_wrapper),
        Some("health.check")
    );
}

#[test]
fn health_check_output_appears_in_runtime_json() {
    let output = runtime_json(&health_request_json());

    assert_eq!(output["response"]["result"]["service"], "aegis-gateway");
    assert_eq!(output["response"]["result"]["status"], "healthy");
    assert_eq!(output["response"]["result"]["wrapper"], "health.check");
    assert_eq!(output["wrapper_execution"]["wrapper_name"], "health.check");
}

#[test]
fn health_check_execution_evidence_appears_in_audit_record() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let evidence = output
        .audit_record
        .details
        .wrapper_execution_evidence
        .expect("allowed health check should record wrapper execution evidence");

    assert_eq!(evidence.wrapper_name.as_str(), "health.check");
    assert_eq!(evidence.wrapper_version.as_str(), "1.0.0");
    assert_eq!(
        evidence.wrapper_execution_mode,
        WrapperExecutionMode::Enforce
    );
    assert_eq!(
        evidence
            .wrapper_result_summary
            .as_ref()
            .and_then(summary_wrapper),
        Some("health.check")
    );
}

#[test]
fn denied_policy_decision_does_not_execute_wrapper() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &request_with_tool_and_capability("email.send", "L1"),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&PanicWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_eq!(
        output.response.status,
        aegis::gateway::GatewayStatus::Denied
    );
    assert!(output.wrapper_execution.is_none());
    assert!(output
        .audit_record
        .details
        .wrapper_execution_evidence
        .is_none());
}

#[test]
fn pending_policy_decision_does_not_execute_wrapper() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &request_with_tool_and_capability("deploy.prod", "L2"),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&PanicWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_eq!(
        output.response.status,
        aegis::gateway::GatewayStatus::Pending
    );
    assert!(output.wrapper_execution.is_none());
    assert!(output
        .audit_record
        .details
        .wrapper_execution_evidence
        .is_none());
}

#[test]
fn missing_wrapper_fails_closed_with_structured_error() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_wrapper_failure(output, "wrapper_missing");
}

#[test]
fn wrapper_version_mismatch_fails_closed_with_structured_error() {
    let wrapper = HealthCheckWrapper;
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&wrapper],
        Some(wrapper_context("health.check", "2.0.0")),
    );

    assert_wrapper_failure(output, "wrapper_version_incompatible");
}

#[test]
fn wrapper_execution_error_fails_closed_with_structured_error() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&FailingHealthWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_wrapper_failure(output, "health_check_failed");
}

#[test]
fn health_check_performs_no_filesystem_mutation() {
    let marker = Path::new("target")
        .join("health-check-wrapper-tests")
        .join("filesystem-marker.txt");
    if let Some(parent) = marker.parent() {
        fs::create_dir_all(parent)
            .unwrap_or_else(|error| panic!("marker parent should be creatable: {error}"));
    }
    fs::write(&marker, "before")
        .unwrap_or_else(|error| panic!("marker should be writable before wrapper runs: {error}"));

    let before = fs::read_to_string(&marker)
        .unwrap_or_else(|error| panic!("marker should be readable before wrapper runs: {error}"));
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));
    let after = fs::read_to_string(&marker)
        .unwrap_or_else(|error| panic!("marker should be readable after wrapper runs: {error}"));

    assert_eq!(before, after);
    assert_eq!(
        output.response.result.as_ref().and_then(result_wrapper),
        Some("health.check")
    );
}

fn assert_wrapper_failure(output: aegis::runtime::local::LocalRuntimeOutput, reason_code: &str) {
    assert_eq!(
        output.response.status,
        aegis::gateway::GatewayStatus::Denied
    );
    assert_eq!(output.response.reason_code.as_deref(), Some(reason_code));
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.location),
        Some(&aegis::error::ErrorLocation::WrapperDispatch)
    );
    assert_eq!(
        output
            .audit_record
            .details
            .error_report
            .as_ref()
            .map(|report| &report.error_location),
        Some(&aegis::error::ErrorLocation::WrapperDispatch)
    );
    assert!(output.wrapper_execution.is_none());
}

fn runtime_json(input: &str) -> Value {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_aegis-gateway"))
        .arg("--bundle")
        .arg(LOCAL_DEV_BUNDLE)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("local gateway runtime should start: {error}"));

    command
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("stdin should be available"))
        .write_all(input.as_bytes())
        .unwrap_or_else(|error| panic!("request JSON should write to stdin: {error}"));

    let output = command
        .wait_with_output()
        .unwrap_or_else(|error| panic!("local gateway runtime should finish: {error}"));

    assert!(
        output.status.success(),
        "runtime should succeed for health check: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("runtime stdout should be valid JSON: {error}"))
}

struct PanicWrapper;

impl WrapperExecutor for PanicWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        panic!("wrapper should not execute for denied or pending decisions")
    }
}

struct FailingHealthWrapper;

impl WrapperExecutor for FailingHealthWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
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
            reason_code: Some("health_check_failed".to_string()),
            safe_message: "Health check wrapper failed.".to_string(),
        })
    }
}

fn result_wrapper(value: &BTreeMap<String, Value>) -> Option<&str> {
    value.get("wrapper").and_then(Value::as_str)
}

fn summary_wrapper(value: &BTreeMap<String, String>) -> Option<&str> {
    value.get("wrapper").map(String::as_str)
}

fn wrapper_context(wrapper_name: &str, wrapper_version: &str) -> WrapperExecutionContext {
    serde_json::from_value(json!({
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
