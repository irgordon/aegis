use std::{
    env, fs,
    io::{self, Read},
};

use aegis::runtime::local::process_local_gateway_request;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let input = read_input()?;
    let output = process_local_gateway_request(&input);
    let json = serde_json::to_string_pretty(&output)
        .map_err(|error| format!("failed to serialize local gateway output: {error}"))?;

    println!("{json}");
    Ok(())
}

fn read_input() -> Result<String, String> {
    match input_path()? {
        Some(path) => {
            fs::read_to_string(&path).map_err(|error| format!("failed to read {path}: {error}"))
        }
        None => read_stdin(),
    }
}

fn input_path() -> Result<Option<String>, String> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [] => Ok(None),
        [path] => Ok(Some(path.clone())),
        _ => Err("usage: aegis-gateway [request-json-path]".to_string()),
    }
}

fn read_stdin() -> Result<String, String> {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {error}"))?;
    Ok(input)
}
