use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

/// system tray
pub fn menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let config = CustomMenuItem::new("config".to_string(), "Config");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(config)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "config" => {
                match app.get_window("config") {
                    Some(w) => {
                        w.show().unwrap();
                    }
                    None => {
                        // main 窗口如果被关闭，重新实例化
                        let config_win = tauri::WindowBuilder::new(
                            app,
                            "config",
                            tauri::WindowUrl::App("index.html".into()),
                        )
                        .build()
                        .unwrap();
                        config_win.center();
                        config_win.set_resizable(true);
                        config_win.set_title("live2d看板娘配置");
                    }
                };
            }
            "show" => {
                match app.get_window("main") {
                    Some(w) => {
                        w.show().unwrap();
                    }
                    None => {
                        // live2d 窗口如果被关闭，重新实例化
                        let live2d_win = tauri::WindowBuilder::new(
                            app,
                            "main",
                            tauri::WindowUrl::App("live2d.html".into()),
                        )
                        .build()
                        .unwrap();
                    }
                };
            }
            _ => {}
        },
        _ => {}
    }
}
