//! Shared global definitions for the application.

use std::path::PathBuf;

use once_cell::sync::Lazy;

pub const APP_ID: &str = "com.rburmorrison.commitment";

pub static APP_DATA_DIR: Lazy<PathBuf> = Lazy::new(|| dirs::data_local_dir().unwrap().join(APP_ID));
