use aegis::{
    audit::{AuditRecordMetadata, AuditStatus},
    gateway::{
        Gateway, GatewayEntrypointResult, GatewayEntrypointSummary, GatewayPolicyAdapterContext,
        GatewayStatus, PendingReference, ResponseDecision, ResponseMetadata, SupportedTools,
        ToolCallRequest, ToolCallResponse,
    },
    policy::{
        PendingApprovalDecision, PolicyAdapterError, PolicyDecision, PolicyDecisionAdapter,
        PolicyDenial,
    },
};

#[test]
fn adapter_allow_decision_maps_to_allowed_response() {
    let adapter = TestPolicyAdapter::allow();
    let result = process_request_with_adapter(&adapter);

    assert_adapter_result(&result, GatewayStatus::Allowed, ResponseDecision::Allow);
}

#[test]
fn adapter_deny_decision_maps_to_denied_response() {
    let adapter = TestPolicyAdapter::deny();
    let result = process_request_with_adapter(&adapter);

    assert_adapter_result(&result, GatewayStatus::Denied, ResponseDecision::Deny);
    assert_eq!(
        result.response.reason_code.as_deref(),
        Some("adapter_denied")
    );
}

#[test]
fn adapter_pending_decision_maps_to_pending_response() {
    let pending_reference = pending_reference();
    let adapter = TestPolicyAdapter::pending(pending_reference.clone());
    let result = process_request_with_adapter(&adapter);

    assert_adapter_result(
        &result,
        GatewayStatus::Pending,
        ResponseDecision::PendingApproval,
    );
    assert_eq!(result.response.pending_reference, Some(pending_reference));
}

#[test]
fn adapter_error_fails_closed_with_denied_response() {
    let adapter = TestPolicyAdapter::error();
    let result = process_request_with_adapter(&adapter);

    assert_eq!(
        result.summary,
        GatewayEntrypointSummary::PolicyAdapterFailedClosed
    );
    assert_eq!(result.response.status(), &GatewayStatus::Denied);
    assert_eq!(result.response.decision, Some(ResponseDecision::Deny));
    assert_eq!(
        result.response.reason_code.as_deref(),
        Some("policy_adapter_error")
    );
    assert_eq!(
        result.response.safe_message(),
        Some("Policy adapter failed closed.")
    );
    assert_eq!(result.audit_record.status, AuditStatus::Denied);
}

struct TestPolicyAdapter {
    result: Result<PolicyDecision, PolicyAdapterError>,
}

impl TestPolicyAdapter {
    fn allow() -> Self {
        Self {
            result: Ok(PolicyDecision::Allow),
        }
    }

    fn deny() -> Self {
        Self {
            result: Ok(PolicyDecision::Deny(PolicyDenial {
                reason_code: Some("adapter_denied".to_string()),
                safe_message: "Adapter denied this request.".to_string(),
            })),
        }
    }

    fn pending(pending_reference: PendingReference) -> Self {
        Self {
            result: Ok(PolicyDecision::PendingApproval(PendingApprovalDecision {
                pending_reference,
                reason_code: Some("approval_required".to_string()),
                safe_message: Some("Approval is required.".to_string()),
            })),
        }
    }

    fn error() -> Self {
        Self {
            result: Err(PolicyAdapterError {
                reason_code: Some("policy_adapter_error".to_string()),
                safe_message: "Policy adapter failed closed.".to_string(),
            }),
        }
    }
}

impl PolicyDecisionAdapter for TestPolicyAdapter {
    fn decide(&self, _request: &ToolCallRequest) -> Result<PolicyDecision, PolicyAdapterError> {
        self.result.clone()
    }
}

fn assert_adapter_result(
    result: &GatewayEntrypointResult,
    status: GatewayStatus,
    decision: ResponseDecision,
) {
    assert_eq!(
        result.summary,
        GatewayEntrypointSummary::PolicyDecisionMapped
    );
    assert_eq!(result.response.status(), &status);
    assert_eq!(result.response.decision, Some(decision.clone()));
    assert_eq!(result.audit_record.status, AuditStatus::from(&status));
    assert_eq!(result.audit_record.details.decision, Some(decision));
}

fn process_request_with_adapter(adapter: &dyn PolicyDecisionAdapter) -> GatewayEntrypointResult {
    Gateway::process_entrypoint_request_with_policy_adapter(
        &read_fixture("schemas/examples/valid/ToolCallRequest.json"),
        GatewayPolicyAdapterContext {
            supported_tools: SupportedTools::from_names(["metrics.read"]),
            policy_adapter: adapter,
            response_metadata: response_metadata(),
            audit_metadata: audit_metadata(),
            idempotency_context: None,
        },
    )
}

fn load_valid_response() -> ToolCallResponse {
    serde_json::from_str(&read_fixture(
        "schemas/examples/valid/ToolCallResponse.json",
    ))
    .unwrap_or_else(|error| panic!("valid ToolCallResponse fixture should parse: {error}"))
}

fn response_metadata() -> ResponseMetadata {
    let fixture = load_valid_response();

    ResponseMetadata {
        execution_id: fixture.execution_id,
        policy_provenance: fixture.policy_provenance,
        audit_record_id: fixture.audit_record_id,
        completed_at: fixture.completed_at,
    }
}

fn pending_reference() -> PendingReference {
    serde_json::from_value(serde_json::json!({
        "approval_id": "approval_001",
        "expires_at": "2026-06-28T00:10:00Z"
    }))
    .unwrap_or_else(|error| panic!("pending reference fixture should parse: {error}"))
}

fn audit_metadata() -> AuditRecordMetadata {
    serde_json::from_value(serde_json::json!({
        "component": "gateway"
    }))
    .unwrap_or_else(|error| panic!("audit metadata fixture should parse: {error}"))
}

fn read_fixture(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
