use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aegis::{
    gateway::{
        ToolCallRequest, WrapperExecutionContext, WrapperExecutionError, WrapperExecutionOutput,
        WrapperExecutor,
    },
    runtime::local::{
        process_local_gateway_request, process_local_gateway_request_with_wrapper_registry,
    },
    state::{ExecutionLifecycle, ExecutionState},
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn successful_runtime_reports_complete_lifecycle() {
    let output = run_gateway_success(&health_request_json(), |command| {
        command.arg("--bundle").arg(LOCAL_DEV_BUNDLE);
    });
    let lifecycle = &output["execution_lifecycle"];

    assert_eq!(lifecycle["execution_state"], "completed");
    assert_transition_order(
        lifecycle,
        &[
            ("created", "validated"),
            ("validated", "bundle_verified"),
            ("bundle_verified", "policy_evaluated"),
            ("policy_evaluated", "dispatching"),
            ("dispatching", "executed"),
            ("executed", "audited"),
            ("audited", "completed"),
        ],
    );
    assert_eq!(
        output["audit_record"]["details"]["execution_lifecycle"]["execution_state"],
        "completed"
    );
}

#[test]
fn malformed_request_fails_closed_from_created() {
    let output = process_local_gateway_request(
        &read_fixture("schemas/examples/invalid/ToolCallRequest.json"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(&output.execution_lifecycle, &[ExecutionState::FailedClosed]);
}

#[test]
fn denied_policy_fails_closed_after_policy_evaluation() {
    let output = process_local_gateway_request(
        &request_with_tool_and_capability("email.send", "L1"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[
            ExecutionState::Validated,
            ExecutionState::BundleVerified,
            ExecutionState::PolicyEvaluated,
            ExecutionState::FailedClosed,
        ],
    );
}

#[test]
fn unsupported_tool_fails_closed_after_validation() {
    let output = process_local_gateway_request(
        &request_with_tool_and_capability("unknown.tool", "L0"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[ExecutionState::Validated, ExecutionState::FailedClosed],
    );
}

#[test]
fn invalid_bundle_fails_closed_after_validation() {
    let output = process_local_gateway_request(
        &health_request_json(),
        Path::new("examples/policy-bundles/missing"),
    );

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[ExecutionState::Validated, ExecutionState::FailedClosed],
    );
}

#[test]
fn checksum_failure_fails_closed_after_validation() {
    let bundle = checksum_mismatch_bundle("checksum_failure_fails_closed_after_validation");
    let output = process_local_gateway_request(&health_request_json(), &bundle);

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[ExecutionState::Validated, ExecutionState::FailedClosed],
    );
}

#[test]
fn signature_failure_fails_closed_after_validation() {
    let bundle = signature_mismatch_bundle("signature_failure_fails_closed_after_validation");
    let output = process_local_gateway_request(&health_request_json(), &bundle);

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[ExecutionState::Validated, ExecutionState::FailedClosed],
    );
}

#[test]
fn wrapper_dispatch_failure_fails_closed_from_dispatching() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[
            ExecutionState::Validated,
            ExecutionState::BundleVerified,
            ExecutionState::PolicyEvaluated,
            ExecutionState::Dispatching,
            ExecutionState::FailedClosed,
        ],
    );
}

#[test]
fn wrapper_execution_failure_fails_closed_from_dispatching() {
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&FailingHealthWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
    assert_states(
        &output.execution_lifecycle,
        &[
            ExecutionState::Validated,
            ExecutionState::BundleVerified,
            ExecutionState::PolicyEvaluated,
            ExecutionState::Dispatching,
            ExecutionState::FailedClosed,
        ],
    );
}

#[test]
fn audit_persistence_failure_reports_audit_failed() {
    let audit_dir = Path::new("target")
        .join("execution-lifecycle-tests")
        .join("audit_persistence_failure_reports_audit_failed");
    if audit_dir.exists() {
        fs::remove_dir_all(&audit_dir)
            .unwrap_or_else(|error| panic!("old audit fixture should be removable: {error}"));
    }
    fs::create_dir_all(&audit_dir)
        .unwrap_or_else(|error| panic!("audit fixture directory should be creatable: {error}"));

    let output = run_gateway_failure(&health_request_json(), |command| {
        command.arg("--bundle").arg(LOCAL_DEV_BUNDLE);
        command.arg("--audit-log").arg(&audit_dir);
    });
    let lifecycle = &output["execution_lifecycle"];

    assert_eq!(lifecycle["execution_state"], "audit_failed");
    assert_transition_order(
        lifecycle,
        &[
            ("created", "validated"),
            ("validated", "bundle_verified"),
            ("bundle_verified", "policy_evaluated"),
            ("policy_evaluated", "dispatching"),
            ("dispatching", "executed"),
            ("executed", "audit_failed"),
        ],
    );
}

