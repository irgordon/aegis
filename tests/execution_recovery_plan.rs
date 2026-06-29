use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use aegis::state::{
    AllowedFutureRecoveryAction, ExecutionRecoveryInspector, RecoveryPlanGenerator,
    RecoveryPlanOutcome, RecoveryPlanReport, RecoveryPlanStatus,
};
use serde_json::{json, Value};

#[test]
fn completed_execution_produces_not_recoverable_terminal_plan() {
    let plan = plan_records(&completed_records("exec_completed"));
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableTerminal
    );
    assert_eq!(record.plan_reason, "execution already completed");
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::None
    );
}

#[test]
fn failed_closed_execution_produces_not_recoverable_terminal_plan() {
    let plan = plan_records(&failed_closed_records("exec_failed_closed"));
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableTerminal
    );
    assert_eq!(
        record.plan_reason,
        "execution failed closed and must not be resumed automatically"
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::None
    );
}

#[test]
fn audit_failed_execution_produces_audit_retry_candidate_plan() {
    let plan = plan_records(&audit_failed_records("exec_audit_failed"));
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::CandidateForAuditRetry
    );
    assert_eq!(
        record.plan_reason,
        "execution reached audit failure and may be eligible for future audit-specific recovery"
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::AuditRetryOnly
    );
}

#[test]
fn valid_non_terminal_execution_produces_future_replay_evaluation_candidate() {
    let plan = plan_records(&non_terminal_records("exec_non_terminal"));
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::CandidateForFutureReplay
    );
    assert_eq!(
        record.plan_reason,
        "execution has valid non-terminal evidence and may be eligible for future replay evaluation"
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::FutureReplayEvaluationOnly
    );
}

#[test]
fn corrupted_execution_produces_not_recoverable_corrupted_plan() {
    let plan = plan_records(&corrupted_records("exec_corrupted"));
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableCorrupted
    );
    assert_eq!(
        record.plan_reason,
        "state evidence is corrupted or inconsistent"
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::ManualReviewOnly
    );
}

#[test]
fn inspection_failure_produces_inspection_failed_status() {
    let path = test_dir("missing_state_log").join("missing.jsonl");
    let inspection = ExecutionRecoveryInspector::inspect_path(path);
    let plan = RecoveryPlanGenerator::plan(&inspection);

    assert_eq!(plan.plan_status, RecoveryPlanStatus::InspectionFailed);
    assert!(plan.plans.is_empty());
    assert!(!plan.planning_errors.is_empty());
}

#[test]
fn corrupted_execution_is_not_omitted_from_plan() {
    let plan = plan_records(&corrupted_records("exec_corrupted_present"));

    assert!(plan
        .plans
        .iter()
        .any(|record| record.execution_id == "exec_corrupted_present"));
}

#[test]
fn corrupted_execution_is_never_future_replay_candidate() {
    let plan = plan_records(&corrupted_records("exec_corrupted_no_replay"));

    assert_ne!(
        only_plan(&plan).plan_outcome,
        RecoveryPlanOutcome::CandidateForFutureReplay
    );
}

#[test]
fn failed_closed_execution_is_never_future_replay_candidate() {
    let plan = plan_records(&failed_closed_records("exec_failed_no_replay"));

    assert_ne!(
        only_plan(&plan).plan_outcome,
        RecoveryPlanOutcome::CandidateForFutureReplay
    );
}

#[test]
fn completed_execution_is_never_future_replay_candidate() {
    let plan = plan_records(&completed_records("exec_completed_no_replay"));

    assert_ne!(
        only_plan(&plan).plan_outcome,
        RecoveryPlanOutcome::CandidateForFutureReplay
    );
}

#[test]
fn plan_output_uses_bounded_enum_values_only() {
    let mut records = completed_records("exec_completed");
    records.extend(failed_closed_records("exec_failed"));
    records.extend(audit_failed_records("exec_audit_failed"));
    records.extend(non_terminal_records("exec_non_terminal"));
    records.extend(corrupted_records("exec_corrupted"));

    let plan = plan_records(&records);
    let output = serde_json::to_value(plan).expect("plan should serialize");
    let outcomes = output["plans"]
        .as_array()
        .expect("plans should be an array")
        .iter()
        .map(|record| record["plan_outcome"].as_str().unwrap())
        .collect::<Vec<_>>();

    for outcome in outcomes {
        assert!(matches!(
            outcome,
            "not_recoverable_terminal"
                | "not_recoverable_corrupted"
                | "candidate_for_audit_retry"
                | "candidate_for_future_replay"
                | "inspection_failed"
        ));
    }
}

#[test]
fn plan_output_includes_allowed_future_action() {
    let plan = plan_records(&completed_records("exec_action_field"));
    let output = serde_json::to_value(plan).expect("plan should serialize");

    assert!(output["plans"][0].get("allowed_future_action").is_some());
}

#[test]
fn completed_and_failed_closed_allowed_future_action_is_none() {
    let mut records = completed_records("exec_completed_action");
    records.extend(failed_closed_records("exec_failed_action"));

    let plan = plan_records(&records);

    for record in plan.plans {
        assert_eq!(
            record.allowed_future_action,
            AllowedFutureRecoveryAction::None
        );
    }
}

