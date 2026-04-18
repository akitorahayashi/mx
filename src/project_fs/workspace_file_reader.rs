use super::SafePath;
use std::fs;
use std::path::PathBuf;

pub trait WorkspaceFileReader {
    fn read_workspace_file(&self, relative_path: &SafePath) -> Result<String, std::io::Error>;
}

#[derive(Debug, Clone)]
pub struct LocalWorkspaceFileReader {
    workspace_root: PathBuf,
}

impl LocalWorkspaceFileReader {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }
}

impl WorkspaceFileReader for LocalWorkspaceFileReader {
    fn read_workspace_file(&self, relative_path: &SafePath) -> Result<String, std::io::Error> {
        fs::read_to_string(self.workspace_root.join(relative_path))
    }
}
