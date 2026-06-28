mod request;
mod response;
mod schema;

pub use request::{
    ActorType, CapabilityClass, OrchestratorReference, RequestActor, RequestedTool, ToolCallRequest,
};
pub use response::{
    GatewayError, GatewayStatus, PendingReference, PolicyProvenance, ReplayReference,
    ResponseDecision, ToolCallResponse,
};
pub use schema::{NonEmptyString, SchemaVersion, Timestamp};

pub struct Gateway;

pub fn entrypoint_status() -> &'static str {
    "AEGIS Gateway MVP scaffold is present; governed execution is not implemented."
}
