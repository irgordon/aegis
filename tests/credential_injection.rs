use std::{
    cell::Cell,
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use aegis::{
    auth::{
        CredentialBoundary, CredentialClass, CredentialInjectionError, CredentialInjectionResult,
        CredentialInjectionStatus, CredentialRequirement, CredentialSource, ExecutionAuthorization,
    },
    gateway::{
        ToolCallRequest, ToolCallResponse, WrapperDispatcher, WrapperExecutionContext,
        WrapperExecutionError, WrapperExecutionOutput, WrapperExecutor,
    },
    runtime::local::{
        local_wrapper_executors, process_local_gateway_request,
        process_local_gateway_request_with_context, LocalRuntimeContext,
    },
    wrappers::SandboxNoteWriteWrapper,
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn sandbox_note_write_receives_local_credential_handle() {
    let paths = case_paths("sandbox_note_write_receives_local_credential_handle");
    let output = sandbox_runtime_output(&paths);
    let injection = output
        .credential_injection
        .expect("sandbox mutation should receive credential handle evidence");

    assert_eq!(
        injection.credential_injection_status,
        CredentialInjectionStatus::Injected
    );
    assert_eq!(injection.credential_class, CredentialClass::LocalRuntime);
    assert_eq!(
        injection.credential_source,
        CredentialSource::LocalDevelopment
    );
    assert_eq!(injection.wrapper_name.as_str(), "sandbox.note.write");
    assert_eq!(injection.wrapper_version.as_str(), "1.0.0");
    assert_safe_handle(injection.credential_handle_ref.as_ref().unwrap().as_str());
    assert!(paths
        .sandbox
        .join("notes")
        .join("example-note.txt")
        .is_file());
}

#[test]
fn sandbox_note_write_fails_when_handle_is_missing() {
    let paths = case_paths("sandbox_note_write_fails_when_handle_is_missing");
    let error = execute_sandbox_with_injection(&paths, None)
        .expect_err("missing credential handle should fail closed");

    assert_eq!(
        error.reason_code.as_deref(),
        Some("credential_handle_missing")
    );
    assert!(!paths
        .sandbox
        .join("notes")
        .join("example-note.txt")
        .exists());
}

#[test]
fn sandbox_note_write_fails_when_credential_class_mismatches() {
    let paths = case_paths("sandbox_note_write_fails_when_credential_class_mismatches");
    let mut injection = valid_credential_injection(&sandbox_context(&paths.sandbox));
    injection.credential_class = CredentialClass::None;
    let error = execute_sandbox_with_injection(&paths, Some(injection))
        .expect_err("credential class mismatch should fail closed");

    assert_eq!(
        error.reason_code.as_deref(),
        Some("credential_class_unsupported")
    );
    assert!(!paths
        .sandbox
        .join("notes")
        .join("example-note.txt")
        .exists());
}

#[test]
fn sandbox_note_write_fails_when_handle_wrapper_binding_mismatches() {
    let paths = case_paths("sandbox_note_write_fails_when_handle_wrapper_binding_mismatches");
    let mut injection = valid_credential_injection(&sandbox_context(&paths.sandbox));
    injection.wrapper_name = non_empty("health.check");
    let error = execute_sandbox_with_injection(&paths, Some(injection))
        .expect_err("wrapper binding mismatch should fail closed");

    assert_eq!(
        error.reason_code.as_deref(),
        Some("credential_handle_wrapper_mismatch")
    );
    assert!(!paths
        .sandbox
        .join("notes")
        .join("example-note.txt")
        .exists());
}

#[test]
fn sandbox_note_write_fails_when_handle_authorization_binding_mismatches() {
    let paths = case_paths("sandbox_note_write_fails_when_handle_authorization_binding_mismatches");
    let mut injection = valid_credential_injection(&sandbox_context(&paths.sandbox));
    injection.authorization_id = non_empty("auth_other_request");
    let error = execute_sandbox_with_injection(&paths, Some(injection))
        .expect_err("authorization binding mismatch should fail closed");

    assert_eq!(
        error.reason_code.as_deref(),
        Some("credential_handle_authorization_mismatch")
    );
    assert!(!paths
        .sandbox
        .join("notes")
        .join("example-note.txt")
        .exists());
}

#[test]
fn health_check_does_not_receive_credential_handle() {
    let output = process_local_gateway_request(&health_request(), Path::new(LOCAL_DEV_BUNDLE));

    assert!(output.credential_injection.is_none());
    assert!(output.audit_record.details.credential_injection.is_none());
}

#[test]
fn credential_injection_evidence_appears_in_runtime_output() {
    let paths = case_paths("credential_injection_evidence_appears_in_runtime_output");
    let output = sandbox_runtime_output(&paths);

    assert_eq!(
        output
            .credential_injection
            .as_ref()
            .map(|injection| &injection.credential_injection_status),
        Some(&CredentialInjectionStatus::Injected)
    );
}

#[test]
fn credential_injection_evidence_appears_in_audit_record() {
    let paths = case_paths("credential_injection_evidence_appears_in_audit_record");
    let output = sandbox_runtime_output(&paths);
    let injection = output
        .audit_record
        .details
        .credential_injection
        .expect("audit should include credential injection evidence");

    assert_eq!(injection.credential_class, CredentialClass::LocalRuntime);
    assert_safe_handle(injection.credential_handle_ref.as_ref().unwrap().as_str());
}

#[test]
fn credential_injection_evidence_appears_in_state_log() {
    let paths = case_paths("credential_injection_evidence_appears_in_state_log");

    run_gateway_with_logs(&paths);

    let records = state_records(&paths.state_log);
    assert!(records.iter().any(|record| {
        record["credential_injection_status"] == "injected"
            && record["credential_class"] == "local_runtime"
            && record["credential_handle_ref"]
                .as_str()
                .is_some_and(|handle| handle.starts_with("local-development-handle:"))
    }));
}

#[test]
fn stdout_audit_and_state_logs_contain_no_secret_material() {
    let paths = case_paths("stdout_audit_and_state_logs_contain_no_secret_material");
    let output = run_gateway_with_logs(&paths);

    assert_no_secret_material(&String::from_utf8_lossy(&output.stdout));
    assert_no_secret_material(&fs::read_to_string(&paths.audit_log).unwrap());
    assert_no_secret_material(&fs::read_to_string(&paths.state_log).unwrap());
}

#[test]
fn structured_credential_errors_are_bounded_and_plain_language() {
    let output = process_local_gateway_request_with_rejecting_wrapper();

    assert_eq!(
        output["error_report"]["code"],
        "credential_handle_wrapper_mismatch"
    );
    assert_eq!(output["error_report"]["location"], "credential_injection");
    assert!(output["error_report"]["message"]
        .as_str()
        .unwrap()
        .contains("credential handle"));
    assert!(output["error_report"]["next_action"]
        .as_str()
        .unwrap()
        .contains("credential"));
}

#[test]
fn registered_wrappers_explicitly_declare_credential_requirements() {
    for wrapper in local_wrapper_executors() {
        assert!(
            wrapper.credential_requirement().credential_class.is_some(),
            "{} must explicitly declare its credential class",
            wrapper.wrapper_name()
        );
    }
}

#[test]
fn wrapper_dispatch_invokes_credential_boundary_once() {
    let request = health_request_model();
    let context = health_context();
    let authorization = authorization_for(&request, &context);
    let wrapper = CountingWrapper::default();
    let dispatcher = WrapperDispatcher::new([&wrapper as &dyn WrapperExecutor]);

    dispatcher
        .dispatch(&request, &context, &authorization)
        .expect("counting wrapper should dispatch");

    assert_eq!(wrapper.credential_requirement_calls.get(), 1);
    assert_eq!(wrapper.execute_calls.get(), 1);
}

fn process_local_gateway_request_with_rejecting_wrapper() -> Value {
    let paths = case_paths("process_local_gateway_request_with_rejecting_wrapper");
    let context = sandbox_context(&paths.sandbox);
    let output =
        aegis::runtime::local::process_local_gateway_request_with_wrapper_registry_and_context(
            &sandbox_request(),
            Path::new(LOCAL_DEV_BUNDLE),
            &[&RejectingCredentialWrapper],
            LocalRuntimeContext {
                wrapper_context: Some(context),
                sandbox_dir: None,
            },
        );

    serde_json::to_value(output).expect("runtime output should serialize")
}

fn execute_sandbox_with_injection(
    paths: &CasePaths,
    credential_injection: Option<CredentialInjectionResult>,
) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
    let request = sandbox_request_model();
    let context = sandbox_context(&paths.sandbox);
    let authorization = authorization_for(&request, &context);

    SandboxNoteWriteWrapper.execute(
        &request,
        &context,
        &authorization,
        credential_injection.as_ref(),
    )
}

fn valid_credential_injection(context: &WrapperExecutionContext) -> CredentialInjectionResult {
    let request = sandbox_request_model();
    let authorization = authorization_for(&request, context);
    let boundary =
        CredentialBoundary::evaluate(&CredentialRequirement::local_runtime(), &authorization);

    CredentialInjectionResult::inject_local_development(&boundary, &authorization)
        .expect("local development injection should succeed")
        .expect("local runtime credentials should produce a handle")
}

fn sandbox_runtime_output(paths: &CasePaths) -> aegis::runtime::local::LocalRuntimeOutput {
    process_local_gateway_request_with_context(
        &sandbox_request(),
        Path::new(LOCAL_DEV_BUNDLE),
        LocalRuntimeContext {
            wrapper_context: None,
            sandbox_dir: Some(paths.sandbox.clone()),
        },
    )
}

fn run_gateway_with_logs(paths: &CasePaths) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_aegis-gateway"));
    command
        .arg("--bundle")
        .arg(LOCAL_DEV_BUNDLE)
        .arg("--audit-log")
        .arg(&paths.audit_log)
        .arg("--state-log")
        .arg(&paths.state_log)
        .arg("--sandbox-dir")
        .arg(&paths.sandbox)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .unwrap_or_else(|error| panic!("runtime should start: {error}"));
    child
        .stdin
        .as_mut()
        .unwrap_or_else(|| panic!("stdin should be available"))
        .write_all(sandbox_request().as_bytes())
        .unwrap_or_else(|error| panic!("request should write to stdin: {error}"));

    let output = child
        .wait_with_output()
        .unwrap_or_else(|error| panic!("runtime should finish: {error}"));
    assert!(
        output.status.success(),
        "runtime should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn state_records(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("state log should read: {error}"))
        .lines()
        .map(|line| serde_json::from_str(line).expect("state log line should parse"))
        .collect()
}

fn authorization_for(
    request: &ToolCallRequest,
    context: &WrapperExecutionContext,
) -> ExecutionAuthorization {
    ExecutionAuthorization::policy_allow(request, &allowed_response(), context)
        .expect("authorization should be valid")
}

fn health_context() -> WrapperExecutionContext {
    wrapper_context("health.check", "1.0.0", None)
}

fn sandbox_context(sandbox: &Path) -> WrapperExecutionContext {
    wrapper_context("sandbox.note.write", "1.0.0", Some(sandbox))
}

fn wrapper_context(
    wrapper_name: &str,
    wrapper_version: &str,
    sandbox: Option<&Path>,
) -> WrapperExecutionContext {
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
        "credential_injection_required": false,
        "sandbox_root": sandbox.map(|path| path.display().to_string())
    }))
    .expect("wrapper context should parse")
}

