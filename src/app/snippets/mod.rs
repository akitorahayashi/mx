pub mod add;
pub mod checkout;
pub mod copy;
pub mod create_command;
pub mod list;
pub mod remove;

use crate::clipboard::clipboard_from_env;
use crate::error::AppError;
use crate::project_fs::WorkspaceFileReader;
use crate::snippets::{SnippetCatalog, SnippetCheckout, SnippetStore};
use std::path::Path;

pub use add::AddOutcome;
pub use checkout::CheckoutOutcome;
pub use copy::CopyOutcome;
pub use create_command::CreateCommandOutcome;
pub use list::ListEntry;
pub use remove::RemoveOutcome;

pub fn add_snippet(
    path: &str,
    title: Option<&str>,
    description: Option<&str>,
    force: bool,
    store: &impl SnippetStore,
) -> Result<AddOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    add::execute(path, title, description, force, store, clipboard.as_ref())
}

pub fn checkout_snippets(
    query: Option<&str>,
    all: bool,
    catalog: &impl SnippetCatalog,
    checkout: &impl SnippetCheckout,
    target_root: &Path,
) -> Result<CheckoutOutcome, AppError> {
    checkout::execute(query, all, catalog, checkout, target_root)
}

pub fn copy_snippet(
    snippet: &str,
    catalog: &impl SnippetCatalog,
    workspace_files: Option<&impl WorkspaceFileReader>,
) -> Result<CopyOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    copy::execute(
        snippet,
        catalog,
        clipboard.as_ref(),
        workspace_files.map(|store| store as &dyn WorkspaceFileReader),
    )
}

pub fn create_command(
    path: &str,
    force: bool,
    store: &impl SnippetStore,
) -> Result<CreateCommandOutcome, AppError> {
    create_command::execute(path, force, store)
}

pub fn list_snippets(catalog: &impl SnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    list::execute(catalog)
}

pub fn remove_snippet(
    snippet: &str,
    catalog: &impl SnippetCatalog,
    store: &impl SnippetStore,
) -> Result<RemoveOutcome, AppError> {
    remove::execute(snippet, catalog, store)
}
