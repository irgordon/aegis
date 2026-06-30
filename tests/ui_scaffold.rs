use std::{collections::BTreeSet, fs, path::Path};

use serde_json::Value;

const DESKTOP_ENTRYPOINT: &str = "src-tauri/src/main.rs";
const SAMPLE_EVIDENCE: &str = "src-tauri/ui/sample_evidence.json";
const SLINT_UI: &str = "src-tauri/ui/main.slint";
const TAURI_CONFIG: &str = "src-tauri/tauri.conf.json";

#[test]
fn desktop_scaffold_files_exist() {
    assert!(Path::new("src-tauri/Cargo.toml").is_file());
    assert!(Path::new("src-tauri/build.rs").is_file());
    assert!(Path::new(DESKTOP_ENTRYPOINT).is_file());
    assert!(Path::new(SAMPLE_EVIDENCE).is_file());
    assert!(Path::new(SLINT_UI).is_file());
    assert!(Path::new(TAURI_CONFIG).is_file());
}

#[test]
fn slint_landing_screen_states_ui_boundary() {
    let slint_ui = read(SLINT_UI);

    assert!(slint_ui.contains("AEGIS"));
    assert!(slint_ui.contains("PRE-ALPHA"));
    assert!(slint_ui.contains("Backend evidence drives this UI"));
    assert!(slint_ui.contains("The UI is an operator surface, not an authority boundary."));
    assert!(slint_ui.contains("sample evidence only"));
}

#[test]
fn desktop_entrypoint_does_not_import_backend_execution() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let forbidden_imports = [
        "aegis::gateway",
        "aegis::runtime",
        "aegis::policy",
        "aegis::auth",
        "aegis::audit",
        "aegis::state",
        "aegis::wrappers",
    ];

    for forbidden in forbidden_imports {
        assert!(
            !entrypoint.contains(forbidden),
            "desktop scaffold must not import backend execution module {forbidden}"
        );
    }
}

#[test]
fn desktop_entrypoint_defines_no_ipc_commands() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);

    assert!(!entrypoint.contains("#[tauri::command]"));
    assert!(!entrypoint.contains(".invoke_handler("));
}

#[test]
fn tauri_config_uses_no_frontend_framework_stack() {
    let tauri_config = read(TAURI_CONFIG);
    let forbidden_terms = ["react", "vite", "dashboard"];

    for forbidden in forbidden_terms {
        assert!(
            !tauri_config.to_lowercase().contains(forbidden),
            "Tauri config must not introduce {forbidden}"
        );
    }
}

#[test]
fn sample_evidence_has_expected_timeline_stages() {
    let evidence = sample_evidence();
    let stages = timeline_field_set(&evidence, "stage");

    for expected in expected_timeline_stages() {
        assert!(
            stages.contains(expected),
            "sample timeline must include {expected}"
        );
    }
}

#[test]
fn slint_ui_renders_sample_timeline_stages() {
    let slint_ui = read(SLINT_UI);

    for expected in expected_timeline_stages() {
        assert!(
            slint_ui.contains(expected),
            "Slint timeline must render {expected}"
        );
    }
}

#[test]
fn sample_evidence_has_status_cards() {
    let evidence = sample_evidence();
    let card_titles = status_card_field_set(&evidence, "title");

    for expected in expected_status_cards() {
        assert!(
            card_titles.contains(expected),
            "sample status cards must include {expected}"
        );
    }
}

#[test]
fn slint_ui_renders_status_card_labels() {
    let slint_ui = read(SLINT_UI);

    for expected in expected_status_cards() {
        assert!(
            slint_ui.contains(expected),
            "Slint status card grid must render {expected}"
        );
    }
}

