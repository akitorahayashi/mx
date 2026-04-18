use crate::app::snippets;
use crate::error::AppError;
use crate::snippets::FilesystemSnippetCatalog;

pub(crate) fn run() -> Result<(), AppError> {
    let storage = FilesystemSnippetCatalog::from_env()?;
    let entries = snippets::list_snippets(&storage)?;
    if entries.is_empty() {
        println!("(no snippets found)");
        return Ok(());
    }

    println!("📚 Available snippets:\n");
    for snippets::ListEntry { snippet, relative_path, title, description } in entries {
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
