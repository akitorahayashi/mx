use crate::domain::error::AppError;
use crate::ports::{ContextFileStore, ContextWriteStatus};
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct InMemoryContextStore {
    files: RefCell<HashMap<PathBuf, String>>,
    workspace_files: RefCell<HashMap<PathBuf, String>>,
}

impl InMemoryContextStore {
    pub fn set_workspace_file<P: Into<PathBuf>, S: Into<String>>(&self, path: P, value: S) {
        self.workspace_files.borrow_mut().insert(path.into(), value.into());
    }
}

impl ContextFileStore for InMemoryContextStore {
    fn prepare_context_file(
        &self,
        relative_path: &Path,
        force: bool,
    ) -> Result<ContextWriteStatus, AppError> {
        let path = PathBuf::from(".mx").join(relative_path);
        let existed = self.files.borrow().contains_key(&path);
        let overwritten = force && existed;

        if !existed || force {
            self.files.borrow_mut().entry(path.clone()).or_default();
        }

        Ok(ContextWriteStatus { path, existed, overwritten })
    }

    fn write_context_contents(&self, absolute_path: &Path, contents: &str) -> Result<(), AppError> {
        self.files.borrow_mut().insert(absolute_path.to_path_buf(), contents.to_string());
        Ok(())
    }

    fn read_context_contents(&self, relative_path: &Path) -> Result<String, AppError> {
        let path = PathBuf::from(".mx").join(relative_path);
        self.files.borrow().get(&path).cloned().ok_or_else(|| {
            AppError::not_found(format!("⚠️ Context file not found: {}", relative_path.display()))
        })
    }

    fn remove_context_root(&self) -> Result<bool, AppError> {
        let had_entries = !self.files.borrow().is_empty();
        self.files.borrow_mut().clear();
        Ok(had_entries)
    }

    fn remove_context_file(&self, relative_path: &Path) -> Result<PathBuf, AppError> {
        let path = PathBuf::from(".mx").join(relative_path);
        if self.files.borrow_mut().remove(&path).is_some() {
            return Ok(path);
        }

        Err(AppError::not_found(format!("File not found: {}", path.display())))
    }

    fn read_workspace_file(&self, relative_path: &Path) -> Result<String, std::io::Error> {
        self.workspace_files.borrow().get(&relative_path.to_path_buf()).cloned().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Missing file: {}", relative_path.display()),
            )
        })
    }
}
