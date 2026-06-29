use std::path::Path;

use serde::Serialize;

use crate::{
    audit::{AuditRecord, AuditRecordBuilder, AuditRecordMetadata, GatewayAuditContexts},
    error::{AuditErrorReport, GatewayErrorReport},
    gateway::{
        Gateway, GatewayStatus, GatewayValidationOutcome, ResponseDecision, ResponseMetadata,
        SupportedTools, ToolCallRequest, ToolCallResponse, WrapperDispatcher,
        WrapperExecutionContext, WrapperExecutionEvidence, WrapperExecutor,
    },
    policy::{
        evaluate_local_policy_bundle, load_policy_bundle, PolicyBundleVerification, PolicyDecision,
        PolicyDenial, PolicyEvaluation,
    },
    wrappers::HealthCheckWrapper,
};

static HEALTH_CHECK_WRAPPER: HealthCheckWrapper = HealthCheckWrapper;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocalRuntimeOutput {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
    pub policy_bundle: PolicyBundleVerification,
    pub policy_evaluation: Option<PolicyEvaluation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapper_execution: Option<WrapperExecutionEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_report: Option<GatewayErrorReport>,
}

pub fn process_local_gateway_request(input: &str, bundle_path: &Path) -> LocalRuntimeOutput {
    process_local_gateway_request_with_wrapper_registry(
        input,
        bundle_path,
        &local_wrapper_executors(),
        None,
    )
}

pub fn process_local_gateway_request_with_wrapper_registry(
    input: &str,
    bundle_path: &Path,
    wrapper_executors: &[&dyn WrapperExecutor],
    wrapper_context: Option<WrapperExecutionContext>,
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
            process_validated_request(*request, policy_bundle, wrapper_executors, wrapper_context)
        }
        GatewayValidationOutcome::Denied(evidence) => {
            LocalRuntimeOutput::from_denial(*evidence, policy_bundle)
        }
    }
}

impl LocalRuntimeOutput {
    pub fn attach_error_report(&mut self, report: GatewayErrorReport) {
        self.audit_record.details.error_report = Some(report.audit_fields());
        self.error_report = Some(report);
    }

    fn from_denial(
        evidence: crate::gateway::GatewayDecisionEvidence,
        policy_bundle: PolicyBundleVerification,
    ) -> Self {
        let error_report = GatewayErrorReport::from_validation_denial(&evidence.response);
        let mut audit_record = evidence.audit_record;
        audit_record.details.error_report = Some(error_report.audit_fields());

        Self {
            response: evidence.response,
            audit_record,
            policy_bundle,
            policy_evaluation: None,
            wrapper_execution: None,
            error_report: Some(error_report),
        }
    }
}

fn process_validated_request(
    request: ToolCallRequest,
    policy_bundle: PolicyBundleVerification,
    wrapper_executors: &[&dyn WrapperExecutor],
    wrapper_context: Option<WrapperExecutionContext>,
) -> LocalRuntimeOutput {
    let evaluation_result = evaluate_local_policy_bundle(&request, &policy_bundle);
    let policy_evaluation = evaluation_result.evaluation;
    let error_report = gateway_error_report(&request, &policy_bundle, &policy_evaluation);
    let wrapper_context =
        wrapper_context.unwrap_or_else(|| local_wrapper_context_for_request(&request));
    let response = Gateway::map_policy_decision(
        &request,
        evaluation_result.decision,
        local_response_metadata(&policy_bundle),
    );

    if should_dispatch_wrapper(&response) {
        return dispatch_allowed_request(
            request,
            response,
            policy_bundle,
            policy_evaluation,
            wrapper_executors,
            wrapper_context,
        );
    }

    build_runtime_output(
        &request,
        response,
        policy_bundle,
        policy_evaluation,
        None,
        None,
        error_report,
    )
}

