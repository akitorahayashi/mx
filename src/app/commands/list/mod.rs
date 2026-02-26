use crate::domain::error::AppError;
use crate::ports::SnippetCatalog;

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub snippet: String,
    pub relative_path: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub fn execute(catalog: &dyn SnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    let snippets = catalog.enumerate_snippets()?;

    let mut entries: Vec<ListEntry> = snippets
        .into_iter()
        .map(|snippet| ListEntry {
            snippet: snippet.key,
            relative_path: snippet.relative_path,
            title: None,
            description: None,
        })
        .collect();

    entries.sort_by(|a, b| a.snippet.cmp(&b.snippet));
    Ok(entries)
}
