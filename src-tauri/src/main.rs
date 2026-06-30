slint::include_modules!();

type DesktopResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DesktopResult<()> {
    let _tauri_shell = build_tauri_shell()?;
    show_static_operator_surface()?;
    Ok(())
}

fn build_tauri_shell() -> tauri::Result<tauri::App<tauri::Wry>> {
    tauri::Builder::default().build(tauri::generate_context!())
}

fn show_static_operator_surface() -> Result<(), slint::PlatformError> {
    AegisWindow::new()?.run()
}
