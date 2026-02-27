use crate::domain::error::AppError;
use crate::domain::ports::SnippetCatalog;
use crate::domain::snippet::SnippetEntry;

pub struct InMemoryCatalog {
    entries: Vec<SnippetEntry>,
}

impl InMemoryCatalog {
    pub fn new(entries: Vec<SnippetEntry>) -> Self {
        Self { entries }
    }
}

impl SnippetCatalog for InMemoryCatalog {
    fn enumerate_snippets(&self) -> Result<Vec<SnippetEntry>, AppError> {
        Ok(self.entries.clone())
    }

    fn resolve_snippet(&self, raw_query: &str) -> Result<SnippetEntry, AppError> {
        let exact = self.entries.iter().find(|entry| entry.relative_path == raw_query);
        if let Some(found) = exact {
            return Ok(found.clone());
        }

        let key_match: Vec<&SnippetEntry> =
            self.entries.iter().filter(|entry| entry.key == raw_query).collect();

        if key_match.len() == 1 {
            return Ok(key_match[0].clone());
        }

        if key_match.len() > 1 {
            return Err(AppError::config_error(format!(
                "Multiple snippets share the name '{raw_query}'",
            )));
        }

        Err(AppError::not_found(format!("No snippet named '{raw_query}'")))
    }
}
