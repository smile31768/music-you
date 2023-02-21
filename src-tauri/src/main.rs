#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod utils;
mod core;

use tauri::{api, SystemTray, Menu};
use tauri::{CustomMenuItem, MenuItem};
use tauri::{utils::config::AppUrl,WindowUrl};

use crate::utils::{ resolve };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command



fn main() -> std::io::Result<()> {
    let port = portpicker::pick_unused_port().expect("failed to find unused port");

    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", port).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());
    
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
        .add_native_item(MenuItem::Copy);
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_localhost::Builder::new(port).build())
        .menu(menu)
        .system_tray(SystemTray::new())
        .setup(|_app| Ok(resolve::resolve_setup(_app, window_url)))
        .on_system_tray_event(core::tray::Tray::on_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            commands::close_splashscreen,
            commands::app_quit,
            commands::minimized,
            commands::open_web_url,
        ]);

    let _app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, e| match e {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            tauri::RunEvent::Exit => {
                api::process::kill_children();
                app_handle.exit(0);
            }
            #[cfg(target_os = "macos")]
            tauri::RunEvent::WindowEvent { label, event, .. } => {
                use tauri::Manager;

                if label == "main" {
                    match event {
                        tauri::WindowEvent::CloseRequested { api, .. } => {
                            api.prevent_close();
                            app_handle.get_window("main").map(|win| {
                                let _ = win.hide();
                            });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
    Ok(())
}
