use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use aegis::{
    auth::{CredentialRequirement, ExecutionAuthorization},
    gateway::{
        ToolCallRequest, WrapperExecutionContext, WrapperExecutionError, WrapperExecutionOutput,
        WrapperExecutor,
    },
    runtime::local::{process_local_gateway_request_with_wrapper_registry, LocalRuntimeOutput},
    state::{
        valid_transition, ExecutionLifecycle, ExecutionState, ExecutionStateLogContext,
        ExecutionStateSink, ExecutionStateWriter, ExecutionTransition,
    },
    wrappers::HealthCheckWrapper,
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn state_log_file_is_created_when_requested() {
    let paths = case_paths("state_log_file_is_created_when_requested");

    run_gateway_success(&health_request(), &paths, |_| {});

    assert!(paths.state_log.is_file());
    assert!(!state_records(&paths.state_log).is_empty());
}

#[test]
fn health_check_writes_ordered_lifecycle_transitions() {
    let paths = case_paths("health_check_writes_ordered_lifecycle_transitions");

    run_gateway_success(&health_request(), &paths, |_| {});

    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "authorized",
            "dispatching",
            "executed",
            "audited",
            "completed",
        ],
    );
}

#[test]
fn sandbox_note_write_writes_ordered_lifecycle_transitions() {
    let paths = case_paths("sandbox_note_write_writes_ordered_lifecycle_transitions");
    fs::create_dir_all(&paths.sandbox)
        .unwrap_or_else(|error| panic!("sandbox directory should create: {error}"));

    run_gateway_success(&sandbox_note_request(), &paths, |command| {
        command.arg("--sandbox-dir").arg(&paths.sandbox);
    });

    assert_eq!(
        fs::read_to_string(paths.sandbox.join("notes").join("example-note.txt"))
            .unwrap_or_else(|error| panic!("sandbox note should be readable: {error}")),
        "hello from aegis"
    );
    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "authorized",
            "dispatching",
            "executed",
            "audited",
            "completed",
        ],
    );
}

#[test]
fn malformed_request_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("malformed_request_writes_fail_closed_lifecycle_evidence");

    run_gateway_success(
        &read_fixture("schemas/examples/invalid/ToolCallRequest.json"),
        &paths,
        |_| {},
    );

    assert_new_states(&state_records(&paths.state_log), &["failed_closed"]);
}

#[test]
fn denied_policy_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("denied_policy_writes_fail_closed_lifecycle_evidence");

    run_gateway_success(
        &request_with_tool_and_capability("email.send", "L1"),
        &paths,
        |_| {},
    );

    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "failed_closed",
        ],
    );
}

#[test]
fn pending_policy_writes_terminal_non_execution_lifecycle_evidence() {
    let paths = case_paths("pending_policy_writes_terminal_non_execution_lifecycle_evidence");

    run_gateway_success(
        &request_with_tool_and_capability("deploy.prod", "L2"),
        &paths,
        |_| {},
    );

    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "failed_closed",
        ],
    );
}

#[test]
fn invalid_bundle_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("invalid_bundle_writes_fail_closed_lifecycle_evidence");
    let bundle = bundle_without_manifest("invalid_bundle_writes_fail_closed_lifecycle_evidence");

    run_gateway_success_with_bundle(&health_request(), &paths, &bundle, |_| {});

    assert_new_states(
        &state_records(&paths.state_log),
        &["validated", "failed_closed"],
    );
}

#[test]
fn checksum_mismatch_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("checksum_mismatch_writes_fail_closed_lifecycle_evidence");
    let bundle =
        checksum_mismatch_bundle("checksum_mismatch_writes_fail_closed_lifecycle_evidence");

    run_gateway_success_with_bundle(&health_request(), &paths, &bundle, |_| {});

    assert_new_states(
        &state_records(&paths.state_log),
        &["validated", "failed_closed"],
    );
}

#[test]
fn invalid_signature_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("invalid_signature_writes_fail_closed_lifecycle_evidence");
    let bundle =
        signature_mismatch_bundle("invalid_signature_writes_fail_closed_lifecycle_evidence");

    run_gateway_success_with_bundle(&health_request(), &paths, &bundle, |_| {});

    assert_new_states(
        &state_records(&paths.state_log),
        &["validated", "failed_closed"],
    );
}

