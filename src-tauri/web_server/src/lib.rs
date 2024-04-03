use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};

use rand::Rng;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};
use tower_http::cors::{Any, CorsLayer};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Port(pub u16, pub String);

pub async fn app(conf: Port) {
    let app = Router::new()
        .route("/ok", get(move || async { "ok" }))
        .route("/ws", get(ws_handler))
        .nest_service(
            "/",
            get_service(ServeDir::new(conf.1).not_found_service(handle_404.into_service())),
        )
        .layer(
            CorsLayer::new()
                .allow_headers(Any)
                .allow_origin(Any)
                .allow_methods(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], conf.0));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

pub fn get_available_port() -> u16 {
    let mut rng = rand::thread_rng();
    let mut port: u16;

    loop {
        port = rng.gen_range(1024..65535); // 生成一个1024到65535之间的随机端口号
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);
        // 尝试创建一个TCP监听器来检查端口是否可用
        match TcpListener::bind(addr) {
            Ok(_) => break,     // 如果绑定成功，端口可用
            Err(_) => continue, // 如果绑定失败，生成新的随机端口号
        }
    }

    port
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        // Echo
                        if socket
                            .send(Message::Text(format!("Echo from backend: {}", t)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    Message::Close(_) => {
                        return;
                    }
                    _ => {}
                }
            } else {
                return;
            }
        }
    }
}
