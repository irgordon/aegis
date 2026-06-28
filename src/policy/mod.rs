use crate::gateway::{PendingReference, ToolCallRequest};

pub trait PolicyDecisionAdapter {
    fn decide(&self, request: &ToolCallRequest) -> Result<PolicyDecision, PolicyAdapterError>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolicyAdapterError {
    pub reason_code: Option<String>,
    pub safe_message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolicyDecision {
    Allow,
    Deny(PolicyDenial),
    PendingApproval(PendingApprovalDecision),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolicyDenial {
    pub reason_code: Option<String>,
    pub safe_message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PendingApprovalDecision {
    pub pending_reference: PendingReference,
    pub reason_code: Option<String>,
    pub safe_message: Option<String>,
}