#[test]
fn missing_wrapper_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("missing_wrapper_writes_fail_closed_lifecycle_evidence");
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[],
        None,
    );

    write_output_state_log(&output, &paths);

    assert_output_ended_failed_closed(&output);
    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "authorized",
            "dispatching",
            "failed_closed",
        ],
    );
}

#[test]
fn wrapper_version_mismatch_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("wrapper_version_mismatch_writes_fail_closed_lifecycle_evidence");
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&HealthCheckWrapper],
        Some(wrapper_context("health.check", "2.0.0")),
    );

    write_output_state_log(&output, &paths);

    assert_output_ended_failed_closed(&output);
    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "failed_closed",
        ],
    );
}

#[test]
fn authorization_failure_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("authorization_failure_writes_fail_closed_lifecycle_evidence");
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&HealthCheckWrapper],
        Some(wrapper_context("metrics.read", "1.0.0")),
    );

    write_output_state_log(&output, &paths);

    assert_output_ended_failed_closed(&output);
    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "failed_closed",
        ],
    );
}

#[test]
fn credential_boundary_failure_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("credential_boundary_failure_writes_fail_closed_lifecycle_evidence");
    let output = process_local_gateway_request_with_wrapper_registry(
        &health_request(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&LocalRuntimeCredentialWrapper],
        Some(wrapper_context("health.check", "1.0.0")),
    );

    write_output_state_log(&output, &paths);

    assert_output_ended_failed_closed(&output);
    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "authorized",
            "dispatching",
            "failed_closed",
        ],
    );
}

#[test]
fn wrapper_failure_writes_fail_closed_lifecycle_evidence() {
    let paths = case_paths("wrapper_failure_writes_fail_closed_lifecycle_evidence");

    run_gateway_success(&sandbox_note_request(), &paths, |_| {});

    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "authorized",
            "dispatching",
            "failed_closed",
        ],
    );
}

#[test]
fn audit_persistence_failure_does_not_log_completed_state() {
    let paths = case_paths("audit_persistence_failure_does_not_log_completed_state");
    fs::create_dir_all(&paths.audit_log)
        .unwrap_or_else(|error| panic!("audit directory should create: {error}"));

    let output = run_gateway_raw(&health_request(), &paths, |_| {});
    let body = json_body(&output);

    assert!(!output.status.success());
    assert_eq!(body["error_report"]["code"], "audit_persistence_failed");
    assert_new_states(
        &state_records(&paths.state_log),
        &[
            "validated",
            "bundle_verified",
            "policy_evaluated",
            "authorized",
            "dispatching",
            "executed",
            "audit_failed",
        ],
    );
    assert!(!state_log_content(&paths.state_log).contains("\"completed\""));
}

#[test]
fn state_log_append_preserves_previous_entries() {
    let paths = case_paths("state_log_append_preserves_previous_entries");
    fs::write(&paths.state_log, "{\"existing\":true}\n")
        .unwrap_or_else(|error| panic!("seed state log should write: {error}"));

    run_gateway_success(&health_request(), &paths, |_| {});

    let lines = state_lines(&paths.state_log);
    assert_eq!(
        lines.first().map(String::as_str),
        Some("{\"existing\":true}")
    );
    assert_eq!(lines.len(), 9);
}

#[test]
fn invalid_state_log_path_fails_closed_with_structured_error() {
    let paths = case_paths("invalid_state_log_path_fails_closed_with_structured_error");
    fs::create_dir_all(&paths.state_log)
        .unwrap_or_else(|error| panic!("state log directory should create: {error}"));

    let output = run_gateway_raw(&health_request(), &paths, |_| {});
    let body = json_body(&output);

    assert!(!output.status.success());
    assert_eq!(body["code"], "state_log_open_failed");
    assert_eq!(body["location"], "execution_state_log");
}

#[test]
fn state_log_records_are_valid_json() {
    let paths = case_paths("state_log_records_are_valid_json");

    run_gateway_success(&health_request(), &paths, |_| {});

    for line in state_lines(&paths.state_log) {
        let _: Value = serde_json::from_str(&line)
            .unwrap_or_else(|error| panic!("state JSONL line should parse: {error}"));
    }
}

