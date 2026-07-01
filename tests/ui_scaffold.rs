use std::{collections::BTreeSet, fs, path::Path, process::Command};

use serde_json::Value;

const DESKTOP_ENTRYPOINT: &str = "src-tauri/src/main.rs";
const SAMPLE_EVIDENCE: &str = "src-tauri/ui/sample_evidence.json";
const SLINT_UI: &str = "src-tauri/ui/main.slint";
const TAURI_CARGO: &str = "src-tauri/Cargo.toml";
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
    assert!(read(DESKTOP_ENTRYPOINT).contains("Live backend health.check evidence"));
    assert!(slint_ui.contains("Sample evidence fallback"));
}

#[test]
fn slint_ui_uses_v040_palette_tokens() {
    let slint_ui = read(SLINT_UI);

    assert!(slint_ui.contains("export global AegisPalette"));

    for color in ["#0D3B66", "#FAF0CA", "#F4D35E", "#EE964B", "#F95738"] {
        assert!(
            slint_ui.contains(color),
            "Slint UI must include documented v0.4.0 palette color {color}"
        );
    }

    for token in [
        "AegisPalette.primary_dark",
        "AegisPalette.surface",
        "AegisPalette.attention",
        "AegisPalette.secondary_accent",
        "AegisPalette.critical",
    ] {
        assert!(
            slint_ui.contains(token),
            "Slint UI must reuse palette token {token}"
        );
    }
}

#[test]
fn slint_ui_uses_serif_first_typography_without_font_assets() {
    let slint_ui = read(SLINT_UI);
    let slint_ui_lower = slint_ui.to_lowercase();

    assert!(slint_ui.contains("export global AegisTypography"));
    assert!(slint_ui.contains("serif_first"));
    assert!(slint_ui.contains("font-family: AegisTypography.serif_first"));
    assert_absent(
        &slint_ui_lower,
        [
            "@font-face",
            ".ttf",
            ".otf",
            "download font",
            "bundled font",
        ],
    );
}

#[test]
fn slint_ui_preserves_release_posture_labels() {
    let slint_ui = read(SLINT_UI);
    let slint_ui_lower = slint_ui.to_lowercase();

    for label in ["PRE-ALPHA", "LOCAL-ONLY", "v0.4.0"] {
        assert!(
            slint_ui.contains(label),
            "Slint UI must preserve release posture label {label}"
        );
    }

    for label in [
        "developer-oriented",
        "pre-alpha",
        "local-only",
        "not production-ready",
    ] {
        assert!(
            slint_ui_lower.contains(label),
            "Slint UI must preserve release posture wording {label}"
        );
    }
}

#[test]
fn slint_ui_preserves_visual_evidence_labels() {
    let combined = format!("{}\n{}", read(SLINT_UI), read(DESKTOP_ENTRYPOINT));
    let combined_lower = combined.to_lowercase();

    for label in [
        "live backend health.check evidence",
        "sample evidence",
        "error evidence",
        "not available",
    ] {
        assert!(
            combined_lower.contains(label),
            "UI evidence labels must include {label}"
        );
    }

    assert!(combined.contains("Sample evidence remains labeled"));
}

#[test]
fn desktop_entrypoint_imports_only_local_runtime_bridge() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let forbidden_imports = [
        "aegis::gateway",
        "aegis::policy",
        "aegis::auth",
        "aegis::audit",
        "aegis::state",
        "aegis::wrappers",
    ];

    assert!(entrypoint.contains("runtime::local::process_local_gateway_request"));

    for forbidden in forbidden_imports {
        assert!(
            !entrypoint.contains(forbidden),
            "desktop IPC bridge must not import broad backend execution module {forbidden}"
        );
    }
}

#[test]
fn desktop_entrypoint_defines_one_read_only_ipc_command() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let command_names = tauri_command_names(&entrypoint);

    assert!(entrypoint.contains("#[tauri::command]"));
    assert!(entrypoint.contains("fn get_health_check_evidence() -> UiEvidence"));
    assert!(entrypoint.contains("tauri::generate_handler![get_health_check_evidence]"));
    assert_eq!(
        command_names,
        BTreeSet::from(["get_health_check_evidence".to_string()])
    );
    assert_eq!(entrypoint.matches("#[tauri::command]").count(), 1);
    assert_eq!(entrypoint.matches("generate_handler!").count(), 1);
}

