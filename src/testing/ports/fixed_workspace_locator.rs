use crate::domain::error::AppError;
use crate::domain::ports::WorkspaceLocator;
use std::path::PathBuf;

pub struct FixedWorkspaceLocator {
    root: PathBuf,
}

impl FixedWorkspaceLocator {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl WorkspaceLocator for FixedWorkspaceLocator {
    fn find_workspace_root(&self) -> Result<PathBuf, AppError> {
        Ok(self.root.clone())
    }
}
