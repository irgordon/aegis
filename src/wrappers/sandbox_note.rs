use std::{
    collections::BTreeMap,
    fs, io,
    path::{Component, Path, PathBuf},
};

use serde_json::Value;

use crate::{
    auth::{
        CredentialClass, CredentialInjectionError, CredentialInjectionResult,
        CredentialRequirement, ExecutionAuthorization,
    },
    gateway::{
        ToolCallRequest, WrapperExecutionContext, WrapperExecutionError, WrapperExecutionOutput,
        WrapperExecutor,
    },
};

pub struct SandboxNoteWriteWrapper;

impl WrapperExecutor for SandboxNoteWriteWrapper {
    fn wrapper_name(&self) -> &str {
        "sandbox.note.write"
    }

    fn wrapper_version(&self) -> &str {
        "1.0.0"
    }

    fn credential_requirement(&self) -> CredentialRequirement {
        CredentialRequirement::local_runtime()
    }

    fn execute(
        &self,
        request: &ToolCallRequest,
        context: &WrapperExecutionContext,
        authorization: &ExecutionAuthorization,
        credential_injection: Option<&CredentialInjectionResult>,
    ) -> Result<WrapperExecutionOutput, WrapperExecutionError> {
        let command = SandboxNoteCommand::from_request(
            request,
            context,
            authorization,
            credential_injection,
        )?;
        let result = command.write()?;

        Ok(WrapperExecutionOutput {
            result: Some(result.into_output()),
        })
    }
}

struct SandboxNoteCommand {
    note_id: SandboxNoteId,
    content: SandboxNoteContent,
    root: SandboxRoot,
}

struct SandboxNoteId(String);
struct SandboxNoteContent(String);
struct SandboxRoot(PathBuf);

struct SandboxWriteResult {
    sandbox_root_reference: String,
    sandbox_relative_path: String,
    mutation_status: &'static str,
    wrapper: &'static str,
}

impl SandboxNoteCommand {
    fn from_request(
        request: &ToolCallRequest,
        context: &WrapperExecutionContext,
        authorization: &ExecutionAuthorization,
        credential_injection: Option<&CredentialInjectionResult>,
    ) -> Result<Self, WrapperExecutionError> {
        ensure_local_runtime_credential_handle(context, authorization, credential_injection)?;
        ensure_idempotency_key(request)?;

        Ok(Self {
            note_id: SandboxNoteId::from_request(request)?,
            content: SandboxNoteContent::from_request(request)?,
            root: SandboxRoot::from_context(context)?,
        })
    }

    fn write(&self) -> Result<SandboxWriteResult, WrapperExecutionError> {
        let relative_path = self.note_id.relative_note_path();
        let note_path = self.root.note_path(&relative_path)?;

        create_notes_directory(&note_path)?;
        write_note_file(&note_path, self.content.as_str())?;

        Ok(SandboxWriteResult {
            sandbox_root_reference: self.root.reference(),
            sandbox_relative_path: relative_path,
            mutation_status: "written",
            wrapper: "sandbox.note.write",
        })
    }
}

fn ensure_local_runtime_credential_handle(
    context: &WrapperExecutionContext,
    authorization: &ExecutionAuthorization,
    credential_injection: Option<&CredentialInjectionResult>,
) -> Result<(), WrapperExecutionError> {
    CredentialInjectionResult::validate_for(
        credential_injection,
        &CredentialClass::LocalRuntime,
        context,
        authorization,
    )
    .map_err(credential_injection_error)
}

impl SandboxNoteId {
    fn from_request(request: &ToolCallRequest) -> Result<Self, WrapperExecutionError> {
        let value = required_string_param(request, "note_id", SandboxPathError::UnsafeNoteId)?;

        if !is_safe_note_id(value) {
            return Err(SandboxPathError::UnsafeNoteId.into_error());
        }

        Ok(Self(value.to_string()))
    }

    fn relative_note_path(&self) -> String {
        format!("notes/{}.txt", self.0)
    }
}

