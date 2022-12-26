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