#[test]
fn invalid_transition_is_rejected() {
    let mut lifecycle = ExecutionLifecycle::created();
    lifecycle
        .transition_to(ExecutionState::Validated)
        .expect("created to validated should be valid");

    let error = lifecycle
        .transition_to(ExecutionState::Created)
        .expect_err("validated to created should be invalid");

    assert_eq!(error.previous_state, ExecutionState::Validated);
    assert_eq!(error.attempted_state, ExecutionState::Created);
}

#[test]
fn transition_order_is_deterministic() {
    let mut lifecycle = ExecutionLifecycle::created();
    for state in [
        ExecutionState::Validated,
        ExecutionState::BundleVerified,
        ExecutionState::PolicyEvaluated,
        ExecutionState::Dispatching,
        ExecutionState::Executed,
        ExecutionState::Audited,
        ExecutionState::Completed,
    ] {
        lifecycle
            .transition_to(state)
            .expect("successful lifecycle transition should be valid");
    }

    let states: Vec<_> = lifecycle
        .transitions
        .iter()
        .map(|transition| transition.execution_state.clone())
        .collect();

    assert_eq!(
        states,
        vec![
            ExecutionState::Validated,
            ExecutionState::BundleVerified,
            ExecutionState::PolicyEvaluated,
            ExecutionState::Dispatching,
            ExecutionState::Executed,
            ExecutionState::Audited,
            ExecutionState::Completed,
        ]
    );
}

fn assert_states(lifecycle: &ExecutionLifecycle, expected: &[ExecutionState]) {
    let states: Vec<_> = lifecycle
        .transitions
        .iter()
        .map(|transition| transition.execution_state.clone())
        .collect();

    assert_eq!(states, expected);
}

fn assert_transition_order(lifecycle: &Value, expected: &[(&str, &str)]) {
    let transitions = lifecycle["transitions"]
        .as_array()
        .expect("lifecycle transitions should be an array");

    assert_eq!(transitions.len(), expected.len());
    for (transition, (previous, current)) in transitions.iter().zip(expected) {
        assert_eq!(transition["previous_state"], *previous);
        assert_eq!(transition["execution_state"], *current);
    }
}

fn run_gateway_success(input: &str, configure: impl FnOnce(&mut Command)) -> Value {
    let output = run_gateway(input, configure);
    assert!(
        output.status.success(),
        "runtime should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("runtime stdout should be valid JSON: {error}"))
}

fn run_gateway_failure(input: &str, configure: impl FnOnce(&mut Command)) -> Value {
    let output = run_gateway(input, configure);
    assert!(!output.status.success(), "runtime should fail");
    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("runtime stdout should be valid JSON: {error}"))
}

fn run_gateway(input: &str, configure: impl FnOnce(&mut Command)) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_aegis-gateway"));
    configure(&mut command);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .unwrap_or_else(|error| panic!("local gateway runtime should start: {error}"));
    child
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("stdin should be available"))
        .write_all(input.as_bytes())
        .unwrap_or_else(|error| panic!("request JSON should write to stdin: {error}"));

    child
        .wait_with_output()
        .unwrap_or_else(|error| panic!("local gateway runtime should finish: {error}"))
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

fn checksum_mismatch_bundle(case_name: &str) -> PathBuf {
    let target = mutable_bundle(case_name);
    append_to_file(
        target.join("gateway_policy.yaml"),
        "# unsigned policy change\n",
    );
    target
}

fn signature_mismatch_bundle(case_name: &str) -> PathBuf {
    let target = mutable_bundle(case_name);
    append_to_file(
        target.join("checksums").join("SHA256SUMS"),
        "# unsigned change\n",
    );
    target
}

fn mutable_bundle(case_name: &str) -> PathBuf {
    let target = Path::new("target")
        .join("execution-lifecycle-tests")
        .join(case_name);
    if target.exists() {
        fs::remove_dir_all(&target)
            .unwrap_or_else(|error| panic!("old mutable fixture should be removable: {error}"));
    }
    copy_dir(Path::new(LOCAL_DEV_BUNDLE), &target);
    target
}

fn copy_dir(source: &Path, target: &Path) {
    fs::create_dir_all(target)
        .unwrap_or_else(|error| panic!("target fixture directory should be creatable: {error}"));

    for entry in fs::read_dir(source)
        .unwrap_or_else(|error| panic!("source fixture directory should be readable: {error}"))
    {
        let entry =
            entry.unwrap_or_else(|error| panic!("fixture entry should be readable: {error}"));
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &target_path);
        } else {
            fs::copy(&source_path, &target_path)
                .unwrap_or_else(|error| panic!("fixture file should copy: {error}"));
        }
    }
}

fn append_to_file(path: PathBuf, text: &str) {
    let mut content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture should be readable: {error}"));
    content.push_str(text);
    fs::write(path, content).unwrap_or_else(|error| panic!("fixture should be writable: {error}"));
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
    read_fixture("schemas/examples/valid/HealthCheckRequest.json")
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
