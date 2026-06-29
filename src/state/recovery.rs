use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{ErrorCode, ErrorLocation, ErrorSeverity, OperatorAction};

use super::{valid_transition, ExecutionState, ExecutionStateLogRecord};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionRecoveryStatus {
    Inspected,
    InspectionFailed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionTerminalStatus {
    Terminal,
    NonTerminal,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionRecoverability {
    NotRecoverableTerminal,
    RecoverableCandidate,
    InspectionFailed,
    Unknown,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionRecoveryReport {
    pub inspection_status: ExecutionRecoveryStatus,
    pub executions: Vec<ExecutionRecoveryExecution>,
    pub inspection_errors: Vec<MalformedStateRecord>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionRecoveryExecution {
    pub execution_id: String,
    pub last_known_state: ExecutionState,
    pub terminal_status: ExecutionTerminalStatus,
    pub recoverability: ExecutionRecoverability,
    pub transition_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MalformedStateRecord {
    pub code: ErrorCode,
    pub severity: ErrorSeverity,
    pub message: String,
    pub reason: String,
    pub next_action: OperatorAction,
    pub location: ErrorLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_number: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_index: Option<usize>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StateLogInspectionPath(pub PathBuf);

pub struct ExecutionRecoveryInspector;

impl ExecutionRecoveryInspector {
    pub fn inspect_path(path: impl AsRef<Path>) -> ExecutionRecoveryReport {
        match fs::read_to_string(path.as_ref()) {
            Ok(content) => Self::inspect_str(&content),
            Err(_) => report_with_error(read_failed(path.as_ref())),
        }
    }

    pub fn inspect_str(content: &str) -> ExecutionRecoveryReport {
        let mut groups = BTreeMap::new();
        let mut errors = Vec::new();

        for (line_index, line) in content.lines().enumerate() {
            collect_record(line_index + 1, line, &mut groups, &mut errors);
        }

        let executions = summarize_executions(groups, &mut errors);
        recovery_report(executions, errors)
    }
}

fn collect_record(
    line_number: usize,
    line: &str,
    groups: &mut BTreeMap<String, Vec<ExecutionStateLogRecord>>,
    errors: &mut Vec<MalformedStateRecord>,
) {
    match parse_record(line_number, line) {
        Ok(record) => {
            groups
                .entry(record.execution_id.clone())
                .or_default()
                .push(record);
        }
        Err(error) => errors.push(*error),
    }
}

fn parse_record(
    line_number: usize,
    line: &str,
) -> Result<ExecutionStateLogRecord, Box<MalformedStateRecord>> {
    let value: Value =
        serde_json::from_str(line).map_err(|_| Box::new(invalid_json(line_number)))?;
    reject_unknown_state(line_number, &value, "previous_state")?;
    reject_unknown_state(line_number, &value, "new_state")?;
    serde_json::from_value(value).map_err(|_| Box::new(invalid_json(line_number)))
}

fn reject_unknown_state(
    line_number: usize,
    value: &Value,
    field_name: &str,
) -> Result<(), Box<MalformedStateRecord>> {
    let Some(Value::String(state)) = value.get(field_name) else {
        return Ok(());
    };

    serde_json::from_value::<ExecutionState>(Value::String(state.clone()))
        .map(|_| ())
        .map_err(|_| Box::new(unknown_state(line_number)))
}

fn summarize_executions(
    groups: BTreeMap<String, Vec<ExecutionStateLogRecord>>,
    errors: &mut Vec<MalformedStateRecord>,
) -> Vec<ExecutionRecoveryExecution> {
    groups
        .into_iter()
        .map(|(execution_id, records)| summarize_execution(execution_id, records, errors))
        .collect()
}

fn summarize_execution(
    execution_id: String,
    records: Vec<ExecutionStateLogRecord>,
    errors: &mut Vec<MalformedStateRecord>,
) -> ExecutionRecoveryExecution {
    let has_errors = validate_execution_records(&execution_id, &records, errors);
    let last = records
        .last()
        .expect("execution group should contain records");
    execution_summary(&execution_id, last, records.len(), has_errors)
}

fn validate_execution_records(
    execution_id: &str,
    records: &[ExecutionStateLogRecord],
    errors: &mut Vec<MalformedStateRecord>,
) -> bool {
    let mut seen_indexes = BTreeSet::new();
    let mut previous_new_state = None;
    let mut has_errors = false;

    for (position, record) in records.iter().enumerate() {
        has_errors |=
            validate_record_order(execution_id, record, position, &mut seen_indexes, errors);
        has_errors |=
            validate_record_transition(execution_id, record, previous_new_state.as_ref(), errors);
        previous_new_state = Some(record.new_state.clone());
    }

    has_errors
}

fn validate_record_order(
    execution_id: &str,
    record: &ExecutionStateLogRecord,
    position: usize,
    seen_indexes: &mut BTreeSet<usize>,
    errors: &mut Vec<MalformedStateRecord>,
) -> bool {
    let mut has_errors = false;

    if position == 0 && record.lifecycle_index != 0 {
        errors.push(missing_first_state(execution_id, record));
        has_errors = true;
    }

    if !seen_indexes.insert(record.lifecycle_index) {
        errors.push(duplicate_index(execution_id, record));
        return true;
    }

    if record.lifecycle_index != position {
        errors.push(order_invalid(execution_id, record));
        has_errors = true;
    }

    has_errors
}

fn validate_record_transition(
    execution_id: &str,
    record: &ExecutionStateLogRecord,
    previous_new_state: Option<&ExecutionState>,
    errors: &mut Vec<MalformedStateRecord>,
) -> bool {
    if !transition_is_valid(record, previous_new_state) {
        errors.push(invalid_transition(execution_id, record));
        return true;
    }

    false
}

fn transition_is_valid(
    record: &ExecutionStateLogRecord,
    previous_new_state: Option<&ExecutionState>,
) -> bool {
    valid_transition(&record.previous_state, &record.new_state)
        && previous_new_state.is_none_or(|state| state == &record.previous_state)
}

fn execution_summary(
    execution_id: &str,
    last: &ExecutionStateLogRecord,
    transition_count: usize,
    has_errors: bool,
) -> ExecutionRecoveryExecution {
    ExecutionRecoveryExecution {
        execution_id: execution_id.to_string(),
        last_known_state: last.new_state.clone(),
        terminal_status: terminal_status(&last.new_state),
        recoverability: recoverability(&last.new_state, has_errors),
        transition_count,
        failure_reason: failure_reason(last),
    }
}

fn terminal_status(state: &ExecutionState) -> ExecutionTerminalStatus {
    match state {
        ExecutionState::Completed | ExecutionState::FailedClosed | ExecutionState::AuditFailed => {
            ExecutionTerminalStatus::Terminal
        }
        _ => ExecutionTerminalStatus::NonTerminal,
    }
}

fn recoverability(state: &ExecutionState, has_errors: bool) -> ExecutionRecoverability {
    if has_errors {
        return ExecutionRecoverability::InspectionFailed;
    }

    match state {
        ExecutionState::Completed | ExecutionState::FailedClosed => {
            ExecutionRecoverability::NotRecoverableTerminal
        }
        ExecutionState::AuditFailed => ExecutionRecoverability::RecoverableCandidate,
        _ => ExecutionRecoverability::RecoverableCandidate,
    }
}

fn failure_reason(last: &ExecutionStateLogRecord) -> Option<String> {
    match last.new_state {
        ExecutionState::FailedClosed | ExecutionState::AuditFailed => {
            Some(last.transition_reason.clone())
        }
        _ => None,
    }
}

fn recovery_report(
    executions: Vec<ExecutionRecoveryExecution>,
    inspection_errors: Vec<MalformedStateRecord>,
) -> ExecutionRecoveryReport {
    let inspection_status = if inspection_errors.is_empty() {
        ExecutionRecoveryStatus::Inspected
    } else {
        ExecutionRecoveryStatus::InspectionFailed
    };

    ExecutionRecoveryReport {
        inspection_status,
        executions,
        inspection_errors,
    }
}

fn report_with_error(error: MalformedStateRecord) -> ExecutionRecoveryReport {
    recovery_report(Vec::new(), vec![error])
}

fn read_failed(path: &Path) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionReadFailed,
        "The execution state log could not be read.",
        format!(
            "The state log file was missing or unreadable: {}.",
            path.display()
        ),
        "Check the state log path and file permissions, then run inspection again.",
        None,
        None,
        None,
    )
}

fn invalid_json(line_number: usize) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionInvalidJsonRecord,
        "A state log record could not be read.",
        "One line in the state log is not valid JSON for a lifecycle transition.",
        "Inspect the state log producer and keep the original file for incident review.",
        Some(line_number),
        None,
        None,
    )
}

fn unknown_state(line_number: usize) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionUnknownLifecycleState,
        "A state log record uses an unknown lifecycle state.",
        "The record contains a lifecycle state that is not part of the AEGIS state model.",
        "Use only the bounded execution states supported by this version of AEGIS.",
        Some(line_number),
        None,
        None,
    )
}