#[test]
fn ipc_command_accepts_no_arbitrary_ui_input() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let command_signature = "fn get_health_check_evidence() -> UiEvidence";
    let command_line = tauri_command_signature(&entrypoint, "get_health_check_evidence");
    let forbidden_inputs = [
        "request_json",
        "request_path",
        "bundle_path:",
        "audit_log_path",
        "state_log_path",
        "sandbox_dir",
        "wrapper_name:",
        "policy_bundle_path",
        "credential_class",
    ];

    assert!(entrypoint.contains(command_signature));
    assert!(
        command_line.contains("get_health_check_evidence()"),
        "IPC command must remain input-free"
    );

    for forbidden in forbidden_inputs {
        assert!(
            !entrypoint.contains(&format!("get_health_check_evidence({forbidden}")),
            "IPC command must not accept {forbidden}"
        );
    }
}

#[test]
fn ipc_handler_registers_no_general_execution_commands() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let handler = tauri_generate_handler_line(&entrypoint);
    let forbidden_commands = [
        "execute_request",
        "run_wrapper",
        "load_bundle",
        "inspect_state",
        "plan_recovery",
        "replay",
        "resume",
        "approve",
        "authorize",
        "issue_credentials",
        "inject_credentials",
        "sandbox_note_write",
    ];

    assert!(handler.contains("get_health_check_evidence"));
    assert_absent(&handler.to_lowercase(), forbidden_commands);
}

#[test]
fn ipc_command_uses_fixed_health_check_evidence_source() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);

    assert!(entrypoint.contains("HealthCheckRequest.json"));
    assert!(entrypoint.contains("policy-bundles/local-dev"));
    assert!(entrypoint.contains("artifact_policy_bundle_path()"));
    assert!(entrypoint.contains("development_policy_bundle_path()"));
    assert!(entrypoint.contains("process_local_gateway_request"));
    assert!(!entrypoint.contains("SandboxNoteWriteRequest.json"));
    assert!(!entrypoint.contains("sandbox.note.write"));
    assert!(!entrypoint.contains("--sandbox-dir"));
}

#[test]
fn desktop_policy_bundle_resolution_prefers_artifact_path() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let resolver = function_body(&entrypoint, "resolve_policy_bundle_path");

    assert!(entrypoint.contains("std::env::current_exe()"));
    assert!(entrypoint.contains("artifact_policy_bundle_path_for_executable"));
    assert!(resolver.contains("if artifact_path.is_dir()"));
    assert!(resolver.contains("} else if development_path.is_dir()"));
    assert!(
        resolver.find("artifact_path.is_dir()") < resolver.find("development_path.is_dir()"),
        "artifact-relative policy bundle path must be checked before source fallback"
    );
    assert!(!resolver.contains("CARGO_MANIFEST_DIR"));
}

#[test]
fn desktop_source_policy_bundle_path_is_development_fallback_only() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let command = function_body(&entrypoint, "get_health_check_evidence");
    let artifact_path = function_body(&entrypoint, "artifact_policy_bundle_path");
    let development_path = function_body(&entrypoint, "development_policy_bundle_path");

    assert!(!command.contains("CARGO_MANIFEST_DIR"));
    assert!(!artifact_path.contains("CARGO_MANIFEST_DIR"));
    assert!(development_path.contains("CARGO_MANIFEST_DIR"));
    assert!(development_path.contains("../examples/policy-bundles/local-dev"));
}

#[test]
fn ipc_bridge_does_not_accept_runtime_paths_from_ui() {
    let entrypoint = read(DESKTOP_ENTRYPOINT);
    let command_signature = tauri_command_signature(&entrypoint, "get_health_check_evidence");
    let forbidden_path_inputs = [
        "bundle",
        "manifest",
        "policy",
        "risk_matrix",
        "audit",
        "state",
        "sandbox",
        "wrapper",
        "path",
    ];

    assert!(command_signature.starts_with("fn get_health_check_evidence() -> UiEvidence"));
    assert_absent(&command_signature.to_lowercase(), forbidden_path_inputs);
}

