use serde::{Deserialize, Serialize};

use crate::{
    audit::AuditWriteError,
    auth::{AuthorizationError, ExecutionAuthorization},
    gateway::{
        NonEmptyString, ToolCallRequest, ToolCallResponse, WrapperDispatchError,
        WrapperExecutionContext,
    },
    policy::{
        PolicyBundleRef, PolicyBundleVerification, PolicyEvaluation, PolicyEvaluationFailure,
        PolicyEvaluationStatus,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    MalformedRequest,
    UnsupportedTool,
    PolicyDenied,
    PolicyBundleVerificationFailed,
    PolicyEvaluationFailed,
    AuthorizationMissing,
    AuthorizationWrapperMismatch,
    AuthorizationVersionMismatch,
    AuthorizationCapabilityMismatch,
    AuthorizationScopeInvalid,
    AuthorizationDenied,
    WrapperDispatchFailed,
    AuditPersistenceFailed,
    RuntimeIoFailed,
    UnexpectedInternal,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorSeverity {
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorLocation {
    RequestValidation,
    PolicyBundleVerification,
    PolicyEvaluation,
    ExecutionAuthorization,
    WrapperDispatch,
    AuditPersistence,
    RuntimeIo,
    UnexpectedInternal,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperatorAction(pub String);

impl OperatorAction {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatewayErrorReport {
    pub code: ErrorCode,
    pub severity: ErrorSeverity,
    pub message: String,
    pub reason: String,
    pub next_action: OperatorAction,
    pub location: ErrorLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_bundle_id: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_name: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_name: Option<NonEmptyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_error_kind: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditErrorReport {
    pub error_code: ErrorCode,
    pub error_location: ErrorLocation,
    pub error_reason: String,
    pub next_action: OperatorAction,
}

impl GatewayErrorReport {
    pub fn from_validation_denial(response: &ToolCallResponse) -> Self {
        match response.reason_code.as_deref() {
            Some("unsupported_tool") => Self::unsupported_tool(response),
            _ => Self::malformed_request(response),
        }
    }

    pub fn policy_bundle_verification_failed(bundle: &PolicyBundleVerification) -> Self {
        Self {
            code: ErrorCode::PolicyBundleVerificationFailed,
            severity: ErrorSeverity::Error,
            message: "The policy bundle could not be used.".to_string(),
            reason: bundle_failure_reason(bundle),
            next_action: OperatorAction(
                "Check the bundle files, checksums, and signature, then rerun validation."
                    .to_string(),
            ),
            location: ErrorLocation::PolicyBundleVerification,
            request_id: None,
            execution_id: None,
            policy_bundle_id: bundle_id(bundle.bundle.as_ref()),
            tool_name: None,
            wrapper_name: None,
            source_error_kind: Some("policy_bundle_rejected".to_string()),
        }
    }

    pub fn policy_evaluation_report(
        request: &ToolCallRequest,
        evaluation: &PolicyEvaluation,
    ) -> Option<Self> {
        match evaluation.evaluation_status {
            PolicyEvaluationStatus::Evaluated if evaluation.decision_is_deny() => {
                Some(Self::policy_denied(request, evaluation))
            }
            PolicyEvaluationStatus::FailedClosed => {
                Some(Self::policy_evaluation_failed(request, evaluation))
            }
            _ => None,
        }
    }

    pub fn wrapper_dispatch_failed(
        error: &WrapperDispatchError,
        context: &WrapperExecutionContext,
    ) -> Self {
        Self {
            code: ErrorCode::WrapperDispatchFailed,
            severity: ErrorSeverity::Error,
            message: "The wrapper could not be dispatched.".to_string(),
            reason: error.safe_message(),
            next_action: wrapper_next_action(error),
            location: ErrorLocation::WrapperDispatch,
            request_id: None,
            execution_id: None,
            policy_bundle_id: None,
            tool_name: None,
            wrapper_name: Some(context.config.wrapper_name.clone()),
            source_error_kind: Some(error.reason_code().to_string()),
        }
    }

    pub fn execution_authorization_failed(
        error: &AuthorizationError,
        authorization: &ExecutionAuthorization,
        request: &ToolCallRequest,
    ) -> Self {
        Self {
            code: auth_error_code(error),
            severity: ErrorSeverity::Error,
            message: "Wrapper execution was not authorized.".to_string(),
            reason: error.safe_message(),
            next_action: auth_next_action(error),
            location: ErrorLocation::ExecutionAuthorization,
            request_id: Some(request.request_id.clone()),
            execution_id: Some(authorization.binding.execution_id.clone()),
            policy_bundle_id: None,
            tool_name: Some(request.tool.name.clone()),
            wrapper_name: Some(authorization.binding.wrapper_name.clone()),
            source_error_kind: Some(error.reason_code().to_string()),
        }
    }

    pub fn audit_persistence_failed(error: &AuditWriteError, response: &ToolCallResponse) -> Self {
        Self {
            code: ErrorCode::AuditPersistenceFailed,
            severity: ErrorSeverity::Critical,
            message: "The audit record could not be saved.".to_string(),
            reason: "The local audit log could not be opened, written, serialized, or flushed."
                .to_string(),
            next_action: OperatorAction(
                "Check the audit log path and file permissions, then rerun the gateway."
                    .to_string(),
            ),
            location: ErrorLocation::AuditPersistence,
            request_id: response.request_id.clone(),
            execution_id: Some(response.execution_id.clone()),
            policy_bundle_id: None,
            tool_name: None,
            wrapper_name: None,
            source_error_kind: Some(audit_error_kind(error).to_string()),
        }
    }

    pub fn runtime_io_failed(reason: impl Into<String>, source_error_kind: &str) -> Self {
        Self {
            code: ErrorCode::RuntimeIoFailed,
            severity: ErrorSeverity::Error,
            message: "The runtime could not read the request input.".to_string(),
            reason: reason.into(),
            next_action: OperatorAction(
                "Check the command arguments and input path, then run the gateway again."
                    .to_string(),
            ),
            location: ErrorLocation::RuntimeIo,
            request_id: None,
            execution_id: None,
            policy_bundle_id: None,
            tool_name: None,
            wrapper_name: None,
            source_error_kind: Some(source_error_kind.to_string()),
        }
    }

    pub fn unexpected_internal(reason: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::UnexpectedInternal,
            severity: ErrorSeverity::Critical,
            message: "The gateway hit an unexpected internal error.".to_string(),
            reason: reason.into(),
            next_action: OperatorAction(
                "Save the command output and open a bug report with the failing input.".to_string(),
            ),
            location: ErrorLocation::UnexpectedInternal,
            request_id: None,
            execution_id: None,
            policy_bundle_id: None,
            tool_name: None,
            wrapper_name: None,
            source_error_kind: None,
        }
    }

    pub fn audit_fields(&self) -> AuditErrorReport {
        AuditErrorReport {
            error_code: self.code.clone(),
            error_location: self.location.clone(),
            error_reason: self.reason.clone(),
            next_action: self.next_action.clone(),
        }
    }

    fn malformed_request(response: &ToolCallResponse) -> Self {
        Self {
            code: ErrorCode::MalformedRequest,
            severity: ErrorSeverity::Error,
            message: "The request could not be used.".to_string(),
            reason: "The request JSON is malformed or does not match the required schema."
                .to_string(),
            next_action: OperatorAction(
                "Fix the request JSON and run the gateway again.".to_string(),
            ),
            location: ErrorLocation::RequestValidation,
            request_id: response.request_id.clone(),
            execution_id: Some(response.execution_id.clone()),
            policy_bundle_id: None,
            tool_name: None,
            wrapper_name: None,
            source_error_kind: response.reason_code.clone(),
        }
    }

    fn unsupported_tool(response: &ToolCallResponse) -> Self {
        Self {
            code: ErrorCode::UnsupportedTool,
            severity: ErrorSeverity::Error,
            message: "The requested tool is not supported.".to_string(),
            reason: "The requested tool is not in the local gateway allowlist.".to_string(),
            next_action: OperatorAction(
                "Choose a supported tool or update the local gateway allowlist.".to_string(),
            ),
            location: ErrorLocation::RequestValidation,
            request_id: response.request_id.clone(),
            execution_id: Some(response.execution_id.clone()),
            policy_bundle_id: None,
            tool_name: None,
            wrapper_name: None,
            source_error_kind: response.reason_code.clone(),
        }
    }

    fn policy_denied(request: &ToolCallRequest, evaluation: &PolicyEvaluation) -> Self {
        Self {
            code: ErrorCode::PolicyDenied,
            severity: ErrorSeverity::Warning,
            message: "The request was denied by policy.".to_string(),
            reason: "A verified policy rule matched the request and returned a deny decision."
                .to_string(),
            next_action: OperatorAction(
                "Review the matched policy rule and request details before trying again."
                    .to_string(),
            ),
            location: ErrorLocation::PolicyEvaluation,
            request_id: Some(request.request_id.clone()),
            execution_id: request.execution_id.clone(),
            policy_bundle_id: bundle_id(evaluation.policy_bundle_id.as_ref()),
            tool_name: Some(request.tool.name.clone()),
            wrapper_name: None,
            source_error_kind: evaluation.decision_reason.clone(),
        }
    }

    fn policy_evaluation_failed(request: &ToolCallRequest, evaluation: &PolicyEvaluation) -> Self {
        let failure = evaluation.failure_reason.as_ref();

        Self {
            code: ErrorCode::PolicyEvaluationFailed,
            severity: ErrorSeverity::Error,
            message: "The request was denied because policy evaluation failed.".to_string(),
            reason: policy_evaluation_reason(failure),
            next_action: OperatorAction(
                "Fix the policy bundle or request so exactly one supported rule can be evaluated."
                    .to_string(),
            ),
            location: ErrorLocation::PolicyEvaluation,
            request_id: Some(request.request_id.clone()),
            execution_id: request.execution_id.clone(),
            policy_bundle_id: bundle_id(evaluation.policy_bundle_id.as_ref()),
            tool_name: Some(request.tool.name.clone()),
            wrapper_name: None,
            source_error_kind: failure.map(|failure| format!("{failure:?}")),
        }
    }
}

impl PolicyEvaluation {
    fn decision_is_deny(&self) -> bool {
        self.decision == Some(crate::gateway::ResponseDecision::Deny)
    }
}

fn bundle_id(bundle: Option<&PolicyBundleRef>) -> Option<NonEmptyString> {
    bundle.map(|bundle| bundle.0.clone())
}

fn bundle_failure_reason(bundle: &PolicyBundleVerification) -> String {
    bundle
        .failure_reason
        .clone()
        .unwrap_or_else(|| "The bundle did not pass verification.".to_string())
}

fn policy_evaluation_reason(failure: Option<&PolicyEvaluationFailure>) -> String {
    match failure {
        Some(PolicyEvaluationFailure::BundleNotVerified) => {
            "The policy bundle was not verified, so no policy decision was trusted.".to_string()
        }
        Some(PolicyEvaluationFailure::GatewayPolicyMalformed) => {
            "The gateway policy file could not be parsed.".to_string()
        }
        Some(PolicyEvaluationFailure::RiskMatrixMalformed) => {
            "The risk matrix file could not be parsed.".to_string()
        }
        Some(PolicyEvaluationFailure::NoMatchingPolicyRule) => {
            "No policy rule matched the request.".to_string()
        }
        Some(PolicyEvaluationFailure::AmbiguousPolicyRules) => {
            "More than one policy rule matched the request.".to_string()
        }
        Some(PolicyEvaluationFailure::MissingRiskMatrixEntry) => {
            "The matched policy rule references a missing risk matrix entry.".to_string()
        }
        Some(PolicyEvaluationFailure::UnsupportedCapabilityClass) => {
            "The policy uses a capability class that this gateway does not support.".to_string()
        }
        Some(PolicyEvaluationFailure::UnsupportedDecisionValue) => {
            "The risk matrix uses a decision value that this gateway does not support.".to_string()
        }
        None => "Policy evaluation failed closed without a specific failure reason.".to_string(),
    }
}

fn wrapper_next_action(error: &WrapperDispatchError) -> OperatorAction {
    let action = match error {
        WrapperDispatchError::MissingWrapper { .. } => {
            "Register the required wrapper before dispatching this request."
        }
        WrapperDispatchError::IncompatibleWrapperVersion { .. } => {
            "Register the requested wrapper version or update the wrapper context."
        }
        WrapperDispatchError::AuthorizationFailed(_) => {
            "Fix the execution authorization binding before dispatching the wrapper."
        }
        WrapperDispatchError::ExecutionFailed(_) => {
            "Inspect the wrapper result and fix the wrapper before retrying."
        }
    };

    OperatorAction(action.to_string())
}

fn auth_error_code(error: &AuthorizationError) -> ErrorCode {
    match error {
        AuthorizationError::AuthorizationMissing => ErrorCode::AuthorizationMissing,
        AuthorizationError::WrapperMismatch { .. } => ErrorCode::AuthorizationWrapperMismatch,
        AuthorizationError::WrapperVersionMismatch { .. } => {
            ErrorCode::AuthorizationVersionMismatch
        }
        AuthorizationError::CapabilityMismatch { .. } => ErrorCode::AuthorizationCapabilityMismatch,
        AuthorizationError::ScopeInvalid { .. } => ErrorCode::AuthorizationScopeInvalid,
        AuthorizationError::AuthorizationDenied => ErrorCode::AuthorizationDenied,
    }
}

fn auth_next_action(error: &AuthorizationError) -> OperatorAction {
    let action = match error {
        AuthorizationError::AuthorizationMissing => {
            "Create explicit execution authorization before dispatching a wrapper."
        }
        AuthorizationError::WrapperMismatch { .. } => {
            "Use the wrapper named in the authorization or create a matching authorization."
        }
        AuthorizationError::WrapperVersionMismatch { .. } => {
            "Use the authorized wrapper version or update the wrapper registration."
        }
        AuthorizationError::CapabilityMismatch { .. } => {
            "Re-evaluate the request and create authorization for the same capability class."
        }
        AuthorizationError::ScopeInvalid { .. } => {
            "Use a narrow supported execution scope for the requested wrapper."
        }
        AuthorizationError::AuthorizationDenied => {
            "Do not dispatch the wrapper until authorization is granted."
        }
    };

    OperatorAction(action.to_string())
}

fn audit_error_kind(error: &AuditWriteError) -> &'static str {
    match error {
        AuditWriteError::Open { .. } => "open",
        AuditWriteError::Serialize { .. } => "serialize",
        AuditWriteError::Write { .. } => "write",
        AuditWriteError::Flush { .. } => "flush",
    }
}
