use std::collections::BTreeSet;

use crate::{
    audit::{AuditRecord, AuditRecordBuilder, AuditRecordMetadata},
    policy::{PolicyDecision, PolicyDenial},
};

use super::{Gateway, ResponseMetadata, ToolCallRequest, ToolCallResponse};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SupportedTools {
    names: BTreeSet<String>,
}

impl SupportedTools {
    pub fn from_names<I, S>(names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }

    pub fn contains(&self, tool_name: &str) -> bool {
        self.names.contains(tool_name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GatewayDecisionEvidence {
    pub response: ToolCallResponse,
    pub audit_record: AuditRecord,
}

impl Gateway {
    pub fn deny_unsupported_tool(
        request: &ToolCallRequest,
        supported_tools: &SupportedTools,
        response_metadata: ResponseMetadata,
        audit_metadata: AuditRecordMetadata,
    ) -> Option<GatewayDecisionEvidence> {
        if supported_tools.contains(request.tool_name()) {
            return None;
        }

        let response =
            Self::map_policy_decision(request, unsupported_tool_decision(), response_metadata);
        let audit_record =
            AuditRecordBuilder::build_gateway_decision_record(request, &response, audit_metadata);

        Some(GatewayDecisionEvidence {
            response,
            audit_record,
        })
    }
}

fn unsupported_tool_decision() -> PolicyDecision {
    PolicyDecision::Deny(PolicyDenial {
        reason_code: Some("unsupported_tool".to_string()),
        safe_message: "Tool is not supported by this gateway.".to_string(),
    })
}
