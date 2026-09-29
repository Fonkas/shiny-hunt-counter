// Hide the extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        // Global hotkeys (F8 = +1, etc.) that work while another window, like an emulator, is focused.
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // Closing the main window closes the whole app, including the mini counter window.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if window.label() == "main" {
                    window.app_handle().exit(0);
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Shiny Hunt Counter");
}
