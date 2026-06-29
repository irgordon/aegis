use serde::{Deserialize, Serialize};

use crate::gateway::{
    CapabilityClass, NonEmptyString, ToolCallRequest, ToolCallResponse, WrapperExecutionContext,
};

use super::CredentialClass;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionAuthority {
    PolicyAllow,
    DevelopmentFixture,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionScope {
    LocalGatewayHealth,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationStatus {
    Authorized,
    Denied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationFailureReason {
    AuthorizationMissing,
    WrapperMismatch,
    WrapperVersionMismatch,
    CapabilityMismatch,
    ScopeInvalid,
    AuthorizationDenied,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationBinding {
    pub execution_id: NonEmptyString,
    pub wrapper_name: NonEmptyString,
    pub wrapper_version: NonEmptyString,
    pub tool_name: NonEmptyString,
    pub capability_class: CapabilityClass,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionAuthorization {
    pub authorization_id: NonEmptyString,
    pub binding: AuthorizationBinding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_scope: Option<ExecutionScope>,
    pub authority_source: ExecutionAuthority,
    pub authorized_credential_class: CredentialClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_ref: Option<NonEmptyString>,
    pub authorization_status: AuthorizationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<AuthorizationFailureReason>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AuthorizationError {
    AuthorizationMissing,
    WrapperMismatch {
        authorized_wrapper: String,
        requested_wrapper: String,
    },
    WrapperVersionMismatch {
        authorized_version: String,
        requested_version: String,
    },
    CapabilityMismatch {
        authorized_capability: CapabilityClass,
        requested_capability: CapabilityClass,
    },
    ScopeInvalid {
        scope: String,
    },
    AuthorizationDenied,
}

impl ExecutionAuthorization {
    pub fn policy_allow(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        context: &WrapperExecutionContext,
    ) -> Result<Self, AuthorizationError> {
        let capability = request_capability(request)?;
        verify_wrapper_matches_tool(request, context)?;
        verify_expected_wrapper_version(context)?;
        let scope = execution_scope_for(context)?;

        Ok(Self {
            authorization_id: authorization_id_for(request),
            binding: AuthorizationBinding {
                execution_id: response.execution_id.clone(),
                wrapper_name: context.config.wrapper_name.clone(),
                wrapper_version: context.config.wrapper_version.clone(),
                tool_name: request.tool.name.clone(),
                capability_class: capability,
            },
            execution_scope: Some(scope),
            authority_source: ExecutionAuthority::PolicyAllow,
            authorized_credential_class: CredentialClass::None,
            expiration_ref: None,
            authorization_status: AuthorizationStatus::Authorized,
            failure_reason: None,
        })
    }

    pub fn denied(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        context: &WrapperExecutionContext,
        error: &AuthorizationError,
    ) -> Self {
        Self {
            authorization_id: authorization_id_for(request),
            binding: AuthorizationBinding {
                execution_id: response.execution_id.clone(),
                wrapper_name: context.config.wrapper_name.clone(),
                wrapper_version: context.config.wrapper_version.clone(),
                tool_name: request.tool.name.clone(),
                capability_class: request
                    .tool
                    .capability_class
                    .clone()
                    .unwrap_or(CapabilityClass::L0),
            },
            execution_scope: execution_scope_for(context).ok(),
            authority_source: ExecutionAuthority::PolicyAllow,
            authorized_credential_class: CredentialClass::None,
            expiration_ref: None,
            authorization_status: AuthorizationStatus::Denied,
            failure_reason: Some(error.failure_reason()),
        }
    }

    pub fn validate_for(
        &self,
        request: &ToolCallRequest,
        context: &WrapperExecutionContext,
    ) -> Result<(), AuthorizationError> {
        if self.authorization_status != AuthorizationStatus::Authorized {
            return Err(AuthorizationError::AuthorizationDenied);
        }
        if self.execution_scope.is_none() {
            return Err(AuthorizationError::ScopeInvalid {
                scope: "missing".to_string(),
            });
        }
        if self.binding.wrapper_name != context.config.wrapper_name {
            return Err(AuthorizationError::WrapperMismatch {
                authorized_wrapper: self.binding.wrapper_name.as_str().to_string(),
                requested_wrapper: context.config.wrapper_name.as_str().to_string(),
            });
        }
        if self.binding.wrapper_version != context.config.wrapper_version {
            return Err(AuthorizationError::WrapperVersionMismatch {
                authorized_version: self.binding.wrapper_version.as_str().to_string(),
                requested_version: context.config.wrapper_version.as_str().to_string(),
            });
        }
        if self.binding.tool_name != request.tool.name {
            return Err(AuthorizationError::WrapperMismatch {
                authorized_wrapper: self.binding.tool_name.as_str().to_string(),
                requested_wrapper: request.tool.name.as_str().to_string(),
            });
        }

        let requested_capability = request_capability(request)?;
        if self.binding.capability_class != requested_capability {
            return Err(AuthorizationError::CapabilityMismatch {
                authorized_capability: self.binding.capability_class.clone(),
                requested_capability,
            });
        }

        Ok(())
    }
}

impl AuthorizationError {
    pub fn reason_code(&self) -> &'static str {
        match self {
            Self::AuthorizationMissing => "authorization_missing",
            Self::WrapperMismatch { .. } => "authorization_wrapper_mismatch",
            Self::WrapperVersionMismatch { .. } => "authorization_version_mismatch",
            Self::CapabilityMismatch { .. } => "authorization_capability_mismatch",
            Self::ScopeInvalid { .. } => "authorization_scope_invalid",
            Self::AuthorizationDenied => "authorization_denied",
        }
    }

    pub fn safe_message(&self) -> String {
        match self {
            Self::AuthorizationMissing => {
                "Wrapper execution requires explicit execution authorization.".to_string()
            }
            Self::WrapperMismatch {
                authorized_wrapper,
                requested_wrapper,
            } => format!(
                "Execution authorization is for {authorized_wrapper}, but dispatch requested {requested_wrapper}."
            ),
            Self::WrapperVersionMismatch {
                authorized_version,
                requested_version,
            } => format!(
                "Execution authorization is for wrapper version {authorized_version}, but dispatch requested {requested_version}."
            ),
            Self::CapabilityMismatch { .. } => {
                "Execution authorization does not match the request capability class.".to_string()
            }
            Self::ScopeInvalid { scope } => {
                format!("Execution scope is not permitted for local wrapper execution: {scope}.")
            }
            Self::AuthorizationDenied => {
                "Execution authorization did not permit wrapper execution.".to_string()
            }
        }
    }

    pub fn failure_reason(&self) -> AuthorizationFailureReason {
        match self {
            Self::AuthorizationMissing => AuthorizationFailureReason::AuthorizationMissing,
            Self::WrapperMismatch { .. } => AuthorizationFailureReason::WrapperMismatch,
            Self::WrapperVersionMismatch { .. } => {
                AuthorizationFailureReason::WrapperVersionMismatch
            }
            Self::CapabilityMismatch { .. } => AuthorizationFailureReason::CapabilityMismatch,
            Self::ScopeInvalid { .. } => AuthorizationFailureReason::ScopeInvalid,
            Self::AuthorizationDenied => AuthorizationFailureReason::AuthorizationDenied,
        }
    }
}

fn request_capability(request: &ToolCallRequest) -> Result<CapabilityClass, AuthorizationError> {
    request
        .tool
        .capability_class
        .clone()
        .ok_or(AuthorizationError::CapabilityMismatch {
            authorized_capability: CapabilityClass::L0,
            requested_capability: CapabilityClass::L0,
        })
}

fn execution_scope_for(
    context: &WrapperExecutionContext,
) -> Result<ExecutionScope, AuthorizationError> {
    match context.config.wrapper_name.as_str() {
        "health.check" => Ok(ExecutionScope::LocalGatewayHealth),
        scope => Err(AuthorizationError::ScopeInvalid {
            scope: scope.to_string(),
        }),
    }
}

fn verify_wrapper_matches_tool(
    request: &ToolCallRequest,
    context: &WrapperExecutionContext,
) -> Result<(), AuthorizationError> {
    if context.config.wrapper_name == request.tool.name {
        return Ok(());
    }

    Err(AuthorizationError::WrapperMismatch {
        authorized_wrapper: request.tool.name.as_str().to_string(),
        requested_wrapper: context.config.wrapper_name.as_str().to_string(),
    })
}

fn verify_expected_wrapper_version(
    context: &WrapperExecutionContext,
) -> Result<(), AuthorizationError> {
    let expected_version = expected_version_for(context.config.wrapper_name.as_str())?;
    if context.config.wrapper_version.as_str() == expected_version {
        return Ok(());
    }

    Err(AuthorizationError::WrapperVersionMismatch {
        authorized_version: expected_version.to_string(),
        requested_version: context.config.wrapper_version.as_str().to_string(),
    })
}

fn expected_version_for(wrapper_name: &str) -> Result<&'static str, AuthorizationError> {
    match wrapper_name {
        "health.check" => Ok("1.0.0"),
        name => Err(AuthorizationError::ScopeInvalid {
            scope: name.to_string(),
        }),
    }
}

fn authorization_id_for(request: &ToolCallRequest) -> NonEmptyString {
    NonEmptyString::new(format!("auth_{}", request.request_id.as_str()))
        .expect("request_id is already non-empty")
}
