use std::{
    io::Write,
    process::{Command, Stdio},
};

use serde_json::Value;

#[test]
fn valid_supported_request_returns_response_and_audit_json() {
    let output = run_gateway_with_stdin(&valid_request());

    assert_eq!(output["response"]["status"], "allowed");
    assert_eq!(output["response"]["decision"], "allow");
    assert_eq!(output["audit_record"]["status"], "allowed");
    assert_eq!(output["audit_record"]["event_type"], "policy_decision");
}

#[test]
fn malformed_json_fails_closed_with_denied_response_and_audit_record() {
    let output = run_gateway_with_stdin(&read_fixture(
        "schemas/examples/invalid/ToolCallRequest.json",
    ));

    assert_eq!(output["response"]["status"], "denied");
    assert_eq!(output["response"]["decision"], "deny");
    assert_eq!(output["response"]["reason_code"], "malformed_request");
    assert_eq!(output["audit_record"]["status"], "denied");
    assert_eq!(output["audit_record"]["event_type"], "validation_result");
}

#[test]
fn unsupported_tool_fails_closed_with_denied_response_and_audit_record() {
    let output = run_gateway_with_stdin(&request_with_tool("email.send"));

    assert_eq!(output["response"]["status"], "denied");
    assert_eq!(output["response"]["decision"], "deny");
    assert_eq!(output["response"]["reason_code"], "unsupported_tool");
    assert_eq!(output["audit_record"]["status"], "denied");
    assert_eq!(output["audit_record"]["event_type"], "policy_decision");
}

#[test]
fn policy_adapter_error_fails_closed_with_denied_response_and_audit_record() {
    let output = run_gateway_with_stdin(&request_with_tool("policy.error"));

    assert_eq!(output["response"]["status"], "denied");
    assert_eq!(output["response"]["decision"], "deny");
    assert_eq!(
        output["response"]["reason_code"],
        "local_policy_adapter_error"
    );
    assert_eq!(output["audit_record"]["status"], "denied");
    assert_eq!(output["audit_record"]["event_type"], "policy_decision");
}

#[test]
fn output_is_valid_json() {
    let output = run_gateway_with_stdin(&valid_request());

    assert!(output.get("response").is_some());
    assert!(output.get("audit_record").is_some());
}

#[test]
fn runtime_stdin_path_does_not_require_external_runtime_dependencies() {
    let output = run_gateway_with_stdin(&valid_request());

    assert_eq!(output["audit_record"]["component"], "local_gateway_mvp");
    assert!(output["response"]["result"].is_null());
}

fn run_gateway_with_stdin(input: &str) -> Value {
    let mut command = Command::new(env!("CARGO_BIN_EXE_aegis-gateway"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
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
        "runtime should fail closed in JSON instead of exiting unsuccessfully: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("runtime stdout should be valid JSON: {error}"))
}

fn request_with_tool(tool_name: &str) -> String {
    let mut request: Value = serde_json::from_str(&valid_request())
        .unwrap_or_else(|error| panic!("valid request fixture should parse: {error}"));

    request["tool"]["name"] = Value::String(tool_name.to_string());
    serde_json::to_string(&request)
        .unwrap_or_else(|error| panic!("modified request should serialize: {error}"))
}

fn valid_request() -> String {
    read_fixture("schemas/examples/valid/ToolCallRequest.json")
}

fn read_fixture(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