fn dispatch_allowed_request(
    request: ToolCallRequest,
    response: ToolCallResponse,
    policy_bundle: PolicyBundleVerification,
    policy_evaluation: PolicyEvaluation,
    wrapper_executors: &[&dyn WrapperExecutor],
    wrapper_context: WrapperExecutionContext,
) -> LocalRuntimeOutput {
    let dispatcher = WrapperDispatcher::new(wrapper_executors.iter().copied());

    match dispatcher.dispatch(&request, &wrapper_context) {
        Ok(result) => {
            build_executed_output(request, response, policy_bundle, policy_evaluation, result)
        }
        Err(error) => build_wrapper_failure_output(
            request,
            policy_bundle,
            policy_evaluation,
            wrapper_context,
            error,
        ),
    }
}

fn build_executed_output(
    request: ToolCallRequest,
    mut response: ToolCallResponse,
    policy_bundle: PolicyBundleVerification,
    policy_evaluation: PolicyEvaluation,
    wrapper_result: crate::gateway::WrapperExecutionResult,
) -> LocalRuntimeOutput {
    response.result = wrapper_result.result.clone();
    let wrapper_execution = WrapperExecutionEvidence::from(&wrapper_result);

    build_runtime_output(
        &request,
        response,
        policy_bundle,
        policy_evaluation,
        Some(wrapper_result.context.clone()),
        Some(wrapper_execution),
        None,
    )
}

fn build_wrapper_failure_output(
    request: ToolCallRequest,
    policy_bundle: PolicyBundleVerification,
    policy_evaluation: PolicyEvaluation,
    wrapper_context: WrapperExecutionContext,
    error: crate::gateway::WrapperDispatchError,
) -> LocalRuntimeOutput {
    let response = wrapper_failure_response(&request, &policy_bundle, &error);
    let error_report = GatewayErrorReport::wrapper_dispatch_failed(&error, &wrapper_context);

    build_runtime_output(
        &request,
        response,
        policy_bundle,
        policy_evaluation,
        Some(wrapper_context),
        None,
        Some(error_report),
    )
}

fn build_runtime_output(
    request: &ToolCallRequest,
    response: ToolCallResponse,
    policy_bundle: PolicyBundleVerification,
    policy_evaluation: PolicyEvaluation,
    wrapper_context: Option<WrapperExecutionContext>,
    wrapper_execution: Option<WrapperExecutionEvidence>,
    error_report: Option<GatewayErrorReport>,
) -> LocalRuntimeOutput {
    let audit_record = AuditRecordBuilder::build_gateway_decision_record_with_contexts(
        request,
        &response,
        local_audit_metadata(),
        GatewayAuditContexts {
            policy_bundle_verification: Some(policy_bundle.clone()),
            policy_evaluation: Some(policy_evaluation.clone()),
            wrapper_context,
            wrapper_execution_evidence: wrapper_execution.clone(),
            error_report: audit_error_report(error_report.as_ref()),
            ..GatewayAuditContexts::default()
        },
    );

    LocalRuntimeOutput {
        response,
        audit_record,
        policy_bundle,
        policy_evaluation: Some(policy_evaluation),
        wrapper_execution,
        error_report,
    }
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
        "metrics.read",
        "email.send",
        "deploy.prod",
        "storage.read",
    ])
}

fn local_wrapper_executors() -> [&'static dyn WrapperExecutor; 1] {
    [&HEALTH_CHECK_WRAPPER]
}

fn local_health_check_context() -> WrapperExecutionContext {
    local_wrapper_context("health.check", "1.0.0", "local")
}

fn local_wrapper_context_for_request(request: &ToolCallRequest) -> WrapperExecutionContext {
    match request.tool_name() {
        "health.check" => local_health_check_context(),
        tool_name => local_wrapper_context(tool_name, "1.0.0", "local"),
    }
}

fn local_wrapper_context(
    wrapper_name: &str,
    wrapper_version: &str,
    target_system: &str,
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
        "credential_injection_required": false
    }))
    .unwrap_or_else(|error| panic!("static local wrapper context should parse: {error}"))
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
