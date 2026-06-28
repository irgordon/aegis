use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

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

    println!("{json}");
    Ok(())
}

struct LocalRuntimeArgs {
    bundle_path: PathBuf,
    request_path: Option<PathBuf>,
}

fn parse_args() -> Result<LocalRuntimeArgs, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [bundle_flag, bundle_path] if bundle_flag == "--bundle" => Ok(LocalRuntimeArgs {
            bundle_path: PathBuf::from(bundle_path),
            request_path: None,
        }),
        [bundle_flag, bundle_path, request_path] if bundle_flag == "--bundle" => {
            Ok(LocalRuntimeArgs {
                bundle_path: PathBuf::from(bundle_path),
                request_path: Some(PathBuf::from(request_path)),
            })
        }
        _ => Err(
            "usage: aegis-gateway --bundle <policy-bundle-path> [request-json-path]".to_string(),
        ),
    }
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
