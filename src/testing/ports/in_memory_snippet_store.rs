use crate::domain::error::AppError;
use crate::domain::ports::SnippetStore;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct InMemorySnippetStore {
    files: Mutex<HashMap<String, String>>,
}

impl InMemorySnippetStore {
    pub fn new() -> Self {
        Self { files: Mutex::new(HashMap::new()) }
    }

    /// Pre-populate with an existing entry (for testing remove/duplicate scenarios).
    pub fn seed(&self, relative: &str, contents: &str) {
        self.files.lock().unwrap().insert(relative.to_string(), contents.to_string());
    }

    pub fn has(&self, relative: &str) -> bool {
        self.files.lock().unwrap().contains_key(relative)
    }

    pub fn read(&self, relative: &str) -> String {
        self.files.lock().unwrap().get(relative).cloned().unwrap_or_default()
    }
}

impl Default for InMemorySnippetStore {
    fn default() -> Self {
        Self::new()
    }
}

fn key(relative: &Path) -> String {
    if relative.extension().is_some() {
        relative.to_string_lossy().to_string()
    } else {
        format!("{}.md", relative.to_string_lossy())
    }
}

impl SnippetStore for InMemorySnippetStore {
    fn write_snippet(&self, relative_path: &Path, contents: &str) -> Result<PathBuf, AppError> {
        let k = key(relative_path);
        self.files.lock().unwrap().insert(k.clone(), contents.to_string());
        Ok(PathBuf::from(k))
    }

    fn snippet_exists(&self, relative_path: &Path) -> bool {
        self.files.lock().unwrap().contains_key(&key(relative_path))
    }

    fn remove_snippet(&self, relative_path: &Path) -> Result<PathBuf, AppError> {
        let k = key(relative_path);
        let mut files = self.files.lock().unwrap();
        if files.remove(&k).is_none() {
            return Err(AppError::not_found(format!("Snippet not found: {k}")));
        }
        Ok(PathBuf::from(k))
    }
}
