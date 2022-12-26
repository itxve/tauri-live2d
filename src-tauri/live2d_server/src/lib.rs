use axum::extract::{Json, Path, Query};
use axum::{
    body::Body,
    handler::HandlerWithoutStateExt,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::ptr::null;
use std::{fs, io};
use tower::ServiceExt;
use tower_http::{services::ServeDir, trace::TraceLayer};
mod config;
mod entity;
mod util;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
pub async fn start_serve(path: &'static str) {
    unsafe { config::models_path = path }
    tokio::join!(serve(using_serve_dir_with_handler_as_service(), 3004),);
}

/// 服务文件目录
fn using_serve_dir_with_handler_as_service() -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    // you can convert handler function to service
    let service = handle_404
        .into_service()
        .map_err(|err| -> std::io::Error { match err {} });

    // Path
    let serve_dir = ServeDir::new(unsafe { config::models_path }).not_found_service(service);
    let serve_dir = get_service(serve_dir).handle_error(handle_error);

    Router::new()
        .route("/get/", get(fetch_id))
        .route("/rand/", get(fetch_rand))
        .route("/switch/", get(fetch_switch))
        .route("/switch_textures/", get(switch_textures))
        .route("/rand_textures/", get(rand_textures))
        .fallback_service(serve_dir)
}

/// switch
async fn fetch_switch(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let list = util::models_list();
    let mut m_id = 1;
    if let Some(_m_id) = params.get("id") {
        m_id = if let Ok(id) = _m_id.parse::<usize>() {
            id
        } else {
            m_id
        };
    }
    list.get(m_id).map_or_else(
        || {
            let john = json!({
                "model":
                {
                    "id":"",
                    "name":"",
                    "message":""
                }
            });
            (StatusCode::OK, Json::from(john))
        },
        |md| {
            let john = json!({
                "model":
                {
                    "id":md.id,
                    "name":md.name,
                    "message":""
                }
            });
            (StatusCode::OK, Json::from(john))
        },
    )
}

/// random
async fn fetch_rand() -> impl IntoResponse {
    let list = util::models_list();
    let random_index = util::rand_number(list.len() - 1);
    list.get(random_index).map_or_else(
        || {
            let john = json!({
                "model":
                {
                    "id":"",
                    "name":"",
                    "message":""
                }
            });
            (StatusCode::OK, Json::from(john))
        },
        |md| {
            let john = json!({
                "model":
                {
                    "id":md.id,
                    "name":md.name,
                    "message":""
                }
            });
            (StatusCode::OK, Json::from(john))
        },
    );
}

/// getId
async fn fetch_id(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let path = std::path::Path::new(unsafe { config::models_path });
    let id_str = params.get("id").unwrap();
    let ids = id_str.split("-").collect::<Vec<&str>>();
    let mut m_id = 1;
    let mut skin_id = 0;
    if let Some(_m_id) = ids.get(0) {
        m_id = if let Ok(m_id) = _m_id.parse::<usize>() {
            m_id
        } else {
            m_id
        }
    }

    let model = util::name_by_id(m_id).unwrap();
    if let Some(_s_id) = ids.get(1) {
        skin_id = _s_id.parse::<usize>().unwrap();
    }
    let skin = model
        .name
        .get(skin_id)
        .unwrap_or_else(|| model.name.get(0).unwrap());
    let c = fs::read_to_string(path.join(&skin).join("index.json")).unwrap();
    let mut v: entity::ModelRoot = serde_json::from_str(&c).unwrap();
    warrper_model(skin, &mut v);
    return (StatusCode::OK, Json::from(v));
}

