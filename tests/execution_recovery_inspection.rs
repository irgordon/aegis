use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use aegis::{
    error::{ErrorCode, ErrorLocation},
    state::{
        ExecutionRecoverability, ExecutionRecoveryInspector, ExecutionRecoveryReport,
        ExecutionState, ExecutionTerminalStatus,
    },
};
use serde_json::{json, Value};

#[test]
fn completed_execution_is_terminal_and_not_recoverable() {
    let report = inspect_records(&completed_records("exec_completed"));
    let execution = only_execution(&report);

    assert_eq!(execution.last_known_state, ExecutionState::Completed);
    assert_eq!(execution.terminal_status, ExecutionTerminalStatus::Terminal);
    assert_eq!(
        execution.recoverability,
        ExecutionRecoverability::NotRecoverableTerminal
    );
    assert_eq!(execution.transition_count, 8);
    assert!(report.inspection_errors.is_empty());
}

#[test]
fn failed_closed_execution_is_terminal_and_not_recoverable() {
    let report = inspect_records(&failed_closed_records("exec_failed"));
    let execution = only_execution(&report);

    assert_eq!(execution.last_known_state, ExecutionState::FailedClosed);
    assert_eq!(execution.terminal_status, ExecutionTerminalStatus::Terminal);
    assert_eq!(
        execution.recoverability,
        ExecutionRecoverability::NotRecoverableTerminal
    );
    assert_eq!(execution.failure_reason.as_deref(), Some("failed_closed"));
}

#[test]
fn audit_failed_execution_is_recoverable_candidate() {
    let report = inspect_records(&audit_failed_records("exec_audit_failed"));
    let execution = only_execution(&report);

    assert_eq!(execution.last_known_state, ExecutionState::AuditFailed);
    assert_eq!(execution.terminal_status, ExecutionTerminalStatus::Terminal);
    assert_eq!(
        execution.recoverability,
        ExecutionRecoverability::RecoverableCandidate
    );
}

#[test]
fn non_terminal_execution_is_recoverable_candidate() {
    let report = inspect_records(&non_terminal_records("exec_in_progress"));
    let execution = only_execution(&report);

    assert_eq!(execution.last_known_state, ExecutionState::Dispatching);
    assert_eq!(
        execution.terminal_status,
        ExecutionTerminalStatus::NonTerminal
    );
    assert_eq!(
        execution.recoverability,
        ExecutionRecoverability::RecoverableCandidate
    );
}

#[test]
fn multiple_executions_are_grouped_by_execution_id() {
    let mut records = completed_records("exec_one");
    records.extend(failed_closed_records("exec_two"));

    let report = inspect_records(&records);
    let execution_ids: Vec<_> = report
        .executions
        .iter()
        .map(|execution| execution.execution_id.as_str())
        .collect();

    assert_eq!(execution_ids, vec!["exec_one", "exec_two"]);
    assert!(report.inspection_errors.is_empty());
}

#[test]
fn transition_order_is_preserved() {
    let report = inspect_records(&completed_records("exec_order"));
    let execution = only_execution(&report);

    assert_eq!(execution.transition_count, 8);
    assert_eq!(execution.last_known_state, ExecutionState::Completed);
}

#[test]
fn duplicate_lifecycle_index_is_rejected() {
    let mut records = completed_records("exec_duplicate");
    records[2]["lifecycle_index"] = json!(1);

    let report = inspect_records(&records);

    assert_error_code(&report, ErrorCode::StateInspectionDuplicateLifecycleIndex);
    assert_eq!(
        only_execution(&report).recoverability,
        ExecutionRecoverability::InspectionFailed
    );
}

#[test]
fn invalid_transition_is_rejected() {
    let report = inspect_lines(&[state_line(
        "exec_invalid",
        "created",
        "executed",
        "invalid_transition",
        0,
    )]);

    assert_error_code(&report, ErrorCode::StateInspectionInvalidTransition);
    assert_eq!(
        only_execution(&report).recoverability,
        ExecutionRecoverability::InspectionFailed
    );
}

