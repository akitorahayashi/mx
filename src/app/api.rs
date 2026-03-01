use crate::adapters::clipboard::clipboard_from_env;
use crate::app::commands;
use crate::domain::error::AppError;
use crate::domain::ports::{ContextFileStore, SnippetCatalog, SnippetCheckout, SnippetStore};
use std::path::Path;

pub use crate::app::commands::add::AddOutcome;
pub use crate::app::commands::checkout::CheckoutOutcome;
pub use crate::app::commands::clean::CleanOutcome;
pub use crate::app::commands::copy::CopyOutcome;
pub use crate::app::commands::create_command::CreateCommandOutcome;
pub use crate::app::commands::list::ListEntry;
pub use crate::app::commands::remove::RemoveOutcome;
pub use crate::app::commands::touch::TouchOutcome;

pub fn cat_context(key: &str, store: &impl ContextFileStore) -> Result<String, AppError> {
    commands::cat::execute(key, store)
}

pub fn clean_context(
    key: Option<String>,
    store: &impl ContextFileStore,
) -> Result<CleanOutcome, AppError> {
    commands::clean::execute(key, store)
}

pub fn copy_snippet(
    snippet: &str,
    catalog: &impl SnippetCatalog,
    store: Option<&impl ContextFileStore>,
) -> Result<CopyOutcome, AppError> {
    let clipboard = clipboard_from_env()?;

    commands::copy::execute(
        snippet,
        catalog,
        clipboard.as_ref(),
        store.map(|s| s as &dyn ContextFileStore),
    )
}

pub fn list_snippets(catalog: &impl SnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    commands::list::execute(catalog)
}

pub fn touch_context(
    key: &str,
    force: bool,
    store: &impl ContextFileStore,
) -> Result<TouchOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    commands::touch::execute(key, force, store, clipboard.as_ref())
}

pub fn checkout_snippets(
    query: Option<&str>,
    all: bool,
    catalog: &impl SnippetCatalog,
    checkout: &impl SnippetCheckout,
    target_root: &Path,
) -> Result<CheckoutOutcome, AppError> {
    commands::checkout::execute(query, all, catalog, checkout, target_root)
}

pub fn add_snippet(
    path: &str,
    title: Option<&str>,
    description: Option<&str>,
    force: bool,
    store: &impl SnippetStore,
) -> Result<AddOutcome, AppError> {
    let clipboard = clipboard_from_env()?;
    commands::add::execute(path, title, description, force, store, clipboard.as_ref())
}

pub fn remove_snippet(
    snippet: &str,
    catalog: &impl SnippetCatalog,
    store: &impl SnippetStore,
) -> Result<RemoveOutcome, AppError> {
    commands::remove::execute(snippet, catalog, store)
}

pub fn create_command(
    path: &str,
    force: bool,
    store: &impl SnippetStore,
) -> Result<CreateCommandOutcome, AppError> {
    commands::create_command::execute(path, force, store)
}
