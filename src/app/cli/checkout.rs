use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::app::api;
use crate::domain::error::AppError;

pub(crate) fn run(path: Option<&str>, all: bool) -> Result<(), AppError> {
    let catalog = FilesystemSnippetCatalog::from_env()?;
    let outcome = api::checkout_snippets(path, all, &catalog)?;

    for created in &outcome.created {
        println!("  linked {}", created.display());
    }
    if outcome.skipped > 0 {
        println!("  {} already linked (skipped)", outcome.skipped);
    }
    println!(
        "✅ Checked out {} snippet(s) into {}",
        outcome.created.len(),
        outcome.gitignore_path.parent().map(|p| p.display().to_string()).unwrap_or_default()
    );
    Ok(())
}
