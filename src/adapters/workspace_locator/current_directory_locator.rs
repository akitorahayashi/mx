use crate::domain::error::AppError;
use crate::domain::ports::WorkspaceLocator;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct CurrentDirectoryLocator;

impl WorkspaceLocator for CurrentDirectoryLocator {
    fn find_workspace_root(&self) -> Result<PathBuf, AppError> {
        std::env::current_dir().map_err(AppError::Io)
    }
}
