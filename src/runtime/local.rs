use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{
    audit::{AuditRecord, AuditRecordBuilder, AuditRecordMetadata, GatewayAuditContexts},
    auth::{AuthorizationError, CredentialBoundary, ExecutionAuthorization},
    error::{AuditErrorReport, GatewayErrorReport},
    gateway::{
        Gateway, GatewayStatus, GatewayValidationOutcome, IdempotencyContext, IdempotencyKey,
        NonEmptyString, OperationType, ResponseDecision, ResponseMetadata, SupportedTools,
        ToolCallRequest, ToolCallResponse, WrapperDispatcher, WrapperExecutionContext,
        WrapperExecutionEvidence, WrapperExecutor,
    },
    policy::{
        evaluate_local_policy_bundle, load_policy_bundle, PolicyBundleVerification, PolicyDecision,
        PolicyDenial, PolicyEvaluation,
    },
    state::{ExecutionLifecycle, ExecutionState},
    wrappers::{HealthCheckWrapper, SandboxNoteWriteWrapper},
};

static HEALTH_CHECK_WRAPPER: HealthCheckWrapper = HealthCheckWrapper;
static SANDBOX_NOTE_WRITE_WRAPPER: SandboxNoteWriteWrapper = SandboxNoteWriteWrapper;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocalRuntimeOutput {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
    pub policy_bundle: PolicyBundleVerification,
    pub policy_evaluation: Option<PolicyEvaluation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_authorization: Option<ExecutionAuthorization>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_boundary: Option<CredentialBoundary>,
    pub execution_lifecycle: ExecutionLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_execution: Option<WrapperExecutionEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_report: Option<GatewayErrorReport>,
}

pub fn process_local_gateway_request(input: &str, bundle_path: &Path) -> LocalRuntimeOutput {
    process_local_gateway_request_with_context(input, bundle_path, LocalRuntimeContext::default())
}

pub fn process_local_gateway_request_with_context(
    input: &str,
    bundle_path: &Path,
    context: LocalRuntimeContext,
) -> LocalRuntimeOutput {
    process_local_gateway_request_with_wrapper_registry_and_context(
        input,
        bundle_path,
        &local_wrapper_executors(),
        context,
    )
}

pub fn process_local_gateway_request_with_wrapper_registry(
    input: &str,
    bundle_path: &Path,
    wrapper_executors: &[&dyn WrapperExecutor],
    wrapper_context: Option<WrapperExecutionContext>,
) -> LocalRuntimeOutput {
    process_local_gateway_request_with_wrapper_registry_and_context(
        input,
        bundle_path,
        wrapper_executors,
        LocalRuntimeContext {
            wrapper_context,
            sandbox_dir: None,
        },
    )
}

