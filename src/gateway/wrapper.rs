use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::{
    AuthorizationError, CredentialBoundary, CredentialBoundaryError, CredentialRequirement,
    ExecutionAuthorization,
};

use super::{NonEmptyString, ToolCallRequest};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperConfigRef {
    pub wrapper_name: NonEmptyString,
    pub wrapper_version: NonEmptyString,
    pub target_system: NonEmptyString,
    pub config_reference: Option<NonEmptyString>,
    pub config_digest: Option<NonEmptyString>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RedactionProfileRef(pub NonEmptyString);

impl RedactionProfileRef {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemSchemaVersion(pub NonEmptyString);

impl ExternalSystemSchemaVersion {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrapperExecutionMode {
    ObserveOnly,
    Enforce,
    DryRun,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperExecutionContext {
    pub config: WrapperConfigRef,
    pub external_system_schema_version: ExternalSystemSchemaVersion,
    pub redaction_profile: RedactionProfileRef,
    pub execution_mode: WrapperExecutionMode,
    pub credential_injection_required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_root: Option<NonEmptyString>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrapperExecutionStatus {
    Executed,
    Observed,
    DryRun,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperExecutionOutput {
    pub result: Option<BTreeMap<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperExecutionResult {
    pub context: WrapperExecutionContext,
    pub credential_boundary: CredentialBoundary,
    pub status: WrapperExecutionStatus,
    pub result: Option<BTreeMap<String, Value>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrapperExecutionEvidence {
    pub wrapper_name: NonEmptyString,
    pub wrapper_version: NonEmptyString,
    pub wrapper_status: WrapperExecutionStatus,
    pub wrapper_execution_mode: WrapperExecutionMode,
    pub wrapper_result_summary: Option<BTreeMap<String, String>>,
}

impl From<&WrapperExecutionResult> for WrapperExecutionEvidence {
    fn from(result: &WrapperExecutionResult) -> Self {
        Self {
            wrapper_name: result.context.config.wrapper_name.clone(),
            wrapper_version: result.context.config.wrapper_version.clone(),
            wrapper_status: result.status.clone(),
            wrapper_execution_mode: result.context.execution_mode.clone(),
            wrapper_result_summary: result.result.as_ref().map(string_result_summary),
        }
    }
}

fn string_result_summary(result: &BTreeMap<String, Value>) -> BTreeMap<String, String> {
    result
        .iter()
        .filter_map(|(key, value)| value.as_str().map(|text| (key.clone(), text.to_string())))
        .collect()
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WrapperExecutionError {
    pub reason_code: Option<String>,
    pub safe_message: String,
}

pub trait WrapperExecutor {
    fn wrapper_name(&self) -> &str;
    fn wrapper_version(&self) -> &str;
    fn credential_requirement(&self) -> CredentialRequirement;

    fn execute(
        &self,
        request: &ToolCallRequest,
        context: &WrapperExecutionContext,
        authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum WrapperDispatchError {
    MissingWrapper {
        wrapper_name: String,
    },
    IncompatibleWrapperVersion {
        wrapper_name: String,
        requested_version: String,
    },
    AuthorizationFailed(AuthorizationError),
    CredentialBoundaryFailed {
        error: CredentialBoundaryError,
        boundary: CredentialBoundary,
    },
    ExecutionFailed(WrapperExecutionError),
}

impl WrapperDispatchError {
    pub fn reason_code(&self) -> &str {
        match self {
            Self::MissingWrapper { .. } => "wrapper_missing",
            Self::IncompatibleWrapperVersion { .. } => "wrapper_version_incompatible",
            Self::AuthorizationFailed(error) => error.reason_code(),
            Self::CredentialBoundaryFailed { error, .. } => error.reason_code(),
            Self::ExecutionFailed(error) => error
                .reason_code
                .as_deref()
                .unwrap_or("wrapper_execution_failed"),
        }
    }

    pub fn safe_message(&self) -> String {
        match self {
            Self::MissingWrapper { wrapper_name } => {
                format!("Required wrapper is not registered: {wrapper_name}.")
            }
            Self::IncompatibleWrapperVersion {
                wrapper_name,
                requested_version,
            } => {
                format!(
                    "Required wrapper version is not registered: {wrapper_name}@{requested_version}."
                )
            }
            Self::AuthorizationFailed(error) => error.safe_message(),
            Self::CredentialBoundaryFailed { error, .. } => error.safe_message(),
            Self::ExecutionFailed(error) => error.safe_message.clone(),
        }
    }
}

pub struct WrapperDispatcher<'a> {
    executors: Vec<&'a dyn WrapperExecutor>,
}

impl<'a> WrapperDispatcher<'a> {
    pub fn new(executors: impl IntoIterator<Item = &'a dyn WrapperExecutor>) -> Self {
        Self {
            executors: executors.into_iter().collect(),
        }
    }

    pub fn dispatch(
        &self,
        request: &ToolCallRequest,
        context: &WrapperExecutionContext,
        authorization: &ExecutionAuthorization,
    ) -> Result<WrapperExecutionResult, WrapperDispatchError> {
        authorization
            .validate_for(request, context)
            .map_err(WrapperDispatchError::AuthorizationFailed)?;
        let executor = self.executor_for(context)?;
        let credential_boundary = self.credential_boundary_for(executor, authorization)?;
        let output = executor
            .execute(request, context, authorization)
            .map_err(WrapperDispatchError::ExecutionFailed)?;

        Ok(WrapperExecutionResult {
            context: context.clone(),
            credential_boundary,
            status: status_for_mode(&context.execution_mode),
            result: output.result,
        })
    }

    fn executor_for(
        &self,
        context: &WrapperExecutionContext,
    ) -> Result<&'a dyn WrapperExecutor, WrapperDispatchError> {
        if let Some(executor) = self.matching_executor(context) {
            return Ok(executor);
        }

        Err(self.dispatch_error_for(context))
    }

    fn matching_executor(
        &self,
        context: &WrapperExecutionContext,
    ) -> Option<&'a dyn WrapperExecutor> {
        self.executors
            .iter()
            .copied()
            .find(|executor| wrapper_identity_matches(*executor, context))
    }

    fn dispatch_error_for(&self, context: &WrapperExecutionContext) -> WrapperDispatchError {
        let wrapper_name = context.config.wrapper_name.as_str().to_string();
        let requested_version = context.config.wrapper_version.as_str().to_string();

        if self
            .executors
            .iter()
            .any(|executor| executor.wrapper_name() == wrapper_name)
        {
            return WrapperDispatchError::IncompatibleWrapperVersion {
                wrapper_name,
                requested_version,
            };
        }

        WrapperDispatchError::MissingWrapper { wrapper_name }
    }

    fn credential_boundary_for(
        &self,
        executor: &dyn WrapperExecutor,
        authorization: &ExecutionAuthorization,
    ) -> Result<CredentialBoundary, WrapperDispatchError> {
        let requirement = executor.credential_requirement();
        let boundary = CredentialBoundary::evaluate(&requirement, authorization);

        boundary
            .validate()
            .map(|()| boundary.clone())
            .map_err(|error| WrapperDispatchError::CredentialBoundaryFailed { error, boundary })
    }
}

fn wrapper_identity_matches(
    executor: &dyn WrapperExecutor,
    context: &WrapperExecutionContext,
) -> bool {
    executor.wrapper_name() == context.config.wrapper_name.as_str()
        && executor.wrapper_version() == context.config.wrapper_version.as_str()
}

fn status_for_mode(mode: &WrapperExecutionMode) -> WrapperExecutionStatus {
    match mode {
        WrapperExecutionMode::ObserveOnly => WrapperExecutionStatus::Observed,
        WrapperExecutionMode::Enforce => WrapperExecutionStatus::Executed,
        WrapperExecutionMode::DryRun => WrapperExecutionStatus::DryRun,
    }
}
