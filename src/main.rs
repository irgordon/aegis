use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use aegis::audit::{AuditSink, AuditWriter};
use aegis::error::GatewayErrorReport;
use aegis::runtime::local::{
    process_local_gateway_request_with_context, LocalRuntimeContext, LocalRuntimeOutput,
};
use aegis::state::{
    ExecutionRecoveryInspector, ExecutionStateLogContext, ExecutionStateSink, ExecutionStateWriter,
};
use serde::Serialize;

type RuntimeResult<T> = Result<T, Box<GatewayErrorReport>>;

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    match try_run() {
        Ok(code) => code,
        Err(error_report) => {
            print_structured_json(error_report.as_ref())
                .unwrap_or_else(|error| eprintln!("{error:?}"));
            1
        }
    }
}

fn try_run() -> RuntimeResult<i32> {
    match parse_cli_mode()? {
        LocalCliMode::InspectState(path) => inspect_state_log(&path),
        LocalCliMode::Run(args) => run_gateway(args),
    }
}

fn run_gateway(args: LocalRuntimeArgs) -> RuntimeResult<i32> {
    let input = read_input(args.request_path.as_deref())?;
    validate_state_log_path(args.state_log_path.as_deref())?;
    let mut output = process_local_gateway_request_with_context(
        &input,
        &args.bundle_path,
        LocalRuntimeContext {
            wrapper_context: None,
            sandbox_dir: args.sandbox_dir,
        },
    );

    match persist_audit_record(args.audit_log_path.as_deref(), &output) {
        Ok(()) => {
            let mut completed_output = output.clone();
            completed_output.mark_audited_completed();
            match persist_execution_state_log(args.state_log_path.as_deref(), &completed_output) {
                Ok(()) => {
                    print_structured_json(&completed_output)?;
                    Ok(0)
                }
                Err(error_report) => {
                    output.attach_error_report(*error_report);
                    print_structured_json(&output)?;
                    Ok(1)
                }
            }
        }
        Err(error_report) => {
            output.mark_audit_failed();
            output.attach_error_report(*error_report);
            if let Err(state_error_report) =
                persist_execution_state_log(args.state_log_path.as_deref(), &output)
            {
                output.attach_error_report(*state_error_report);
            }
            print_structured_json(&output)?;
            Ok(1)
        }
    }
}

fn inspect_state_log(path: &Path) -> RuntimeResult<i32> {
    let report = ExecutionRecoveryInspector::inspect_path(path);
    let has_errors = !report.inspection_errors.is_empty();
    print_structured_json(&report)?;
    Ok(if has_errors { 1 } else { 0 })
}

enum LocalCliMode {
    Run(LocalRuntimeArgs),
    InspectState(PathBuf),
}

struct LocalRuntimeArgs {
    bundle_path: PathBuf,
    audit_log_path: Option<PathBuf>,
    state_log_path: Option<PathBuf>,
    sandbox_dir: Option<PathBuf>,
    request_path: Option<PathBuf>,
}

fn parse_cli_mode() -> RuntimeResult<LocalCliMode> {
    let mut args = env::args().skip(1).peekable();

    if matches!(args.peek().map(String::as_str), Some("--inspect-state")) {
        return parse_inspection_args(args);
    }

    parse_runtime_args(args).map(LocalCliMode::Run)
}

fn parse_inspection_args(mut args: impl Iterator<Item = String>) -> RuntimeResult<LocalCliMode> {
    let _flag = args.next();
    let path = args.next().ok_or_else(runtime_usage_error)?;

    if args.next().is_some() {
        return Err(runtime_usage_error());
    }

    Ok(LocalCliMode::InspectState(PathBuf::from(path)))
}

fn parse_runtime_args(mut args: impl Iterator<Item = String>) -> RuntimeResult<LocalRuntimeArgs> {
    let mut bundle_path = None;
    let mut audit_log_path = None;
    let mut state_log_path = None;
    let mut sandbox_dir = None;
    let mut request_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = next_path_arg(&mut args, "--bundle")?,
            "--audit-log" => audit_log_path = next_path_arg(&mut args, "--audit-log")?,
            "--state-log" => state_log_path = next_path_arg(&mut args, "--state-log")?,
            "--sandbox-dir" => sandbox_dir = next_path_arg(&mut args, "--sandbox-dir")?,
            _ if request_path.is_none() => request_path = Some(PathBuf::from(arg)),
            _ => return Err(runtime_usage_error()),
        }
    }

    Ok(LocalRuntimeArgs {
        bundle_path: bundle_path.ok_or_else(runtime_usage_error)?,
        audit_log_path,
        state_log_path,
        sandbox_dir,
        request_path,
    })
}

fn validate_state_log_path(state_log_path: Option<&Path>) -> RuntimeResult<()> {
    if let Some(path) = state_log_path {
        ExecutionStateWriter::new(path.to_path_buf())
            .validate_writable()
            .map_err(|error| {
                boxed_report(GatewayErrorReport::execution_state_log_failed(&error, None))
            })?;
    }

    Ok(())
}

fn read_input(request_path: Option<&Path>) -> RuntimeResult<String> {
    match request_path {
        Some(path) => fs::read_to_string(path).map_err(|_| {
            boxed_report(GatewayErrorReport::runtime_io_failed(
                format!("The request file could not be read: {}.", path.display()),
                "request_file_read_failed",
            ))
        }),
        None => read_stdin(),
    }
}

