//! Library entry point exposing the mx CLI command handlers.

pub mod error;

mod commands;
mod storage;

use commands::cat;
use commands::clean;
use commands::clipboard::clipboard_from_env;
use commands::copy_snippet::CopySnippet;
use commands::list_snippets;
use commands::touch;
use error::AppError;
use storage::SnippetStorage;

pub use commands::clean::CleanOutcome;
pub use commands::copy_snippet::CopyOutcome;
pub use commands::list_snippets::ListEntry;
pub use commands::touch::TouchOutcome;

/// Displays the contents of a context file from the `.mx/` directory.
///
/// This command reuses the same path resolution logic as `touch`, supporting:
/// - Predefined aliases (tk, rq, pdt, etc.)
/// - Dynamic numbered aliases (tk1, tk2, etc.)
/// - Pending prefix (pd-tk, pd-rq, etc.)
/// - Custom relative paths with automatic .md extension
///
/// # Arguments
///
/// * `key` - The key to resolve to a file path (e.g., "tk", "rq", "docs/spec")
///
/// # Returns
///
/// The file contents as a String, or an error if:
/// - The file does not exist
/// - Path traversal is attempted
/// - The project root cannot be found
///
/// # Examples
///
/// ```no_run
/// use mx::cat_context;
///
/// // Read the tasks file
/// let content = cat_context("tk").expect("Failed to read tasks");
/// println!("{}", content);
/// ```
pub fn cat_context(key: &str) -> Result<String, AppError> {
    cat::cat(key)
}

pub fn clean_context(key: Option<String>) -> Result<CleanOutcome, AppError> {
    clean::clean(key)
}

pub fn copy_snippet(query: &str) -> Result<CopyOutcome, AppError> {
    let storage = SnippetStorage::new_default()?;
    let clipboard = clipboard_from_env()?;
    CopySnippet { query }.execute(&storage, clipboard.as_ref())
}

pub fn list_snippets() -> Result<Vec<ListEntry>, AppError> {
    let storage = SnippetStorage::new_default()?;
    list_snippets::list(&storage)
}

pub fn touch_context(key: &str, force: bool) -> Result<TouchOutcome, AppError> {
    touch::touch(key, force)
}
