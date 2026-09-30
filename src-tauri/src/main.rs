// Hide the extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        // Global hotkeys (F8 = +1, etc.) that work while another window, like an emulator, is focused.
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // When the main window closes, close the mini counter too. The app then shuts down normally,
        // which gives the main window time to finish saving.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                if window.label() == "main" {
                    if let Some(mini) = window.app_handle().get_webview_window("mini") {
                        let _ = mini.close();
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Shiny Hunt Counter");
}