///rand_textures
async fn rand_textures(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let id_str = params.get("id").unwrap();
    let ids = id_str.split("-").collect::<Vec<&str>>();
    let mut m_id = 1;
    // let mut skin_id = 0;
    if let Some(_m_id) = ids.get(0) {
        m_id = if let Ok(m_id) = _m_id.parse::<usize>() {
            m_id
        } else {
            m_id
        }
    }
    let model = util::name_by_id(m_id).unwrap();
    // if let Some(_s_id) = ids.get(1) {
    //     skin_id = _s_id.parse::<usize>().unwrap();
    // }
    let skin = model
        .name
        .get(util::rand_number(model.name.len()))
        .unwrap_or_else(|| model.name.get(0).unwrap());
    let textures = util::get_textures(skin);
    let textures_index = util::rand_number(textures.len());
    let textures = textures.get(textures_index);

    return (
        StatusCode::OK,
        Json::from(json!({"textures": {
          "id":textures_index,
          "name": textures,
          "model":skin
        }})),
    );
}

/// switch_textures
async fn switch_textures(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let id_str = params.get("id").unwrap();
    let ids = id_str.split("-").collect::<Vec<&str>>();
    let mut m_id = 1;
    let mut skin_id = 0;
    if let Some(_m_id) = ids.get(0) {
        m_id = if let Ok(m_id) = _m_id.parse::<usize>() {
            m_id
        } else {
            m_id
        }
    }
    let model = util::name_by_id(m_id).unwrap();
    if let Some(_s_id) = ids.get(1) {
        skin_id = _s_id.parse::<usize>().unwrap();
    }
    let skin = model
        .name
        .get(skin_id % model.name.len())
        .unwrap_or_else(|| model.name.get(0).unwrap());
    let textures = util::get_textures(skin);
    let textures = textures.get(skin_id);

    return (
        StatusCode::OK,
        Json::from(json!({"textures": {
          "id":skin_id,
          "name": textures,
          "model":skin
        }})),
    );
}

///追加路径
fn append_root(model_name: &str, p: &str) -> String {
    format!("../{}/{}", model_name, p)
}

///错误处理
async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

///启动服务
async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
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

///处理model字段
fn warrper_model(model_name: &str, root: &mut entity::ModelRoot) {
    //physics
    if let Some(physics) = &mut root.physics {
        *physics = append_root(model_name, &physics);
    }
    //pose
    if let Some(pose) = &mut root.pose {
        *pose = append_root(model_name, &pose);
    }
    //voice
    if let Some(voice) = &mut root.voice {
        *voice = append_root(model_name, &voice);
    }
    //model
    if let Some(model) = &mut root.model {
        *model = append_root(model_name, &model);
    }
    //expressions
    if let Some(expressions) = &mut root.expressions {
        for ele in expressions.iter_mut() {
            if let Some(it_file) = &ele.file {
                ele.file = Some(append_root(model_name, &it_file));
            }
        }
    }
    //motions
    if let Some(motions) = &mut root.motions {
        for (key, value) in motions.iter_mut() {
            for ele in value.iter_mut() {
                if let Some(it_file) = &ele.file {
                    ele.file = Some(append_root(model_name, &it_file));
                }
                if let Some(it_sound) = &ele.sound {
                    ele.sound = Some(append_root(model_name, &it_sound));
                }
            }
        }
    }

    //textures
    if let Some(textures) = &mut root.textures {
        for ele in textures.iter_mut() {
            *ele = append_root(model_name, &ele);
        }
    }

    let mut textures = util::get_textures(model_name);
    let textures_index = util::rand_number(textures.len());
    let textures = textures.get_mut(textures_index);

    // //new textures
    if let Some(textures) = textures {
        for ele in textures.iter_mut() {
            *ele = append_root(model_name, &ele);
        }
        root.textures = Some(textures.clone());
    }

    //layout
    if let None = &mut root.layout {
        root.layout = Some(entity::Layout {
            center_x: Some(0.0 as f64),
            center_y: Some(0.0 as f64),
            width: None,
            height: None,
        });
    }
    //hit_areas_custom
    if let None = &mut root.hit_areas_custom {
        root.hit_areas_custom = Some(entity::HitAreasCustom {
            head_x: Some(vec![0.0 as f64, 0.0 as f64]),
            head_y: Some(vec![0.0 as f64, 0.0 as f64]),
            body_x: Some(vec![0.0 as f64, 0.0 as f64]),
            body_y: Some(vec![0.0 as f64, 0.0 as f64]),
        });
    }
}
