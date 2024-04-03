use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, Wry,
};

/// system tray
pub fn tray_menu() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "关闭软件");
    let show = CustomMenuItem::new("show".to_string(), "显示桌宠");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏桌宠");
    let config = CustomMenuItem::new("config".to_string(), "配置中心");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(config)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

pub fn tray_handler(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = &app.get_window("main").unwrap();
                window.hide().unwrap();
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
            "config" => {
                match app.get_window("config") {
                    Some(w) => {
                        w.show().unwrap();
                    }
                    None => {
                        // main 窗口如果被关闭，重新实例化
                        tauri::WindowBuilder::new(
                            app,
                            "config",
                            tauri::WindowUrl::App("index.html".into()),
                        )
                        .title("配置")
                        .center()
                        .resizable(true)
                        .always_on_top(true)
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
