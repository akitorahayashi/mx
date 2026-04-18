use crate::error::AppError;
use crate::project_fs::SafePath;
use std::path::PathBuf;

pub trait SnippetStore {
    fn write_snippet(&self, relative_path: &SafePath, contents: &str) -> Result<PathBuf, AppError>;
    fn snippet_exists(&self, relative_path: &SafePath) -> bool;
    fn remove_snippet(&self, relative_path: &SafePath) -> Result<PathBuf, AppError>;
}
