mod request;
mod response;

pub use request::GatewayRequest;
pub use response::{GatewayError, GatewayResponse, GatewayStatus};

use crate::policy::PolicyDecision;

pub struct Gateway;

impl Gateway {
    pub fn map_policy_decision(
        request: &GatewayRequest,
        decision: PolicyDecision,
    ) -> GatewayResponse {
        match decision {
            PolicyDecision::Allow => GatewayResponse::allowed(request.request_id()),
            PolicyDecision::Deny(reason) => GatewayResponse::denied(request.request_id(), reason),
            PolicyDecision::Pending(reference) => {
                GatewayResponse::pending(request.request_id(), reference)
            }
        }
    }
}

pub fn entrypoint_status() -> &'static str {
    "AEGIS Gateway MVP scaffold is present; governed execution is not implemented."
}
