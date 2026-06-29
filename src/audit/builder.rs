use serde::{Deserialize, Serialize};

use super::{
    record::AuditRecordDetailContexts, AuditEventType, AuditRecord, AuditRecordDetails, AuditStatus,
};
use crate::error::AuditErrorReport;
use crate::gateway::{
    ApprovalContext, ExecutionIdentityContext, IdempotencyContext, NonEmptyString, ToolCallRequest,
    ToolCallResponse, WrapperExecutionContext, WrapperExecutionEvidence,
};
use crate::policy::{PolicyBundleVerification, PolicyEvaluation};

pub struct AuditRecordBuilder;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct GatewayAuditContexts {
    pub idempotency_context: Option<IdempotencyContext>,
    pub wrapper_context: Option<WrapperExecutionContext>,
    pub wrapper_execution_evidence: Option<WrapperExecutionEvidence>,
    pub execution_identity_context: Option<ExecutionIdentityContext>,
    pub approval_context: Option<ApprovalContext>,
    pub policy_bundle_verification: Option<PolicyBundleVerification>,
    pub policy_evaluation: Option<PolicyEvaluation>,
    pub error_report: Option<AuditErrorReport>,
}

impl AuditRecordBuilder {
    pub fn build_gateway_decision_record(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        metadata: AuditRecordMetadata,
    ) -> AuditRecord {
        Self::build_gateway_decision_record_with_idempotency(request, response, metadata, None)
    }

    pub fn build_gateway_decision_record_with_idempotency(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        metadata: AuditRecordMetadata,
        idempotency_context: Option<IdempotencyContext>,
    ) -> AuditRecord {
        Self::build_gateway_decision_record_with_contexts(
            request,
            response,
            metadata,
            GatewayAuditContexts {
                idempotency_context,
                ..GatewayAuditContexts::default()
            },
        )
    }

    pub fn build_gateway_decision_record_with_contexts(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        metadata: AuditRecordMetadata,
        contexts: GatewayAuditContexts,
    ) -> AuditRecord {
        AuditRecord {
            schema_version: response.schema_version.clone(),
            audit_record_id: response.audit_record_id.clone(),
            timestamp: response.completed_at.clone(),
            event_type: AuditEventType::PolicyDecision,
            execution_id: response.execution_id.clone(),
            run_id: Some(request.run_id.clone()),
            task_id: Some(request.task_id.clone()),
            action_id: Some(request.action_id.clone()),
            tool_name: Some(request.tool.name.clone()),
            actor_identity: Some(request.actor.actor_id.clone()),
            status: AuditStatus::from(&response.status),
            reason_code: response.reason_code.clone(),
            policy_provenance: response.policy_provenance.clone(),
            component: metadata.component,
            details: AuditRecordDetails::from_gateway_decision_with_contexts(
                request,
                response,
                AuditRecordDetailContexts {
                    idempotency_context: contexts.idempotency_context,
                    wrapper_context: contexts.wrapper_context,
                    wrapper_execution_evidence: contexts.wrapper_execution_evidence,
                    execution_identity_context: contexts.execution_identity_context,
                    approval_context: contexts.approval_context,
                    policy_bundle_verification: contexts.policy_bundle_verification,
                    policy_evaluation: contexts.policy_evaluation,
                    error_report: contexts.error_report,
                },
            ),
        }
    }

    pub fn build_gateway_validation_denial_record(
        response: &ToolCallResponse,
        metadata: AuditRecordMetadata,
    ) -> AuditRecord {
        AuditRecord {
            schema_version: response.schema_version.clone(),
            audit_record_id: response.audit_record_id.clone(),
            timestamp: response.completed_at.clone(),
            event_type: AuditEventType::ValidationResult,
            execution_id: response.execution_id.clone(),
            run_id: None,
            task_id: None,
            action_id: None,
            tool_name: None,
            actor_identity: None,
            status: AuditStatus::from(&response.status),
            reason_code: response.reason_code.clone(),
            policy_provenance: response.policy_provenance.clone(),
            component: metadata.component,
            details: AuditRecordDetails::from_response(response),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRecordMetadata {
    pub component: NonEmptyString,
}
