pub mod app;
pub mod cli;
pub mod clipboard;
pub mod context_files;
pub mod error;
pub mod project_fs;
pub mod snippets;

pub use app::context_files::{
    cat_context, clean_context, touch_context, CleanOutcome, TouchOutcome,
};
pub use app::snippets::{
    add_snippet, checkout_snippets, copy_snippet, create_command, list_snippets, remove_snippet,
    AddOutcome, CheckoutOutcome, CopyOutcome, CreateCommandOutcome, ListEntry, RemoveOutcome,
};
pub use cli::run as cli;
pub use error::AppError;