#[test]
fn audit_failed_allowed_future_action_is_audit_retry_only() {
    let plan = plan_records(&audit_failed_records("exec_audit_action"));

    assert_eq!(
        only_plan(&plan).allowed_future_action,
        AllowedFutureRecoveryAction::AuditRetryOnly
    );
}

#[test]
fn valid_non_terminal_allowed_future_action_is_future_replay_evaluation_only() {
    let plan = plan_records(&non_terminal_records("exec_replay_action"));

    assert_eq!(
        only_plan(&plan).allowed_future_action,
        AllowedFutureRecoveryAction::FutureReplayEvaluationOnly
    );
}

#[test]
fn planner_does_not_write_state_logs() {
    let dir = test_dir("planner_state_read_only");
    let state_path = write_state_log(&dir, &completed_records("exec_state_read_only"));
    let before = fs::read_to_string(&state_path).expect("state log should read before planning");

    let output = run_plan(&dir, &state_path);
    let after = fs::read_to_string(&state_path).expect("state log should read after planning");

    assert!(output.status.success());
    assert_eq!(before, after);
}

#[test]
fn planner_does_not_write_audit_logs() {
    let dir = test_dir("planner_no_audit_write");
    let state_path = write_state_log(&dir, &completed_records("exec_no_audit"));

    let output = run_plan(&dir, &state_path);

    assert!(output.status.success());
    assert!(!dir.join("audit.jsonl").exists());
}

#[test]
fn planner_does_not_execute_wrappers() {
    let dir = test_dir("planner_no_wrapper_execution");
    let state_path = write_state_log(&dir, &completed_records("exec_no_wrapper"));

    let output = run_plan(&dir, &state_path);
    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf8");

    assert!(!stdout.contains("wrapper_result"));
    assert!(!stdout.contains("wrapper_execution"));
    assert!(!dir.join("notes").exists());
}

#[test]
fn planner_does_not_load_policy_bundles() {
    let dir = test_dir("planner_no_policy_bundle");
    let state_path = write_state_log(&dir, &completed_records("exec_no_bundle"));

    let output = run_plan(&dir, &state_path);

    assert!(output.status.success());
    assert!(!dir.join("examples").exists());
}

#[test]
fn planner_output_contains_no_secret_like_material() {
    let inspection = ExecutionRecoveryInspector::inspect_str(
        "{password=abc token=def secret=ghi private_key=jkl}\n",
    );
    let plan = RecoveryPlanGenerator::plan(&inspection);
    let output = serde_json::to_string(&plan).expect("plan should serialize");

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
fn cli_plan_recovery_returns_valid_json() {
    let dir = test_dir("cli_valid_json");
    let state_path = write_state_log(&dir, &completed_records("exec_cli_json"));

    let output = run_plan(&dir, &state_path);
    let _: Value = serde_json::from_slice(&output.stdout).expect("stdout should be valid JSON");

    assert!(output.status.success());
}

#[test]
fn cli_plan_recovery_does_not_require_request_json() {
    let dir = test_dir("cli_no_request_json");
    let state_path = write_state_log(&dir, &completed_records("exec_cli_no_request"));

    let output = run_plan(&dir, &state_path);

    assert!(output.status.success());
}

#[test]
fn cli_plan_recovery_does_not_require_bundle() {
    let dir = test_dir("cli_no_bundle");
    let state_path = write_state_log(&dir, &completed_records("exec_cli_no_bundle"));

    let output = run_plan(&dir, &state_path);

    assert!(output.status.success());
}

fn plan_records(records: &[Value]) -> RecoveryPlanReport {
    let inspection = inspect_records(records);
    RecoveryPlanGenerator::plan(&inspection)
}

fn inspect_records(records: &[Value]) -> aegis::state::ExecutionRecoveryReport {
    let lines: Vec<_> = records.iter().map(Value::to_string).collect();
    ExecutionRecoveryInspector::inspect_str(&lines.join("\n"))
}

fn only_plan(report: &RecoveryPlanReport) -> &aegis::state::RecoveryPlanRecord {
    assert_eq!(report.plans.len(), 1);
    &report.plans[0]
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
    ]
}

fn corrupted_records(execution_id: &str) -> Vec<Value> {
    vec![
        record(execution_id, "created", "validated", 0),
        record(execution_id, "validated", "bundle_verified", 2),
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

fn run_plan(dir: &Path, state_path: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aegis-gateway"))
        .current_dir(dir)
        .arg("--plan-recovery")
        .arg(state_path)
        .output()
        .expect("planning command should run")
}

fn test_dir(name: &str) -> PathBuf {
    let path = PathBuf::from("target")
        .join("execution-recovery-plan")
        .join(format!("{}-{}", name, std::process::id()));

    if path.exists() {
        fs::remove_dir_all(&path).expect("test directory should reset");
    }

    fs::create_dir_all(&path).expect("test directory should exist");
    path
}
