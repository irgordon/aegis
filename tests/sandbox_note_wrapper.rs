use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use aegis::{
    auth::{
        CredentialBoundaryStatus, CredentialClass, CredentialRequirement, ExecutionAuthorization,
    },
    error::{ErrorCode, ErrorLocation},
    gateway::{
        GatewayStatus, ToolCallRequest, WrapperExecutionContext, WrapperExecutionError,
        WrapperExecutionOutput, WrapperExecutor,
    },
    runtime::local::{
        process_local_gateway_request, process_local_gateway_request_with_context,
        process_local_gateway_request_with_wrapper_registry_and_context, LocalRuntimeContext,
    },
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn sandbox_note_write_succeeds_when_all_mutation_gates_pass() {
    let sandbox = sandbox_dir("success");
    let output = sandbox_request_output(&sandbox, &sandbox_note_request_json());

    assert_eq!(output.response.status, GatewayStatus::Allowed);
    assert_eq!(
        note_path(&sandbox),
        sandbox.join("notes").join("example-note.txt")
    );
    assert_eq!(read_note(&sandbox), "hello from aegis");
    assert_eq!(
        output
            .response
            .result
            .as_ref()
            .and_then(|result| result.get("mutation_status"))
            .and_then(Value::as_str),
        Some("written")
    );
}

#[test]
fn sandbox_note_write_creates_file_only_under_sandbox_root() {
    let sandbox = sandbox_dir("contained_write");
    let output = sandbox_request_output(&sandbox, &sandbox_note_request_json());

    assert_eq!(output.response.status, GatewayStatus::Allowed);
    assert!(note_path(&sandbox).starts_with(&sandbox));
    assert!(!sandbox.with_file_name("example-note.txt").exists());
}

#[test]
fn missing_sandbox_directory_fails_closed() {
    let output =
        process_local_gateway_request(&sandbox_note_request_json(), Path::new(LOCAL_DEV_BUNDLE));

    assert_sandbox_failure(output, ErrorCode::SandboxDirectoryMissing);
}

#[test]
fn missing_idempotency_context_fails_closed() {
    let sandbox = sandbox_dir("missing_idempotency");
    let output = sandbox_request_output(&sandbox, &request_without_idempotency());

    assert_sandbox_failure(output, ErrorCode::IdempotencyContextMissing);
    assert!(!note_path(&sandbox).exists());
}

#[test]
fn credential_class_mismatch_fails_closed() {
    let sandbox = sandbox_dir("credential_mismatch");
    let output = process_local_gateway_request_with_wrapper_registry_and_context(
        &sandbox_note_request_json(),
        Path::new(LOCAL_DEV_BUNDLE),
        &[&MismatchedCredentialWrapper],
        LocalRuntimeContext {
            wrapper_context: None,
            sandbox_dir: Some(sandbox.clone()),
        },
    );

    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.code),
        Some(&ErrorCode::CredentialClassMismatch)
    );
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.location),
        Some(&ErrorLocation::CredentialBoundary)
    );
    assert!(!note_path(&sandbox).exists());
}

#[test]
fn denied_policy_decision_does_not_write() {
    let sandbox = sandbox_dir("denied_policy");
    let output = sandbox_request_output(
        &sandbox,
        &request_with_tool_and_capability("email.send", "L1"),
    );

    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert!(output.wrapper_execution.is_none());
    assert!(!note_path(&sandbox).exists());
}

#[test]
fn pending_policy_decision_does_not_write() {
    let sandbox = sandbox_dir("pending_policy");
    let output = sandbox_request_output(
        &sandbox,
        &request_with_tool_and_capability("deploy.prod", "L2"),
    );

    assert_eq!(output.response.status, GatewayStatus::Pending);
    assert!(output.wrapper_execution.is_none());
    assert!(!note_path(&sandbox).exists());
}

#[test]
fn path_traversal_note_id_fails_closed() {
    let sandbox = sandbox_dir("path_traversal");
    let output = sandbox_request_output(&sandbox, &request_with_note_id("../escape"));

    assert_sandbox_failure(output, ErrorCode::SandboxUnsafeNoteId);
    assert!(!sandbox.with_file_name("escape.txt").exists());
}

#[test]
fn absolute_note_id_fails_closed() {
    let sandbox = sandbox_dir("absolute_note");
    let output = sandbox_request_output(&sandbox, &request_with_note_id("/tmp/escape"));

    assert_sandbox_failure(output, ErrorCode::SandboxUnsafeNoteId);
}

#[test]
fn empty_note_id_fails_closed() {
    let sandbox = sandbox_dir("empty_note");
    let output = sandbox_request_output(&sandbox, &request_with_note_id(""));

    assert_sandbox_failure(output, ErrorCode::SandboxUnsafeNoteId);
}

#[test]
fn empty_content_fails_closed() {
    let sandbox = sandbox_dir("empty_content");
    let output = sandbox_request_output(&sandbox, &request_with_content(""));

    assert_sandbox_failure(output, ErrorCode::SandboxEmptyContent);
}

#[test]
fn wrapper_output_appears_in_runtime_json() {
    let sandbox = sandbox_dir("runtime_json");
    let output = sandbox_request_output(&sandbox, &sandbox_note_request_json());
    let json = serde_json::to_value(output)
        .unwrap_or_else(|error| panic!("runtime output should serialize: {error}"));

    assert_eq!(json["response"]["result"]["wrapper"], "sandbox.note.write");
    assert_eq!(
        json["response"]["result"]["sandbox_relative_path"],
        "notes/example-note.txt"
    );
    assert_eq!(
        json["wrapper_execution"]["wrapper_name"],
        "sandbox.note.write"
    );
    assert_eq!(
        json["wrapper_execution"]["wrapper_result_summary"]["mutation_status"],
        "written"
    );
}

