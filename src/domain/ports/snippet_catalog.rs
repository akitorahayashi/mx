use crate::domain::error::AppError;
use crate::domain::snippet::SnippetEntry;

pub trait SnippetCatalog {
    fn enumerate_snippets(&self) -> Result<Vec<SnippetEntry>, AppError>;
    fn resolve_snippet(&self, raw_query: &str) -> Result<SnippetEntry, AppError>;
}
