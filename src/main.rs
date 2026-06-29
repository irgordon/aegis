use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use aegis::audit::{AuditSink, AuditWriter};
use aegis::runtime::local::process_local_gateway_request;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let input = read_input(args.request_path.as_deref())?;
    let output = process_local_gateway_request(&input, &args.bundle_path);
    let json = serde_json::to_string_pretty(&output)
        .map_err(|error| format!("failed to serialize local gateway output: {error}"))?;

    persist_audit_record(args.audit_log_path.as_deref(), &output.audit_record)?;
    println!("{json}");
    Ok(())
}

struct LocalRuntimeArgs {
    bundle_path: PathBuf,
    audit_log_path: Option<PathBuf>,
    request_path: Option<PathBuf>,
}

fn parse_args() -> Result<LocalRuntimeArgs, String> {
    let mut args = env::args().skip(1);
    let mut bundle_path = None;
    let mut audit_log_path = None;
    let mut request_path = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bundle" => bundle_path = next_path_arg(&mut args, "--bundle")?,
            "--audit-log" => audit_log_path = next_path_arg(&mut args, "--audit-log")?,
            _ if request_path.is_none() => request_path = Some(PathBuf::from(arg)),
            _ => return Err(usage()),
        }
    }

    Ok(LocalRuntimeArgs {
        bundle_path: bundle_path.ok_or_else(usage)?,
        audit_log_path,
        request_path,
    })
}

fn read_input(request_path: Option<&Path>) -> Result<String, String> {
    match request_path {
        Some(path) => fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {error}", path.display())),
        None => read_stdin(),
    }
}

fn read_stdin() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {error}"))?;
    Ok(input)
}

fn persist_audit_record(
    audit_log_path: Option<&Path>,
    record: &aegis::audit::AuditRecord,
) -> Result<(), String> {
    if let Some(path) = audit_log_path {
        AuditWriter::new(path.to_path_buf())
            .append(record)
            .map_err(|error| error.to_string())?;
    }

    Ok(())
}

fn next_path_arg(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<Option<PathBuf>, String> {
    args.next()
        .map(PathBuf::from)
        .map(Some)
        .ok_or_else(|| format!("{flag} requires a path\n{}", usage()))
}

fn usage() -> String {
    "usage: aegis-gateway --bundle <policy-bundle-path> [--audit-log <audit-jsonl-path>] [request-json-path]"
        .to_string()
}
