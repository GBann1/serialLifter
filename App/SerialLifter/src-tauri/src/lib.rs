#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::{Manager, AppHandle, Result};

pub mod serial;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_port_listen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_port_listen(app: AppHandle) -> Result<()> {
    std::thread::spawn(move || {
        serial::setup_connection(app.clone())
    });
    Ok(())
}