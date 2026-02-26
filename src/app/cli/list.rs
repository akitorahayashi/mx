use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run() -> Result<(), AppError> {
    let storage = FilesystemSnippetCatalog::from_env()?;
    let entries = api::list_snippets(&storage)?;
    if entries.is_empty() {
        println!("(no snippets found)");
        return Ok(());
    }

    println!("📚 Available snippets:\n");
    for api::ListEntry { snippet, relative_path, title, description } in entries {
        println!("- {snippet} ({relative_path})");
        if let Some(title) = title {
            println!("  • {title}");
        }
        if let Some(description) = description {
            println!("    {description}");
        }
    }

    Ok(())
}
