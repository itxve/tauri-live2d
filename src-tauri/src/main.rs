#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod app;
mod plugins;
mod utils;
use log::info;
use web_server;

use app::{config::AppConf, commands};

#[tauri::command(main)]
fn main() {
    let context = tauri::generate_context!();
    let app = tauri::Builder::default();
    let app_conf = AppConf::read().write();
    let port = web_server::Port(web_server::get_available_port(), app_conf.model_dir.clone());
    if app_conf.model_dir != "" {
        AppConf::read()
            .amend(serde_json::json!({ "port":port.0 }))
            .write();
        tauri::async_runtime::spawn(web_server::app(port.clone()));
    }

    app.manage(port.clone())
        .setup(|app| {
            info!("app running...");
            Ok(())
        })
        .plugin(plugins::autostart::init(
            plugins::autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(plugins::checkupdate::init())
        .system_tray(app::menu::tray_menu())
        .on_system_tray_event(app::menu::tray_handler)
        .invoke_handler(tauri::generate_handler![
            commands::read_file,
            commands::write_file,
            commands::model_list,
            commands::read_config,
            commands::write_config
        ])
        .build(context)
        .expect("error while running live2d application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                println!("last close");
                api.prevent_exit();
            }
            _ => {}
        })
}
