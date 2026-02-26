use crate::domain::error::AppError;
use std::path::PathBuf;

pub trait WorkspaceLocator {
    fn find_workspace_root(&self) -> Result<PathBuf, AppError>;
}
