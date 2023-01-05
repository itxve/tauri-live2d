use axum::handler::HandlerWithoutStateExt;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};

use axum_server::Handle;
use std::fs;
use std::net::SocketAddr;
use tauri::api::http;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use tauri::{window, AppHandle, Manager};
use tower::ServiceExt;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{services::ServeDir, trace::TraceLayer};

// 服务标识
static mut HAS_WATCH: bool = false;

//web本地服务
#[tauri::command]
pub fn start_serve(serve_dir: String) {
    if unsafe { HAS_WATCH } == true {
        println!("serve is running");
    } else {
        unsafe { HAS_WATCH = true }
        std::thread::spawn(move || {
            _serve(&serve_dir);
        });
        println!("serve start listen");
    }
}
#[tokio::main]
pub async fn _serve(serve_dir: &str) {
    let handle = Handle::new();
    let handlec = handle.clone();
    let mut app = Router::new()
        .route(
            "/shutdown",
            axum::routing::get(move || async {
                shutdown(handlec).await;
                "ok"
            }),
        )
        .route("/running", axum::routing::get(move || async { "ok" }));

    for it in vec!["/"] {
        app = app.nest_service(
            it,
            get_service(
                ServeDir::new(serve_dir).not_found_service(
                    handle_404
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            )
            .handle_error(handle_error),
        )
    }

    println!("app router:{:?}", &app);
    axum_server::bind(SocketAddr::from(([127, 0, 0, 1], 13004)))
        .handle(handle)
        .serve(
            app.layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    .allow_origin(Any)
                    .allow_methods(Any),
            )
            .layer(TraceLayer::new_for_http())
            .into_make_service(),
        )
        .await
        .unwrap();
}

async fn shutdown(handel: Handle) {
    handel.shutdown();
    unsafe { HAS_WATCH = false }
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

///错误处理
async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误")
}
use crate::live2d::{config, mstruct};
use crate::plugins::{Error, Result};

#[tauri::command]
async fn shutdown_cmd() -> Result<u16> {
    use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .send(
            HttpRequestBuilder::new("GET", "http://127.0.0.1:13004/shutdown")
                .unwrap()
                .response_type(ResponseType::Binary),
        )
        .await;
    if let Ok(response) = response {
        if response.status().as_u16() == 200u16 {
            return Ok(200u16);
        }
    }
    return Err(Error::Anyhow("shutdown fail".to_string()));
}

#[tauri::command]
async fn server_running() -> Result<u16> {
    use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};
    let client = ClientBuilder::new().build().unwrap();
    let response = client
        .send(
            HttpRequestBuilder::new("GET", "http://127.0.0.1:13004/running")
                .unwrap()
                .response_type(ResponseType::Binary),
        )
        .await;
    if let Ok(response) = response {
        if response.status().as_u16() == 200u16 {
            return Ok(200u16);
        }
    }
    return Err(Error::Anyhow("serve not running".to_string()));
}

#[tauri::command]
fn model_list(serve_dir: &str) -> Result<Vec<String>> {
    use glob::glob;
    let mut models = vec![];
    for file_name_result in glob(&format!("{}/**/*.model3.json", serve_dir))
        .unwrap()
        .chain(glob(&format!("{}/**/index.json", serve_dir)).unwrap())
        .chain(glob(&format!("{}/**/*.model.json", serve_dir)).unwrap())
    {
        match file_name_result {
            Ok(file_path) => {
                models.push(file_path.to_str().unwrap().to_string());
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
            }
        };
    }
    Ok(models)
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("model")
        .invoke_handler(tauri::generate_handler![
            model_list,
            start_serve,
            server_running,
            shutdown_cmd
        ])
        .setup(move |app| {
            println!("TauriPlugin [modelserver] ");
            Ok(())
        })
        .on_page_load(|window, payload| {
            let app_data_path = tauri::api::path::app_config_dir(&window.config()).unwrap();
            let config_path = app_data_path.join(config::APP_CONFIG_FILE);
            if config_path.exists() {
                let config = fs::read_to_string(config_path).unwrap();
                let config: mstruct::ConfigFile = serde_json::from_str(&config).unwrap();
                if let Some(serve_path) = config.serve_path {
                    println!("serve_path:{}", &serve_path);
                    start_serve(serve_path);
                }
            }
        })
        .build()
}