fn read_stdin() -> RuntimeResult<String> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|_| {
        boxed_report(GatewayErrorReport::runtime_io_failed(
            "The runtime could not read JSON from stdin.",
            "stdin_read_failed",
        ))
    })?;
    Ok(input)
}

fn persist_audit_record(
    audit_log_path: Option<&Path>,
    output: &LocalRuntimeOutput,
) -> RuntimeResult<()> {
    if let Some(path) = audit_log_path {
        AuditWriter::new(path.to_path_buf())
            .append(&output.audit_record)
            .map_err(|error| {
                boxed_report(GatewayErrorReport::audit_persistence_failed(
                    &error,
                    &output.response,
                ))
            })?;
    }

    Ok(())
}

fn persist_execution_state_log(
    state_log_path: Option<&Path>,
    output: &LocalRuntimeOutput,
) -> RuntimeResult<()> {
    if let Some(path) = state_log_path {
        let context = execution_state_log_context(output);
        ExecutionStateWriter::new(path.to_path_buf())
            .append_lifecycle(&output.execution_lifecycle, &context)
            .map_err(|error| {
                boxed_report(GatewayErrorReport::execution_state_log_failed(
                    &error,
                    Some(&output.response),
                ))
            })?;
    }

    Ok(())
}

fn execution_state_log_context(output: &LocalRuntimeOutput) -> ExecutionStateLogContext {
    ExecutionStateLogContext {
        execution_id: output.response.execution_id.as_str().to_string(),
        request_id: output
            .response
            .request_id
            .as_ref()
            .map(|request_id| request_id.as_str().to_string()),
        tool_name: output
            .audit_record
            .tool_name
            .as_ref()
            .map(|tool_name| tool_name.as_str().to_string()),
        policy_bundle_id: output
            .policy_bundle
            .bundle
            .as_ref()
            .map(|bundle| bundle.0.as_str().to_string()),
        policy_rule_id: output
            .policy_evaluation
            .as_ref()
            .and_then(|evaluation| evaluation.policy_rule_id.as_ref())
            .map(|rule_id| rule_id.0.as_str().to_string()),
        wrapper_name: output
            .wrapper_execution
            .as_ref()
            .map(|wrapper| wrapper.wrapper_name.as_str().to_string())
            .or_else(|| {
                output
                    .execution_authorization
                    .as_ref()
                    .map(|authorization| authorization.binding.wrapper_name.as_str().to_string())
            }),
        wrapper_version: output
            .wrapper_execution
            .as_ref()
            .map(|wrapper| wrapper.wrapper_version.as_str().to_string())
            .or_else(|| {
                output
                    .execution_authorization
                    .as_ref()
                    .map(|authorization| authorization.binding.wrapper_version.as_str().to_string())
            }),
        authorization_id: output
            .execution_authorization
            .as_ref()
            .map(|authorization| authorization.authorization_id.as_str().to_string()),
        credential_boundary_status: output
            .credential_boundary
            .as_ref()
            .map(|boundary| format!("{:?}", boundary.credential_boundary_status).to_lowercase()),
        credential_injection_status: output
            .credential_injection
            .as_ref()
            .map(|injection| format!("{:?}", injection.credential_injection_status).to_lowercase()),
        credential_class: output
            .credential_injection
            .as_ref()
            .map(|injection| credential_class_name(&injection.credential_class).to_string()),
        credential_handle_ref: output
            .credential_injection
            .as_ref()
            .and_then(|injection| injection.credential_handle_ref.as_ref())
            .map(|handle_ref| handle_ref.as_str().to_string()),
        idempotency_key_ref: output
            .audit_record
            .details
            .idempotency_context
            .as_ref()
            .map(|_| "caller_supplied_idempotency_key".to_string()),
    }
}

fn credential_class_name(class: &aegis::auth::CredentialClass) -> &'static str {
    match class {
        aegis::auth::CredentialClass::None => "none",
        aegis::auth::CredentialClass::LocalRuntime => "local_runtime",
    }
}

fn next_path_arg(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> RuntimeResult<Option<PathBuf>> {
    args.next().map(PathBuf::from).map(Some).ok_or_else(|| {
        boxed_report(GatewayErrorReport::runtime_io_failed(
            format!("{flag} requires a path. {}", usage()),
            "missing_cli_path",
        ))
    })
}

fn usage() -> String {
    "usage: aegis-gateway --bundle <policy-bundle-path> [--audit-log <audit-jsonl-path>] [--state-log <state-jsonl-path>] [--sandbox-dir <sandbox-path>] [request-json-path]\n       aegis-gateway --inspect-state <state-jsonl-path>"
        .to_string()
}

fn runtime_usage_error() -> Box<GatewayErrorReport> {
    boxed_report(GatewayErrorReport::runtime_io_failed(
        usage(),
        "invalid_cli_arguments",
    ))
}

fn print_structured_json(value: &impl Serialize) -> RuntimeResult<()> {
    let json = serde_json::to_string_pretty(value).map_err(|error| {
        boxed_report(GatewayErrorReport::unexpected_internal(format!(
            "The runtime could not serialize structured JSON output: {error}."
        )))
    })?;

    println!("{json}");
    Ok(())
}

fn boxed_report(report: GatewayErrorReport) -> Box<GatewayErrorReport> {
    Box::new(report)
}
