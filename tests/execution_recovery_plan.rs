use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use aegis::state::{
    AllowedFutureRecoveryAction, ExecutionRecoverability, ExecutionRecoveryExecution,
    ExecutionRecoveryInspector, ExecutionRecoveryReport, ExecutionRecoveryStatus, ExecutionState,
    ExecutionTerminalStatus, RecoveryPlanGenerator, RecoveryPlanOutcome, RecoveryPlanReport,
    RecoveryPlanStatus,
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
fn terminal_executions_never_produce_recovery_actions() {
    let mut records = completed_records("exec_completed_no_actions");
    records.extend(failed_closed_records("exec_failed_no_actions"));

    let plan = plan_records(&records);

    for record in plan.plans {
        assert_eq!(
            record.plan_outcome,
            RecoveryPlanOutcome::NotRecoverableTerminal
        );
        assert_eq!(
            record.allowed_future_action,
            AllowedFutureRecoveryAction::None
        );
        assert_ne!(
            record.plan_outcome,
            RecoveryPlanOutcome::CandidateForFutureReplay
        );
        assert_ne!(
            record.plan_outcome,
            RecoveryPlanOutcome::CandidateForAuditRetry
        );
    }
}

#[test]
fn audit_failed_execution_is_not_replay_candidate() {
    let plan = plan_records(&audit_failed_records("exec_audit_not_replay"));
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::CandidateForAuditRetry
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::AuditRetryOnly
    );
    assert_ne!(
        record.plan_outcome,
        RecoveryPlanOutcome::CandidateForFutureReplay
    );
    assert_ne!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::FutureReplayEvaluationOnly
    );
}

#[test]
fn unknown_recoverability_status_fails_closed() {
    let report = recovery_report(vec![ExecutionRecoveryExecution {
        execution_id: "exec_unknown_recoverability".to_string(),
        last_known_state: ExecutionState::Authorized,
        terminal_status: ExecutionTerminalStatus::NonTerminal,
        recoverability: ExecutionRecoverability::Unknown,
        transition_count: 4,
        failure_reason: None,
    }]);

    let plan = RecoveryPlanGenerator::plan(&report);
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableCorrupted
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::ManualReviewOnly
    );
}

#[test]
fn inconsistent_terminal_status_fails_closed() {
    let report = recovery_report(vec![ExecutionRecoveryExecution {
        execution_id: "exec_inconsistent_terminal".to_string(),
        last_known_state: ExecutionState::Authorized,
        terminal_status: ExecutionTerminalStatus::Terminal,
        recoverability: ExecutionRecoverability::RecoverableCandidate,
        transition_count: 4,
        failure_reason: None,
    }]);

    let plan = RecoveryPlanGenerator::plan(&report);
    let record = only_plan(&plan);

    assert_eq!(
        record.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableCorrupted
    );
    assert_eq!(
        record.allowed_future_action,
        AllowedFutureRecoveryAction::ManualReviewOnly
    );
}

