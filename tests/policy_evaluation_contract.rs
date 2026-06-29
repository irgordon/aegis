use std::{
    fs,
    path::{Path, PathBuf},
};

use aegis::{
    gateway::{GatewayStatus, ResponseDecision, ToolCallRequest},
    policy::{
        evaluate_local_policy_bundle, load_policy_bundle, PolicyBundleVerification,
        PolicyEvaluationFailure, PolicyEvaluationStatus,
    },
};
use serde_json::Value;

const LOCAL_DEV_BUNDLE: &str = "examples/policy-bundles/local-dev";

#[test]
fn valid_bundle_allows_matching_allowed_policy_rule() {
    let result = evaluate_request(request_with_tool("metrics.read", "L0"));

    assert_eq!(
        result.evaluation.evaluation_status,
        PolicyEvaluationStatus::Evaluated
    );
    assert_eq!(result.evaluation.decision, Some(ResponseDecision::Allow));
    assert_eq!(
        result
            .evaluation
            .policy_rule_id
            .as_ref()
            .map(|rule| rule.as_str()),
        Some("allow_metrics_read_agent")
    );
}

#[test]
fn valid_bundle_denies_matching_denied_policy_rule() {
    let request = request_with_tool("email.send", "L1");
    let result = evaluate_request(request.clone());
    let response = aegis::gateway::Gateway::map_policy_decision(
        &request,
        result.decision,
        response_metadata(),
    );

    assert_eq!(response.status, GatewayStatus::Denied);
    assert_eq!(response.reason_code.as_deref(), Some("local_l1_denied"));
    assert_eq!(result.evaluation.decision, Some(ResponseDecision::Deny));
}

#[test]
fn valid_bundle_pends_matching_pending_policy_rule() {
    let request = request_with_tool("deploy.prod", "L2");
    let result = evaluate_request(request.clone());
    let response = aegis::gateway::Gateway::map_policy_decision(
        &request,
        result.decision,
        response_metadata(),
    );

    assert_eq!(response.status, GatewayStatus::Pending);
    assert_eq!(response.decision, Some(ResponseDecision::PendingApproval));
    assert_eq!(
        response
            .pending_reference
            .as_ref()
            .map(|pending| pending.approval_id.as_str()),
        Some("local_dev_approval_deploy_prod")
    );
}

#[test]
fn no_matching_policy_rule_fails_closed() {
    let result = evaluate_request(request_with_tool("storage.read", "L0"));

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::NoMatchingPolicyRule,
    );
}

#[test]
fn ambiguous_matching_policy_rules_fail_closed() {
    let bundle = mutable_verified_bundle("ambiguous_matching_policy_rules");
    append_policy_rule(
        &bundle,
        "  - id: duplicate_metrics_read_agent\n    tool: metrics.read\n    capability: L0\n    actor_type: agent\n    risk: local_l0_allow\n",
    );
    let result = evaluate_with_bundle(request_with_tool("metrics.read", "L0"), bundle);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::AmbiguousPolicyRules,
    );
}

#[test]
fn missing_risk_matrix_entry_fails_closed() {
    let bundle = mutable_verified_bundle("missing_risk_matrix_entry");
    replace_in_file(
        bundle.join("gateway_policy.yaml"),
        "risk: local_l0_allow",
        "risk: missing_risk_key",
    );
    let result = evaluate_with_bundle(request_with_tool("metrics.read", "L0"), bundle);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::MissingRiskMatrixEntry,
    );
}

#[test]
fn malformed_gateway_policy_fails_closed() {
    let bundle = mutable_verified_bundle("malformed_gateway_policy");
    fs::write(
        bundle.join("gateway_policy.yaml"),
        "policy_version 0.1.0-local\n",
    )
    .unwrap_or_else(|error| panic!("gateway policy should be writable: {error}"));
    let result = evaluate_with_bundle(request_with_tool("metrics.read", "L0"), bundle);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::GatewayPolicyMalformed,
    );
}

#[test]
fn malformed_risk_matrix_fails_closed() {
    let bundle = mutable_verified_bundle("malformed_risk_matrix");
    fs::write(
        bundle.join("risk_matrix.yaml"),
        "risk_matrix_version 0.1.0-local\n",
    )
    .unwrap_or_else(|error| panic!("risk matrix should be writable: {error}"));
    let result = evaluate_with_bundle(request_with_tool("metrics.read", "L0"), bundle);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::RiskMatrixMalformed,
    );
}

#[test]
fn unsupported_decision_value_fails_closed() {
    let bundle = mutable_verified_bundle("unsupported_decision_value");
    replace_in_file(
        bundle.join("risk_matrix.yaml"),
        "decision: allow",
        "decision: escalate",
    );
    let result = evaluate_with_bundle(request_with_tool("metrics.read", "L0"), bundle);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::UnsupportedDecisionValue,
    );
}

#[test]
fn unsupported_capability_class_fails_closed() {
    let bundle = mutable_verified_bundle("unsupported_capability_class");
    replace_in_file(
        bundle.join("gateway_policy.yaml"),
        "capability: L0",
        "capability: LX",
    );
    let result = evaluate_with_bundle(request_with_tool("metrics.read", "L0"), bundle);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::UnsupportedCapabilityClass,
    );
}

#[test]
fn invalid_signature_prevents_policy_evaluation() {
    let bundle = mutable_bundle("invalid_signature_prevents_policy_evaluation");
    append_to_file(
        bundle.join("checksums").join("SHA256SUMS"),
        "# unsigned change\n",
    );
    let verification = rejected_bundle(bundle);
    let result =
        evaluate_local_policy_bundle(&request_with_tool("metrics.read", "L0"), &verification);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::BundleNotVerified,
    );
}

