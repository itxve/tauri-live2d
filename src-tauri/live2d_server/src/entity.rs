use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::ffi::OsString;

use crate::config;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelRoot {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub name: Option<String>,
    pub label: Option<String>,
    pub model: Option<String>,
    pub voice: Option<String>,
    pub textures: Option<Vec<String>>,
    pub physics: Option<String>,
    pub pose: Option<String>,
    pub expressions: Option<Vec<Expression>>,
    pub layout: Option<Layout>,
    #[serde(rename = "hit_areas_custom")]
    pub hit_areas_custom: Option<HitAreasCustom>,
    pub motions: Option<HashMap<String, Vec<MotionsItem>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    pub name: Option<String>,
    pub file: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layout {
    #[serde(rename = "center_x")]
    pub center_x: Option<f64>,
    #[serde(rename = "center_y")]
    pub center_y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitAreasCustom {
    #[serde(rename = "head_x")]
    pub head_x: Option<Vec<f64>>,
    #[serde(rename = "head_y")]
    pub head_y: Option<Vec<f64>>,
    #[serde(rename = "body_x")]
    pub body_x: Option<Vec<f64>>,
    #[serde(rename = "body_y")]
    pub body_y: Option<Vec<f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MotionsItem {
    pub file: Option<String>,
    pub sound: Option<String>,
    #[serde(rename = "fade_in")]
    pub fade_in: Option<f64>,
    #[serde(rename = "fade_out")]
    pub fade_out: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelFileItem {
    pub id: usize,
    pub name: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TexturesRoot {
    Second(Vec<Vec<String>>),
    One(Vec<String>),
}
