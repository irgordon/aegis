use serde::{Deserialize, Serialize};

use crate::error::{ErrorCode, ErrorLocation, ErrorSeverity, OperatorAction};

use super::{
    ExecutionRecoverability, ExecutionRecoveryExecution, ExecutionRecoveryReport,
    ExecutionRecoveryStatus, ExecutionState, ExecutionTerminalStatus, MalformedStateRecord,
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPlanStatus {
    Planned,
    InspectionFailed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPlanOutcome {
    NotRecoverableTerminal,
    NotRecoverableCorrupted,
    CandidateForAuditRetry,
    CandidateForFutureReplay,
    InspectionFailed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedFutureRecoveryAction {
    None,
    AuditRetryOnly,
    FutureReplayEvaluationOnly,
    ManualReviewOnly,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryPlanReport {
    pub plan_status: RecoveryPlanStatus,
    pub plans: Vec<RecoveryPlanRecord>,
    pub planning_errors: Vec<RecoveryPlanningError>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryPlanRecord {
    pub execution_id: String,
    pub last_known_state: ExecutionState,
    pub terminal_status: ExecutionTerminalStatus,
    pub inspection_status: ExecutionRecoveryStatus,
    pub recoverability_status: ExecutionRecoverability,
    pub plan_outcome: RecoveryPlanOutcome,
    pub plan_reason: String,
    pub allowed_future_action: AllowedFutureRecoveryAction,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecoveryPlanningError {
    pub code: ErrorCode,
    pub severity: ErrorSeverity,
    pub message: String,
    pub reason: String,
    pub next_action: OperatorAction,
    pub location: ErrorLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_error_code: Option<ErrorCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_index: Option<usize>,
}

pub struct RecoveryPlanGenerator;

impl RecoveryPlanGenerator {
    pub fn plan(report: &ExecutionRecoveryReport) -> RecoveryPlanReport {
        RecoveryPlanReport {
            plan_status: plan_status(report),
            plans: plan_records(report),
            planning_errors: planning_errors(report),
        }
    }
}

fn plan_status(report: &ExecutionRecoveryReport) -> RecoveryPlanStatus {
    match report.inspection_status {
        ExecutionRecoveryStatus::Inspected => RecoveryPlanStatus::Planned,
        ExecutionRecoveryStatus::InspectionFailed => RecoveryPlanStatus::InspectionFailed,
    }
}

fn plan_records(report: &ExecutionRecoveryReport) -> Vec<RecoveryPlanRecord> {
    report
        .executions
        .iter()
        .map(|execution| plan_record(report.inspection_status.clone(), execution))
        .collect()
}

fn plan_record(
    inspection_status: ExecutionRecoveryStatus,
    execution: &ExecutionRecoveryExecution,
) -> RecoveryPlanRecord {
    let classification = classify_execution(execution);

    RecoveryPlanRecord {
        execution_id: execution.execution_id.clone(),
        last_known_state: execution.last_known_state.clone(),
        terminal_status: execution.terminal_status.clone(),
        inspection_status,
        recoverability_status: execution.recoverability.clone(),
        plan_outcome: classification.outcome,
        plan_reason: classification.reason.to_string(),
        allowed_future_action: classification.allowed_action,
    }
}

struct PlanClassification {
    outcome: RecoveryPlanOutcome,
    reason: &'static str,
    allowed_action: AllowedFutureRecoveryAction,
}

fn classify_execution(execution: &ExecutionRecoveryExecution) -> PlanClassification {
    if execution_is_corrupted(execution) {
        return corrupted_classification();
    }

    match execution.last_known_state {
        ExecutionState::Completed => terminal_completed_classification(),
        ExecutionState::FailedClosed => terminal_failed_closed_classification(),
        ExecutionState::AuditFailed => audit_failed_classification(),
        _ => non_terminal_classification(),
    }
}

fn execution_is_corrupted(execution: &ExecutionRecoveryExecution) -> bool {
    recovery_status_is_unsafe(&execution.recoverability)
        || !terminal_status_matches_state(&execution.terminal_status, &execution.last_known_state)
}

fn recovery_status_is_unsafe(recoverability: &ExecutionRecoverability) -> bool {
    matches!(
        recoverability,
        ExecutionRecoverability::InspectionFailed | ExecutionRecoverability::Unknown
    )
}

fn terminal_status_matches_state(
    terminal_status: &ExecutionTerminalStatus,
    state: &ExecutionState,
) -> bool {
    matches!(
        (terminal_status, state_is_terminal(state)),
        (ExecutionTerminalStatus::Terminal, true) | (ExecutionTerminalStatus::NonTerminal, false)
    )
}

fn state_is_terminal(state: &ExecutionState) -> bool {
    matches!(
        state,
        ExecutionState::Completed | ExecutionState::FailedClosed | ExecutionState::AuditFailed
    )
}

fn terminal_completed_classification() -> PlanClassification {
    PlanClassification {
        outcome: RecoveryPlanOutcome::NotRecoverableTerminal,
        reason: "execution already completed",
        allowed_action: AllowedFutureRecoveryAction::None,
    }
}

fn terminal_failed_closed_classification() -> PlanClassification {
    PlanClassification {
        outcome: RecoveryPlanOutcome::NotRecoverableTerminal,
        reason: "execution failed closed and must not be resumed automatically",
        allowed_action: AllowedFutureRecoveryAction::None,
    }
}

fn audit_failed_classification() -> PlanClassification {
    PlanClassification {
        outcome: RecoveryPlanOutcome::CandidateForAuditRetry,
        reason:
            "execution reached audit failure and may be eligible for future audit-specific recovery",
        allowed_action: AllowedFutureRecoveryAction::AuditRetryOnly,
    }
}

fn non_terminal_classification() -> PlanClassification {
    PlanClassification {
        outcome: RecoveryPlanOutcome::CandidateForFutureReplay,
        reason: "execution has valid non-terminal evidence and may be eligible for future replay evaluation",
        allowed_action: AllowedFutureRecoveryAction::FutureReplayEvaluationOnly,
    }
}

fn corrupted_classification() -> PlanClassification {
    PlanClassification {
        outcome: RecoveryPlanOutcome::NotRecoverableCorrupted,
        reason: "state evidence is corrupted or inconsistent",
        allowed_action: AllowedFutureRecoveryAction::ManualReviewOnly,
    }
}

fn planning_errors(report: &ExecutionRecoveryReport) -> Vec<RecoveryPlanningError> {
    report
        .inspection_errors
        .iter()
        .map(planning_error_from_inspection_error)
        .collect()
}

fn planning_error_from_inspection_error(error: &MalformedStateRecord) -> RecoveryPlanningError {
    RecoveryPlanningError {
        code: ErrorCode::RecoveryPlanInspectionFailed,
        severity: ErrorSeverity::Error,
        message: "A recovery plan could not trust all inspected state evidence.".to_string(),
        reason: "state evidence could not be inspected safely".to_string(),
        next_action: OperatorAction(
            "Review the inspection errors and preserve the original state log.".to_string(),
        ),
        location: ErrorLocation::RecoveryPlanGeneration,
        source_error_code: Some(error.code.clone()),
        execution_id: error.execution_id.clone(),
        line_number: error.line_number,
        lifecycle_index: error.lifecycle_index,
    }
}
