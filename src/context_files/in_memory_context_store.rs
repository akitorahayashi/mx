use crate::context_files::{ContextFileStore, ContextWriteStatus};
use crate::error::AppError;
use crate::project_fs::SafePath;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct InMemoryContextStore {
    files: RefCell<HashMap<PathBuf, String>>,
}

impl ContextFileStore for InMemoryContextStore {
    fn prepare_context_file(
        &self,
        relative_path: &SafePath,
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

    fn read_context_contents(&self, relative_path: &SafePath) -> Result<String, AppError> {
        let path = PathBuf::from(".mx").join(relative_path);
        self.files.borrow().get(&path).cloned().ok_or_else(|| {
            AppError::NotFound(crate::error::NotFoundError::ContextFile(format!(
                "⚠️ Context file not found: {}",
                relative_path.display()
            )))
        })
    }

    fn remove_context_root(&self) -> Result<bool, AppError> {
        let had_entries = !self.files.borrow().is_empty();
        self.files.borrow_mut().clear();
        Ok(had_entries)
    }

    fn remove_context_file(&self, relative_path: &SafePath) -> Result<PathBuf, AppError> {
        let path = PathBuf::from(".mx").join(relative_path);
        if self.files.borrow_mut().remove(&path).is_some() {
            return Ok(path);
        }

        Err(AppError::NotFound(crate::error::NotFoundError::ContextFile(format!(
            "File not found: {}",
            path.display()
        ))))
    }
}
