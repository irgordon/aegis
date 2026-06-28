use serde::Serialize;

use crate::{
    audit::{AuditRecord, AuditRecordMetadata},
    gateway::{
        CapabilityClass, Gateway, GatewayEntrypointResult, GatewayPolicyAdapterContext,
        GatewayStatus, PolicyProvenance, ResponseDecision, ResponseMetadata, SupportedTools,
        ToolCallRequest, ToolCallResponse,
    },
    policy::{PolicyAdapterError, PolicyDecision, PolicyDecisionAdapter, PolicyDenial},
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LocalRuntimeOutput {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
}

pub fn process_local_gateway_request(input: &str) -> LocalRuntimeOutput {
    let adapter = LocalMvpPolicyAdapter;
    let result = Gateway::process_entrypoint_request_with_policy_adapter(
        input,
        GatewayPolicyAdapterContext {
            supported_tools: local_supported_tools(),
            policy_adapter: &adapter,
            response_metadata: local_response_metadata(),
            audit_metadata: local_audit_metadata(),
            idempotency_context: None,
            wrapper_context: None,
            execution_identity_context: None,
            approval_context: None,
        },
    );

    LocalRuntimeOutput::from(result)
}

impl From<GatewayEntrypointResult> for LocalRuntimeOutput {
    fn from(result: GatewayEntrypointResult) -> Self {
        Self {
            response: result.response,
            audit_record: result.audit_record,
        }
    }
}

struct LocalMvpPolicyAdapter;

impl PolicyDecisionAdapter for LocalMvpPolicyAdapter {
    fn decide(&self, request: &ToolCallRequest) -> Result<PolicyDecision, PolicyAdapterError> {
        local_policy_decision(request)
    }
}

fn local_policy_decision(request: &ToolCallRequest) -> Result<PolicyDecision, PolicyAdapterError> {
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

fn local_supported_tools() -> SupportedTools {
    SupportedTools::from_names(["metrics.read", "policy.error"])
}

fn local_response_metadata() -> ResponseMetadata {
    let fixture = local_response_metadata_fixture();

    ResponseMetadata {
        execution_id: fixture.execution_id,
        policy_provenance: fixture.policy_provenance,
        audit_record_id: fixture.audit_record_id,
        completed_at: fixture.completed_at,
    }
}

fn local_response_metadata_fixture() -> ToolCallResponse {
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
        "policy_provenance": local_policy_provenance(),
        "audit_record_id": "local_audit_001",
        "completed_at": "2026-06-28T00:00:00Z"
    }))
    .unwrap_or_else(|error| panic!("static local MVP response metadata should parse: {error}"))
}

fn local_policy_provenance() -> PolicyProvenance {
    serde_json::from_value(serde_json::json!({
        "bundle_id": "local_mvp_static_bundle",
        "version": "local-mvp",
        "policy_hash": "static-local-mvp-policy",
        "environment": "local",
        "signer_identity": "local-mvp",
        "activated_at": "2026-06-28T00:00:00Z"
    }))
    .unwrap_or_else(|error| panic!("static local MVP policy provenance should parse: {error}"))
}

fn local_audit_metadata() -> AuditRecordMetadata {
    serde_json::from_value(serde_json::json!({
        "component": "local_gateway_mvp"
    }))
    .unwrap_or_else(|error| panic!("static local MVP audit metadata should parse: {error}"))
}