#[test]
fn malformed_jsonl_record_is_reported_as_inspection_error() {
    let report = ExecutionRecoveryInspector::inspect_str("{not-json}\n");

    assert_error_code(&report, ErrorCode::StateInspectionInvalidJsonRecord);
    assert!(report.executions.is_empty());
}

#[test]
fn unknown_state_value_is_reported_as_inspection_error() {
    let report = inspect_lines(&[state_line(
        "exec_unknown",
        "created",
        "paused",
        "unknown_state",
        0,
    )]);

    assert_error_code(&report, ErrorCode::StateInspectionUnknownLifecycleState);
    assert!(report.executions.is_empty());
}

#[test]
fn missing_state_log_returns_structured_error() {
    let path = test_dir("missing_state_log").join("missing.jsonl");
    let report = ExecutionRecoveryInspector::inspect_path(path);

    assert_error_code(&report, ErrorCode::StateInspectionReadFailed);
    assert_eq!(
        report.inspection_errors[0].location,
        ErrorLocation::ExecutionRecoveryInspection
    );
    assert!(report.executions.is_empty());
}

#[test]
fn missing_first_state_is_reported_when_detectable() {
    let report = inspect_lines(&[state_line(
        "exec_missing_first",
        "validated",
        "bundle_verified",
        "policy_bundle_verified",
        1,
    )]);

    assert_error_code(&report, ErrorCode::StateInspectionMissingFirstState);
}

#[test]
fn inspection_mode_does_not_write_to_state_log() {
    let dir = test_dir("cli_state_read_only");
    let state_path = write_state_log(&dir, &completed_records("exec_read_only"));
    let before = fs::read_to_string(&state_path).expect("state log should read before inspection");

    let output = run_inspection(&dir, &state_path);
    let after = fs::read_to_string(&state_path).expect("state log should read after inspection");

    assert!(output.status.success());
    assert_eq!(before, after);
}

#[test]
fn inspection_mode_does_not_write_audit_log() {
    let dir = test_dir("cli_no_audit_write");
    let state_path = write_state_log(&dir, &completed_records("exec_no_audit"));

    let output = run_inspection(&dir, &state_path);

    assert!(output.status.success());
    assert!(!dir.join("audit.jsonl").exists());
}

#[test]
fn inspection_mode_does_not_execute_wrappers() {
    let dir = test_dir("cli_no_wrapper_execution");
    let state_path = write_state_log(&dir, &completed_records("exec_no_wrapper"));

    let output = run_inspection(&dir, &state_path);
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");

    assert!(!stdout.contains("wrapper_result"));
    assert!(!stdout.contains("wrapper_execution"));
    assert!(!dir.join("notes").exists());
}

#[test]
fn inspection_output_contains_no_secrets() {
    let report = inspect_records(&completed_records("exec_clean"));
    let output = serde_json::to_string(&report).expect("report should serialize");

    assert!(!output.contains("password"));
    assert!(!output.contains("secret"));
    assert!(!output.contains("token"));
    assert!(!output.contains("api_key"));
}

fn inspect_records(records: &[Value]) -> ExecutionRecoveryReport {
    let lines: Vec<_> = records.iter().map(Value::to_string).collect();
    ExecutionRecoveryInspector::inspect_str(&lines.join("\n"))
}

fn inspect_lines(lines: &[String]) -> ExecutionRecoveryReport {
    ExecutionRecoveryInspector::inspect_str(&lines.join("\n"))
}

fn only_execution(report: &ExecutionRecoveryReport) -> &aegis::state::ExecutionRecoveryExecution {
    assert_eq!(report.executions.len(), 1);
    &report.executions[0]
}

fn assert_error_code(report: &ExecutionRecoveryReport, code: ErrorCode) {
    assert!(
        report
            .inspection_errors
            .iter()
            .any(|error| error.code == code),
        "expected inspection error {code:?}, got {:?}",
        report.inspection_errors
    );
}