#[test]
fn state_log_does_not_contain_secret_material() {
    let paths = case_paths("state_log_does_not_contain_secret_material");

    fs::create_dir_all(&paths.sandbox)
        .unwrap_or_else(|error| panic!("sandbox directory should create: {error}"));
    run_gateway_success(&sandbox_note_request(), &paths, |command| {
        command.arg("--sandbox-dir").arg(&paths.sandbox);
    });

    let content = state_log_content(&paths.state_log).to_lowercase();
    for forbidden in [
        "private key",
        "begin private",
        "bearer",
        "password",
        "raw_credentials",
        "runtime_credential",
        "approval_token",
    ] {
        assert!(!content.contains(forbidden));
    }
}

#[test]
fn runtime_works_unchanged_when_state_log_is_omitted() {
    let output = run_gateway_without_state_log(&health_request());
    let body = json_body(&output);

    assert!(output.status.success());
    assert_eq!(body["response"]["status"], "allowed");
}

#[test]
fn audit_log_and_state_log_are_separate_files() {
    let paths = case_paths("audit_log_and_state_log_are_separate_files");

    run_gateway_success(&health_request(), &paths, |_| {});

    assert_ne!(paths.audit_log, paths.state_log);
    assert_eq!(audit_records(&paths.audit_log).len(), 1);
    assert_eq!(state_records(&paths.state_log).len(), 8);
    assert!(audit_log_content(&paths.audit_log).contains("\"event_type\""));
    assert!(state_log_content(&paths.state_log).contains("\"new_state\""));
    assert_state_log_has_no_full_audit_records(&state_records(&paths.state_log));
}

#[test]
fn invalid_lifecycle_transitions_are_not_written() {
    let paths = case_paths("invalid_lifecycle_transitions_are_not_written");
    let writer = ExecutionStateWriter::new(paths.state_log.clone());
    let lifecycle = ExecutionLifecycle {
        execution_state: ExecutionState::Created,
        transitions: vec![ExecutionTransition {
            previous_state: ExecutionState::PolicyEvaluated,
            execution_state: ExecutionState::Created,
        }],
    };

    let result = writer.append_lifecycle(
        &lifecycle,
        &ExecutionStateLogContext {
            execution_id: "local_exec_001".to_string(),
            ..ExecutionStateLogContext::default()
        },
    );

    assert!(result.is_err());
    assert!(!paths.state_log.exists());
}

#[test]
fn terminal_states_do_not_transition_further() {
    for terminal in [
        ExecutionState::Completed,
        ExecutionState::FailedClosed,
        ExecutionState::AuditFailed,
    ] {
        for next in all_execution_states() {
            assert!(
                !valid_transition(&terminal, &next),
                "terminal state {terminal:?} must not transition to {next:?}"
            );
        }
    }
}

#[test]
fn lifecycle_indexes_are_strictly_ordered() {
    let paths = case_paths("lifecycle_indexes_are_strictly_ordered");

    run_gateway_success(&health_request(), &paths, |_| {});

    for (index, record) in state_records(&paths.state_log).iter().enumerate() {
        assert_eq!(record["lifecycle_index"], index);
    }
}

#[test]
fn state_log_uses_only_known_execution_state_names() {
    let paths = case_paths("state_log_uses_only_known_execution_state_names");

    run_gateway_success(&health_request(), &paths, |_| {});

    let known_states = known_state_names();
    for record in state_records(&paths.state_log) {
        assert!(known_states.contains(&record["previous_state"].as_str().unwrap()));
        assert!(known_states.contains(&record["new_state"].as_str().unwrap()));
    }
}

struct CasePaths {
    audit_log: PathBuf,
    state_log: PathBuf,
    sandbox: PathBuf,
}

fn run_gateway_success(
    input: &str,
    paths: &CasePaths,
    configure: impl FnOnce(&mut Command),
) -> Value {
    let output = run_gateway_raw(input, paths, configure);
    assert_runtime_success(output)
}

