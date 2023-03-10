use crate::live2d::mstruct::{InitType, Rt};
use std::fs;

#[tauri::command]
pub fn read_file(file_path: std::path::PathBuf) -> Rt<Vec<u8>> {
    std::fs::read(file_path).map_or_else(
        |err| Rt {
            data: vec![],
            err: err.to_string(),
        },
        |data| Rt {
            data,
            err: String::from(""),
        },
    )
}

#[tauri::command]
pub fn write_file(file_path: std::path::PathBuf, data: &str) -> Rt<String> {
    std::fs::write(file_path, data).map_or_else(
        |err| Rt {
            data: "".to_string(),
            err: err.to_string(),
        },
        |_| Rt {
            data: "".to_string(),
            err: "".to_string(),
        },
    )
}

#[tauri::command]
pub fn init_app_data_path(file_path: std::path::PathBuf) -> InitType {
    println!("file_path: {:?}", &file_path);
    if file_path.exists() {
        InitType::EXIST
    } else {
        fs::DirBuilder::new()
            .recursive(true)
            .create(file_path)
            .map_or_else(|_| InitType::CreateError, |_| InitType::SUCCESS)
    }
}
