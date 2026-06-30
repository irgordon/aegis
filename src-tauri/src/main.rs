use std::path::{Path, PathBuf};

use aegis::{error::GatewayErrorReport, runtime::local::process_local_gateway_request};
use serde::Serialize;
use slint::SharedString;

slint::include_modules!();

type DesktopResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DesktopResult<()> {
    let _tauri_shell = build_tauri_shell()?;
    show_operator_surface(get_health_check_evidence())?;
    Ok(())
}

fn build_tauri_shell() -> tauri::Result<tauri::App<tauri::Wry>> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_health_check_evidence])
        .build(tauri::generate_context!())
}

fn show_operator_surface(evidence: UiEvidence) -> Result<(), slint::PlatformError> {
    let window = AegisWindow::new()?;
    apply_evidence_to_window(&window, evidence);
    window.run()
}

#[tauri::command]
fn get_health_check_evidence() -> UiEvidence {
    live_health_check_evidence(&fixed_policy_bundle_path())
}

fn live_health_check_evidence(bundle_path: &Path) -> UiEvidence {
    let mut output = process_local_gateway_request(fixed_health_check_request(), bundle_path);
    output.mark_audited_completed();
    UiEvidence::from_runtime_output(output)
}

fn fixed_health_check_request() -> &'static str {
    include_str!("../../schemas/examples/valid/HealthCheckRequest.json")
}

fn fixed_policy_bundle_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/policy-bundles/local-dev")
}

fn apply_evidence_to_window(window: &AegisWindow, evidence: UiEvidence) {
    apply_header_to_window(window, &evidence);
    apply_status_cards_to_window(window, &evidence);
    apply_timeline_to_window(window, &evidence);
    apply_error_to_window(window, &evidence);
}

fn apply_header_to_window(window: &AegisWindow, evidence: &UiEvidence) {
    window.set_evidence_mode_label(shared(&evidence.evidence_mode_label));
    window.set_header_summary(shared(&evidence.header_summary));
}

fn apply_status_cards_to_window(window: &AegisWindow, evidence: &UiEvidence) {
    window.set_policy_bundle_status(shared(&evidence.policy_bundle_status));
    window.set_policy_bundle_summary(shared(&evidence.policy_bundle_summary));
    window.set_policy_decision_status(shared(&evidence.policy_decision_status));
    window.set_policy_decision_summary(shared(&evidence.policy_decision_summary));
    window.set_authorization_status(shared(&evidence.authorization_status));
    window.set_authorization_summary(shared(&evidence.authorization_summary));
    window.set_credential_boundary_status(shared(&evidence.credential_boundary_status));
    window.set_credential_boundary_summary(shared(&evidence.credential_boundary_summary));
    window.set_credential_injection_status(shared(&evidence.credential_injection_status));
    window.set_credential_injection_summary(shared(&evidence.credential_injection_summary));
    window.set_wrapper_execution_status(shared(&evidence.wrapper_execution_status));
    window.set_wrapper_execution_summary(shared(&evidence.wrapper_execution_summary));
    window.set_audit_status(shared(&evidence.audit_status));
    window.set_audit_summary(shared(&evidence.audit_summary));
    window.set_state_status(shared(&evidence.state_status));
    window.set_state_summary(shared(&evidence.state_summary));
}

fn apply_timeline_to_window(window: &AegisWindow, evidence: &UiEvidence) {
    window.set_request_status(shared(&evidence.request_status));
    window.set_request_summary(shared(&evidence.request_summary));
    window.set_validation_status(shared(&evidence.validation_status));
    window.set_validation_summary(shared(&evidence.validation_summary));
    window.set_bundle_timeline_status(shared(&evidence.bundle_timeline_status));
    window.set_bundle_timeline_summary(shared(&evidence.bundle_timeline_summary));
    window.set_policy_timeline_status(shared(&evidence.policy_timeline_status));
    window.set_policy_timeline_summary(shared(&evidence.policy_timeline_summary));
    window.set_authorization_timeline_status(shared(&evidence.authorization_timeline_status));
    window.set_authorization_timeline_summary(shared(&evidence.authorization_timeline_summary));
    window.set_credential_boundary_timeline_status(shared(
        &evidence.credential_boundary_timeline_status,
    ));
    window.set_credential_boundary_timeline_summary(shared(
        &evidence.credential_boundary_timeline_summary,
    ));
    window.set_credential_injection_timeline_status(shared(
        &evidence.credential_injection_timeline_status,
    ));
    window.set_credential_injection_timeline_summary(shared(
        &evidence.credential_injection_timeline_summary,
    ));
    window.set_dispatch_status(shared(&evidence.dispatch_status));
    window.set_dispatch_summary(shared(&evidence.dispatch_summary));
    window.set_wrapper_timeline_status(shared(&evidence.wrapper_timeline_status));
    window.set_wrapper_timeline_summary(shared(&evidence.wrapper_timeline_summary));
    window.set_audit_timeline_status(shared(&evidence.audit_timeline_status));
    window.set_audit_timeline_summary(shared(&evidence.audit_timeline_summary));
    window.set_state_timeline_status(shared(&evidence.state_timeline_status));
    window.set_state_timeline_summary(shared(&evidence.state_timeline_summary));
}

