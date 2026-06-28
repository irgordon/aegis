use serde::{Deserialize, Serialize};

use super::{AuditEventType, AuditRecord, AuditRecordDetails, AuditStatus};
use crate::gateway::{NonEmptyString, ToolCallRequest, ToolCallResponse};

pub struct AuditRecordBuilder;

impl AuditRecordBuilder {
    pub fn build_gateway_decision_record(
        request: &ToolCallRequest,
        response: &ToolCallResponse,
        metadata: AuditRecordMetadata,
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
            details: AuditRecordDetails::from_gateway_decision(request, response),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditRecordMetadata {
    pub component: NonEmptyString,
}
