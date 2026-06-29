use serde::{Deserialize, Serialize};

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