fn apply_error_to_window(window: &AegisWindow, evidence: &UiEvidence) {
    window.set_error_title(shared(&evidence.error_title));
    window.set_error_source(shared(&evidence.error_source));
    window.set_error_code(shared(&evidence.error_code));
    window.set_error_severity(shared(&evidence.error_severity));
    window.set_error_message(shared(&evidence.error_message));
    window.set_error_reason(shared(&evidence.error_reason));
    window.set_error_next_action(shared(&evidence.error_next_action));
    window.set_error_location(shared(&evidence.error_location));
}

fn shared(value: &str) -> SharedString {
    SharedString::from(value)
}

#[derive(Debug, Clone, Default, Serialize)]
struct UiEvidence {
    live_backend_connected: bool,
    evidence_mode_label: String,
    header_summary: String,
    policy_bundle_status: String,
    policy_bundle_summary: String,
    policy_decision_status: String,
    policy_decision_summary: String,
    authorization_status: String,
    authorization_summary: String,
    credential_boundary_status: String,
    credential_boundary_summary: String,
    credential_injection_status: String,
    credential_injection_summary: String,
    wrapper_execution_status: String,
    wrapper_execution_summary: String,
    audit_status: String,
    audit_summary: String,
    state_status: String,
    state_summary: String,
    request_status: String,
    request_summary: String,
    validation_status: String,
    validation_summary: String,
    bundle_timeline_status: String,
    bundle_timeline_summary: String,
    policy_timeline_status: String,
    policy_timeline_summary: String,
    authorization_timeline_status: String,
    authorization_timeline_summary: String,
    credential_boundary_timeline_status: String,
    credential_boundary_timeline_summary: String,
    credential_injection_timeline_status: String,
    credential_injection_timeline_summary: String,
    dispatch_status: String,
    dispatch_summary: String,
    wrapper_timeline_status: String,
    wrapper_timeline_summary: String,
    audit_timeline_status: String,
    audit_timeline_summary: String,
    state_timeline_status: String,
    state_timeline_summary: String,
    error_title: String,
    error_source: String,
    error_code: String,
    error_severity: String,
    error_message: String,
    error_reason: String,
    error_next_action: String,
    error_location: String,
}

impl UiEvidence {
    fn from_runtime_output(output: aegis::runtime::local::LocalRuntimeOutput) -> Self {
        let evidence_json = to_json_value(&output);
        let error_report = output.error_report.as_ref();
        let live_backend_connected = error_report.is_none();

        let mut evidence = Self::default();
        evidence.apply_header(live_backend_connected);
        evidence.apply_status_cards(&evidence_json, live_backend_connected);
        evidence.apply_timeline(&evidence_json);
        evidence.apply_error(error_report);
        evidence
    }

    fn apply_header(&mut self, live_backend_connected: bool) {
        self.live_backend_connected = live_backend_connected;
        self.evidence_mode_label = evidence_mode_label(live_backend_connected);
        self.header_summary = header_summary(live_backend_connected);
    }

    fn apply_status_cards(&mut self, json: &serde_json::Value, live_backend_connected: bool) {
        self.policy_bundle_status = display_value(json, "/policy_bundle/verification_status");
        self.policy_bundle_summary = policy_bundle_summary(live_backend_connected);
        self.policy_decision_status = display_value(json, "/policy_evaluation/decision");
        self.policy_decision_summary = policy_decision_summary(json);
        self.authorization_status =
            display_value(json, "/execution_authorization/authorization_status");
        self.authorization_summary = authorization_summary(json);
        self.credential_boundary_status =
            display_value(json, "/credential_boundary/credential_boundary_status");
        self.credential_boundary_summary = credential_boundary_summary(json);
        self.credential_injection_status = credential_injection_status(json);
        self.credential_injection_summary = credential_injection_summary(json);
        self.wrapper_execution_status = display_value(json, "/wrapper_execution/wrapper_status");
        self.wrapper_execution_summary = wrapper_execution_summary(json);
        self.audit_status = "Recorded".to_string();
        self.audit_summary =
            "In-memory audit evidence returned; no UI audit log write.".to_string();
        self.state_status = display_value(json, "/execution_lifecycle/execution_state");
        self.state_summary = state_summary(json);
    }