fn completed_records(execution_id: &str) -> Vec<Value> {
    vec![
        record(execution_id, "created", "validated", 0),
        record(execution_id, "validated", "bundle_verified", 1),
        record(execution_id, "bundle_verified", "policy_evaluated", 2),
        record(execution_id, "policy_evaluated", "authorized", 3),
        record(execution_id, "authorized", "dispatching", 4),
        record(execution_id, "dispatching", "executed", 5),
        record(execution_id, "executed", "audited", 6),
        record(execution_id, "audited", "completed", 7),
    ]
}

fn failed_closed_records(execution_id: &str) -> Vec<Value> {
    vec![
        record(execution_id, "created", "validated", 0),
        record(execution_id, "validated", "failed_closed", 1),
    ]
}

fn audit_failed_records(execution_id: &str) -> Vec<Value> {
    vec![
        record(execution_id, "created", "validated", 0),
        record(execution_id, "validated", "bundle_verified", 1),
        record(execution_id, "bundle_verified", "policy_evaluated", 2),
        record(execution_id, "policy_evaluated", "authorized", 3),
        record(execution_id, "authorized", "dispatching", 4),
        record(execution_id, "dispatching", "executed", 5),
        record(execution_id, "executed", "audit_failed", 6),
    ]
}

fn non_terminal_records(execution_id: &str) -> Vec<Value> {
    vec![
        record(execution_id, "created", "validated", 0),
        record(execution_id, "validated", "bundle_verified", 1),
        record(execution_id, "bundle_verified", "policy_evaluated", 2),
        record(execution_id, "policy_evaluated", "authorized", 3),
        record(execution_id, "authorized", "dispatching", 4),
    ]
}

fn record(execution_id: &str, previous_state: &str, new_state: &str, index: usize) -> Value {
    json!({
        "execution_id": execution_id,
        "previous_state": previous_state,
        "new_state": new_state,
        "transition_reason": reason_for(new_state),
        "lifecycle_index": index
    })
}

fn state_line(
    execution_id: &str,
    previous_state: &str,
    new_state: &str,
    reason: &str,
    index: usize,
) -> String {
    json!({
        "execution_id": execution_id,
        "previous_state": previous_state,
        "new_state": new_state,
        "transition_reason": reason,
        "lifecycle_index": index
    })
    .to_string()
}

fn reason_for(state: &str) -> &'static str {
    match state {
        "validated" => "request_validated",
        "bundle_verified" => "policy_bundle_verified",
        "policy_evaluated" => "policy_evaluated",
        "authorized" => "execution_authorized",
        "dispatching" => "wrapper_dispatching",
        "executed" => "wrapper_executed",
        "audited" => "audit_recorded",
        "completed" => "execution_completed",
        "failed_closed" => "failed_closed",
        "audit_failed" => "audit_failed",
        _ => "unknown_state",
    }
}

fn write_state_log(dir: &Path, records: &[Value]) -> PathBuf {
    let path = dir.join("state.jsonl");
    let lines: Vec<_> = records.iter().map(Value::to_string).collect();
    fs::write(&path, format!("{}\n", lines.join("\n"))).expect("state log should write");
    fs::canonicalize(path).expect("state log path should canonicalize")
}

fn run_inspection(dir: &Path, state_path: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aegis-gateway"))
        .current_dir(dir)
        .arg("--inspect-state")
        .arg(state_path)
        .output()
        .expect("inspection command should run")
}

fn test_dir(name: &str) -> PathBuf {
    let path = PathBuf::from("target")
        .join("execution-recovery-inspection")
        .join(format!("{}-{}", name, std::process::id()));

    if path.exists() {
        fs::remove_dir_all(&path).expect("test directory should reset");
    }

    fs::create_dir_all(&path).expect("test directory should exist");
    path
}