pub fn process_local_gateway_request_with_wrapper_registry_and_context(
    input: &str,
    bundle_path: &Path,
    wrapper_executors: &[&dyn WrapperExecutor],
    context: LocalRuntimeContext,
) -> LocalRuntimeOutput {
    let policy_bundle = verified_or_rejected_bundle(bundle_path);
    let response_metadata = local_response_metadata(&policy_bundle);
    let audit_metadata = local_audit_metadata();

    match Gateway::validate_request_json(
        input,
        &local_supported_tools(),
        response_metadata,
        audit_metadata,
    ) {
        GatewayValidationOutcome::Accepted(request) => {
            process_validated_request(*request, policy_bundle, wrapper_executors, context)
        }
        GatewayValidationOutcome::Denied(evidence) => {
            LocalRuntimeOutput::from_denial(*evidence, policy_bundle)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LocalRuntimeContext {
    pub wrapper_context: Option<WrapperExecutionContext>,
    pub sandbox_dir: Option<PathBuf>,
}

impl LocalRuntimeOutput {
    pub fn attach_error_report(&mut self, report: GatewayErrorReport) {
        self.audit_record.details.error_report = Some(report.audit_fields());
        self.error_report = Some(report);
    }

    pub fn mark_audited_completed(&mut self) {
        if self.execution_lifecycle.execution_state == ExecutionState::Executed
            && self.execution_lifecycle.audited_completed().is_ok()
        {
            self.sync_lifecycle_evidence();
        }
    }

    pub fn mark_audit_failed(&mut self) {
        if self.execution_lifecycle.execution_state == ExecutionState::Executed
            && self
                .execution_lifecycle
                .transition_to(ExecutionState::AuditFailed)
                .is_ok()
        {
            self.sync_lifecycle_evidence();
        }
    }

    fn sync_lifecycle_evidence(&mut self) {
        self.audit_record.details.execution_lifecycle = Some(self.execution_lifecycle.clone());
    }

    fn from_denial(
        evidence: crate::gateway::GatewayDecisionEvidence,
        policy_bundle: PolicyBundleVerification,
    ) -> Self {
        let error_report = GatewayErrorReport::from_validation_denial(&evidence.response);
        let mut audit_record = evidence.audit_record;
        let execution_lifecycle = validation_denial_lifecycle(&evidence.response);
        audit_record.details.error_report = Some(error_report.audit_fields());
        audit_record.details.execution_lifecycle = Some(execution_lifecycle.clone());

        Self {
            response: evidence.response,
            audit_record,
            policy_bundle,
            policy_evaluation: None,
            execution_authorization: None,
            credential_boundary: None,
            execution_lifecycle,
            wrapper_execution: None,
            error_report: Some(error_report),
        }
    }
}

struct RuntimeOutputParts {
    response: ToolCallResponse,
    policy_bundle: PolicyBundleVerification,
    policy_evaluation: PolicyEvaluation,
    idempotency_context: Option<IdempotencyContext>,
    execution_authorization: Option<ExecutionAuthorization>,
    credential_boundary: Option<CredentialBoundary>,
    execution_lifecycle: ExecutionLifecycle,
    wrapper_context: Option<WrapperExecutionContext>,
    wrapper_execution: Option<WrapperExecutionEvidence>,
    error_report: Option<GatewayErrorReport>,
}

struct AllowedDispatchParts {
    request: ToolCallRequest,
    response: ToolCallResponse,
    policy_bundle: PolicyBundleVerification,
    policy_evaluation: PolicyEvaluation,
    idempotency_context: Option<IdempotencyContext>,
    lifecycle: ExecutionLifecycle,
}

fn process_validated_request(
    request: ToolCallRequest,
    policy_bundle: PolicyBundleVerification,
    wrapper_executors: &[&dyn WrapperExecutor],
    runtime_context: LocalRuntimeContext,
) -> LocalRuntimeOutput {
    let mut lifecycle = validated_lifecycle();

    if !policy_bundle.is_verified() {
        transition_or_panic(&mut lifecycle, ExecutionState::FailedClosed);
        return process_unverified_bundle_request(request, policy_bundle, lifecycle);
    }

    transition_or_panic(&mut lifecycle, ExecutionState::BundleVerified);
    let evaluation_result = evaluate_local_policy_bundle(&request, &policy_bundle);
    let policy_evaluation = evaluation_result.evaluation;
    transition_or_panic(&mut lifecycle, ExecutionState::PolicyEvaluated);
    let error_report = gateway_error_report(&request, &policy_bundle, &policy_evaluation);
    let idempotency_context = local_idempotency_context_for_request(&request, &policy_bundle);
    let wrapper_context = runtime_context.wrapper_context.unwrap_or_else(|| {
        local_wrapper_context_for_request(&request, runtime_context.sandbox_dir.as_deref())
    });
    let response = Gateway::map_policy_decision(
        &request,
        evaluation_result.decision,
        local_response_metadata(&policy_bundle),
    );

    if should_dispatch_wrapper(&response) {
        return dispatch_allowed_request(
            AllowedDispatchParts {
                request,
                response,
                policy_bundle,
                policy_evaluation,
                idempotency_context,
                lifecycle,
            },
            wrapper_executors,
            wrapper_context,
        );
    }

    if response.status != GatewayStatus::Allowed {
        transition_or_panic(&mut lifecycle, ExecutionState::FailedClosed);
    }

    build_runtime_output(
        &request,
        RuntimeOutputParts {
            response,
            policy_bundle,
            policy_evaluation,
            idempotency_context,
            execution_authorization: None,
            credential_boundary: None,
            execution_lifecycle: lifecycle,
            wrapper_context: None,
            wrapper_execution: None,
            error_report,
        },
    )
}

fn process_unverified_bundle_request(
    request: ToolCallRequest,
    policy_bundle: PolicyBundleVerification,
    lifecycle: ExecutionLifecycle,
) -> LocalRuntimeOutput {
    let evaluation_result = evaluate_local_policy_bundle(&request, &policy_bundle);
    let policy_evaluation = evaluation_result.evaluation;
    let error_report = gateway_error_report(&request, &policy_bundle, &policy_evaluation);
    let response = Gateway::map_policy_decision(
        &request,
        evaluation_result.decision,
        local_response_metadata(&policy_bundle),
    );

    build_runtime_output(
        &request,
        RuntimeOutputParts {
            response,
            policy_bundle,
            policy_evaluation,
            idempotency_context: None,
            execution_authorization: None,
            credential_boundary: None,
            execution_lifecycle: lifecycle,
            wrapper_context: None,
            wrapper_execution: None,
            error_report,
        },
    )
}

fn dispatch_allowed_request(
    mut parts: AllowedDispatchParts,
    wrapper_executors: &[&dyn WrapperExecutor],
    wrapper_context: WrapperExecutionContext,
) -> LocalRuntimeOutput {
    let authorization = match ExecutionAuthorization::policy_allow(
        &parts.request,
        &parts.response,
        &wrapper_context,
    ) {
        Ok(authorization) => authorization,
        Err(error) => {
            let denied_authorization = ExecutionAuthorization::denied(
                &parts.request,
                &parts.response,
                &wrapper_context,
                &error,
            );
            return build_authorization_failure_output(
                parts,
                wrapper_context,
                denied_authorization,
                error,
            );
        }
    };

    transition_or_panic(&mut parts.lifecycle, ExecutionState::Authorized);
    transition_or_panic(&mut parts.lifecycle, ExecutionState::Dispatching);
    let dispatcher = WrapperDispatcher::new(wrapper_executors.iter().copied());

    match dispatcher.dispatch(&parts.request, &wrapper_context, &authorization) {
        Ok(result) => build_executed_output(parts, authorization, result),
        Err(error) => build_wrapper_failure_output(parts, wrapper_context, authorization, error),
    }
}

fn build_executed_output(
    mut parts: AllowedDispatchParts,
    authorization: ExecutionAuthorization,
    wrapper_result: crate::gateway::WrapperExecutionResult,
) -> LocalRuntimeOutput {
    parts.response.result = wrapper_result.result.clone();
    let wrapper_execution = WrapperExecutionEvidence::from(&wrapper_result);
    transition_or_panic(&mut parts.lifecycle, ExecutionState::Executed);
    let request = parts.request;

    build_runtime_output(
        &request,
        RuntimeOutputParts {
            response: parts.response,
            policy_bundle: parts.policy_bundle,
            policy_evaluation: parts.policy_evaluation,
            idempotency_context: parts.idempotency_context,
            execution_authorization: Some(authorization),
            credential_boundary: Some(wrapper_result.credential_boundary.clone()),
            execution_lifecycle: parts.lifecycle,
            wrapper_context: Some(wrapper_result.context.clone()),
            wrapper_execution: Some(wrapper_execution),
            error_report: None,
        },
    )
}

fn build_wrapper_failure_output(
    mut parts: AllowedDispatchParts,
    wrapper_context: WrapperExecutionContext,
    authorization: ExecutionAuthorization,
    error: crate::gateway::WrapperDispatchError,
) -> LocalRuntimeOutput {
    let response = wrapper_failure_response(&parts.request, &parts.policy_bundle, &error);
    let error_report = GatewayErrorReport::wrapper_dispatch_failed(&error, &wrapper_context);
    let credential_boundary = credential_boundary_from_dispatch_error(&error);
    transition_or_panic(&mut parts.lifecycle, ExecutionState::FailedClosed);
    let request = parts.request;

    build_runtime_output(
        &request,
        RuntimeOutputParts {
            response,
            policy_bundle: parts.policy_bundle,
            policy_evaluation: parts.policy_evaluation,
            idempotency_context: parts.idempotency_context,
            execution_authorization: Some(authorization),
            credential_boundary,
            execution_lifecycle: parts.lifecycle,
            wrapper_context: Some(wrapper_context),
            wrapper_execution: None,
            error_report: Some(error_report),
        },
    )
}

fn build_authorization_failure_output(
    mut parts: AllowedDispatchParts,
    wrapper_context: WrapperExecutionContext,
    authorization: ExecutionAuthorization,
    error: AuthorizationError,
) -> LocalRuntimeOutput {
    let response = authorization_failure_response(&parts.request, &parts.policy_bundle, &error);
    let error_report =
        GatewayErrorReport::execution_authorization_failed(&error, &authorization, &parts.request);
    transition_or_panic(&mut parts.lifecycle, ExecutionState::FailedClosed);
    let request = parts.request;

    build_runtime_output(
        &request,
        RuntimeOutputParts {
            response,
            policy_bundle: parts.policy_bundle,
            policy_evaluation: parts.policy_evaluation,
            idempotency_context: parts.idempotency_context,
            execution_authorization: Some(authorization),
            credential_boundary: None,
            execution_lifecycle: parts.lifecycle,
            wrapper_context: Some(wrapper_context),
            wrapper_execution: None,
            error_report: Some(error_report),
        },
    )
}

fn build_runtime_output(
    request: &ToolCallRequest,
    parts: RuntimeOutputParts,
) -> LocalRuntimeOutput {
    let audit_record = AuditRecordBuilder::build_gateway_decision_record_with_contexts(
        request,
        &parts.response,
        local_audit_metadata(),
        GatewayAuditContexts {
            policy_bundle_verification: Some(parts.policy_bundle.clone()),
            policy_evaluation: Some(parts.policy_evaluation.clone()),
            idempotency_context: parts.idempotency_context,
            execution_authorization: parts.execution_authorization.clone(),
            credential_boundary: parts.credential_boundary.clone(),
            execution_lifecycle: Some(parts.execution_lifecycle.clone()),
            wrapper_context: parts.wrapper_context,
            wrapper_execution_evidence: parts.wrapper_execution.clone(),
            error_report: audit_error_report(parts.error_report.as_ref()),
            ..GatewayAuditContexts::default()
        },
    );

    LocalRuntimeOutput {
        response: parts.response,
        audit_record,
        policy_bundle: parts.policy_bundle,
        policy_evaluation: Some(parts.policy_evaluation),
        execution_authorization: parts.execution_authorization,
        credential_boundary: parts.credential_boundary,
        execution_lifecycle: parts.execution_lifecycle,
        wrapper_execution: parts.wrapper_execution,
        error_report: parts.error_report,
    }
}

fn validation_denial_lifecycle(response: &ToolCallResponse) -> ExecutionLifecycle {
    let mut lifecycle = ExecutionLifecycle::created();

    if response.request_id.is_some() {
        transition_or_panic(&mut lifecycle, ExecutionState::Validated);
    }

    transition_or_panic(&mut lifecycle, ExecutionState::FailedClosed);
    lifecycle
}

fn validated_lifecycle() -> ExecutionLifecycle {
    let mut lifecycle = ExecutionLifecycle::created();
    transition_or_panic(&mut lifecycle, ExecutionState::Validated);
    lifecycle
}

fn transition_or_panic(lifecycle: &mut ExecutionLifecycle, state: ExecutionState) {
    lifecycle.transition_to(state).unwrap_or_else(|error| {
        panic!("local runtime lifecycle transition should be valid: {error:?}")
    });
}

fn should_dispatch_wrapper(response: &ToolCallResponse) -> bool {
    response.status == GatewayStatus::Allowed && response.decision == Some(ResponseDecision::Allow)
}

fn wrapper_failure_response(
    request: &ToolCallRequest,
    policy_bundle: &PolicyBundleVerification,
    error: &crate::gateway::WrapperDispatchError,
) -> ToolCallResponse {
    Gateway::map_policy_decision(
        request,
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some(error.reason_code().to_string()),
            safe_message: error.safe_message(),
        }),
        local_response_metadata(policy_bundle),
    )
}

fn credential_boundary_from_dispatch_error(
    error: &crate::gateway::WrapperDispatchError,
) -> Option<CredentialBoundary> {
    match error {
        crate::gateway::WrapperDispatchError::CredentialBoundaryFailed { boundary, .. } => {
            Some(boundary.clone())
        }
        _ => None,
    }
}

fn authorization_failure_response(
    request: &ToolCallRequest,
    policy_bundle: &PolicyBundleVerification,
    error: &AuthorizationError,
) -> ToolCallResponse {
    Gateway::map_policy_decision(
        request,
        PolicyDecision::Deny(PolicyDenial {
            reason_code: Some(error.reason_code().to_string()),
            safe_message: error.safe_message(),
        }),
        local_response_metadata(policy_bundle),
    )
}

fn gateway_error_report(
    request: &ToolCallRequest,
    policy_bundle: &PolicyBundleVerification,
    policy_evaluation: &PolicyEvaluation,
) -> Option<GatewayErrorReport> {
    if !policy_bundle.is_verified() {
        return Some(GatewayErrorReport::policy_bundle_verification_failed(
            policy_bundle,
        ));
    }

    GatewayErrorReport::policy_evaluation_report(request, policy_evaluation)
}

fn audit_error_report(report: Option<&GatewayErrorReport>) -> Option<AuditErrorReport> {
    report.map(GatewayErrorReport::audit_fields)
}

fn verified_or_rejected_bundle(bundle_path: &Path) -> PolicyBundleVerification {
    match load_policy_bundle(bundle_path) {
        Ok(verification) => verification,
        Err(verification) => *verification,
    }
}

fn local_supported_tools() -> SupportedTools {
    SupportedTools::from_names([
        "health.check",
        "sandbox.note.write",
        "metrics.read",
        "email.send",
        "deploy.prod",
        "storage.read",
    ])
}

fn local_wrapper_executors() -> [&'static dyn WrapperExecutor; 2] {
    [&HEALTH_CHECK_WRAPPER, &SANDBOX_NOTE_WRITE_WRAPPER]
}

fn local_health_check_context() -> WrapperExecutionContext {
    local_wrapper_context("health.check", "1.0.0", "local")
}

fn local_sandbox_note_context(sandbox_dir: Option<&Path>) -> WrapperExecutionContext {
    local_wrapper_context_with_sandbox("sandbox.note.write", "1.0.0", "local", sandbox_dir)
}

fn local_wrapper_context_for_request(
    request: &ToolCallRequest,
    sandbox_dir: Option<&Path>,
) -> WrapperExecutionContext {
    match request.tool_name() {
        "health.check" => local_health_check_context(),
        "sandbox.note.write" => local_sandbox_note_context(sandbox_dir),
        tool_name => local_wrapper_context(tool_name, "1.0.0", "local"),
    }
}

fn local_wrapper_context(
    wrapper_name: &str,
    wrapper_version: &str,
    target_system: &str,
) -> WrapperExecutionContext {
    local_wrapper_context_with_sandbox(wrapper_name, wrapper_version, target_system, None)
}

fn local_wrapper_context_with_sandbox(
    wrapper_name: &str,
    wrapper_version: &str,
    target_system: &str,
    sandbox_dir: Option<&Path>,
) -> WrapperExecutionContext {
    serde_json::from_value(serde_json::json!({
        "config": {
            "wrapper_name": wrapper_name,
            "wrapper_version": wrapper_version,
            "target_system": target_system,
            "config_reference": format!("builtins/{wrapper_name}"),
            "config_digest": format!("builtin:{wrapper_name}@{wrapper_version}")
        },
        "external_system_schema_version": "aegis-local-v1",
        "redaction_profile": "no-secrets",
        "execution_mode": "enforce",
        "credential_injection_required": false,
        "sandbox_root": sandbox_dir.map(|path| path.display().to_string())
    }))
    .unwrap_or_else(|error| panic!("static local wrapper context should parse: {error}"))
}

fn local_idempotency_context_for_request(
    request: &ToolCallRequest,
    policy_bundle: &PolicyBundleVerification,
) -> Option<IdempotencyContext> {
    if !request.carries_mutation_risk() {
        return None;
    }

    let key = request.idempotency_key.clone()?;

    Some(IdempotencyContext {
        key: IdempotencyKey(key.clone()),
        execution_id: request.execution_id.clone().unwrap_or_else(|| key.clone()),
        tool_call_hash: tool_call_hash_reference(request),
        target_system: NonEmptyString::new("local_sandbox")
            .expect("static target system is non-empty"),
        operation_type: OperationType::Mutation,
        policy_bundle_version: policy_bundle
            .policy_version
            .as_ref()
            .map(|version| version.0.clone())
            .unwrap_or_else(|| {
                NonEmptyString::new("unknown_policy_version")
                    .expect("static policy version fallback is non-empty")
            }),
    })
}

fn tool_call_hash_reference(request: &ToolCallRequest) -> NonEmptyString {
    NonEmptyString::new(format!(
        "local_tool_call_ref_{}_{}",
        request.action_id.as_str(),
        request.tool.name.as_str().replace('.', "_")
    ))
    .expect("request action and tool identity are non-empty")
}

fn local_response_metadata(policy_bundle: &PolicyBundleVerification) -> ResponseMetadata {
    let fixture = local_response_metadata_fixture(policy_bundle);

    ResponseMetadata {
        execution_id: fixture.execution_id,
        policy_provenance: fixture.policy_provenance,
        audit_record_id: fixture.audit_record_id,
        completed_at: fixture.completed_at,
    }
}

fn local_response_metadata_fixture(policy_bundle: &PolicyBundleVerification) -> ToolCallResponse {
    serde_json::from_value(serde_json::json!({
        "schema_version": "1.0",
        "execution_id": "local_exec_001",
        "request_id": "local_req_001",
        "status": GatewayStatus::Allowed,
        "decision": ResponseDecision::Allow,
        "result": null,
        "reason_code": null,
        "safe_message": null,
        "pending_reference": null,
        "replay_reference": null,
        "policy_provenance": policy_bundle.policy_provenance(),
        "audit_record_id": "local_audit_001",
        "completed_at": "2026-06-28T00:00:00Z"
    }))
    .unwrap_or_else(|error| panic!("static local MVP response metadata should parse: {error}"))
}

fn local_audit_metadata() -> AuditRecordMetadata {
    serde_json::from_value(serde_json::json!({
        "component": "local_gateway_mvp"
    }))
    .unwrap_or_else(|error| panic!("static local MVP audit metadata should parse: {error}"))
}
