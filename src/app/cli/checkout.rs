use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::adapters::snippet_checkout::SymlinkCheckout;
use crate::adapters::workspace_locator::CurrentDirectoryLocator;
use crate::app::api;
use crate::domain::error::AppError;
use crate::domain::ports::WorkspaceLocator;

pub(crate) fn run(path: Option<&str>, all: bool) -> Result<(), AppError> {
    let catalog = FilesystemSnippetCatalog::from_env()?;
    let workspace_root = CurrentDirectoryLocator.find_workspace_root()?;
    let target_root = workspace_root.join(".mx").join("commands");
    let checkout = SymlinkCheckout::new();

    let outcome = api::checkout_snippets(path, all, &catalog, &checkout, &target_root)?;

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
