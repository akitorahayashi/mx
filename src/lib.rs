pub mod app;
pub mod cli;
pub mod clipboard;
pub mod context_files;
pub mod error;
pub mod project_fs;
pub mod snippets;

pub use app::{
    cat_context, clean_context, copy_snippet, list_snippets, touch_context, which_path,
    CleanOutcome, CopyOutcome, ListEntry, TouchOutcome, WhichOutcome,
};
pub use cli::run as cli;
pub use error::AppError;
