use std::fs;

use crate::config;
use crate::entity;

///所有模型
pub fn models_list() -> Vec<entity::ModelFileItem> {
    read_dir_model_list(unsafe { config::models_path })
}

///所有模型
pub fn read_dir_model_list(path_dir: &str) -> Vec<entity::ModelFileItem> {
    let mut ms: Vec<entity::ModelFileItem> = vec![];
    for (i, entry) in std::fs::read_dir(path_dir).unwrap().enumerate() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            let parent_path = entry.path();
            let m_f = parent_path.join(std::path::Path::new("index.json"));
            if m_f.exists() {
                if let Some(f_name) = parent_path.file_name() {
                    ms.push(entity::ModelFileItem {
                        id: i + 1,
                        name: vec![String::from(f_name.to_str().unwrap())],
                    });
                }
                //多目录
            } else {
                let mut names: Vec<String> = vec![];
                for c_entry in std::fs::read_dir(std::path::Path::new(path_dir).join(&parent_path))
                    .unwrap()
                    .into_iter()
                {
                    let c_entry = c_entry.unwrap();
                    let metadata = c_entry.metadata().unwrap();
                    if metadata.is_dir() {
                        let c_path = c_entry.path();
                        let c_m_f = c_path.join(std::path::Path::new("index.json"));
                        if c_m_f.exists() {
                            if let Some(c_f_name) = c_path.file_name() {
                                names.push(String::from(format!(
                                    "{}/{}",
                                    parent_path.file_name().unwrap().to_str().unwrap(),
                                    c_path.file_name().unwrap().to_str().unwrap()
                                )));
                            }
                        } else {
                        }
                    }
                }
                ms.push(entity::ModelFileItem {
                    id: i + 1,
                    name: names,
                });
            }
        }
    }
    ms
}

///根据Id获取模型
pub fn name_by_id(mod_id: usize) -> Option<entity::ModelFileItem> {
    let ms = models_list();
    println!("ms :{:#?}   mod_id: {}", ms, mod_id);
    ms.into_iter().find(|it| it.id == mod_id)
}

///随机数
pub fn rand_number(max: usize) -> usize {
    use rand::prelude::*;
    let mut rng = thread_rng();
    rng.gen_range(0..=max)
}

///根据Id获取模型
pub fn get_textures(path: &str) -> Vec<Vec<String>> {
    let textures_path = std::path::Path::new(unsafe { config::models_path })
        .join(path)
        .join("textures.cache");
    if !textures_path.exists() {
        return vec![];
    }
    let content = fs::read_to_string(textures_path).unwrap();
    if content != "null" {
        let js: entity::TexturesRoot = serde_json::from_str(&content).unwrap();
        return match js {
            entity::TexturesRoot::Second(second) => second,
            entity::TexturesRoot::One(one) => vec![one],
        };
    }
    vec![]
}

#[test]
fn lis() {
    unsafe { config::models_path = "/Users/apple/Desktop/models" }
    let ms = models_list();

    println!("ms: {:?}", ms);
}
