use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Restage {
    pub extensions: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    #[serde(default, rename = "can-fail")]
    pub can_fail: bool,
    pub restage: Option<Restage>,
    pub execute: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub tasks: IndexMap<String, Task>,
}