fn run_gateway_success_with_bundle(
    input: &str,
    paths: &CasePaths,
    bundle_path: &Path,
    configure: impl FnOnce(&mut Command),
) -> Value {
    let output = run_gateway_raw_with_bundle(input, paths, bundle_path, configure);
    assert_runtime_success(output)
}

fn assert_runtime_success(output: Output) -> Value {
    assert!(
        output.status.success(),
        "runtime should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    json_body(&output)
}

fn run_gateway_raw(input: &str, paths: &CasePaths, configure: impl FnOnce(&mut Command)) -> Output {
    run_gateway_raw_with_bundle(input, paths, Path::new(LOCAL_DEV_BUNDLE), configure)
}

fn run_gateway_raw_with_bundle(
    input: &str,
    paths: &CasePaths,
    bundle_path: &Path,
    configure: impl FnOnce(&mut Command),
) -> Output {
    run_gateway_command_with_bundle(input, bundle_path, |command| {
        command.arg("--audit-log").arg(&paths.audit_log);
        command.arg("--state-log").arg(&paths.state_log);
        configure(command);
    })
}

fn run_gateway_without_state_log(input: &str) -> Output {
    run_gateway_command_with_bundle(input, Path::new(LOCAL_DEV_BUNDLE), |_| {})
}

fn run_gateway_command_with_bundle(
    input: &str,
    bundle_path: &Path,
    configure: impl FnOnce(&mut Command),
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_aegis-gateway"));
    command.arg("--bundle").arg(bundle_path);
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

fn assert_new_states(records: &[Value], expected: &[&str]) {
    let actual: Vec<_> = records
        .iter()
        .map(|record| {
            record["new_state"]
                .as_str()
                .expect("new_state should be a string")
        })
        .collect();
    assert_eq!(actual, expected);

    for (index, record) in records.iter().enumerate() {
        assert_eq!(record["lifecycle_index"], index);
    }
    assert_logged_transitions_are_valid(records);
}

fn assert_logged_transitions_are_valid(records: &[Value]) {
    for record in records {
        let previous_state = execution_state_from_value(&record["previous_state"]);
        let new_state = execution_state_from_value(&record["new_state"]);

        assert!(
            valid_transition(&previous_state, &new_state),
            "state log must not write invalid transition {previous_state:?} -> {new_state:?}"
        );
    }
}

fn execution_state_from_value(value: &Value) -> ExecutionState {
    serde_json::from_value(value.clone())
        .unwrap_or_else(|error| panic!("state name should be known: {error}"))
}

fn assert_output_ended_failed_closed(output: &LocalRuntimeOutput) {
    assert_eq!(
        output.execution_lifecycle.execution_state,
        ExecutionState::FailedClosed
    );
}

fn assert_state_log_has_no_full_audit_records(records: &[Value]) {
    for record in records {
        assert!(record.get("event_type").is_none());
        assert!(record.get("audit_record_id").is_none());
        assert!(record.get("details").is_none());
        assert!(record.get("status").is_none());
    }
}

fn write_output_state_log(output: &LocalRuntimeOutput, paths: &CasePaths) {
    let writer = ExecutionStateWriter::new(paths.state_log.clone());
    writer
        .append_lifecycle(
            &output.execution_lifecycle,
            &ExecutionStateLogContext {
                execution_id: output.response.execution_id.as_str().to_string(),
                request_id: output
                    .response
                    .request_id
                    .as_ref()
                    .map(|request_id| request_id.as_str().to_string()),
                ..ExecutionStateLogContext::default()
            },
        )
        .unwrap_or_else(|error| panic!("direct runtime lifecycle should write: {error}"));
}

fn state_records(path: &Path) -> Vec<Value> {
    state_lines(path)
        .into_iter()
        .map(|line| {
            serde_json::from_str(&line)
                .unwrap_or_else(|error| panic!("state JSONL line should parse: {error}"))
        })
        .collect()
}

fn audit_records(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("audit log should be readable: {error}"))
        .lines()
        .map(|line| {
            serde_json::from_str(line)
                .unwrap_or_else(|error| panic!("audit JSONL line should parse: {error}"))
        })
        .collect()
}

fn state_lines(path: &Path) -> Vec<String> {
    state_log_content(path)
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

fn state_log_content(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("state log should be readable: {error}"))
}

fn audit_log_content(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("audit log should be readable: {error}"))
}

