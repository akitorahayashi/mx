use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SnippetEntry {
    pub key: String,
    pub relative_path: String,
    pub absolute_path: PathBuf,
}
