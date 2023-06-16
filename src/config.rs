use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
pub struct CommitmentFile {
    pub defaults: Defaults,

    #[serde(flatten)]
    pub tasks: HashMap<String, Task>,
}
