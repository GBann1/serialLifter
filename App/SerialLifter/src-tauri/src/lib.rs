#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod serial;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                serial::setup_connection(app.handle().clone());
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