#[test]
fn audit_evidence_includes_mutation_boundaries() {
    let sandbox = sandbox_dir("audit_evidence");
    let output = sandbox_request_output(&sandbox, &sandbox_note_request_json());
    let details = output.audit_record.details;

    assert!(details.idempotency_context.is_some());
    assert!(details.execution_authorization.is_some());
    assert!(details.credential_boundary.is_some());
    assert_eq!(
        details
            .credential_boundary
            .as_ref()
            .map(|boundary| &boundary.credential_boundary_status),
        Some(&CredentialBoundaryStatus::Satisfied)
    );
    assert_eq!(
        details
            .credential_boundary
            .as_ref()
            .and_then(|boundary| boundary.credential_class.clone()),
        Some(CredentialClass::LocalRuntime)
    );
    assert_eq!(
        details
            .wrapper_execution_evidence
            .as_ref()
            .and_then(|evidence| evidence.wrapper_result_summary.as_ref())
            .and_then(|summary| summary.get("sandbox_relative_path"))
            .map(String::as_str),
        Some("notes/example-note.txt")
    );
}

#[test]
fn sandbox_output_and_audit_do_not_include_secrets() {
    let sandbox = sandbox_dir("no_secrets");
    let output = sandbox_request_output(&sandbox, &sandbox_note_request_json());
    let serialized = serde_json::to_string(&output)
        .unwrap_or_else(|error| panic!("runtime output should serialize: {error}"));

    for forbidden in [
        "password",
        "api_key",
        "bearer",
        "credential_value",
        "raw_token",
        "private_key",
    ] {
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn health_check_still_works_without_sandbox_directory() {
    let output = process_local_gateway_request(&health_request_json(), Path::new(LOCAL_DEV_BUNDLE));

    assert_eq!(output.response.status, GatewayStatus::Allowed);
    assert_eq!(
        output
            .response
            .result
            .as_ref()
            .and_then(|result| result.get("wrapper"))
            .and_then(Value::as_str),
        Some("health.check")
    );
}

struct MismatchedCredentialWrapper;

impl WrapperExecutor for MismatchedCredentialWrapper {
    fn wrapper_name(&self) -> &str {
        "sandbox.note.write"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement {
            requires_credentials: true,
            credential_class: Some(CredentialClass::None),
        }
    }

    fn execute(
        &self,
        _request: &ToolCallRequest,
        _context: &WrapperExecutionContext,
        _authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        Ok(WrapperExecutionOutput {
            result: Some(BTreeMap::new()),
        })
    }
}

fn sandbox_request_output(
    sandbox: &Path,
    request_json: &str,
) -> aegis::runtime::local::LocalRuntimeOutput {
    process_local_gateway_request_with_context(
        request_json,
        Path::new(LOCAL_DEV_BUNDLE),
        LocalRuntimeContext {
            wrapper_context: None,
            sandbox_dir: Some(sandbox.to_path_buf()),
        },
    )
}

fn assert_sandbox_failure(output: aegis::runtime::local::LocalRuntimeOutput, code: ErrorCode) {
    assert_eq!(output.response.status, GatewayStatus::Denied);
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.code),
        Some(&code)
    );
    assert_eq!(
        output.error_report.as_ref().map(|report| &report.location),
        Some(&ErrorLocation::SandboxMutation)
    );
    assert!(output.wrapper_execution.is_none());
}

fn sandbox_dir(name: &str) -> PathBuf {
    let path = Path::new("target")
        .join("sandbox-note-wrapper-tests")
        .join(name);
    if path.exists() {
        fs::remove_dir_all(&path)
            .unwrap_or_else(|error| panic!("old sandbox test directory should remove: {error}"));
    }
    fs::create_dir_all(&path)
        .unwrap_or_else(|error| panic!("sandbox test directory should create: {error}"));
    path
}

fn note_path(sandbox: &Path) -> PathBuf {
    sandbox.join("notes").join("example-note.txt")
}

fn read_note(sandbox: &Path) -> String {
    fs::read_to_string(note_path(sandbox))
        .unwrap_or_else(|error| panic!("sandbox note should be readable: {error}"))
}

fn request_without_idempotency() -> String {
    let mut request = sandbox_note_request_value();
    request
        .as_object_mut()
        .expect("request should be an object")
        .remove("idempotency_key");
    request_to_string(request)
}

fn request_with_tool_and_capability(tool_name: &str, capability_class: &str) -> String {
    let mut request = sandbox_note_request_value();
    request["tool"]["name"] = Value::String(tool_name.to_string());
    request["tool"]["capability_class"] = Value::String(capability_class.to_string());
    request_to_string(request)
}

fn request_with_note_id(note_id: &str) -> String {
    let mut request = sandbox_note_request_value();
    request["params"]["note_id"] = Value::String(note_id.to_string());
    request_to_string(request)
}

fn request_with_content(content: &str) -> String {
    let mut request = sandbox_note_request_value();
    request["params"]["content"] = Value::String(content.to_string());
    request_to_string(request)
}

fn sandbox_note_request_value() -> Value {
    serde_json::from_str(&sandbox_note_request_json())
        .unwrap_or_else(|error| panic!("sandbox note fixture should parse: {error}"))
}

fn request_to_string(request: Value) -> String {
    serde_json::to_string(&request)
        .unwrap_or_else(|error| panic!("modified request should serialize: {error}"))
}

fn sandbox_note_request_json() -> String {
    read_fixture("schemas/examples/valid/SandboxNoteWriteRequest.json")
}

fn health_request_json() -> String {
    read_fixture("schemas/examples/valid/HealthCheckRequest.json")
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
