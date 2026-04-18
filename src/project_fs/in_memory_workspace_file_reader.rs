use super::{SafePath, WorkspaceFileReader};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct InMemoryWorkspaceFileReader {
    files: RefCell<HashMap<PathBuf, String>>,
}

impl InMemoryWorkspaceFileReader {
    pub fn set_file<P: Into<PathBuf>, S: Into<String>>(&self, path: P, value: S) {
        self.files.borrow_mut().insert(path.into(), value.into());
    }
}

impl WorkspaceFileReader for InMemoryWorkspaceFileReader {
    fn read_workspace_file(&self, relative_path: &SafePath) -> Result<String, std::io::Error> {
        let path = relative_path.to_path_buf();
        self.files.borrow().get(&path).cloned().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Missing file: {}", path.display()),
            )
        })
    }
}
