use aegis::gateway::{Gateway, GatewayRequest, GatewayResponse, GatewayStatus};
use aegis::policy::PolicyDecision;

#[test]
fn denied_response_can_be_constructed() {
    let response = GatewayResponse::denied("request-1", "unknown tool");

    assert_eq!(response.status(), &GatewayStatus::Denied);
    assert_eq!(response.reason(), Some("unknown tool"));
}

#[test]
fn allowed_response_can_be_constructed() {
    let response = GatewayResponse::allowed("request-1");

    assert_eq!(response.status(), &GatewayStatus::Allowed);
}

#[test]
fn pending_response_can_be_constructed() {
    let response = GatewayResponse::pending("request-1", "approval-1");

    assert_eq!(response.status(), &GatewayStatus::Pending);
    assert_eq!(response.pending_reference(), Some("approval-1"));
}

#[test]
fn status_values_are_bounded_by_enum() {
    let statuses = [
        GatewayStatus::Allowed,
        GatewayStatus::Denied,
        GatewayStatus::Pending,
        GatewayStatus::Failed,
        GatewayStatus::Canceled,
        GatewayStatus::Replayed,
    ];

    assert_eq!(statuses.len(), 6);
}

#[test]
fn gateway_response_mapping_requires_policy_decision() {
    let request = GatewayRequest::new("request-1", "read.metrics");

    let response = Gateway::map_policy_decision(
        &request,
        PolicyDecision::Deny("policy not implemented".to_string()),
    );

    assert_eq!(response.status(), &GatewayStatus::Denied);
    assert_eq!(response.reason(), Some("policy not implemented"));
}
