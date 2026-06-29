use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use aegis::state::{
    ExecutionLifecycle, ExecutionState, ExecutionStateLogContext, ExecutionStateSink,
    ExecutionStateWriter, ExecutionTransition,
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
    assert!(
        output.status.success(),
        "runtime should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    json_body(&output)
}

fn run_gateway_raw(input: &str, paths: &CasePaths, configure: impl FnOnce(&mut Command)) -> Output {
    run_gateway_command(input, |command| {
        command.arg("--audit-log").arg(&paths.audit_log);
        command.arg("--state-log").arg(&paths.state_log);
        configure(command);
    })
}

fn run_gateway_without_state_log(input: &str) -> Output {
    run_gateway_command(input, |_| {})
}

fn run_gateway_command(input: &str, configure: impl FnOnce(&mut Command)) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_aegis-gateway"));
    command.arg("--bundle").arg(LOCAL_DEV_BUNDLE);
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
