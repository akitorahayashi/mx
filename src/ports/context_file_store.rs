use crate::domain::error::AppError;
use std::path::{Path, PathBuf};

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
        relative_path: &Path,
        force: bool,
    ) -> Result<ContextWriteStatus, AppError>;
    fn write_context_contents(&self, absolute_path: &Path, contents: &str) -> Result<(), AppError>;
    fn read_context_contents(&self, relative_path: &Path) -> Result<String, AppError>;
    fn remove_context_root(&self) -> Result<bool, AppError>;
    fn remove_context_file(&self, relative_path: &Path) -> Result<PathBuf, AppError>;
    /// Workspace reads preserve `std::io::ErrorKind` so placeholder rendering can report
    /// concrete missing/permission reasons without lossy conversion.
    fn read_workspace_file(&self, relative_path: &Path) -> Result<String, std::io::Error>;
}
