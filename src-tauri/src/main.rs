#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![warn(unused_must_use)]
mod live2d;
mod plugins;
use live2d::{config, my_command};
use tauri::{generate_context, Manager};

#[tauri::command(main)]
fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let app_data_path = tauri::api::path::app_config_dir(&app.config());
            //初始化app config目录
            my_command::init_app_data_path(app_data_path.unwrap());
            Ok(())
        })
        .plugin(plugins::autostart::init(
            plugins::autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .system_tray(live2d::tray::menu())
        .on_system_tray_event(live2d::tray::handler)
        .on_page_load(|window, _| {
            //添加文件监视器
            let app_data_path = tauri::api::path::app_config_dir(&window.config()).unwrap();
            my_command::watch_jy_file_change(app_data_path, move || {
                window.emit_all(config::JY_FILE_CHANGE, 1).unwrap();
            });
        })
        .invoke_handler(tauri::generate_handler![
            my_command::read_file,
            my_command::write_file,
        ])
        .build(generate_context!())
        .expect("error while running tauri application");
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            println!("last close");
            api.prevent_exit();
        }
        _ => {}
    })
}
