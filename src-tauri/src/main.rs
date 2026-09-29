// Hide the extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // Global hotkeys (F8 = +1, etc.) that work while another window, like an emulator, is focused.
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .run(tauri::generate_context!())
        .expect("error while running Shiny Hunt Counter");
}