#[test]
fn ipc_bridge_uses_no_filesystem_or_log_loading() {
    let entrypoint = read(DESKTOP_ENTRYPOINT).to_lowercase();
    let forbidden_loading = [
        "std::fs",
        "fs::",
        "file::open",
        "read_to_string",
        "audit.jsonl",
        "state.jsonl",
        "sandbox_dir",
        "env::args",
        "args()",
    ];

    assert!(entrypoint.contains("include_str!"));
    assert_absent(&entrypoint, forbidden_loading);
}

#[test]
fn ipc_bridge_invokes_only_fixed_local_health_check_runtime_path() {
    let entrypoint = read(DESKTOP_ENTRYPOINT).to_lowercase();
    let forbidden_runtime_paths = [
        "process_local_gateway_request_with_context",
        "process_local_gateway_request_with_wrapper_registry",
        "sandboxnotewrite",
        "sandbox.note.write",
        "recoveryinspector",
        "recoveryplangenerator",
        "auditwriter",
        "executionstatewriter",
    ];

    assert!(entrypoint.contains("process_local_gateway_request("));
    assert!(entrypoint.contains("healthcheckrequest.json"));
    assert_absent(&entrypoint, forbidden_runtime_paths);
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
fn sample_evidence_is_static_and_not_live() {
    let evidence = sample_evidence();
    let slint_ui = read(SLINT_UI);

    assert_eq!(
        evidence.get("sample_kind").and_then(Value::as_str),
        Some("static_ui_evidence")
    );
    assert_eq!(
        evidence
            .get("live_backend_connected")
            .and_then(Value::as_bool),
        Some(false)
    );
    assert!(slint_ui.contains("Sample evidence fallback"));
    assert!(slint_ui.contains("Fixture-backed operator evidence rendering"));
    assert!(read(DESKTOP_ENTRYPOINT).contains("Live backend health.check evidence"));
    assert!(slint_ui.contains("PRE-ALPHA"));
}

#[test]
fn live_and_sample_evidence_labels_remain_explicit() {
    let slint_ui = read(SLINT_UI).to_lowercase();
    let entrypoint = read(DESKTOP_ENTRYPOINT).to_lowercase();

    assert!(slint_ui.contains("sample evidence fallback"));
    assert!(slint_ui.contains("sample only"));
    assert!(slint_ui.contains("fixed read-only live health.check evidence"));
    assert!(entrypoint.contains("live backend health.check evidence"));
    assert!(entrypoint.contains("error evidence; sample fallback remains labeled"));
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
fn sample_evidence_timeline_order_is_deterministic() {
    let evidence = sample_evidence();

    assert_eq!(
        ordered_timeline_stages(&evidence),
        expected_timeline_stages()
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
}

#[test]
fn sample_evidence_status_card_order_is_deterministic() {
    let evidence = sample_evidence();

    assert_eq!(
        ordered_status_cards(&evidence),
        expected_status_cards()
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
}

#[test]
fn cli_health_check_behavior_is_unchanged_by_desktop_ipc() {
    let output = gateway_cli_json(&[
        "--bundle",
        "examples/policy-bundles/local-dev",
        "schemas/examples/valid/HealthCheckRequest.json",
    ]);

    assert_eq!(json_string(&output, "/response/status"), "allowed");
    assert_eq!(
        json_string(&output, "/policy_bundle/verification_status"),
        "verified"
    );
    assert_eq!(json_string(&output, "/policy_evaluation/decision"), "allow");
    assert_eq!(
        json_string(&output, "/wrapper_execution/wrapper_name"),
        "health.check"
    );
    assert_eq!(
        json_string(&output, "/wrapper_execution/wrapper_status"),
        "executed"
    );
    assert_eq!(
        json_string(&output, "/execution_lifecycle/execution_state"),
        "completed"
    );
}

#[test]
fn desktop_crate_depends_on_backend_without_server_or_web_frameworks() {
    let cargo = read(TAURI_CARGO).to_lowercase();
    let forbidden_dependencies = [
        "axum", "hyper", "reqwest", "warp", "rocket", "actix", "vite", "react",
    ];

    assert!(cargo.contains("aegis = { path = \"..\" }"));
    assert_absent(&cargo, forbidden_dependencies);
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
fn slint_ui_preserves_normalized_error_field_labels() {
    let slint_ui = read(SLINT_UI);

    for label in ["Code", "Severity", "Message", "Reason", "Next", "Location"] {
        assert!(
            slint_ui.contains(&format!("detail_label: \"{label}\"")),
            "Slint error card must label {label}"
        );
    }
}

#[test]
fn normalized_error_text_is_plain_and_secret_safe() {
    let evidence = sample_evidence();
    let error_text = normalized_error_text(&evidence).to_lowercase();
    let forbidden_terms = [
        "panic",
        "stack backtrace",
        "serde_json::",
        "expected value at line",
        "raw request",
        "wrapper arguments",
        "note content",
        "shell command",
    ];

    assert!(!error_text.trim().is_empty());
    assert_absent(&error_text, forbidden_terms);
    assert_absent(&error_text, secret_like_markers());
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
fn recovery_plan_sample_remains_future_evaluation_only() {
    let evidence = sample_evidence();
    let combined = combined_sample_ui_text().to_lowercase();
    let forbidden_replay_claims = [
        "ready to replay",
        "replay available",
        "replay approved",
        "safe to replay",
        "replay now",
        "resume now",
    ];

    assert_eq!(
        successful_execution(&evidence)
            .get("recovery_plan_outcome")
            .and_then(Value::as_str),
        Some("not_recoverable_terminal")
    );
    assert_eq!(
        successful_execution(&evidence)
            .get("allowed_future_action")
            .and_then(Value::as_str),
        Some("none")
    );
    assert!(combined.contains("future evaluation only"));
    assert!(combined.contains("not replay execution"));
    assert_absent(&combined, forbidden_replay_claims);
}

#[test]
fn recovery_inspection_card_labels_are_present() {
    let slint_ui = read(SLINT_UI);

    for label in [
        "Sample Recovery Inspection",
        "Inspection available",
        "Execution",
        "Last state",
        "Completed",
        "Terminal",
        "Yes",
        "Recoverability",
        "Not recoverable",
        "does not read state logs",
    ] {
        assert!(
            slint_ui.contains(label),
            "recovery inspection card must render {label}"
        );
    }
}

#[test]
fn recovery_plan_card_labels_are_present() {
    let slint_ui = read(SLINT_UI);

    for label in [
        "Sample Recovery Plan",
        "Plan status",
        "Planned",
        "Outcome",
        "Future action",
        "No future action",
        "Reason",
        "Sample guidance only",
    ] {
        assert!(
            slint_ui.contains(label),
            "recovery plan card must render {label}"
        );
    }
}

#[test]
fn recovery_sample_fields_match_backend_recovery_semantics() {
    let evidence = sample_evidence();
    let inspection = recovery_inspection(&evidence);
    let plan = recovery_plan(&evidence);

    assert_eq!(
        inspection.get("inspection_status").and_then(Value::as_str),
        Some("inspected")
    );
    assert_eq!(
        inspection.get("last_known_state").and_then(Value::as_str),
        Some("completed")
    );
    assert_eq!(
        inspection.get("terminal_status").and_then(Value::as_str),
        Some("terminal")
    );
    assert_eq!(
        inspection
            .get("recoverability_status")
            .and_then(Value::as_str),
        Some("not_recoverable_terminal")
    );
    assert_eq!(
        plan.get("plan_outcome").and_then(Value::as_str),
        Some("not_recoverable_terminal")
    );
    assert_eq!(
        plan.get("allowed_future_action").and_then(Value::as_str),
        Some("none")
    );
}

#[test]
fn bounded_recovery_plan_outcomes_are_represented_safely() {
    let evidence = sample_evidence();
    let mappings = recovery_mapping_field_set(&evidence, "plan_outcomes", "value");
    let labels = recovery_mapping_field_set(&evidence, "plan_outcomes", "display_label");
    let slint_ui = read(SLINT_UI);

    for expected in expected_plan_outcomes() {
        assert!(
            mappings.contains(expected),
            "sample recovery mappings must include {expected}"
        );
    }

    for expected in [
        "Not recoverable",
        "Not recoverable: evidence corrupted",
        "Audit retry candidate",
        "Future evaluation only",
        "Inspection failed",
    ] {
        assert!(
            labels.contains(expected) || slint_ui.contains(expected),
            "sample UI must safely render {expected}"
        );
    }
}

#[test]
fn bounded_future_actions_are_represented_safely() {
    let evidence = sample_evidence();
    let mappings = recovery_mapping_field_set(&evidence, "future_actions", "value");
    let labels = recovery_mapping_field_set(&evidence, "future_actions", "display_label");
    let slint_ui = read(SLINT_UI);

    for expected in expected_future_actions() {
        assert!(
            mappings.contains(expected),
            "sample recovery future actions must include {expected}"
        );
    }

    for expected in [
        "No future action",
        "Audit retry only",
        "Future evaluation only",
        "Manual review only",
    ] {
        assert!(
            labels.contains(expected) || slint_ui.contains(expected),
            "sample UI must safely render {expected}"
        );
    }
}

#[test]
fn audit_retry_and_corrupted_recovery_labels_are_not_executed_or_recoverable() {
    let combined = combined_sample_ui_text().to_lowercase();
    let forbidden_recovery_claims = [
        "audit retry executed",
        "audit retry complete",
        "corrupted evidence is recoverable",
        "corrupted evidence recoverable",
        "safe to recover",
        "run recovery",
    ];

    assert!(combined.contains("audit retry candidate"));
    assert!(combined.contains("audit retry only"));
    assert!(combined.contains("not recoverable: evidence corrupted"));
    assert!(combined.contains("manual review only"));
    assert_absent(&combined, forbidden_recovery_claims);
}

#[test]
fn sample_recovery_evidence_is_clearly_sample_and_non_live() {
    let combined = combined_sample_ui_text().to_lowercase();

    assert!(combined.contains("sample evidence"));
    assert!(combined.contains("sample recovery inspection"));
    assert!(combined.contains("sample recovery plan"));
    assert!(combined.contains("does not read state logs"));
    assert!(combined.contains("live health.check does not inspect state logs"));
    assert!(combined.contains("live health.check does not plan recovery"));
    assert!(combined.contains("does not inspect live state"));
}

#[test]
fn desktop_scaffold_does_not_read_audit_state_or_recovery_files() {
    let entrypoint = read(DESKTOP_ENTRYPOINT).to_lowercase();
    let slint_ui = read(SLINT_UI).to_lowercase();
    let forbidden_runtime_loading = [
        "read_to_string",
        "audit.jsonl",
        "state.jsonl",
        "--inspect-state",
        "--plan-recovery",
        "executionrecoveryinspector",
        "recoveryplangenerator",
    ];

    assert_absent(&entrypoint, forbidden_runtime_loading);
    assert_absent(&slint_ui, forbidden_runtime_loading);
}

#[test]
fn sample_ui_does_not_imply_live_backend_or_runtime_control() {
    let combined = combined_sample_ui_text().to_lowercase();
    let forbidden_live_claims = [
        "live backend integration",
        "real-time execution",
        "active gateway control",
        "approved to run",
        "credential granted",
        "issuing credential",
        "dispatching live wrapper",
        "writing audit log",
    ];

    assert_absent(&combined, forbidden_live_claims);
    assert!(combined.contains("fixed read-only live health.check evidence"));
    assert!(combined.contains("labeled sample recovery evidence"));
}

#[test]
fn sample_ui_offers_no_authoritative_action_controls() {
    let slint_ui = read(SLINT_UI);
    let forbidden_controls = ["Button", "TouchArea", "clicked =>", "invoke("];
    let forbidden_action_labels = [
        "Run",
        "Approve",
        "Replay",
        "Resume",
        "Authorize",
        "Issue Credential",
        "Override",
        "Repair",
    ];

    assert_absent(&slint_ui, forbidden_controls);

    for label in forbidden_action_labels {
        assert!(
            !slint_ui.contains(&format!("\"{label}\"")),
            "sample UI must not expose {label} as an action label"
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

#[test]
fn sample_evidence_contains_no_executable_wrapper_parameters() {
    let sample_evidence = read(SAMPLE_EVIDENCE).to_lowercase();
    let forbidden_parameters = [
        "raw_request_payload",
        "wrapper_arguments",
        "note_content",
        "real_sandbox_path",
        "shell_command",
        "mutation_target",
        "subprocess",
    ];

    assert_absent(&sample_evidence, forbidden_parameters);
}

#[test]
fn sample_ui_contains_no_real_local_paths() {
    let combined = combined_sample_ui_text();
    let forbidden_paths = ["/Users/", "/home/", "C:\\", "audit.jsonl", "state.jsonl"];

    assert_absent(&combined, forbidden_paths);
}

#[test]
fn desktop_scaffold_has_no_http_or_server_behavior() {
    let cargo = read(TAURI_CARGO).to_lowercase();
    let entrypoint = read(DESKTOP_ENTRYPOINT).to_lowercase();
    let slint_ui = read(SLINT_UI).to_lowercase();
    let forbidden_cargo_deps = ["reqwest", "axum", "warp", "hyper", "websocket"];
    let forbidden_runtime_terms = ["tcplistener", ".bind(", "listen(", "http://", "https://"];

    assert_absent(&cargo, forbidden_cargo_deps);
    assert_absent(&entrypoint, forbidden_runtime_terms);
    assert_absent(&slint_ui, forbidden_runtime_terms);
}

#[test]
fn ui_direction_remains_tauri_slint_not_web_dashboard_or_tui() {
    let combined = format!(
        "{}\n{}\n{}",
        read(TAURI_CARGO),
        read(DESKTOP_ENTRYPOINT),
        read(SLINT_UI)
    )
    .to_lowercase();
    let forbidden_ui_directions = [
        "react",
        "vite",
        "web dashboard",
        "terminal dashboard",
        "tui",
        "html",
        "typescript",
    ];

    assert_absent(&combined, forbidden_ui_directions);
}

#[test]
fn ui_does_not_introduce_arbitrary_gateway_command_surfaces() {
    let combined = format!("{}\n{}", read(DESKTOP_ENTRYPOINT), read(SLINT_UI)).to_lowercase();
    let forbidden_terms = [
        "request json from ui",
        "choose wrapper",
        "select bundle",
        "policy override",
        "approval button",
        "replay button",
        "recovery button",
        "sandboxnote",
        "sandbox.note.write",
        "audit log path",
        "state log path",
    ];

    assert_absent(&combined, forbidden_terms);
    assert!(combined.contains("get_health_check_evidence"));
}

#[test]
fn ipc_boundary_has_no_approval_replay_recovery_or_credential_command_names() {
    let command_names = tauri_command_names(&read(DESKTOP_ENTRYPOINT))
        .into_iter()
        .collect::<Vec<_>>()
        .join("\n")
        .to_lowercase();
    let forbidden_terms = [
        "approve",
        "deny",
        "override",
        "break_glass",
        "authorize",
        "credential",
        "replay",
        "resume",
        "recover",
        "inspect",
        "plan",
        "sandbox",
        "write",
    ];

    assert_eq!(command_names, "get_health_check_evidence");
    assert_absent(&command_names, forbidden_terms);
}

fn sample_evidence() -> Value {
    serde_json::from_str(&read(SAMPLE_EVIDENCE)).expect("sample evidence should be valid JSON")
}

fn gateway_cli_json(args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_aegis-gateway"))
        .args(args)
        .output()
        .expect("gateway CLI should run");

    assert!(
        output.status.success(),
        "gateway CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    serde_json::from_slice(&output.stdout).expect("gateway CLI stdout should be valid JSON")
}

fn tauri_command_names(entrypoint: &str) -> BTreeSet<String> {
    entrypoint
        .lines()
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|window| command_name_after_attribute(window[0], window[1]))
        .collect()
}

fn command_name_after_attribute(attribute: &str, signature: &str) -> Option<String> {
    if !attribute.trim().starts_with("#[tauri::command]") {
        return None;
    }

    Some(function_name(signature).to_string())
}

fn function_name(signature: &str) -> &str {
    signature
        .trim()
        .strip_prefix("fn ")
        .and_then(|rest| rest.split_once('('))
        .map(|(name, _)| name)
        .expect("tauri command should be followed by a function")
}

fn tauri_command_signature(entrypoint: &str, command: &str) -> String {
    entrypoint
        .lines()
        .find(|line| line.trim().starts_with(&format!("fn {command}(")))
        .map(|line| line.trim().to_string())
        .expect("tauri command signature should exist")
}

fn tauri_generate_handler_line(entrypoint: &str) -> String {
    entrypoint
        .lines()
        .find(|line| line.contains("tauri::generate_handler!"))
        .map(|line| line.trim().to_string())
        .expect("tauri generate_handler line should exist")
}

fn function_body(content: &str, function_name: &str) -> String {
    let marker = format!("fn {function_name}");
    let start = content
        .find(&marker)
        .unwrap_or_else(|| panic!("{function_name} should exist"));
    let rest = &content[start..];
    let end = rest
        .find("\nfn ")
        .or_else(|| rest.find("\n#[cfg(test)]"))
        .unwrap_or(rest.len());

    rest[..end].to_string()
}

fn json_string<'a>(json: &'a Value, pointer: &str) -> &'a str {
    json.pointer(pointer)
        .and_then(Value::as_str)
        .expect("runtime JSON pointer should resolve to a string")
}

fn successful_execution(evidence: &Value) -> &Value {
    evidence
        .get("successful_execution")
        .expect("successful_execution should exist")
}

fn recovery_inspection(evidence: &Value) -> &Value {
    successful_execution(evidence)
        .get("recovery_inspection")
        .expect("recovery_inspection should exist")
}

fn recovery_plan(evidence: &Value) -> &Value {
    successful_execution(evidence)
        .get("recovery_plan")
        .expect("recovery_plan should exist")
}

fn ordered_timeline_stages(evidence: &Value) -> Vec<String> {
    ordered_evidence_strings(evidence, &["successful_execution", "timeline"], "stage")
}

fn ordered_status_cards(evidence: &Value) -> Vec<String> {
    ordered_evidence_strings(evidence, &["successful_execution", "status_cards"], "title")
}

fn timeline_field_set(evidence: &Value, field: &str) -> BTreeSet<String> {
    evidence_string_set(evidence, &["successful_execution", "timeline"], field)
}

fn status_card_field_set(evidence: &Value, field: &str) -> BTreeSet<String> {
    evidence_string_set(evidence, &["successful_execution", "status_cards"], field)
}

fn recovery_mapping_field_set(evidence: &Value, mapping: &str, field: &str) -> BTreeSet<String> {
    evidence_string_set(
        evidence,
        &["successful_execution", "recovery_display_mappings", mapping],
        field,
    )
}

fn ordered_evidence_strings(evidence: &Value, path: &[&str], field: &str) -> Vec<String> {
    evidence_array(evidence, path)
        .iter()
        .map(|entry| evidence_field(entry, field).to_string())
        .collect()
}

fn evidence_string_set(evidence: &Value, path: &[&str], field: &str) -> BTreeSet<String> {
    evidence_array(evidence, path)
        .iter()
        .map(|entry| evidence_field(entry, field).to_string())
        .collect()
}

fn evidence_array<'a>(evidence: &'a Value, path: &[&str]) -> &'a Vec<Value> {
    let mut current = evidence;

    for segment in path {
        current = current
            .get(segment)
            .expect("sample evidence path should exist");
    }

    current
        .as_array()
        .expect("sample evidence path should be an array")
}

fn evidence_field<'a>(entry: &'a Value, field: &str) -> &'a str {
    entry
        .get(field)
        .and_then(Value::as_str)
        .expect("sample evidence field should be a string")
}

fn normalized_error_text(evidence: &Value) -> String {
    let error = evidence
        .get("normalized_error")
        .and_then(Value::as_object)
        .expect("normalized_error object should exist");

    [
        "code",
        "severity",
        "message",
        "reason",
        "next_action",
        "location",
    ]
    .into_iter()
    .filter_map(|field| error.get(field).and_then(Value::as_str))
    .collect::<Vec<_>>()
    .join("\n")
}

fn combined_sample_ui_text() -> String {
    format!("{}\n{}", read(SAMPLE_EVIDENCE), read(SLINT_UI))
}

fn assert_absent<const N: usize>(haystack: &str, forbidden_terms: [&str; N]) {
    for forbidden in forbidden_terms {
        assert!(
            !haystack.contains(forbidden),
            "sample UI hardening must not contain {forbidden}"
        );
    }
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

fn expected_plan_outcomes() -> [&'static str; 5] {
    [
        "not_recoverable_terminal",
        "not_recoverable_corrupted",
        "candidate_for_audit_retry",
        "candidate_for_future_replay",
        "inspection_failed",
    ]
}

fn expected_future_actions() -> [&'static str; 4] {
    [
        "none",
        "audit_retry_only",
        "future_replay_evaluation_only",
        "manual_review_only",
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