#[test]
fn normalized_error_fields_are_present() {
    let evidence = sample_evidence();
    let error = evidence
        .get("normalized_error")
        .and_then(Value::as_object)
        .expect("normalized_error object should exist");
    let slint_ui = read(SLINT_UI);

    for field in [
        "code",
        "severity",
        "message",
        "reason",
        "next_action",
        "location",
    ] {
        assert!(error.contains_key(field), "normalized error needs {field}");
    }

    for rendered in [
        "policy_denied",
        "HIGH",
        "Request was denied by policy.",
        "The requested action did not match an allowed policy rule.",
        "Review the policy decision and request context.",
        "policy_evaluation",
    ] {
        assert!(
            slint_ui.contains(rendered),
            "Slint error card must render {rendered}"
        );
    }
}

#[test]
fn sample_evidence_sources_match_ui_contract_sources() {
    let evidence = sample_evidence();
    let mut sources = timeline_field_set(&evidence, "evidence_source");
    sources.extend(status_card_field_set(&evidence, "evidence_source"));

    for expected in expected_evidence_sources() {
        assert!(
            sources.contains(expected),
            "sample evidence must reference {expected}"
        );
    }
}

#[test]
fn navigation_labels_remain_static_non_authoritative() {
    let slint_ui = read(SLINT_UI);

    for label in [
        "Overview",
        "Executions",
        "Audit",
        "State",
        "Recovery",
        "Policy",
        "Settings",
    ] {
        assert!(slint_ui.contains(label), "navigation label {label} missing");
    }

    assert!(!slint_ui.contains("clicked =>"));
    assert!(!slint_ui.contains("TouchArea"));
}

#[test]
fn sample_ui_files_contain_no_secret_like_markers() {
    let combined = format!("{}\n{}", read(SAMPLE_EVIDENCE), read(SLINT_UI)).to_lowercase();

    for forbidden in secret_like_markers() {
        assert!(
            !combined.contains(forbidden),
            "sample UI evidence must not contain {forbidden}"
        );
    }
}

fn sample_evidence() -> Value {
    serde_json::from_str(&read(SAMPLE_EVIDENCE)).expect("sample evidence should be valid JSON")
}

fn timeline_field_set(evidence: &Value, field: &str) -> BTreeSet<String> {
    evidence_string_set(evidence, &["successful_execution", "timeline"], field)
}

fn status_card_field_set(evidence: &Value, field: &str) -> BTreeSet<String> {
    evidence_string_set(evidence, &["successful_execution", "status_cards"], field)
}

fn evidence_string_set(evidence: &Value, path: &[&str], field: &str) -> BTreeSet<String> {
    let mut current = evidence;

    for segment in path {
        current = current
            .get(segment)
            .expect("sample evidence path should exist");
    }

    current
        .as_array()
        .expect("sample evidence path should be an array")
        .iter()
        .map(|entry| {
            entry
                .get(field)
                .and_then(Value::as_str)
                .expect("sample evidence field should be a string")
                .to_string()
        })
        .collect()
}

fn expected_timeline_stages() -> [&'static str; 13] {
    [
        "Request",
        "Validation",
        "Policy Bundle",
        "Policy Decision",
        "Authorization",
        "Credential Boundary",
        "Credential Injection",
        "Wrapper Dispatch",
        "Wrapper Execution",
        "Audit",
        "State",
        "Recovery Inspection",
        "Recovery Plan",
    ]
}

fn expected_status_cards() -> [&'static str; 10] {
    [
        "Policy Bundle",
        "Policy Decision",
        "Authorization",
        "Credential Boundary",
        "Credential Injection",
        "Wrapper Execution",
        "Audit Log",
        "State Log",
        "Recovery Inspection",
        "Recovery Plan",
    ]
}

fn expected_evidence_sources() -> [&'static str; 11] {
    [
        "response",
        "policy_bundle",
        "policy_evaluation",
        "execution_authorization",
        "credential_boundary",
        "credential_injection",
        "wrapper_execution",
        "audit_record",
        "execution_lifecycle",
        "recovery_inspection_report",
        "recovery_plan_report",
    ]
}

fn secret_like_markers() -> [&'static str; 8] {
    [
        "password",
        "token",
        "secret",
        "private_key",
        "api_key",
        "credential_value",
        "authorization_token",
        "begin private key",
    ]
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{path} should be readable: {error}"))
}
