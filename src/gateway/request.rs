use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{NonEmptyString, SchemaVersion, Timestamp};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolCallRequest {
    pub schema_version: SchemaVersion,
    pub request_id: NonEmptyString,
    pub run_id: NonEmptyString,
    pub task_id: NonEmptyString,
    pub action_id: NonEmptyString,
    pub execution_id: Option<NonEmptyString>,
    pub idempotency_key: Option<NonEmptyString>,
    pub attempt_number: Option<u64>,
    pub replay_token: Option<NonEmptyString>,
    pub actor: RequestActor,
    pub tool: RequestedTool,
    pub params: BTreeMap<String, Value>,
    pub environment: NonEmptyString,
    pub requested_at: Timestamp,
    pub orchestrator: Option<OrchestratorReference>,
}

impl ToolCallRequest {
    pub fn request_id(&self) -> &str {
        self.request_id.as_str()
    }

    pub fn tool_name(&self) -> &str {
        self.tool.name.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestActor {
    pub actor_id: NonEmptyString,
    pub actor_type: ActorType,
    pub display_name: Option<NonEmptyString>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Agent,
    Orchestrator,
    User,
    Service,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestedTool {
    pub name: NonEmptyString,
    pub capability_class: Option<CapabilityClass>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CapabilityClass {
    L0,
    L1,
    L2,
    L3,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrchestratorReference {
    pub name: Option<NonEmptyString>,
    pub run_reference: Option<NonEmptyString>,
}
