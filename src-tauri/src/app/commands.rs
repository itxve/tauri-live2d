use crate::app::config::AppConf;
use crate::app::mstruct::{InitType, Rt};

use crate::plugins::{Error, Result};
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
pub fn model_list() -> Result<Vec<String>> {
    let config = AppConf::read();
    use glob::glob;
    let mut models = vec![];
    let api = format!("http://127.0.0.1:{}", config.port);

    if config.model_dir != "" {
        for file_name_result in glob(&format!("{}/**/*.model3.json", config.model_dir))
            .unwrap()
            .chain(glob(&format!("{}/**/index.json", config.model_dir)).unwrap())
            .chain(glob(&format!("{}/**/*.model.json", config.model_dir)).unwrap())
        {
            match file_name_result {
                Ok(file_path) => {
                    models.push(
                        file_path
                            .to_str()
                            .unwrap()
                            .replace(config.model_dir.as_str(), api.as_str()),
                    );
                }
                Err(e) => {
                    eprintln!("ERROR: {}", e);
                }
            };
        }
    }
    Ok(models)
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

#[tauri::command]
pub fn read_config() -> AppConf {
    AppConf::read()
}

#[tauri::command]
pub fn write_config(value: String) -> AppConf {
    AppConf::read().amend_str(value).write()
}
