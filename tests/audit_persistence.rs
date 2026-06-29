use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn audit_file_is_created() {
    let audit_log = audit_log_path("audit_file_is_created");
    let output = run_gateway_with_audit_log(&audit_log, &health_request());

    assert_eq!(output["response"]["status"], "allowed");
    assert!(audit_log.is_file());
    assert_eq!(audit_records(&audit_log).len(), 1);
}

#[test]
fn append_preserves_previous_entries() {
    let audit_log = audit_log_path("append_preserves_previous_entries");
    fs::write(&audit_log, "{\"existing\":true}\n")
        .unwrap_or_else(|error| panic!("seed audit log should be writable: {error}"));

    run_gateway_with_audit_log(&audit_log, &health_request());

    let lines = audit_lines(&audit_log);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "{\"existing\":true}");
}

#[test]
fn multiple_gateway_requests_append_multiple_records() {
    let audit_log = audit_log_path("multiple_gateway_requests_append_multiple_records");

    run_gateway_with_audit_log(&audit_log, &health_request());
    run_gateway_with_audit_log(
        &audit_log,
        &request_with_tool_and_capability("email.send", "L1"),
    );

    let records = audit_records(&audit_log);
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["status"], "allowed");
    assert_eq!(records[1]["status"], "denied");
}

#[test]
fn malformed_request_still_creates_audit_record() {
    let audit_log = audit_log_path("malformed_request_still_creates_audit_record");

    run_gateway_with_audit_log(
        &audit_log,
        &read_fixture("schemas/examples/invalid/ToolCallRequest.json"),
    );

    let records = audit_records(&audit_log);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["event_type"], "validation_result");
    assert_eq!(records[0]["reason_code"], "malformed_request");
}

#[test]
fn denied_request_creates_audit_record() {
    let audit_log = audit_log_path("denied_request_creates_audit_record");

    run_gateway_with_audit_log(
        &audit_log,
        &request_with_tool_and_capability("email.send", "L1"),
    );

    let records = audit_records(&audit_log);
    assert_eq!(records[0]["status"], "denied");
    assert_eq!(records[0]["reason_code"], "local_l1_denied");
}

#[test]
fn pending_request_creates_audit_record() {
    let audit_log = audit_log_path("pending_request_creates_audit_record");

    run_gateway_with_audit_log(
        &audit_log,
        &request_with_tool_and_capability("deploy.prod", "L2"),
    );

    let records = audit_records(&audit_log);
    assert_eq!(records[0]["status"], "pending");
    assert_eq!(records[0]["details"]["decision"], "pending_approval");
}

#[test]
fn invalid_audit_path_fails_closed() {
    let audit_dir = audit_case_dir("invalid_audit_path_fails_closed");
    fs::create_dir_all(&audit_dir)
        .unwrap_or_else(|error| panic!("audit fixture directory should be creatable: {error}"));

    let output = run_gateway_expect_failure(&audit_dir, &health_request());
    let body: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("failure stdout should be valid JSON: {error}"));

    assert!(!output.status.success());
    assert_eq!(body["error_report"]["code"], "audit_persistence_failed");
    assert_eq!(body["error_report"]["location"], "audit_persistence");
    assert!(body["error_report"]["next_action"].is_string());
}

#[test]
fn audit_records_remain_valid_json() {
    let audit_log = audit_log_path("audit_records_remain_valid_json");

    run_gateway_with_audit_log(&audit_log, &health_request());
    run_gateway_with_audit_log(&audit_log, &request_with_tool("unknown.tool"));

    for line in audit_lines(&audit_log) {
        let _: Value = serde_json::from_str(&line)
            .unwrap_or_else(|error| panic!("audit JSONL line should parse: {error}"));
    }
}

#[test]
fn audit_log_does_not_include_secret_material() {
    let audit_log = audit_log_path("audit_log_does_not_include_secret_material");

    run_gateway_with_audit_log(&audit_log, &valid_request());

    let content = fs::read_to_string(&audit_log)
        .unwrap_or_else(|error| panic!("audit log should be readable: {error}"));
    for forbidden in [
        "PRIVATE KEY",
        "BEGIN PRIVATE",
        "bearer",
        "password",
        "raw_credentials",
        "runtime_credential",
        "approval_token",
    ] {
        assert!(!content.to_lowercase().contains(&forbidden.to_lowercase()));
    }
}

#[test]
fn runtime_stdout_remains_unchanged() {
    let audit_log = audit_log_path("runtime_stdout_remains_unchanged");
    let without_audit = run_gateway_without_audit_log(&health_request());
    let with_audit = run_gateway_raw_with_audit_log(&audit_log, &health_request());

    assert_eq!(without_audit.stdout, with_audit.stdout);
}

fn run_gateway_with_audit_log(audit_log: &Path, input: &str) -> Value {
    let output = run_gateway_raw_with_audit_log(audit_log, input);
    assert!(
        output.status.success(),
        "runtime should succeed and append audit record: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("runtime stdout should be valid JSON: {error}"))
}

fn run_gateway_raw_with_audit_log(audit_log: &Path, input: &str) -> std::process::Output {
    run_gateway_command(input, |command| {
        command.arg("--audit-log").arg(audit_log);
    })
}

fn run_gateway_without_audit_log(input: &str) -> std::process::Output {
    let output = run_gateway_command(input, |_| {});
    assert!(
        output.status.success(),
        "runtime should succeed without audit logging: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn run_gateway_expect_failure(audit_log: &Path, input: &str) -> std::process::Output {
    run_gateway_raw_with_audit_log(audit_log, input)
}

fn run_gateway_command(input: &str, configure: impl FnOnce(&mut Command)) -> std::process::Output {
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

fn audit_records(path: &Path) -> Vec<Value> {
    audit_lines(path)
        .into_iter()
        .map(|line| {
            serde_json::from_str(&line)
                .unwrap_or_else(|error| panic!("audit JSONL line should parse: {error}"))
        })
        .collect()
}

fn audit_lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("audit log should be readable: {error}"))
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

fn audit_log_path(case_name: &str) -> PathBuf {
    let dir = audit_case_dir(case_name);
    fs::create_dir_all(&dir)
        .unwrap_or_else(|error| panic!("audit fixture directory should be creatable: {error}"));
    dir.join("audit.jsonl")
}

fn audit_case_dir(case_name: &str) -> PathBuf {
    let dir = Path::new("target")
        .join("audit-persistence-tests")
        .join(case_name);
    if dir.exists() {
        fs::remove_dir_all(&dir)
            .unwrap_or_else(|error| panic!("old audit fixture should be removable: {error}"));
    }
    dir
}

fn request_with_tool(tool_name: &str) -> String {
    request_with_tool_and_capability(tool_name, "L0")
}

fn request_with_tool_and_capability(tool_name: &str, capability_class: &str) -> String {
    let mut request: Value = serde_json::from_str(&valid_request())
        .unwrap_or_else(|error| panic!("valid request fixture should parse: {error}"));
    request["tool"]["name"] = Value::String(tool_name.to_string());
    request["tool"]["capability_class"] = Value::String(capability_class.to_string());
    serde_json::to_string(&request)
        .unwrap_or_else(|error| panic!("modified request should serialize: {error}"))
}

fn valid_request() -> String {
    read_fixture("schemas/examples/valid/ToolCallRequest.json")
}

fn health_request() -> String {
    read_fixture("schemas/examples/valid/HealthCheckRequest.json")
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
