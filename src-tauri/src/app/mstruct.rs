use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppDataConfig {
    pub root_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rt<T> {
    pub data: T,
    pub err: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InitType {
    EXIST,
    CreateError,
    SUCCESS,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    pub serve_path: Option<String>,
}


