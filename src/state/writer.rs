use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use super::{valid_transition, ExecutionLifecycle, ExecutionState};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecutionStateLogPath(pub PathBuf);

impl ExecutionStateLogPath {
    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ExecutionStateLogContext {
    pub execution_id: String,
    pub request_id: Option<String>,
    pub tool_name: Option<String>,
    pub policy_bundle_id: Option<String>,
    pub policy_rule_id: Option<String>,
    pub wrapper_name: Option<String>,
    pub wrapper_version: Option<String>,
    pub authorization_id: Option<String>,
    pub credential_boundary_status: Option<String>,
    pub credential_injection_status: Option<String>,
    pub credential_class: Option<String>,
    pub credential_handle_ref: Option<String>,
    pub idempotency_key_ref: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionStateLogRecord {
    pub execution_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<String>,
    pub previous_state: ExecutionState,
    pub new_state: ExecutionState,
    pub transition_reason: String,
    pub lifecycle_index: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_bundle_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_boundary_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_injection_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_handle_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key_ref: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecutionStateWriteResult {
    pub path: ExecutionStateLogPath,
    pub records_written: usize,
}

#[derive(Debug)]
pub enum ExecutionStateWriteError {
    Open {
        path: ExecutionStateLogPath,
        source: io::Error,
    },
    Serialize {
        path: ExecutionStateLogPath,
        source: serde_json::Error,
    },
    Write {
        path: ExecutionStateLogPath,
        source: io::Error,
    },
    Flush {
        path: ExecutionStateLogPath,
        source: io::Error,
    },
    InvalidTransition {
        previous_state: ExecutionState,
        new_state: ExecutionState,
    },
}

impl std::fmt::Display for ExecutionStateWriteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open { path, source } => {
                write!(
                    formatter,
                    "failed to open execution state log {}: {source}",
                    path.as_path().display()
                )
            }
            Self::Serialize { path, source } => {
                write!(
                    formatter,
                    "failed to serialize execution state record for {}: {source}",
                    path.as_path().display()
                )
            }
            Self::Write { path, source } => {
                write!(
                    formatter,
                    "failed to append execution state log {}: {source}",
                    path.as_path().display()
                )
            }
            Self::Flush { path, source } => {
                write!(
                    formatter,
                    "failed to flush execution state log {}: {source}",
                    path.as_path().display()
                )
            }
            Self::InvalidTransition {
                previous_state,
                new_state,
            } => {
                write!(
                    formatter,
                    "invalid execution state transition {previous_state:?} -> {new_state:?}"
                )
            }
        }
    }
}

impl std::error::Error for ExecutionStateWriteError {}

pub trait ExecutionStateSink {
    fn append_lifecycle(
        &self,
        lifecycle: &ExecutionLifecycle,
        context: &ExecutionStateLogContext,
    ) -> Result<ExecutionStateWriteResult, ExecutionStateWriteError>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExecutionStateWriter {
    path: ExecutionStateLogPath,
}

impl ExecutionStateWriter {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: ExecutionStateLogPath(path.into()),
        }
    }

    pub fn validate_writable(&self) -> Result<(), ExecutionStateWriteError> {
        let mut file = self.open_append_file()?;
        file.flush()
            .map_err(|source| ExecutionStateWriteError::Flush {
                path: self.path.clone(),
                source,
            })
    }

    fn open_append_file(&self) -> Result<File, ExecutionStateWriteError> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path.as_path())
            .map_err(|source| ExecutionStateWriteError::Open {
                path: self.path.clone(),
                source,
            })
    }
}

impl ExecutionStateSink for ExecutionStateWriter {
    fn append_lifecycle(
        &self,
        lifecycle: &ExecutionLifecycle,
        context: &ExecutionStateLogContext,
    ) -> Result<ExecutionStateWriteResult, ExecutionStateWriteError> {
        let records = records_for_lifecycle(lifecycle, context)?;
        let mut file = self.open_append_file()?;

        for record in &records {
            append_record(&mut file, &self.path, record)?;
        }

        file.flush()
            .map_err(|source| ExecutionStateWriteError::Flush {
                path: self.path.clone(),
                source,
            })?;

        Ok(ExecutionStateWriteResult {
            path: self.path.clone(),
            records_written: records.len(),
        })
    }
}

fn records_for_lifecycle(
    lifecycle: &ExecutionLifecycle,
    context: &ExecutionStateLogContext,
) -> Result<Vec<ExecutionStateLogRecord>, ExecutionStateWriteError> {
    lifecycle
        .transitions
        .iter()
        .enumerate()
        .map(|(index, transition)| {
            if !valid_transition(&transition.previous_state, &transition.execution_state) {
                return Err(ExecutionStateWriteError::InvalidTransition {
                    previous_state: transition.previous_state.clone(),
                    new_state: transition.execution_state.clone(),
                });
            }

            Ok(ExecutionStateLogRecord {
                execution_id: context.execution_id.clone(),
                request_id: context.request_id.clone(),
                tool_name: context.tool_name.clone(),
                previous_state: transition.previous_state.clone(),
                new_state: transition.execution_state.clone(),
                transition_reason: transition_reason(&transition.execution_state).to_string(),
                lifecycle_index: index,
                policy_bundle_id: context.policy_bundle_id.clone(),
                policy_rule_id: context.policy_rule_id.clone(),
                wrapper_name: context.wrapper_name.clone(),
                wrapper_version: context.wrapper_version.clone(),
                authorization_id: context.authorization_id.clone(),
                credential_boundary_status: context.credential_boundary_status.clone(),
                credential_injection_status: context.credential_injection_status.clone(),
                credential_class: context.credential_class.clone(),
                credential_handle_ref: context.credential_handle_ref.clone(),
                idempotency_key_ref: context.idempotency_key_ref.clone(),
            })
        })
        .collect()
}

fn append_record(
    file: &mut File,
    path: &ExecutionStateLogPath,
    record: &ExecutionStateLogRecord,
) -> Result<(), ExecutionStateWriteError> {
    let line =
        serde_json::to_string(record).map_err(|source| ExecutionStateWriteError::Serialize {
            path: path.clone(),
            source,
        })?;

    file.write_all(line.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|source| ExecutionStateWriteError::Write {
            path: path.clone(),
            source,
        })
}

fn transition_reason(state: &ExecutionState) -> &'static str {
    match state {
        ExecutionState::Created => "created",
        ExecutionState::Validated => "request_validated",
        ExecutionState::BundleVerified => "policy_bundle_verified",
        ExecutionState::PolicyEvaluated => "policy_evaluated",
        ExecutionState::Authorized => "execution_authorized",
        ExecutionState::Dispatching => "wrapper_dispatching",
        ExecutionState::Executed => "wrapper_executed",
        ExecutionState::Audited => "audit_recorded",
        ExecutionState::Completed => "execution_completed",
        ExecutionState::FailedClosed => "failed_closed",
        ExecutionState::AuditFailed => "audit_failed",
    }
}
