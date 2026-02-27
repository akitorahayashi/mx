use crate::domain::error::AppError;
use crate::domain::ports::SnippetCatalog;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::snippet::SnippetEntry;
    use crate::testing::InMemoryCatalog;
    use std::path::PathBuf;

    #[test]
    fn execute_sorts_entries_by_snippet_name() {
        let catalog = InMemoryCatalog::new(vec![
            SnippetEntry {
                key: "wc".to_string(),
                relative_path: "w/wc".to_string(),
                absolute_path: PathBuf::from("commands/w/wc.md"),
            },
            SnippetEntry {
                key: "aa".to_string(),
                relative_path: "a/aa".to_string(),
                absolute_path: PathBuf::from("commands/a/aa.md"),
            },
        ]);

        let entries = execute(&catalog).expect("list command should succeed");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].snippet, "aa");
        assert_eq!(entries[1].snippet, "wc");
    }
}
