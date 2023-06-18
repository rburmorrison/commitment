//! Types to help with the handling of temporary files.

use std::{fs::File, path::PathBuf};

use anyhow::Result;

/// A self-deleting file wrapper.
///
/// Upon getting dropped, this object will try to delete the corresponding file
/// on the file system. The deletion is not garanteed and if an error occurs
/// it will be ignored.
pub struct TFile {
    pub file: File,
    pub path: PathBuf,
}

impl TFile {
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path = path.into();
        let file = File::create(path.as_path())?;
        Ok(Self { file, path })
    }
}

impl Drop for TFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