fn allowed_response() -> ToolCallResponse {
    serde_json::from_value(serde_json::json!({
        "schema_version": "1.0",
        "execution_id": "local_exec_001",
        "request_id": "req_sandbox_note_001",
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
    .expect("allowed response should parse")
}

fn sandbox_request_model() -> ToolCallRequest {
    serde_json::from_str(&sandbox_request()).expect("sandbox request should parse")
}

fn health_request_model() -> ToolCallRequest {
    serde_json::from_str(&health_request()).expect("health request should parse")
}

fn sandbox_request() -> String {
    read_fixture("schemas/examples/valid/SandboxNoteWriteRequest.json")
}

fn health_request() -> String {
    read_fixture("schemas/examples/valid/HealthCheckRequest.json")
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{path} should read: {error}"))
}

fn non_empty(value: &str) -> aegis::gateway::NonEmptyString {
    aegis::gateway::NonEmptyString::new(value.to_string()).expect("test value is non-empty")
}

fn case_paths(name: &str) -> CasePaths {
    let root = Path::new("target")
        .join("credential-injection-tests")
        .join(name);
    if root.exists() {
        fs::remove_dir_all(&root).expect("old test directory should remove");
    }
    fs::create_dir_all(root.join("sandbox")).expect("sandbox should create");

    CasePaths {
        audit_log: root.join("audit.jsonl"),
        state_log: root.join("state.jsonl"),
        sandbox: root.join("sandbox"),
    }
}

fn assert_safe_handle(handle: &str) {
    assert!(handle.starts_with("local-development-handle:"));
    assert_no_secret_material(handle);
}

fn assert_no_secret_material(content: &str) {
    let content = content.to_lowercase();
    for forbidden in [
        "password",
        "secret_value",
        "raw_secret",
        "raw_token",
        "bearer",
        "api_key",
        "private key",
        "begin private",
        "credential_value",
    ] {
        assert!(
            !content.contains(forbidden),
            "found secret marker {forbidden}"
        );
    }
}

struct CasePaths {
    audit_log: PathBuf,
    state_log: PathBuf,
    sandbox: PathBuf,
}

#[derive(Default)]
struct CountingWrapper {
    credential_requirement_calls: Cell<usize>,
    execute_calls: Cell<usize>,
}

impl WrapperExecutor for CountingWrapper {
    fn wrapper_name(&self) -> &str {
        "health.check"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        self.credential_requirement_calls
            .set(self.credential_requirement_calls.get() + 1);
        CredentialRequirement::none()
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
        credential_injection: Option<&CredentialInjectionResult>,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        self.execute_calls.set(self.execute_calls.get() + 1);
        assert!(credential_injection.is_none());
        Ok(WrapperExecutionOutput {
            result: Some(BTreeMap::new()),
        })
    }
}

struct RejectingCredentialWrapper;

impl WrapperExecutor for RejectingCredentialWrapper {
    fn wrapper_name(&self) -> &str {
        "sandbox.note.write"
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
        _credential_injection: Option<&CredentialInjectionResult>,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Err(WrapperExecutionError {
            reason_code: Some(
                CredentialInjectionError::CredentialHandleWrapperMismatch
                    .reason_code()
                    .to_string(),
            ),
            safe_message: CredentialInjectionError::CredentialHandleWrapperMismatch.safe_message(),
        })
    }
}