#[test]
fn unknown_last_known_state_from_log_fails_closed() {
    let inspection = ExecutionRecoveryInspector::inspect_str(
        r#"{"execution_id":"exec_unknown_state","previous_state":"created","new_state":"made_up","transition_reason":"unknown","lifecycle_index":0}"#,
    );
    let plan = RecoveryPlanGenerator::plan(&inspection);

    assert_eq!(plan.plan_status, RecoveryPlanStatus::InspectionFailed);
    assert!(plan.plans.is_empty());
    assert!(!plan.planning_errors.is_empty());
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
fn allowed_future_actions_use_bounded_enum_values_only() {
    let mut records = completed_records("exec_completed");
    records.extend(failed_closed_records("exec_failed"));
    records.extend(audit_failed_records("exec_audit_failed"));
    records.extend(non_terminal_records("exec_non_terminal"));
    records.extend(corrupted_records("exec_corrupted"));

    let plan = plan_records(&records);
    let output = serde_json::to_value(plan).expect("plan should serialize");
    let actions = output["plans"]
        .as_array()
        .expect("plans should be an array")
        .iter()
        .map(|record| record["allowed_future_action"].as_str().unwrap())
        .collect::<Vec<_>>();

    for action in actions {
        assert!(matches!(
            action,
            "none" | "audit_retry_only" | "future_replay_evaluation_only" | "manual_review_only"
        ));
    }
}

#[test]
fn unsupported_plan_enum_values_are_rejected_by_serialization_model() {
    assert!(serde_json::from_value::<RecoveryPlanOutcome>(json!("replay_now")).is_err());
    assert!(
        serde_json::from_value::<AllowedFutureRecoveryAction>(json!("execute_wrapper")).is_err()
    );
    assert!(serde_json::from_value::<RecoveryPlanStatus>(json!("planning_failed")).is_err());
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
fn non_terminal_plan_text_remains_evaluative() {
    let plan = plan_records(&non_terminal_records("exec_evaluative_text"));
    let record = only_plan(&plan);

    assert!(record.plan_reason.contains("may be eligible"));
    assert!(record.plan_reason.contains("evaluation"));
    assert!(!record.plan_reason.contains("authorized"));
    assert!(!record.plan_reason.contains("approved"));
    assert!(!record.plan_reason.contains("ready to execute"));
}

#[test]
fn plan_text_contains_no_executable_intent() {
    let mut records = completed_records("exec_completed_text");
    records.extend(failed_closed_records("exec_failed_text"));
    records.extend(audit_failed_records("exec_audit_text"));
    records.extend(non_terminal_records("exec_non_terminal_text"));
    records.extend(corrupted_records("exec_corrupted_text"));

    let plan = plan_records(&records);
    let output = serde_json::to_value(plan).expect("plan should serialize");

    assert_no_forbidden_text(
        &output,
        &[
            "rerun command",
            "resume now",
            "run wrapper",
            "call wrapper",
            "load policy",
            "inject credential",
            "write audit",
            "write state",
            "approve automatically",
        ],
    );
}

#[test]
fn plan_output_contains_no_wrapper_parameters() {
    let plan = plan_records(&records_with_wrapper_parameters("exec_no_wrapper_params"));
    let output = serde_json::to_string(&plan).expect("plan should serialize");

    assert!(!output.contains("sandbox.note.write"));
    assert!(!output.contains("example-note"));
    assert!(!output.contains("hello from aegis"));
    assert!(!output.contains("/tmp/aegis-sandbox"));
    assert!(!output.contains("caller_supplied_idempotency_key"));
}

#[test]
fn plan_output_contains_no_credential_material_or_handles() {
    let plan = plan_records(&records_with_execution_details("exec_no_credentials"));
    let output = serde_json::to_string(&plan).expect("plan should serialize");

    assert!(!output.contains("local_runtime"));
    assert!(!output.contains("local_development"));
    assert!(!output.contains("safe_local_handle_ref"));
    assert!(!output.contains("credential_handle_ref"));
    assert!(!output.contains("credential_class"));
}

#[test]
fn planning_errors_have_normalized_shape() {
    let path = test_dir("planning_error_shape").join("missing.jsonl");
    let inspection = ExecutionRecoveryInspector::inspect_path(path);
    let plan = RecoveryPlanGenerator::plan(&inspection);
    let error = plan
        .planning_errors
        .first()
        .expect("planning error should exist");
    let output = serde_json::to_value(error).expect("error should serialize");

    for field in [
        "code",
        "severity",
        "message",
        "reason",
        "next_action",
        "location",
    ] {
        assert!(output.get(field).is_some(), "missing field {field}");
        assert!(
            !output[field].as_str().unwrap_or_default().is_empty(),
            "empty field {field}"
        );
    }
}

#[test]
fn planning_errors_do_not_leak_secret_like_content() {
    let inspection = ExecutionRecoveryInspector::inspect_str(
        "{password=abc token=def secret=ghi private_key=jkl authorization=mno credential=pqr}\n",
    );
    let plan = RecoveryPlanGenerator::plan(&inspection);
    let output = serde_json::to_string(&plan).expect("plan should serialize");

    assert_no_forbidden_text(
        &serde_json::from_str::<Value>(&output).expect("plan output should be json"),
        &[
            "abc",
            "def",
            "ghi",
            "jkl",
            "mno",
            "pqr",
            "password=",
            "token=",
            "secret=",
            "private_key=",
            "authorization=",
            "credential=",
        ],
    );
}

#[test]
fn repeated_planning_is_deterministic() {
    let mut records = completed_records("exec_deterministic_completed");
    records.extend(audit_failed_records("exec_deterministic_audit"));
    records.extend(corrupted_records("exec_deterministic_corrupted"));

    let first = serde_json::to_value(plan_records(&records)).expect("plan should serialize");
    let second = serde_json::to_value(plan_records(&records)).expect("plan should serialize");

    assert_eq!(first, second);
}

#[test]
fn plan_ordering_is_deterministic_by_execution_id() {
    let mut records = completed_records("exec_b");
    records.extend(completed_records("exec_a"));
    records.extend(completed_records("exec_c"));

    let plan = plan_records(&records);
    let execution_ids = plan
        .plans
        .iter()
        .map(|record| record.execution_id.as_str())
        .collect::<Vec<_>>();

    assert_eq!(execution_ids, vec!["exec_a", "exec_b", "exec_c"]);
}

#[test]
fn mixed_valid_and_corrupted_executions_are_both_planned_safely() {
    let mut records = completed_records("exec_valid");
    records.extend(corrupted_records("exec_corrupted_mixed"));

    let plan = plan_records(&records);
    let valid = plan_for(&plan, "exec_valid");
    let corrupted = plan_for(&plan, "exec_corrupted_mixed");

    assert_eq!(
        valid.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableTerminal
    );
    assert_eq!(
        corrupted.plan_outcome,
        RecoveryPlanOutcome::NotRecoverableCorrupted
    );
    assert_eq!(
        corrupted.allowed_future_action,
        AllowedFutureRecoveryAction::ManualReviewOnly
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

#[test]
fn cli_plan_recovery_does_not_require_sandbox_dir() {
    let dir = test_dir("cli_no_sandbox_dir");
    let state_path = write_state_log(&dir, &completed_records("exec_cli_no_sandbox"));

    let output = run_plan(&dir, &state_path);

    assert!(output.status.success());
}

#[test]
fn cli_plan_recovery_missing_log_returns_json_without_stderr_noise() {
    let dir = test_dir("cli_missing_log_json");
    let state_path = dir.join("missing.jsonl");

    let output = run_plan(&dir, &state_path);
    let plan: RecoveryPlanReport =
        serde_json::from_slice(&output.stdout).expect("stdout should be a plan report");
    let stderr = String::from_utf8(output.stderr).expect("stderr should be utf8");

    assert!(!output.status.success());
    assert_eq!(plan.plan_status, RecoveryPlanStatus::InspectionFailed);
    assert!(stderr.is_empty());
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

fn plan_for<'a>(
    report: &'a RecoveryPlanReport,
    execution_id: &str,
) -> &'a aegis::state::RecoveryPlanRecord {
    report
        .plans
        .iter()
        .find(|record| record.execution_id == execution_id)
        .expect("plan record should exist")
}

fn recovery_report(executions: Vec<ExecutionRecoveryExecution>) -> ExecutionRecoveryReport {
    ExecutionRecoveryReport {
        inspection_status: ExecutionRecoveryStatus::Inspected,
        executions,
        inspection_errors: Vec::new(),
    }
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

fn records_with_execution_details(execution_id: &str) -> Vec<Value> {
    vec![
        detailed_record(execution_id, "created", "validated", 0),
        detailed_record(execution_id, "validated", "bundle_verified", 1),
        detailed_record(execution_id, "bundle_verified", "policy_evaluated", 2),
    ]
}

fn records_with_wrapper_parameters(execution_id: &str) -> Vec<Value> {
    records_with_execution_details(execution_id)
        .into_iter()
        .map(add_wrapper_parameters)
        .collect()
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

fn detailed_record(
    execution_id: &str,
    previous_state: &str,
    new_state: &str,
    index: usize,
) -> Value {
    let mut record = record(execution_id, previous_state, new_state, index);
    let object = record
        .as_object_mut()
        .expect("record should be a JSON object");

    object.insert("request_id".to_string(), json!("req_detailed"));
    object.insert("tool_name".to_string(), json!("sandbox.note.write"));
    object.insert("policy_bundle_id".to_string(), json!("local-dev"));
    object.insert(
        "policy_rule_id".to_string(),
        json!("allow_sandbox_note_write"),
    );
    object.insert("wrapper_name".to_string(), json!("sandbox.note.write"));
    object.insert("wrapper_version".to_string(), json!("1.0.0"));
    object.insert("authorization_id".to_string(), json!("auth_detailed"));
    object.insert("credential_boundary_status".to_string(), json!("satisfied"));
    object.insert("credential_injection_status".to_string(), json!("injected"));
    object.insert("credential_class".to_string(), json!("local_runtime"));
    object.insert(
        "credential_handle_ref".to_string(),
        json!("safe_local_handle_ref"),
    );
    object.insert(
        "idempotency_key_ref".to_string(),
        json!("caller_supplied_idempotency_key"),
    );

    record
}

fn add_wrapper_parameters(mut record: Value) -> Value {
    let object = record
        .as_object_mut()
        .expect("record should be a JSON object");

    object.insert("note_id".to_string(), json!("example-note"));
    object.insert("content".to_string(), json!("hello from aegis"));
    object.insert("sandbox_dir".to_string(), json!("/tmp/aegis-sandbox"));

    record
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

fn assert_no_forbidden_text(value: &Value, forbidden_terms: &[&str]) {
    let output = value.to_string().to_lowercase();

    for term in forbidden_terms {
        assert!(
            !output.contains(term),
            "plan output contained forbidden term {term}"
        );
    }
}
