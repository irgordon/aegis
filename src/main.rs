use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use aegis::audit::{AuditSink, AuditWriter};
use aegis::error::GatewayErrorReport;
use aegis::runtime::local::{process_local_gateway_request, LocalRuntimeOutput};
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
    let args = parse_args()?;
    let input = read_input(args.request_path.as_deref())?;
    let mut output = process_local_gateway_request(&input, &args.bundle_path);

    match persist_audit_record(args.audit_log_path.as_deref(), &output) {
        Ok(()) => {
            output.mark_audited_completed();
            print_structured_json(&output)?;
            Ok(0)
        }
        Err(error_report) => {
            output.mark_audit_failed();
            output.attach_error_report(*error_report);
            print_structured_json(&output)?;
            Ok(1)
        }
    }
}

struct LocalRuntimeArgs {
    bundle_path: PathBuf,
    audit_log_path: Option<PathBuf>,
    request_path: Option<PathBuf>,
}

fn parse_args() -> RuntimeResult<LocalRuntimeArgs> {
    let mut args = env::args().skip(1);
    let mut bundle_path = None;
    let mut audit_log_path = None;
    let mut request_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = next_path_arg(&mut args, "--bundle")?,
            "--audit-log" => audit_log_path = next_path_arg(&mut args, "--audit-log")?,
            _ if request_path.is_none() => request_path = Some(PathBuf::from(arg)),
            _ => return Err(runtime_usage_error()),
        }
    }

    Ok(LocalRuntimeArgs {
        bundle_path: bundle_path.ok_or_else(runtime_usage_error)?,
        audit_log_path,
        request_path,
    })
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
    "usage: aegis-gateway --bundle <policy-bundle-path> [--audit-log <audit-jsonl-path>] [request-json-path]"
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