#[test]
fn checksum_mismatch_prevents_policy_evaluation() {
    let bundle = mutable_bundle("checksum_mismatch_prevents_policy_evaluation");
    append_to_file(
        bundle.join("gateway_policy.yaml"),
        "# unsigned policy change\n",
    );
    let verification = rejected_bundle(bundle);
    let result =
        evaluate_local_policy_bundle(&request_with_tool("metrics.read", "L0"), &verification);

    assert_failed_closed(
        result.evaluation,
        PolicyEvaluationFailure::BundleNotVerified,
    );
}

#[test]
fn policy_evaluation_does_not_execute_tools() {
    let request = request_with_tool("metrics.read", "L0");
    let result = evaluate_request(request.clone());
    let response = aegis::gateway::Gateway::map_policy_decision(
        &request,
        result.decision,
        response_metadata(),
    );

    assert!(response.result.is_none());
}

fn evaluate_request(request: ToolCallRequest) -> aegis::policy::PolicyEvaluationResult {
    let verification = load_policy_bundle(LOCAL_DEV_BUNDLE)
        .unwrap_or_else(|error| panic!("local development bundle should verify: {error:?}"));

    evaluate_local_policy_bundle(&request, &verification)
}

fn evaluate_with_bundle(
    request: ToolCallRequest,
    bundle: PathBuf,
) -> aegis::policy::PolicyEvaluationResult {
    let verification = verification_for_mutable_bundle(bundle);

    evaluate_local_policy_bundle(&request, &verification)
}

fn assert_failed_closed(
    evaluation: aegis::policy::PolicyEvaluation,
    failure: PolicyEvaluationFailure,
) {
    assert_eq!(
        evaluation.evaluation_status,
        PolicyEvaluationStatus::FailedClosed
    );
    assert_eq!(evaluation.failure_reason, Some(failure));
    assert_eq!(evaluation.decision, Some(ResponseDecision::Deny));
}

fn rejected_bundle(bundle: PathBuf) -> PolicyBundleVerification {
    *load_policy_bundle(bundle)
        .expect_err("rejected local bundle should return verification evidence")
}

fn verification_for_mutable_bundle(bundle: PathBuf) -> PolicyBundleVerification {
    let mut verification = load_policy_bundle(LOCAL_DEV_BUNDLE)
        .unwrap_or_else(|error| panic!("local development bundle should verify: {error:?}"));
    verification.gateway_policy_path = path_string(bundle.join("gateway_policy.yaml"));
    verification.risk_matrix_path = path_string(bundle.join("risk_matrix.yaml"));
    verification.manifest_path = path_string(bundle.join("manifest.yaml"));
    verification
}

fn mutable_verified_bundle(case_name: &str) -> PathBuf {
    mutable_bundle(case_name)
}

fn mutable_bundle(case_name: &str) -> PathBuf {
    let target = Path::new("target")
        .join("policy-evaluation-tests")
        .join(case_name);

    if target.exists() {
        fs::remove_dir_all(&target)
            .unwrap_or_else(|error| panic!("old mutable fixture should be removable: {error}"));
    }

    copy_dir(Path::new(LOCAL_DEV_BUNDLE), &target);
    target
}

fn append_policy_rule(bundle: &Path, rule: &str) {
    append_to_file(bundle.join("gateway_policy.yaml"), rule);
}

fn append_to_file(path: PathBuf, text: &str) {
    let mut content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture should be readable: {error}"));
    content.push_str(text);
    fs::write(path, content).unwrap_or_else(|error| panic!("fixture should be writable: {error}"));
}

fn replace_in_file(path: PathBuf, from: &str, to: &str) {
    let content = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture should be readable: {error}"));
    fs::write(path, content.replacen(from, to, 1))
        .unwrap_or_else(|error| panic!("fixture should be writable: {error}"));
}

fn request_with_tool(tool_name: &str, capability_class: &str) -> ToolCallRequest {
    let mut request: Value =
        serde_json::from_str(&read_fixture("schemas/examples/valid/ToolCallRequest.json"))
            .unwrap_or_else(|error| panic!("valid request fixture should parse: {error}"));
    request["tool"]["name"] = Value::String(tool_name.to_string());
    request["tool"]["capability_class"] = Value::String(capability_class.to_string());

    serde_json::from_value(request)
        .unwrap_or_else(|error| panic!("modified request should deserialize: {error}"))
}

fn response_metadata() -> aegis::gateway::ResponseMetadata {
    let response: aegis::gateway::ToolCallResponse = serde_json::from_str(&read_fixture(
        "schemas/examples/valid/ToolCallResponse.json",
    ))
    .unwrap_or_else(|error| panic!("valid response fixture should parse: {error}"));

    aegis::gateway::ResponseMetadata {
        execution_id: response.execution_id,
        policy_provenance: response.policy_provenance,
        audit_record_id: response.audit_record_id,
        completed_at: response.completed_at,
    }
}

fn copy_dir(source: &Path, target: &Path) {
    fs::create_dir_all(target)
        .unwrap_or_else(|error| panic!("target fixture directory should be creatable: {error}"));

    for entry in fs::read_dir(source)
        .unwrap_or_else(|error| panic!("source fixture directory should be readable: {error}"))
    {
        let entry =
            entry.unwrap_or_else(|error| panic!("fixture entry should be readable: {error}"));
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &target_path);
        } else {
            fs::copy(&source_path, &target_path)
                .unwrap_or_else(|error| panic!("fixture file should copy: {error}"));
        }
    }
}

fn path_string(path: PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

fn read_fixture(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("fixture should be readable at {path}: {error}"))
}
