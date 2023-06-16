use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(rename = "can-fail")]
    pub can_fail: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "can-fail")]
    pub can_fail: Option<bool>,
    pub execute: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub defaults: Defaults,

    #[serde(flatten)]
    pub tasks: IndexMap<String, Task>,
}
