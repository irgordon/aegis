use std::path::Path;

use serde::Serialize;

use crate::{
    audit::{AuditRecord, AuditRecordMetadata},
    gateway::{
        CapabilityClass, Gateway, GatewayEntrypointResult, GatewayPolicyAdapterContext,
        GatewayStatus, ResponseDecision, ResponseMetadata, SupportedTools, ToolCallRequest,
        ToolCallResponse,
    },
    policy::{
        load_policy_bundle, PolicyAdapterError, PolicyBundleVerification, PolicyDecision,
        PolicyDecisionAdapter, PolicyDenial,
    },
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocalRuntimeOutput {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
    pub policy_bundle: PolicyBundleVerification,
}

pub fn process_local_gateway_request(input: &str, bundle_path: &Path) -> LocalRuntimeOutput {
    let policy_bundle = verified_or_rejected_bundle(bundle_path);
    let adapter = LocalMvpPolicyAdapter {
        bundle_verified: policy_bundle.is_verified(),
    };
    let result = Gateway::process_entrypoint_request_with_policy_adapter(
        input,
        GatewayPolicyAdapterContext {
            supported_tools: local_supported_tools(),
            policy_adapter: &adapter,
            response_metadata: local_response_metadata(&policy_bundle),
            audit_metadata: local_audit_metadata(),
            idempotency_context: None,
            wrapper_context: None,
            execution_identity_context: None,
            approval_context: None,
            policy_bundle_verification: Some(policy_bundle.clone()),
        },
    );

    LocalRuntimeOutput::from_result(result, policy_bundle)
}

impl LocalRuntimeOutput {
    fn from_result(
        result: GatewayEntrypointResult,
        policy_bundle: PolicyBundleVerification,
    ) -> Self {
        Self {
            response: result.response,
            audit_record: result.audit_record,
            policy_bundle,
        }
    }
}

struct LocalMvpPolicyAdapter {
    bundle_verified: bool,
}

impl PolicyDecisionAdapter for LocalMvpPolicyAdapter {
    fn decide(&self, request: &ToolCallRequest) -> Result<PolicyDecision, PolicyAdapterError> {
        local_policy_decision(request, self.bundle_verified)
    }
}

fn local_policy_decision(
    request: &ToolCallRequest,
    bundle_verified: bool,
) -> Result<PolicyDecision, PolicyAdapterError> {
    if !bundle_verified {
        return Err(policy_bundle_verification_error());
    }

    if request.tool_name() == "policy.error" {
        return Err(local_policy_adapter_error());
    }

    if is_allowed_local_fixture_request(request) {
        return Ok(PolicyDecision::Allow);
    }

    Ok(local_static_denial())
}

fn is_allowed_local_fixture_request(request: &ToolCallRequest) -> bool {
    request.tool_name() == "metrics.read"
        && request
            .tool
            .capability_class
            .as_ref()
            .is_some_and(|capability| capability == &CapabilityClass::L0)
}

fn local_static_denial() -> PolicyDecision {
    PolicyDecision::Deny(PolicyDenial {
        reason_code: Some("local_policy_denied".to_string()),
        safe_message: "Local MVP policy adapter denied this request.".to_string(),
    })
}

fn local_policy_adapter_error() -> PolicyAdapterError {
    PolicyAdapterError {
        reason_code: Some("local_policy_adapter_error".to_string()),
        safe_message: "Local MVP policy adapter failed closed.".to_string(),
    }
}

fn policy_bundle_verification_error() -> PolicyAdapterError {
    PolicyAdapterError {
        reason_code: Some("policy_bundle_verification_failed".to_string()),
        safe_message: "Policy bundle verification failed closed.".to_string(),
    }
}

fn verified_or_rejected_bundle(bundle_path: &Path) -> PolicyBundleVerification {
    match load_policy_bundle(bundle_path) {
        Ok(verification) => verification,
        Err(verification) => *verification,
    }
}

fn local_supported_tools() -> SupportedTools {
    SupportedTools::from_names(["metrics.read", "policy.error"])
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
