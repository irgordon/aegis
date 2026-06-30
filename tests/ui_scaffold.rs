use std::{fs, path::Path};

const DESKTOP_ENTRYPOINT: &str = "src-tauri/src/main.rs";
const SLINT_UI: &str = "src-tauri/ui/main.slint";
const TAURI_CONFIG: &str = "src-tauri/tauri.conf.json";

#[test]
fn desktop_scaffold_files_exist() {
    assert!(Path::new("src-tauri/Cargo.toml").is_file());
    assert!(Path::new("src-tauri/build.rs").is_file());
    assert!(Path::new(DESKTOP_ENTRYPOINT).is_file());
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

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("{path} should be readable: {error}"))
}
