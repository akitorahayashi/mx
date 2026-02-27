use std::path::PathBuf;

use crate::domain::error::AppError;

pub trait WorkspaceLocator {
    fn find_workspace_root(&self) -> Result<PathBuf, AppError>;
}