fn invalid_transition(
    execution_id: &str,
    record: &ExecutionStateLogRecord,
) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionInvalidTransition,
        "A state log transition is not valid.",
        "The record moves between lifecycle states in an order AEGIS does not allow.",
        "Preserve the log and inspect the runtime path that produced this transition.",
        None,
        Some(execution_id),
        Some(record.lifecycle_index),
    )
}

fn duplicate_index(execution_id: &str, record: &ExecutionStateLogRecord) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionDuplicateLifecycleIndex,
        "A state log contains a duplicate lifecycle index.",
        "Two records for the same execution use the same lifecycle index.",
        "Preserve the log and inspect the state writer before attempting recovery.",
        None,
        Some(execution_id),
        Some(record.lifecycle_index),
    )
}

fn order_invalid(execution_id: &str, record: &ExecutionStateLogRecord) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionLifecycleOrderInvalid,
        "A state log has lifecycle records out of order.",
        "The lifecycle index does not match the preserved record order.",
        "Preserve the log and inspect the state writer before attempting recovery.",
        None,
        Some(execution_id),
        Some(record.lifecycle_index),
    )
}

fn missing_first_state(
    execution_id: &str,
    record: &ExecutionStateLogRecord,
) -> MalformedStateRecord {
    state_error(
        ErrorCode::StateInspectionMissingFirstState,
        "A state log appears to be missing its first transition.",
        "The first record for this execution does not start at lifecycle index 0.",
        "Preserve the log and inspect earlier state log writes for this execution.",
        None,
        Some(execution_id),
        Some(record.lifecycle_index),
    )
}

fn state_error(
    code: ErrorCode,
    message: impl Into<String>,
    reason: impl Into<String>,
    next_action: impl Into<String>,
    line_number: Option<usize>,
    execution_id: Option<&str>,
    lifecycle_index: Option<usize>,
) -> MalformedStateRecord {
    MalformedStateRecord {
        code,
        severity: ErrorSeverity::Error,
        message: message.into(),
        reason: reason.into(),
        next_action: OperatorAction(next_action.into()),
        location: ErrorLocation::ExecutionRecoveryInspection,
        line_number,
        execution_id: execution_id.map(str::to_string),
        lifecycle_index,
    }
}
