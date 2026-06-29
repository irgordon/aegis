use serde::{Deserialize, Serialize};

use crate::gateway::{NonEmptyString, WrapperExecutionContext};

use super::ExecutionAuthorization;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialClass {
    None,
    LocalRuntime,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialRequirementStatus {
    NotRequired,
    Required,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialRequirement {
    pub requires_credentials: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_class: Option<CredentialClass>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialBoundaryStatus {
    Satisfied,
    Denied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialBoundaryFailureReason {
    CredentialClassMissing,
    CredentialClassMismatch,
    CredentialBoundaryDenied,
    CredentialsRequiredWithoutAuthorization,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialBoundary {
    pub credential_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_class: Option<CredentialClass>,
    pub authorized_credential_class: CredentialClass,
    pub credential_boundary_status: CredentialBoundaryStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<CredentialBoundaryFailureReason>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CredentialBoundaryError {
    CredentialClassMissing,
    CredentialClassMismatch,
    CredentialBoundaryDenied,
    CredentialsRequiredWithoutAuthorization,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialSource {
    LocalDevelopment,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialInjectionStatus {
    Injected,
    Denied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialInjectionFailureReason {
    CredentialHandleMissing,
    CredentialClassUnsupported,
    CredentialHandleWrapperMismatch,
    CredentialHandleAuthorizationMismatch,
    CredentialInjectionDenied,
    CredentialInjectionUnavailable,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialInjectionResult {
    pub credential_required: bool,
    pub credential_class: CredentialClass,
    pub credential_source: CredentialSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_handle_ref: Option<NonEmptyString>,
    pub wrapper_name: NonEmptyString,
    pub wrapper_version: NonEmptyString,
    pub authorization_id: NonEmptyString,
    pub credential_injection_status: CredentialInjectionStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<CredentialInjectionFailureReason>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CredentialInjectionError {
    CredentialHandleMissing,
    CredentialClassUnsupported,
    CredentialHandleWrapperMismatch,
    CredentialHandleAuthorizationMismatch,
    CredentialInjectionDenied,
    CredentialInjectionUnavailable,
}

impl CredentialRequirement {
    pub fn none() -> Self {
        Self {
            requires_credentials: false,
            credential_class: Some(CredentialClass::None),
        }
    }

    pub fn local_runtime() -> Self {
        Self {
            requires_credentials: true,
            credential_class: Some(CredentialClass::LocalRuntime),
        }
    }

    pub fn status(&self) -> CredentialRequirementStatus {
        if self.requires_credentials {
            CredentialRequirementStatus::Required
        } else {
            CredentialRequirementStatus::NotRequired
        }
    }
}

impl CredentialBoundary {
    pub fn evaluate(
        requirement: &CredentialRequirement,
        authorization: &ExecutionAuthorization,
    ) -> Self {
        match credential_boundary_failure(requirement, authorization) {
            Some(failure_reason) => Self::denied(requirement, authorization, failure_reason),
            None => Self::satisfied(requirement, authorization),
        }
    }

    pub fn validate(&self) -> Result<(), CredentialBoundaryError> {
        if self.credential_boundary_status == CredentialBoundaryStatus::Satisfied {
            return Ok(());
        }

        Err(match self.failure_reason {
            Some(CredentialBoundaryFailureReason::CredentialClassMissing) => {
                CredentialBoundaryError::CredentialClassMissing
            }
            Some(CredentialBoundaryFailureReason::CredentialClassMismatch) => {
                CredentialBoundaryError::CredentialClassMismatch
            }
            Some(CredentialBoundaryFailureReason::CredentialsRequiredWithoutAuthorization) => {
                CredentialBoundaryError::CredentialsRequiredWithoutAuthorization
            }
            Some(CredentialBoundaryFailureReason::CredentialBoundaryDenied) | None => {
                CredentialBoundaryError::CredentialBoundaryDenied
            }
        })
    }

    fn satisfied(
        requirement: &CredentialRequirement,
        authorization: &ExecutionAuthorization,
    ) -> Self {
        Self {
            credential_required: requirement.requires_credentials,
            credential_class: requirement.credential_class.clone(),
            authorized_credential_class: authorization.authorized_credential_class.clone(),
            credential_boundary_status: CredentialBoundaryStatus::Satisfied,
            failure_reason: None,
        }
    }

    fn denied(
        requirement: &CredentialRequirement,
        authorization: &ExecutionAuthorization,
        failure_reason: CredentialBoundaryFailureReason,
    ) -> Self {
        Self {
            credential_required: requirement.requires_credentials,
            credential_class: requirement.credential_class.clone(),
            authorized_credential_class: authorization.authorized_credential_class.clone(),
            credential_boundary_status: CredentialBoundaryStatus::Denied,
            failure_reason: Some(failure_reason),
        }
    }
}

impl CredentialBoundaryError {
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::CredentialClassMissing => "credential_class_missing",
            Self::CredentialClassMismatch => "credential_class_mismatch",
            Self::CredentialBoundaryDenied => "credential_boundary_denied",
            Self::CredentialsRequiredWithoutAuthorization => {
                "credentials_required_without_authorization"
            }
        }
    }

    pub fn safe_message(&self) -> String {
        match self {
            Self::CredentialClassMissing => {
                "The wrapper did not declare a credential class.".to_string()
            }
            Self::CredentialClassMismatch => {
                "The wrapper credential class does not match the execution authorization."
                    .to_string()
            }
            Self::CredentialBoundaryDenied => {
                "The credential boundary did not permit wrapper execution.".to_string()
            }
            Self::CredentialsRequiredWithoutAuthorization => {
                "The wrapper requires credentials, but authorization did not permit that credential class."
                    .to_string()
            }
        }
    }
}

impl CredentialInjectionResult {
    pub fn inject_local_development(
        boundary: &CredentialBoundary,
        authorization: &ExecutionAuthorization,
    ) -> Result<Option<Self>, CredentialInjectionError> {
        if !boundary.credential_required {
            return Ok(None);
        }

        if boundary.credential_boundary_status != CredentialBoundaryStatus::Satisfied {
            return Err(CredentialInjectionError::CredentialInjectionDenied);
        }

        let Some(credential_class) = boundary.credential_class.clone() else {
            return Err(CredentialInjectionError::CredentialClassUnsupported);
        };

        if credential_class != CredentialClass::LocalRuntime {
            return Err(CredentialInjectionError::CredentialClassUnsupported);
        }

        Ok(Some(Self {
            credential_required: true,
            credential_class,
            credential_source: CredentialSource::LocalDevelopment,
            credential_handle_ref: Some(local_development_handle_ref(authorization)),
            wrapper_name: authorization.binding.wrapper_name.clone(),
            wrapper_version: authorization.binding.wrapper_version.clone(),
            authorization_id: authorization.authorization_id.clone(),
            credential_injection_status: CredentialInjectionStatus::Injected,
            failure_reason: None,
        }))
    }

    pub fn validate_for(
        credential_injection: Option<&Self>,
        required_class: &CredentialClass,
        context: &WrapperExecutionContext,
        authorization: &ExecutionAuthorization,
    ) -> Result<(), CredentialInjectionError> {
        let Some(injection) = credential_injection else {
            return Err(CredentialInjectionError::CredentialHandleMissing);
        };

        if injection.credential_handle_ref.is_none() {
            return Err(CredentialInjectionError::CredentialHandleMissing);
        }

        if injection.credential_injection_status != CredentialInjectionStatus::Injected {
            return Err(CredentialInjectionError::CredentialInjectionDenied);
        }

        if &injection.credential_class != required_class {
            return Err(CredentialInjectionError::CredentialClassUnsupported);
        }

        if injection.wrapper_name != context.config.wrapper_name
            || injection.wrapper_version != context.config.wrapper_version
        {
            return Err(CredentialInjectionError::CredentialHandleWrapperMismatch);
        }

        if injection.authorization_id != authorization.authorization_id {
            return Err(CredentialInjectionError::CredentialHandleAuthorizationMismatch);
        }

        Ok(())
    }
}

impl CredentialInjectionError {
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::CredentialHandleMissing => "credential_handle_missing",
            Self::CredentialClassUnsupported => "credential_class_unsupported",
            Self::CredentialHandleWrapperMismatch => "credential_handle_wrapper_mismatch",
            Self::CredentialHandleAuthorizationMismatch => {
                "credential_handle_authorization_mismatch"
            }
            Self::CredentialInjectionDenied => "credential_injection_denied",
            Self::CredentialInjectionUnavailable => "credential_injection_unavailable",
        }
    }

    pub fn safe_message(&self) -> String {
        match self {
            Self::CredentialHandleMissing => {
                "The wrapper requires a local credential handle, but none was supplied.".to_string()
            }
            Self::CredentialClassUnsupported => {
                "The credential handle class is not supported for this wrapper.".to_string()
            }
            Self::CredentialHandleWrapperMismatch => {
                "The credential handle is bound to a different wrapper.".to_string()
            }
            Self::CredentialHandleAuthorizationMismatch => {
                "The credential handle is bound to a different execution authorization.".to_string()
            }
            Self::CredentialInjectionDenied => {
                "The credential injection boundary did not permit a handle.".to_string()
            }
            Self::CredentialInjectionUnavailable => {
                "The local credential injection boundary is unavailable.".to_string()
            }
        }
    }
}

fn credential_boundary_failure(
    requirement: &CredentialRequirement,
    authorization: &ExecutionAuthorization,
) -> Option<CredentialBoundaryFailureReason> {
    let Some(required_class) = requirement.credential_class.as_ref() else {
        return Some(CredentialBoundaryFailureReason::CredentialClassMissing);
    };

    if !requirement.requires_credentials {
        return none_requirement_failure(required_class, authorization);
    }

    required_credential_failure(required_class, authorization)
}

fn none_requirement_failure(
    required_class: &CredentialClass,
    authorization: &ExecutionAuthorization,
) -> Option<CredentialBoundaryFailureReason> {
    if required_class != &CredentialClass::None {
        return Some(CredentialBoundaryFailureReason::CredentialClassMismatch);
    }

    if authorization.authorized_credential_class == CredentialClass::None {
        return None;
    }

    None
}

fn required_credential_failure(
    required_class: &CredentialClass,
    authorization: &ExecutionAuthorization,
) -> Option<CredentialBoundaryFailureReason> {
    if authorization.authorized_credential_class == CredentialClass::None {
        return Some(CredentialBoundaryFailureReason::CredentialsRequiredWithoutAuthorization);
    }

    if required_class == &authorization.authorized_credential_class {
        return None;
    }

    Some(CredentialBoundaryFailureReason::CredentialClassMismatch)
}

fn local_development_handle_ref(authorization: &ExecutionAuthorization) -> NonEmptyString {
    let wrapper = authorization
        .binding
        .wrapper_name
        .as_str()
        .replace('.', "_");
    NonEmptyString::new(format!(
        "local-development-handle:{wrapper}:{}",
        authorization.authorization_id.as_str()
    ))
    .expect("local development handle reference is non-empty")
}