    fn apply_timeline(&mut self, json: &serde_json::Value) {
        self.request_status = "Received".to_string();
        self.request_summary = "Fixed health.check request supplied by backend.".to_string();
        self.validation_status = validation_status(json);
        self.validation_summary = "Backend validation accepted the fixed request.".to_string();
        self.bundle_timeline_status = display_value(json, "/policy_bundle/verification_status");
        self.bundle_timeline_summary =
            "Local development bundle verification evidence.".to_string();
        self.policy_timeline_status = display_value(json, "/policy_evaluation/decision");
        self.policy_timeline_summary = policy_decision_summary(json);
        self.authorization_timeline_status =
            display_value(json, "/execution_authorization/authorization_status");
        self.authorization_timeline_summary = authorization_summary(json);
        self.credential_boundary_timeline_status =
            display_value(json, "/credential_boundary/credential_boundary_status");
        self.credential_boundary_timeline_summary = credential_boundary_summary(json);
        self.credential_injection_timeline_status = credential_injection_status(json);
        self.credential_injection_timeline_summary = credential_injection_summary(json);
        self.dispatch_status = dispatch_status(json);
        self.dispatch_summary = "Registered health.check wrapper selected by backend.".to_string();
        self.wrapper_timeline_status = display_value(json, "/wrapper_execution/wrapper_status");
        self.wrapper_timeline_summary = wrapper_execution_summary(json);
        self.audit_timeline_status = "Recorded".to_string();
        self.audit_timeline_summary = "Audit evidence returned in runtime output.".to_string();
        self.state_timeline_status = display_value(json, "/execution_lifecycle/execution_state");
        self.state_timeline_summary = state_summary(json);
    }

    fn apply_error(&mut self, error_report: Option<&GatewayErrorReport>) {
        self.error_title = error_title(error_report);
        self.error_source = error_source(error_report);
        self.error_code = error_field(error_report, |report| enum_json(&report.code));
        self.error_severity = error_field(error_report, |report| enum_json(&report.severity));
        self.error_message = error_field(error_report, |report| report.message.clone());
        self.error_reason = error_field(error_report, |report| report.reason.clone());
        self.error_next_action = error_field(error_report, |report| {
            report.next_action.as_str().to_string()
        });
        self.error_location = error_field(error_report, |report| enum_json(&report.location));
    }
}

fn to_json_value(output: &aegis::runtime::local::LocalRuntimeOutput) -> serde_json::Value {
    serde_json::to_value(output).unwrap_or_else(|_| serde_json::json!({}))
}

fn display_value(json: &serde_json::Value, pointer: &str) -> String {
    json.pointer(pointer)
        .and_then(serde_json::Value::as_str)
        .map(display_label)
        .unwrap_or_else(|| "Not available".to_string())
}

