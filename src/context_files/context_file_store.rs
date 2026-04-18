use std::path::{Path, PathBuf};

use crate::error::AppError;
use crate::project_fs::SafePath;

#[derive(Debug, Clone)]
pub struct ContextWriteStatus {
    pub path: PathBuf,
    pub existed: bool,
    pub overwritten: bool,
}

impl ContextWriteStatus {
    pub fn should_write(&self) -> bool {
        !self.existed || self.overwritten
    }
}

pub trait ContextFileStore {
    fn prepare_context_file(
        &self,
        relative_path: &SafePath,
        force: bool,
    ) -> Result<ContextWriteStatus, AppError>;
    fn write_context_contents(&self, absolute_path: &Path, contents: &str) -> Result<(), AppError>;
    fn read_context_contents(&self, relative_path: &SafePath) -> Result<String, AppError>;
    fn remove_context_root(&self) -> Result<bool, AppError>;
    fn remove_context_file(&self, relative_path: &SafePath) -> Result<PathBuf, AppError>;
}
