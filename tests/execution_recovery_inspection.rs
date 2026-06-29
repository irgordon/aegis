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
fn valid_interleaved_executions_are_grouped_without_cross_contamination() {
    let lines = vec![
        record("exec_a", "created", "validated", 0),
        record("exec_b", "created", "validated", 0),
        record("exec_a", "validated", "bundle_verified", 1),
        record("exec_b", "validated", "failed_closed", 1),
    ];

    let report = inspect_records(&lines);
    let exec_a = execution_by_id(&report, "exec_a");
    let exec_b = execution_by_id(&report, "exec_b");

    assert_eq!(exec_a.last_known_state, ExecutionState::BundleVerified);
    assert_eq!(exec_a.transition_count, 2);
    assert_eq!(exec_b.last_known_state, ExecutionState::FailedClosed);
    assert_eq!(exec_b.transition_count, 2);
    assert!(report.inspection_errors.is_empty());
}

#[test]
fn duplicate_lifecycle_index_across_different_executions_is_allowed() {
    let report = inspect_records(&[
        record("exec_a", "created", "validated", 0),
        record("exec_b", "created", "validated", 0),
    ]);

    assert_eq!(report.executions.len(), 2);
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
fn non_monotonic_lifecycle_index_is_rejected() {
    let report = inspect_records(&[
        record("exec_non_monotonic", "created", "validated", 0),
        record("exec_non_monotonic", "validated", "bundle_verified", 2),
        record(
            "exec_non_monotonic",
            "bundle_verified",
            "policy_evaluated",
            1,
        ),
    ]);

    assert_error_code(&report, ErrorCode::StateInspectionLifecycleOrderInvalid);
    assert_not_clean_recoverable(&report, "exec_non_monotonic");
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
fn completed_followed_by_another_transition_is_rejected() {
    let mut records = completed_records("exec_terminal_completed");
    records.push(record(
        "exec_terminal_completed",
        "completed",
        "failed_closed",
        8,
    ));

    let report = inspect_records(&records);

    assert_error_code(&report, ErrorCode::StateInspectionInvalidTransition);
    assert_not_clean_recoverable(&report, "exec_terminal_completed");
}

#[test]
fn failed_closed_followed_by_another_transition_is_rejected() {
    let mut records = failed_closed_records("exec_terminal_failed");
    records.push(record(
        "exec_terminal_failed",
        "failed_closed",
        "validated",
        2,
    ));

    let report = inspect_records(&records);

    assert_error_code(&report, ErrorCode::StateInspectionInvalidTransition);
    assert_not_clean_recoverable(&report, "exec_terminal_failed");
}

#[test]
fn audit_failed_followed_by_another_transition_is_rejected() {
    let mut records = audit_failed_records("exec_terminal_audit_failed");
    records.push(record(
        "exec_terminal_audit_failed",
        "audit_failed",
        "completed",
        7,
    ));

    let report = inspect_records(&records);

    assert_error_code(&report, ErrorCode::StateInspectionInvalidTransition);
    assert_not_clean_recoverable(&report, "exec_terminal_audit_failed");
}

#[test]
fn mismatched_request_reference_for_same_execution_is_rejected() {
    let mut records = non_terminal_records("exec_mixed_request");
    records[0]["request_id"] = json!("request_one");
    records[1]["request_id"] = json!("request_two");

    let report = inspect_records(&records);

    assert_error_reason_contains(&report, "request reference");
    assert_not_clean_recoverable(&report, "exec_mixed_request");
}

#[test]
fn mismatched_tool_reference_for_same_execution_is_rejected() {
    let mut records = non_terminal_records("exec_mixed_tool");
    records[0]["tool_name"] = json!("health.check");
    records[1]["tool_name"] = json!("sandbox.note.write");

    let report = inspect_records(&records);

    assert_error_reason_contains(&report, "tool reference");
    assert_not_clean_recoverable(&report, "exec_mixed_tool");
}

#[test]
fn mismatched_wrapper_name_for_same_execution_is_rejected_when_present() {
    let mut records = non_terminal_records("exec_mixed_wrapper_name");
    records[0]["wrapper_name"] = json!("health.check");
    records[1]["wrapper_name"] = json!("sandbox.note.write");

    let report = inspect_records(&records);

    assert_error_reason_contains(&report, "wrapper evidence");
    assert_not_clean_recoverable(&report, "exec_mixed_wrapper_name");
}

#[test]
fn mismatched_wrapper_version_for_same_execution_is_rejected_when_present() {
    let mut records = non_terminal_records("exec_mixed_wrapper_version");
    records[0]["wrapper_name"] = json!("health.check");
    records[1]["wrapper_name"] = json!("health.check");
    records[0]["wrapper_version"] = json!("1.0.0");
    records[1]["wrapper_version"] = json!("2.0.0");

    let report = inspect_records(&records);

    assert_error_reason_contains(&report, "wrapper evidence");
    assert_not_clean_recoverable(&report, "exec_mixed_wrapper_version");
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
fn malformed_line_does_not_block_valid_unrelated_execution() {
    let mut lines: Vec<_> = completed_records("exec_valid")
        .iter()
        .map(Value::to_string)
        .collect();
    lines.push("{not-json}".to_string());

    let report = inspect_lines(&lines);

    assert_eq!(
        execution_by_id(&report, "exec_valid").last_known_state,
        ExecutionState::Completed
    );
    assert_error_code(&report, ErrorCode::StateInspectionInvalidJsonRecord);
    assert_eq!(
        report.inspection_status,
        aegis::state::ExecutionRecoveryStatus::InspectionFailed
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
fn incomplete_corrupted_execution_is_not_clean_recoverable() {
    let report = inspect_records(&[
        record("exec_corrupted_incomplete", "created", "validated", 0),
        record(
            "exec_corrupted_incomplete",
            "validated",
            "bundle_verified",
            2,
        ),
    ]);

    assert_error_code(&report, ErrorCode::StateInspectionLifecycleOrderInvalid);
    assert_not_clean_recoverable(&report, "exec_corrupted_incomplete");
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
fn inspection_errors_do_not_echo_secret_like_malformed_content() {
    let report = ExecutionRecoveryInspector::inspect_str(
        "{password=abc token=def secret=ghi private_key=jkl}\n",
    );
    let output = serde_json::to_string(&report).expect("report should serialize");

    assert_error_code(&report, ErrorCode::StateInspectionInvalidJsonRecord);
    assert!(!output.contains("abc"));
    assert!(!output.contains("def"));
    assert!(!output.contains("ghi"));
    assert!(!output.contains("jkl"));
    assert!(!output.contains("password="));
    assert!(!output.contains("token="));
    assert!(!output.contains("secret="));
    assert!(!output.contains("private_key="));
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

fn execution_by_id<'a>(
    report: &'a ExecutionRecoveryReport,
    execution_id: &str,
) -> &'a aegis::state::ExecutionRecoveryExecution {
    report
        .executions
        .iter()
        .find(|execution| execution.execution_id == execution_id)
        .unwrap_or_else(|| panic!("execution {execution_id} should be present"))
}

fn assert_not_clean_recoverable(report: &ExecutionRecoveryReport, execution_id: &str) {
    assert_ne!(
        execution_by_id(report, execution_id).recoverability,
        ExecutionRecoverability::RecoverableCandidate
    );
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

fn assert_error_reason_contains(report: &ExecutionRecoveryReport, expected: &str) {
    assert!(
        report
            .inspection_errors
            .iter()
            .any(|error| error.reason.contains(expected)),
        "expected inspection error reason containing {expected:?}, got {:?}",
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