fn display_label(value: &str) -> String {
    match value {
        "allow" => "Allowed".to_string(),
        "deny" => "Denied".to_string(),
        "pending_approval" => "Pending approval".to_string(),
        _ => value
            .split('_')
            .filter(|part| !part.is_empty())
            .map(capitalize)
            .collect::<Vec<_>>()
            .join(" "),
    }
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn evidence_mode_label(live_backend_connected: bool) -> String {
    if live_backend_connected {
        "Live backend health.check evidence".to_string()
    } else {
        "Error evidence; sample fallback remains labeled".to_string()
    }
}

fn header_summary(live_backend_connected: bool) -> String {
    if live_backend_connected {
        "Live read-only health.check evidence from the backend.".to_string()
    } else {
        "Live evidence unavailable; showing normalized backend error evidence.".to_string()
    }
}

fn policy_bundle_summary(live_backend_connected: bool) -> String {
    if live_backend_connected {
        "Checksum and signature verified by backend.".to_string()
    } else {
        "Bundle verification did not produce trusted live evidence.".to_string()
    }
}

fn policy_decision_summary(json: &serde_json::Value) -> String {
    match json
        .pointer("/policy_evaluation/policy_rule_id")
        .and_then(serde_json::Value::as_str)
    {
        Some(rule_id) => format!("Backend matched policy rule {rule_id}."),
        None => "No policy rule evidence available.".to_string(),
    }
}

fn authorization_summary(json: &serde_json::Value) -> String {
    match json
        .pointer("/execution_authorization/authorization_id")
        .and_then(serde_json::Value::as_str)
    {
        Some(authorization_id) => format!("Authorization evidence {authorization_id}."),
        None => "Authorization was not reached or not available.".to_string(),
    }
}

fn credential_boundary_summary(json: &serde_json::Value) -> String {
    match json
        .pointer("/credential_boundary/credential_required")
        .and_then(serde_json::Value::as_bool)
    {
        Some(false) => "health.check explicitly requires no credentials.".to_string(),
        Some(true) => "Credential class compatibility checked by backend.".to_string(),
        None => "Credential boundary evidence not available.".to_string(),
    }
}

fn credential_injection_status(json: &serde_json::Value) -> String {
    if json.pointer("/credential_injection").is_none() {
        return "Not required".to_string();
    }

    display_value(json, "/credential_injection/credential_injection_status")
}

fn credential_injection_summary(json: &serde_json::Value) -> String {
    if json.pointer("/credential_injection").is_none() {
        return "No credential handle issued for health.check.".to_string();
    }

    "Safe credential handle reference returned by backend.".to_string()
}

fn wrapper_execution_summary(json: &serde_json::Value) -> String {
    match json
        .pointer("/wrapper_execution/wrapper_result_summary/status")
        .and_then(serde_json::Value::as_str)
    {
        Some(status) => format!("health.check reported {status}."),
        None => "Wrapper execution evidence not available.".to_string(),
    }
}

fn state_summary(json: &serde_json::Value) -> String {
    let transitions = json
        .pointer("/execution_lifecycle/transitions")
        .and_then(serde_json::Value::as_array)
        .map(Vec::len)
        .unwrap_or(0);

    format!("{transitions} lifecycle transitions returned.")
}

fn validation_status(json: &serde_json::Value) -> String {
    if json.pointer("/response/request_id").is_some() {
        "Valid".to_string()
    } else {
        "Not available".to_string()
    }
}

fn dispatch_status(json: &serde_json::Value) -> String {
    if json.pointer("/wrapper_execution").is_some() {
        "Dispatched".to_string()
    } else {
        "Not reached".to_string()
    }
}

fn error_title(report: Option<&GatewayErrorReport>) -> String {
    if report.is_some() {
        "Live Normalized Error".to_string()
    } else {
        "Live Error Evidence".to_string()
    }
}

fn error_source(report: Option<&GatewayErrorReport>) -> String {
    if report.is_some() {
        "Backend-normalized error_report evidence".to_string()
    } else {
        "No live backend error_report present".to_string()
    }
}

fn error_field(
    report: Option<&GatewayErrorReport>,
    read: impl FnOnce(&GatewayErrorReport) -> String,
) -> String {
    report
        .map(read)
        .unwrap_or_else(|| "Not available".to_string())
}

fn enum_json(value: &impl Serialize) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(display_label))
        .unwrap_or_else(|| "Not available".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_check_command_returns_live_evidence() {
        let evidence = get_health_check_evidence();

        assert!(evidence.live_backend_connected);
        assert_eq!(
            evidence.evidence_mode_label,
            "Live backend health.check evidence"
        );
        assert_eq!(evidence.policy_bundle_status, "Verified");
        assert_eq!(evidence.policy_decision_status, "Allowed");
        assert_eq!(evidence.wrapper_execution_status, "Executed");
        assert_eq!(evidence.credential_injection_status, "Not required");
    }

    #[test]
    fn missing_bundle_returns_normalized_error_evidence() {
        let evidence = live_health_check_evidence(Path::new("missing-policy-bundle"));

        assert!(!evidence.live_backend_connected);
        assert_eq!(evidence.error_title, "Live Normalized Error");
        assert_ne!(evidence.error_code, "Not available");
        assert_ne!(evidence.error_message, "Not available");
        assert_ne!(evidence.error_reason, "Not available");
        assert_ne!(evidence.error_next_action, "Not available");
        assert_ne!(evidence.error_location, "Not available");
    }

    #[test]
    fn health_check_evidence_does_not_issue_credential_handle() {
        let evidence = get_health_check_evidence();

        assert_eq!(evidence.credential_boundary_status, "Satisfied");
        assert_eq!(evidence.credential_injection_status, "Not required");
        assert!(evidence
            .credential_injection_summary
            .contains("No credential handle"));
    }
}
