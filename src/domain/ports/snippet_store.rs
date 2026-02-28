use crate::domain::error::AppError;
use std::path::{Path, PathBuf};

pub trait SnippetStore {
    fn write_snippet(&self, relative_path: &Path, contents: &str) -> Result<PathBuf, AppError>;
    fn snippet_exists(&self, relative_path: &Path) -> bool;
    fn remove_snippet(&self, relative_path: &Path) -> Result<PathBuf, AppError>;
}
