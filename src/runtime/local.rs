use std::path::Path;

use serde::Serialize;

use crate::{
    audit::{AuditRecord, AuditRecordBuilder, AuditRecordMetadata, GatewayAuditContexts},
    gateway::{
        Gateway, GatewayStatus, GatewayValidationOutcome, ResponseDecision, ResponseMetadata,
        SupportedTools, ToolCallRequest, ToolCallResponse,
    },
    policy::{
        evaluate_local_policy_bundle, load_policy_bundle, PolicyBundleVerification,
        PolicyEvaluation,
    },
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocalRuntimeOutput {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
    pub policy_bundle: PolicyBundleVerification,
    pub policy_evaluation: Option<PolicyEvaluation>,
}

pub fn process_local_gateway_request(input: &str, bundle_path: &Path) -> LocalRuntimeOutput {
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
            process_validated_request(*request, policy_bundle)
        }
        GatewayValidationOutcome::Denied(evidence) => {
            LocalRuntimeOutput::from_denial(*evidence, policy_bundle)
        }
    }
}

impl LocalRuntimeOutput {
    fn from_denial(
        evidence: crate::gateway::GatewayDecisionEvidence,
        policy_bundle: PolicyBundleVerification,
    ) -> Self {
        Self {
            response: evidence.response,
            audit_record: evidence.audit_record,
            policy_bundle,
            policy_evaluation: None,
        }
    }
}

fn process_validated_request(
    request: ToolCallRequest,
    policy_bundle: PolicyBundleVerification,
) -> LocalRuntimeOutput {
    let evaluation_result = evaluate_local_policy_bundle(&request, &policy_bundle);
    let policy_evaluation = evaluation_result.evaluation;
    let response = Gateway::map_policy_decision(
        &request,
        evaluation_result.decision,
        local_response_metadata(&policy_bundle),
    );
    let audit_record = AuditRecordBuilder::build_gateway_decision_record_with_contexts(
        &request,
        &response,
        local_audit_metadata(),
        GatewayAuditContexts {
            policy_bundle_verification: Some(policy_bundle.clone()),
            policy_evaluation: Some(policy_evaluation.clone()),
            ..GatewayAuditContexts::default()
        },
    );

    LocalRuntimeOutput {
        response,
        audit_record,
        policy_bundle,
        policy_evaluation: Some(policy_evaluation),
    }
}

fn verified_or_rejected_bundle(bundle_path: &Path) -> PolicyBundleVerification {
    match load_policy_bundle(bundle_path) {
        Ok(verification) => verification,
        Err(verification) => *verification,
    }
}

fn local_supported_tools() -> SupportedTools {
    SupportedTools::from_names(["metrics.read", "email.send", "deploy.prod", "storage.read"])
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
