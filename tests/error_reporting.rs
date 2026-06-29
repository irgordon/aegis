use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aegis::{
    auth::ExecutionAuthorization,
    error::{ErrorCode, ErrorLocation, GatewayErrorReport},
    gateway::{
        ToolCallRequest, ToolCallResponse, WrapperDispatcher, WrapperExecutionContext,
        WrapperExecutionMode,
    },
    runtime::local::process_local_gateway_request,
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn malformed_request_returns_structured_error_report() {
    let output = process_local_gateway_request(
        &read_fixture("schemas/examples/invalid/ToolCallRequest.json"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_error_report(
        &json_value(&output.error_report),
        "malformed_request",
        "request_validation",
    );
    assert_eq!(
        output.audit_record.details.error_report.unwrap().error_code,
        ErrorCode::MalformedRequest
    );
}

#[test]
fn missing_policy_bundle_returns_structured_error_report() {
    let output = process_local_gateway_request(
        &valid_request(),
        Path::new("examples/policy-bundles/missing"),
    );

    assert_error_report(
        &json_value(&output.error_report),
        "policy_bundle_verification_failed",
        "policy_bundle_verification",
    );
}

#[test]
fn invalid_checksum_returns_structured_error_report() {
    let bundle = checksum_mismatch_bundle("invalid_checksum_returns_structured_error_report");
    let output = process_local_gateway_request(&valid_request(), &bundle);

    assert_error_report(
        &json_value(&output.error_report),
        "policy_bundle_verification_failed",
        "policy_bundle_verification",
    );
    assert_eq!(
        output
            .audit_record
            .details
            .error_report
            .unwrap()
            .error_location,
        ErrorLocation::PolicyBundleVerification
    );
}

#[test]
fn invalid_signature_returns_structured_error_report() {
    let bundle = signature_mismatch_bundle("invalid_signature_returns_structured_error_report");
    let output = process_local_gateway_request(&valid_request(), &bundle);

    assert_error_report(
        &json_value(&output.error_report),
        "policy_bundle_verification_failed",
        "policy_bundle_verification",
    );
}

#[test]
fn policy_evaluation_failure_returns_structured_error_report() {
    let output = process_local_gateway_request(
        &request_with_tool("storage.read"),
        Path::new(LOCAL_DEV_BUNDLE),
    );

    assert_error_report(
        &json_value(&output.error_report),
        "policy_evaluation_failed",
        "policy_evaluation",
    );
    assert_eq!(
        output
            .policy_evaluation
            .as_ref()
            .and_then(|evaluation| evaluation.failure_reason.as_ref())
            .map(|failure| format!("{failure:?}")),
        Some("NoMatchingPolicyRule".to_string())
    );
}

#[test]
fn wrapper_dispatch_failure_returns_structured_error_report() {
    let dispatcher = WrapperDispatcher::new([]);
    let context = wrapper_context();
    let request = request();
    let authorization = authorization(&request, &context);
    let error = dispatcher
        .dispatch(&request, &context, &authorization)
        .unwrap_err();
    let report = GatewayErrorReport::wrapper_dispatch_failed(&error, &context);
    let value = json_value(&Some(report));

    assert_error_report(&value, "wrapper_dispatch_failed", "wrapper_dispatch");
    assert_eq!(value["wrapper_name"], "health.check");
}

#[test]
fn audit_persistence_failure_returns_structured_error_report() {
    let audit_dir = Path::new("target")
        .join("error-reporting-tests")
        .join("audit_persistence_failure_returns_structured_error_report");
    if audit_dir.exists() {
        fs::remove_dir_all(&audit_dir)
            .unwrap_or_else(|error| panic!("old audit fixture should be removable: {error}"));
    }
    fs::create_dir_all(&audit_dir)
        .unwrap_or_else(|error| panic!("audit fixture directory should be creatable: {error}"));

    let output = run_gateway_expect_failure(&valid_request(), |command| {
        command.arg("--audit-log").arg(&audit_dir);
    });
    let value: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("failure stdout should be valid JSON: {error}"));

    assert!(!output.status.success());
    assert_error_report(
        &value["error_report"],
        "audit_persistence_failed",
        "audit_persistence",
    );
}

#[test]
fn error_report_fields_are_plain_language_and_non_empty() {
    let output = process_local_gateway_request(
        &valid_request(),
        Path::new("examples/policy-bundles/missing"),
    );
    let report = json_value(&output.error_report);

    for field in ["message", "reason", "next_action"] {
        let text = report[field]
            .as_str()
            .unwrap_or_else(|| panic!("{field} should be a string"));
        assert!(!text.trim().is_empty());
        assert!(!text.contains("::"));
        assert!(text.contains(' ') || text.ends_with('.'));
    }
}

#[test]
fn error_report_does_not_contain_secret_material() {
    let output = process_local_gateway_request(
        &valid_request(),
        Path::new("examples/policy-bundles/missing"),
    );
    let serialized = serde_json::to_string(&output.error_report)
        .unwrap_or_else(|error| panic!("error report should serialize: {error}"));

    for forbidden in [
        "PRIVATE KEY",
        "BEGIN PRIVATE",
        "bearer",
        "password",
        "approval_token",
        "raw_credentials",
    ] {
        assert!(!serialized
            .to_lowercase()
            .contains(&forbidden.to_lowercase()));
    }
}

#[test]
fn runtime_stdout_remains_valid_json_on_failure() {
    let output = run_gateway_expect_success(&valid_request(), |command| {
        command
            .arg("--bundle")
            .arg("examples/policy-bundles/missing");
    });
    let value: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("failure stdout should be valid JSON: {error}"));

    assert_eq!(value["response"]["status"], "denied");
    assert_error_report(
        &value["error_report"],
        "policy_bundle_verification_failed",
        "policy_bundle_verification",
    );
}

fn assert_error_report(report: &Value, expected_code: &str, expected_location: &str) {
    assert_eq!(report["code"], expected_code);
    assert_eq!(report["location"], expected_location);
    assert!(report["severity"].is_string());
    assert!(report["message"].as_str().is_some_and(non_empty_text));
    assert!(report["reason"].as_str().is_some_and(non_empty_text));
    assert!(report["next_action"].as_str().is_some_and(non_empty_text));
}

fn non_empty_text(text: &str) -> bool {
    !text.trim().is_empty()
}

fn run_gateway_expect_success(
    input: &str,
    configure: impl FnOnce(&mut Command),
) -> std::process::Output {
    let output = run_gateway(input, configure);
    assert!(
        output.status.success(),
        "runtime should fail closed in JSON for gateway decisions: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn run_gateway_expect_failure(
    input: &str,
    configure: impl FnOnce(&mut Command),
) -> std::process::Output {
    run_gateway(input, |command| {
        command.arg("--bundle").arg(LOCAL_DEV_BUNDLE);
        configure(command);
    })
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

fn signature_mismatch_bundle(case_name: &str) -> PathBuf {
    let target = mutable_bundle(case_name);
    append_to_file(
        target.join("checksums").join("SHA256SUMS"),
        "# unsigned change\n",
    );
    target
}

fn checksum_mismatch_bundle(case_name: &str) -> PathBuf {
    let target = mutable_bundle(case_name);
    append_to_file(
        target.join("gateway_policy.yaml"),
        "# unsigned policy change\n",
    );
    target
}

fn mutable_bundle(case_name: &str) -> PathBuf {
    let target = Path::new("target")
        .join("error-reporting-tests")
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

fn json_value(report: &Option<GatewayErrorReport>) -> Value {
    serde_json::to_value(report.as_ref().expect("error report should be present"))
        .unwrap_or_else(|error| panic!("error report should serialize: {error}"))
}

fn request() -> ToolCallRequest {
    serde_json::from_str(&valid_request())
        .unwrap_or_else(|error| panic!("request fixture should parse: {error}"))
}

fn wrapper_context() -> WrapperExecutionContext {
    let mode = match WrapperExecutionMode::Enforce {
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

fn request_with_tool(tool_name: &str) -> String {
    let mut request: Value = serde_json::from_str(&valid_request())
        .unwrap_or_else(|error| panic!("valid request fixture should parse: {error}"));
    request["tool"]["name"] = Value::String(tool_name.to_string());
    serde_json::to_string(&request)
        .unwrap_or_else(|error| panic!("modified request should serialize: {error}"))
}

fn valid_request() -> String {
    read_fixture("schemas/examples/valid/HealthCheckRequest.json")
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