impl SandboxNoteContent {
    fn from_request(request: &ToolCallRequest) -> Result<Self, WrapperExecutionError> {
        let value = required_string_param(request, "content", SandboxPathError::EmptyContent)?;

        if value.trim().is_empty() {
            return Err(SandboxPathError::EmptyContent.into_error());
        }

        Ok(Self(value.to_string()))
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl SandboxRoot {
    fn from_context(context: &WrapperExecutionContext) -> Result<Self, WrapperExecutionError> {
        let Some(root) = context.sandbox_root.as_ref() else {
            return Err(SandboxPathError::MissingSandboxDirectory.into_error());
        };

        Self::from_path(Path::new(root.as_str()))
    }

    fn from_path(path: &Path) -> Result<Self, WrapperExecutionError> {
        if is_broad_path(path) {
            return Err(SandboxPathError::InvalidSandboxPath.into_error());
        }

        let root = fs::canonicalize(path)
            .map_err(|_| SandboxPathError::InvalidSandboxPath.into_error())?;

        if !root.is_dir() || is_broad_path(&root) {
            return Err(SandboxPathError::InvalidSandboxPath.into_error());
        }

        Ok(Self(root))
    }

    fn note_path(&self, relative_path: &str) -> Result<PathBuf, WrapperExecutionError> {
        let candidate = self.0.join(relative_path);

        if !candidate.starts_with(&self.0) {
            return Err(SandboxPathError::PathTraversal.into_error());
        }

        Ok(candidate)
    }

    fn reference(&self) -> String {
        format!("local-development:{}", self.0.display())
    }
}

impl SandboxWriteResult {
    fn into_output(self) -> BTreeMap<String, Value> {
        BTreeMap::from([
            (
                "wrapper".to_string(),
                Value::String(self.wrapper.to_string()),
            ),
            (
                "mutation_status".to_string(),
                Value::String(self.mutation_status.to_string()),
            ),
            (
                "sandbox_root_reference".to_string(),
                Value::String(self.sandbox_root_reference),
            ),
            (
                "sandbox_relative_path".to_string(),
                Value::String(self.sandbox_relative_path),
            ),
        ])
    }
}

enum SandboxPathError {
    MissingSandboxDirectory,
    InvalidSandboxPath,
    PathTraversal,
    MissingIdempotencyContext,
    UnsafeNoteId,
    EmptyContent,
    WriteFailed,
}

impl SandboxPathError {
    fn reason_code(&self) -> &'static str {
        match self {
            Self::MissingSandboxDirectory => "sandbox_directory_missing",
            Self::InvalidSandboxPath => "sandbox_path_invalid",
            Self::PathTraversal => "sandbox_path_traversal",
            Self::MissingIdempotencyContext => "idempotency_context_missing",
            Self::UnsafeNoteId => "sandbox_note_id_unsafe",
            Self::EmptyContent => "sandbox_note_content_empty",
            Self::WriteFailed => "sandbox_write_failed",
        }
    }

    fn safe_message(&self) -> &'static str {
        match self {
            Self::MissingSandboxDirectory => {
                "A sandbox directory is required before this wrapper can write."
            }
            Self::InvalidSandboxPath => "The sandbox directory is missing, broad, or invalid.",
            Self::PathTraversal => "The note path would leave the sandbox directory.",
            Self::MissingIdempotencyContext => {
                "A mutation request must include an idempotency key."
            }
            Self::UnsafeNoteId => "The note ID is empty or contains unsafe path characters.",
            Self::EmptyContent => "The note content is empty.",
            Self::WriteFailed => "The sandbox note could not be written.",
        }
    }

    fn into_error(self) -> WrapperExecutionError {
        WrapperExecutionError {
            reason_code: Some(self.reason_code().to_string()),
            safe_message: self.safe_message().to_string(),
        }
    }
}

fn ensure_idempotency_key(request: &ToolCallRequest) -> Result<(), WrapperExecutionError> {
    if request.idempotency_key.is_some() {
        return Ok(());
    }

    Err(SandboxPathError::MissingIdempotencyContext.into_error())
}

fn required_string_param<'a>(
    request: &'a ToolCallRequest,
    name: &str,
    error: SandboxPathError,
) -> Result<&'a str, WrapperExecutionError> {
    request
        .params
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| error.into_error())
}

fn is_safe_note_id(value: &str) -> bool {
    !value.is_empty()
        && !Path::new(value).is_absolute()
        && Path::new(value)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
}

fn is_broad_path(path: &Path) -> bool {
    path.as_os_str().is_empty() || path.parent().is_none()
}

fn create_notes_directory(note_path: &Path) -> Result<(), WrapperExecutionError> {
    let Some(parent) = note_path.parent() else {
        return Err(SandboxPathError::PathTraversal.into_error());
    };

    fs::create_dir_all(parent).map_err(|_| SandboxPathError::WriteFailed.into_error())
}

fn write_note_file(note_path: &Path, content: &str) -> Result<(), WrapperExecutionError> {
    match fs::read_to_string(note_path) {
        Ok(existing) if existing == content => Ok(()),
        Ok(_) => Err(SandboxPathError::WriteFailed.into_error()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::write(note_path, content).map_err(|_| SandboxPathError::WriteFailed.into_error())
        }
        Err(_) => Err(SandboxPathError::WriteFailed.into_error()),
    }
}

fn credential_injection_error(error: CredentialInjectionError) -> WrapperExecutionError {
    WrapperExecutionError {
        reason_code: Some(error.reason_code().to_string()),
        safe_message: error.safe_message(),
    }
}
