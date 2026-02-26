use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(snippet: &str) -> Result<(), AppError> {
    let storage = FilesystemSnippetCatalog::from_env()?;
    let api::CopyOutcome { snippet: snippet_key, relative_path, absolute_path } =
        api::copy_snippet(snippet, &storage)?;

    println!("✅ Copied '{snippet_key}' from {relative_path} -> {}", absolute_path.display());
    Ok(())
}
