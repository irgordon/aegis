use std::process::Command;

use serde_json::Value;

#[test]
fn gateway_help_returns_success() {
    let output = run_gateway(["--help"]);

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let help = stdout_text(&output);
    assert_help_content(&help);
}

#[test]
fn gateway_short_help_returns_success() {
    let output = run_gateway(["-h"]);

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let help = stdout_text(&output);
    assert_help_content(&help);
}

#[test]
fn gateway_help_keeps_smoke_test_discoverable() {
    let help = stdout_text(&run_gateway(["--help"]));

    assert!(help.contains("Run the bundled smoke test"));
    assert!(help.contains("./bin/aegis-gateway \\"));
    assert!(help.contains("--bundle policy-bundles/local-dev \\"));
    assert!(help.contains("examples/health-check-request.json"));
    assert!(help.contains("README.md"));
}

#[test]
fn gateway_help_does_not_expose_internal_only_details() {
    let help = stdout_text(&run_gateway(["--help"]));
    let blocked = [
        "local_gateway_mvp",
        "wrapper_context",
        "policy_rule_id",
        "audit_record",
        "credential_handle",
        "sandbox.note.write",
        "mutation wrappers",
        "approval workflow",
        "replay execution",
    ];

    for value in blocked {
        assert!(
            !help.contains(value),
            "help output should not expose `{value}`"
        );
    }
}

#[test]
fn invalid_arguments_continue_returning_structured_errors() {
    let output = run_gateway(["--unknown"]);

    assert!(!output.status.success());

    let value: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("invalid argument stdout should be JSON: {error}"));

    assert_eq!(value["code"], "runtime_io_failed");
    assert_eq!(value["location"], "runtime_io");
    assert!(value["message"].is_string());
    assert!(value["reason"].is_string());
    assert!(value["next_action"].is_string());
    assert!(!stdout_text(&output).contains("AEGIS Gateway"));
}

fn assert_help_content(help: &str) {
    for required in [
        "AEGIS Gateway",
        "A governed local execution gateway.",
        "Usage:",
        "aegis-gateway [OPTIONS] <REQUEST>",
        "Common tasks:",
        "Options:",
        "Developer Preview:",
        "Unsigned release. Not production-ready.",
        "Documentation:",
        "README.md",
    ] {
        assert!(
            help.contains(required),
            "help output should contain `{required}`"
        );
    }
}

fn run_gateway<const N: usize>(args: [&str; N]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_aegis-gateway"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("gateway command should run: {error}"))
}

fn stdout_text(output: &std::process::Output) -> String {
    String::from_utf8(output.stdout.clone())
        .unwrap_or_else(|error| panic!("stdout should be utf8: {error}"))
}
