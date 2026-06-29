use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::gateway::{
    ActorType, CapabilityClass, NonEmptyString, PendingReference, ResponseDecision, ToolCallRequest,
};

use super::{
    PendingApprovalDecision, PolicyBundleRef, PolicyBundleVerification, PolicyDecision,
    PolicyDenial, PolicyVersion, RiskMatrixVersion,
};

const GATEWAY_POLICY_RULES_SECTION: &str = "rules";
const RISK_MATRIX_ENTRIES_SECTION: &str = "entries";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GatewayPolicyRuleId(pub NonEmptyString);

impl GatewayPolicyRuleId {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayPolicy {
    pub policy_version: PolicyVersion,
    pub rules: Vec<GatewayPolicyRule>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GatewayPolicyRule {
    pub rule_id: GatewayPolicyRuleId,
    pub tool_name: NonEmptyString,
    pub capability_class: CapabilityClass,
    pub actor_type: Option<ActorType>,
    pub risk_key: NonEmptyString,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RiskMatrix {
    pub risk_matrix_version: RiskMatrixVersion,
    pub entries: Vec<RiskMatrixEntry>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RiskMatrixEntry {
    pub entry_id: NonEmptyString,
    pub capability_class: CapabilityClass,
    pub decision: RiskOutcome,
    pub reason_code: NonEmptyString,
    pub safe_message: NonEmptyString,
    pub approval_id: Option<NonEmptyString>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskOutcome {
    Allow,
    Deny,
    PendingApproval,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEvaluationStatus {
    Evaluated,
    FailedClosed,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEvaluationFailure {
    BundleNotVerified,
    GatewayPolicyMalformed,
    RiskMatrixMalformed,
    NoMatchingPolicyRule,
    AmbiguousPolicyRules,
    MissingRiskMatrixEntry,
    UnsupportedCapabilityClass,
    UnsupportedDecisionValue,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PolicyEvaluation {
    pub policy_bundle_id: Option<PolicyBundleRef>,
    pub policy_version: Option<PolicyVersion>,
    pub risk_matrix_version: Option<RiskMatrixVersion>,
    pub policy_rule_id: Option<GatewayPolicyRuleId>,
    pub matched_tool_name: Option<NonEmptyString>,
    pub matched_capability_class: Option<CapabilityClass>,
    pub risk_matrix_entry_id: Option<NonEmptyString>,
    pub decision: Option<ResponseDecision>,
    pub decision_reason: Option<String>,
    pub evaluation_status: PolicyEvaluationStatus,
    pub failure_reason: Option<PolicyEvaluationFailure>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolicyEvaluationResult {
    pub decision: PolicyDecision,
    pub evaluation: PolicyEvaluation,
}

pub fn evaluate_local_policy_bundle(
    request: &ToolCallRequest,
    bundle: &PolicyBundleVerification,
) -> PolicyEvaluationResult {
    LocalPolicyEvaluator::new(request, bundle).evaluate()
}

struct LocalPolicyEvaluator<'a> {
    request: &'a ToolCallRequest,
    bundle: &'a PolicyBundleVerification,
}

impl<'a> LocalPolicyEvaluator<'a> {
    fn new(request: &'a ToolCallRequest, bundle: &'a PolicyBundleVerification) -> Self {
        Self { request, bundle }
    }

    fn evaluate(&self) -> PolicyEvaluationResult {
        if !self.bundle.is_verified() {
            return self.fail_closed(PolicyEvaluationFailure::BundleNotVerified);
        }

        let policy = match read_gateway_policy(self.bundle) {
            Ok(policy) => policy,
            Err(error) => return self.fail_closed(error.into_gateway_policy_failure()),
        };
        let risk_matrix = match read_risk_matrix(self.bundle) {
            Ok(risk_matrix) => risk_matrix,
            Err(error) => return self.fail_closed(error.into_risk_matrix_failure()),
        };

        self.evaluate_verified_policy(policy, risk_matrix)
    }

    fn evaluate_verified_policy(
        &self,
        policy: GatewayPolicy,
        risk_matrix: RiskMatrix,
    ) -> PolicyEvaluationResult {
        let matched_rules = matching_rules(self.request, &policy.rules);
        if matched_rules.is_empty() {
            return self.fail_closed(PolicyEvaluationFailure::NoMatchingPolicyRule);
        }
        if matched_rules.len() > 1 {
            return self.fail_closed(PolicyEvaluationFailure::AmbiguousPolicyRules);
        }

        let rule = matched_rules[0];
        let Some(entry) = risk_entry_for_rule(rule, &risk_matrix) else {
            return self
                .fail_closed_with_rule(rule, PolicyEvaluationFailure::MissingRiskMatrixEntry);
        };

        self.map_risk_outcome(rule, entry)
    }

    fn map_risk_outcome(
        &self,
        rule: &GatewayPolicyRule,
        entry: &RiskMatrixEntry,
    ) -> PolicyEvaluationResult {
        PolicyEvaluationResult {
            decision: policy_decision_from_entry(entry),
            evaluation: self.evaluated(rule, entry),
        }
    }

    fn evaluated(&self, rule: &GatewayPolicyRule, entry: &RiskMatrixEntry) -> PolicyEvaluation {
        PolicyEvaluation {
            policy_bundle_id: self.bundle.bundle.clone(),
            policy_version: self.bundle.policy_version.clone(),
            risk_matrix_version: self.bundle.risk_matrix_version.clone(),
            policy_rule_id: Some(rule.rule_id.clone()),
            matched_tool_name: Some(rule.tool_name.clone()),
            matched_capability_class: Some(rule.capability_class.clone()),
            risk_matrix_entry_id: Some(entry.entry_id.clone()),
            decision: Some(response_decision_from_outcome(&entry.decision)),
            decision_reason: Some(entry.reason_code.as_str().to_string()),
            evaluation_status: PolicyEvaluationStatus::Evaluated,
            failure_reason: None,
        }
    }

    fn fail_closed(&self, failure: PolicyEvaluationFailure) -> PolicyEvaluationResult {
        PolicyEvaluationResult {
            decision: fail_closed_decision(&failure),
            evaluation: self.failed_evaluation(failure, None),
        }
    }

    fn fail_closed_with_rule(
        &self,
        rule: &GatewayPolicyRule,
        failure: PolicyEvaluationFailure,
    ) -> PolicyEvaluationResult {
        PolicyEvaluationResult {
            decision: fail_closed_decision(&failure),
            evaluation: self.failed_evaluation(failure, Some(rule)),
        }
    }

    fn failed_evaluation(
        &self,
        failure: PolicyEvaluationFailure,
        rule: Option<&GatewayPolicyRule>,
    ) -> PolicyEvaluation {
        PolicyEvaluation {
            policy_bundle_id: self.bundle.bundle.clone(),
            policy_version: self.bundle.policy_version.clone(),
            risk_matrix_version: self.bundle.risk_matrix_version.clone(),
            policy_rule_id: rule.map(|rule| rule.rule_id.clone()),
            matched_tool_name: rule.map(|rule| rule.tool_name.clone()),
            matched_capability_class: rule.map(|rule| rule.capability_class.clone()),
            risk_matrix_entry_id: None,
            decision: Some(ResponseDecision::Deny),
            decision_reason: Some(failure.reason_code().to_string()),
            evaluation_status: PolicyEvaluationStatus::FailedClosed,
            failure_reason: Some(failure),
        }
    }
}

fn read_gateway_policy(
    bundle: &PolicyBundleVerification,
) -> Result<GatewayPolicy, PolicyParseError> {
    let document = read_policy_document(&bundle.gateway_policy_path)?;
    parse_gateway_policy(&document)
}

fn read_risk_matrix(bundle: &PolicyBundleVerification) -> Result<RiskMatrix, PolicyParseError> {
    let document = read_policy_document(&bundle.risk_matrix_path)?;
    parse_risk_matrix(&document)
}

fn read_policy_document(path: &str) -> Result<String, PolicyParseError> {
    fs::read_to_string(PathBuf::from(path)).map_err(|_| PolicyParseError::Malformed)
}

fn parse_gateway_policy(content: &str) -> Result<GatewayPolicy, PolicyParseError> {
    let document = parse_simple_policy_document(content)?;
    let policy_version = required_scalar(&document.scalars, "policy_version")?;
    let rule_maps = required_section(&document.sections, GATEWAY_POLICY_RULES_SECTION)?;
    let rules = rule_maps
        .iter()
        .map(parse_gateway_policy_rule)
        .collect::<Result<Vec<_>, _>>()?;

    if rules.is_empty() {
        return Err(PolicyParseError::Malformed);
    }

    Ok(GatewayPolicy {
        policy_version: PolicyVersion(policy_version),
        rules,
    })
}

fn parse_gateway_policy_rule(
    fields: &BTreeMap<String, String>,
) -> Result<GatewayPolicyRule, PolicyParseError> {
    Ok(GatewayPolicyRule {
        rule_id: GatewayPolicyRuleId(required_scalar(fields, "id")?),
        tool_name: required_scalar(fields, "tool")?,
        capability_class: capability_class(required_str(fields, "capability")?)?,
        actor_type: optional_actor_type(fields)?,
        risk_key: required_scalar(fields, "risk")?,
    })
}

fn parse_risk_matrix(content: &str) -> Result<RiskMatrix, PolicyParseError> {
    let document = parse_simple_policy_document(content)?;
    let risk_matrix_version = required_scalar(&document.scalars, "risk_matrix_version")?;
    let entry_maps = required_section(&document.sections, RISK_MATRIX_ENTRIES_SECTION)?;
    let entries = entry_maps
        .iter()
        .map(parse_risk_matrix_entry)
        .collect::<Result<Vec<_>, _>>()?;

    if entries.is_empty() {
        return Err(PolicyParseError::Malformed);
    }

    Ok(RiskMatrix {
        risk_matrix_version: RiskMatrixVersion(risk_matrix_version),
        entries,
    })
}

fn parse_risk_matrix_entry(
    fields: &BTreeMap<String, String>,
) -> Result<RiskMatrixEntry, PolicyParseError> {
    Ok(RiskMatrixEntry {
        entry_id: required_scalar(fields, "id")?,
        capability_class: capability_class(required_str(fields, "capability")?)?,
        decision: risk_outcome(required_str(fields, "decision")?)?,
        reason_code: required_scalar(fields, "reason")?,
        safe_message: required_scalar(fields, "message")?,
        approval_id: optional_scalar(fields, "approval_id")?,
    })
}

fn matching_rules<'a>(
    request: &ToolCallRequest,
    rules: &'a [GatewayPolicyRule],
) -> Vec<&'a GatewayPolicyRule> {
    rules
        .iter()
        .filter(|rule| policy_rule_matches_request(rule, request))
        .collect()
}

fn policy_rule_matches_request(rule: &GatewayPolicyRule, request: &ToolCallRequest) -> bool {
    rule.tool_name.as_str() == request.tool_name()
        && request
            .tool
            .capability_class
            .as_ref()
            .is_some_and(|capability| capability == &rule.capability_class)
        && rule
            .actor_type
            .as_ref()
            .is_none_or(|actor_type| actor_type == &request.actor.actor_type)
}

fn risk_entry_for_rule<'a>(
    rule: &GatewayPolicyRule,
    risk_matrix: &'a RiskMatrix,
) -> Option<&'a RiskMatrixEntry> {
    risk_matrix
        .entries
        .iter()
        .find(|entry| entry.entry_id.as_str() == rule.risk_key.as_str())
}

fn policy_decision_from_entry(entry: &RiskMatrixEntry) -> PolicyDecision {
    match entry.decision {
        RiskOutcome::Allow => PolicyDecision::Allow,
        RiskOutcome::Deny => PolicyDecision::Deny(PolicyDenial {
            reason_code: Some(entry.reason_code.as_str().to_string()),
            safe_message: entry.safe_message.as_str().to_string(),
        }),
        RiskOutcome::PendingApproval => PolicyDecision::PendingApproval(PendingApprovalDecision {
            pending_reference: pending_reference_from_entry(entry),
            reason_code: Some(entry.reason_code.as_str().to_string()),
            safe_message: Some(entry.safe_message.as_str().to_string()),
        }),
    }
}

fn pending_reference_from_entry(entry: &RiskMatrixEntry) -> PendingReference {
    PendingReference {
        approval_id: entry
            .approval_id
            .clone()
            .unwrap_or_else(|| entry.entry_id.clone()),
        expires_at: None,
    }
}

fn response_decision_from_outcome(outcome: &RiskOutcome) -> ResponseDecision {
    match outcome {
        RiskOutcome::Allow => ResponseDecision::Allow,
        RiskOutcome::Deny => ResponseDecision::Deny,
        RiskOutcome::PendingApproval => ResponseDecision::PendingApproval,
    }
}

fn fail_closed_decision(failure: &PolicyEvaluationFailure) -> PolicyDecision {
    PolicyDecision::Deny(PolicyDenial {
        reason_code: Some(failure.reason_code().to_string()),
        safe_message: "Policy evaluation failed closed.".to_string(),
    })
}

impl PolicyEvaluationFailure {
    fn reason_code(&self) -> &'static str {
        match self {
            Self::BundleNotVerified => "policy_bundle_not_verified",
            Self::GatewayPolicyMalformed => "gateway_policy_malformed",
            Self::RiskMatrixMalformed => "risk_matrix_malformed",
            Self::NoMatchingPolicyRule => "no_matching_policy_rule",
            Self::AmbiguousPolicyRules => "ambiguous_policy_rules",
            Self::MissingRiskMatrixEntry => "missing_risk_matrix_entry",
            Self::UnsupportedCapabilityClass => "unsupported_capability_class",
            Self::UnsupportedDecisionValue => "unsupported_decision_value",
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PolicyParseError {
    Malformed,
    UnsupportedCapabilityClass,
    UnsupportedDecisionValue,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SimplePolicyDocument {
    scalars: BTreeMap<String, String>,
    sections: BTreeMap<String, Vec<BTreeMap<String, String>>>,
}

fn parse_simple_policy_document(content: &str) -> Result<SimplePolicyDocument, PolicyParseError> {
    let mut scalars = BTreeMap::new();
    let mut sections = BTreeMap::new();
    let mut current_section = None;

    for line in content.lines().map(str::trim_end) {
        let trimmed = line.trim();
        if should_skip_line(trimmed) {
            continue;
        }

        if is_section_header(trimmed) {
            let section_name = trimmed.trim_end_matches(':').to_string();
            sections
                .entry(section_name.clone())
                .or_insert_with(Vec::new);
            current_section = Some(section_name);
            continue;
        }

        if trimmed.starts_with("- ") {
            push_section_item(&mut sections, current_section.as_deref(), trimmed)?;
            continue;
        }

        if let Some(section) = current_section.as_deref() {
            push_section_field(&mut sections, section, trimmed)?;
            continue;
        }

        let (key, value) = parse_key_value(trimmed)?;
        scalars.insert(key.to_string(), value.to_string());
    }

    Ok(SimplePolicyDocument { scalars, sections })
}

fn should_skip_line(line: &str) -> bool {
    line.is_empty() || line.starts_with('#')
}

fn is_section_header(line: &str) -> bool {
    line.ends_with(':') && !line.contains(' ')
}

fn push_section_item(
    sections: &mut BTreeMap<String, Vec<BTreeMap<String, String>>>,
    current_section: Option<&str>,
    line: &str,
) -> Result<(), PolicyParseError> {
    let section = current_section.ok_or(PolicyParseError::Malformed)?;
    let (key, value) = parse_key_value(line.trim_start_matches("- "))?;
    let mut item = BTreeMap::new();
    item.insert(key.to_string(), value.to_string());
    sections
        .get_mut(section)
        .ok_or(PolicyParseError::Malformed)?
        .push(item);
    Ok(())
}

fn push_section_field(
    sections: &mut BTreeMap<String, Vec<BTreeMap<String, String>>>,
    section: &str,
    line: &str,
) -> Result<(), PolicyParseError> {
    let (key, value) = parse_key_value(line.trim())?;
    let items = sections
        .get_mut(section)
        .ok_or(PolicyParseError::Malformed)?;
    let item = items.last_mut().ok_or(PolicyParseError::Malformed)?;
    item.insert(key.to_string(), value.to_string());
    Ok(())
}

fn parse_key_value(line: &str) -> Result<(&str, &str), PolicyParseError> {
    let (key, value) = line.split_once(':').ok_or(PolicyParseError::Malformed)?;
    let key = key.trim();
    let value = value.trim().trim_matches('"').trim_matches('\'');
    if key.is_empty() || value.is_empty() {
        return Err(PolicyParseError::Malformed);
    }

    Ok((key, value))
}

fn required_section<'a>(
    sections: &'a BTreeMap<String, Vec<BTreeMap<String, String>>>,
    key: &str,
) -> Result<&'a Vec<BTreeMap<String, String>>, PolicyParseError> {
    sections.get(key).ok_or(PolicyParseError::Malformed)
}

fn required_str<'a>(
    fields: &'a BTreeMap<String, String>,
    key: &str,
) -> Result<&'a str, PolicyParseError> {
    fields
        .get(key)
        .map(String::as_str)
        .ok_or(PolicyParseError::Malformed)
}

fn required_scalar(
    fields: &BTreeMap<String, String>,
    key: &str,
) -> Result<NonEmptyString, PolicyParseError> {
    non_empty(required_str(fields, key)?).map_err(|_| PolicyParseError::Malformed)
}

fn optional_scalar(
    fields: &BTreeMap<String, String>,
    key: &str,
) -> Result<Option<NonEmptyString>, PolicyParseError> {
    fields
        .get(key)
        .map(|value| non_empty(value).map_err(|_| PolicyParseError::Malformed))
        .transpose()
}

fn optional_actor_type(
    fields: &BTreeMap<String, String>,
) -> Result<Option<ActorType>, PolicyParseError> {
    fields
        .get("actor_type")
        .map(|value| actor_type(value))
        .transpose()
}

fn capability_class(value: &str) -> Result<CapabilityClass, PolicyParseError> {
    match value {
        "L0" => Ok(CapabilityClass::L0),
        "L1" => Ok(CapabilityClass::L1),
        "L2" => Ok(CapabilityClass::L2),
        "L3" => Ok(CapabilityClass::L3),
        _ => Err(PolicyParseError::UnsupportedCapabilityClass),
    }
}

fn actor_type(value: &str) -> Result<ActorType, PolicyParseError> {
    match value {
        "agent" => Ok(ActorType::Agent),
        "orchestrator" => Ok(ActorType::Orchestrator),
        "user" => Ok(ActorType::User),
        "service" => Ok(ActorType::Service),
        _ => Err(PolicyParseError::Malformed),
    }
}

fn risk_outcome(value: &str) -> Result<RiskOutcome, PolicyParseError> {
    match value {
        "allow" => Ok(RiskOutcome::Allow),
        "deny" => Ok(RiskOutcome::Deny),
        "pending_approval" => Ok(RiskOutcome::PendingApproval),
        _ => Err(PolicyParseError::UnsupportedDecisionValue),
    }
}

impl PolicyParseError {
    fn into_gateway_policy_failure(self) -> PolicyEvaluationFailure {
        match self {
            Self::Malformed => PolicyEvaluationFailure::GatewayPolicyMalformed,
            Self::UnsupportedCapabilityClass => PolicyEvaluationFailure::UnsupportedCapabilityClass,
            Self::UnsupportedDecisionValue => PolicyEvaluationFailure::UnsupportedDecisionValue,
        }
    }

    fn into_risk_matrix_failure(self) -> PolicyEvaluationFailure {
        match self {
            Self::Malformed => PolicyEvaluationFailure::RiskMatrixMalformed,
            Self::UnsupportedCapabilityClass => PolicyEvaluationFailure::UnsupportedCapabilityClass,
            Self::UnsupportedDecisionValue => PolicyEvaluationFailure::UnsupportedDecisionValue,
        }
    }
}

fn non_empty(value: &str) -> Result<NonEmptyString, serde_json::Error> {
    serde_json::from_value(serde_json::Value::String(value.to_string()))
}