fn json_body(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("runtime stdout should be valid JSON: {error}"))
}

fn case_paths(name: &str) -> CasePaths {
    let root = Path::new("target")
        .join("execution-state-log-tests")
        .join(name);
    if root.exists() {
        fs::remove_dir_all(&root)
            .unwrap_or_else(|error| panic!("old state log fixture should remove: {error}"));
    }
    fs::create_dir_all(&root)
        .unwrap_or_else(|error| panic!("state log fixture should create: {error}"));

    CasePaths {
        audit_log: root.join("audit.jsonl"),
        state_log: root.join("state.jsonl"),
        sandbox: root.join("sandbox"),
    }
}

fn bundle_without_manifest(name: &str) -> PathBuf {
    let target = policy_bundle_copy(name);
    fs::remove_file(target.join("manifest.yaml"))
        .unwrap_or_else(|error| panic!("manifest should remove from bundle fixture: {error}"));
    target
}

fn checksum_mismatch_bundle(name: &str) -> PathBuf {
    let target = policy_bundle_copy(name);
    fs::write(
        target.join("gateway_policy.yaml"),
        "policy_version: 0.1.0-local\nrules:\n",
    )
    .unwrap_or_else(|error| panic!("gateway policy fixture should be writable: {error}"));
    target
}

fn signature_mismatch_bundle(name: &str) -> PathBuf {
    let target = policy_bundle_copy(name);
    let checksum_path = target.join("checksums").join("SHA256SUMS");
    let mut checksum_content = fs::read_to_string(&checksum_path)
        .unwrap_or_else(|error| panic!("checksum manifest should be readable: {error}"));
    checksum_content.push_str("# unsigned state log invariant test change\n");
    fs::write(checksum_path, checksum_content)
        .unwrap_or_else(|error| panic!("checksum manifest should be writable: {error}"));
    target
}

fn policy_bundle_copy(name: &str) -> PathBuf {
    let target = Path::new("target")
        .join("execution-state-log-policy-bundles")
        .join(name);
    if target.exists() {
        fs::remove_dir_all(&target)
            .unwrap_or_else(|error| panic!("old policy bundle fixture should remove: {error}"));
    }

    copy_dir(Path::new(LOCAL_DEV_BUNDLE), &target);
    target
}

fn copy_dir(source: &Path, target: &Path) {
    fs::create_dir_all(target)
        .unwrap_or_else(|error| panic!("target fixture directory should create: {error}"));

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

fn request_with_tool_and_capability(tool_name: &str, capability_class: &str) -> String {
    let mut request: Value = serde_json::from_str(&sandbox_note_request())
        .unwrap_or_else(|error| panic!("sandbox note request should parse: {error}"));
    request["tool"]["name"] = Value::String(tool_name.to_string());
    request["tool"]["capability_class"] = Value::String(capability_class.to_string());
    serde_json::to_string(&request)
        .unwrap_or_else(|error| panic!("modified request should serialize: {error}"))
}

fn health_request() -> String {
    read_fixture("schemas/examples/valid/HealthCheckRequest.json")
}

fn sandbox_note_request() -> String {
    read_fixture("schemas/examples/valid/SandboxNoteWriteRequest.json")
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
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

fn all_execution_states() -> [ExecutionState; 11] {
    [
        ExecutionState::Created,
        ExecutionState::Validated,
        ExecutionState::BundleVerified,
        ExecutionState::PolicyEvaluated,
        ExecutionState::Authorized,
        ExecutionState::Dispatching,
        ExecutionState::Executed,
        ExecutionState::Audited,
        ExecutionState::Completed,
        ExecutionState::FailedClosed,
        ExecutionState::AuditFailed,
    ]
}

fn known_state_names() -> [&'static str; 11] {
    [
        "created",
        "validated",
        "bundle_verified",
        "policy_evaluated",
        "authorized",
        "dispatching",
        "executed",
        "audited",
        "completed",
        "failed_closed",
        "audit_failed",
    ]
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
