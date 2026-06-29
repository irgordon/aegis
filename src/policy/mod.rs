mod bundle;
mod evaluator;

use crate::gateway::{PendingReference, ToolCallRequest};

pub use bundle::{
    load_policy_bundle, BundleDigestRef, ChecksumAlgorithm, ChecksumDigest, ChecksumMetadataStatus,
    ChecksumRef, ChecksumVerificationFailure, ChecksumVerificationStatus,
    PolicyBundleChecksumEntry, PolicyBundleLoadResult, PolicyBundleManifest, PolicyBundleRef,
    PolicyBundleSignatureVerification, PolicyBundleVerification, PolicyBundleVerificationStatus,
    PolicyVersion, PublicKeyRef, RiskMatrixVersion, SignatureAlgorithm, SignatureMetadataStatus,
    SignatureRef, SignatureVerificationFailure, SignatureVerificationStatus, SignedArtifactRef,
};
pub use evaluator::{
    evaluate_local_policy_bundle, GatewayPolicy, GatewayPolicyRule, GatewayPolicyRuleId,
    PolicyEvaluation, PolicyEvaluationFailure, PolicyEvaluationResult, PolicyEvaluationStatus,
    RiskMatrix, RiskMatrixEntry, RiskOutcome,
};

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
