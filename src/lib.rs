pub mod adapters;
mod app;
pub mod domain;

#[cfg(test)]
pub(crate) mod testing;

pub use app::api::{
    cat_context, clean_context, copy_snippet, list_snippets, touch_context, CleanOutcome,
    CopyOutcome, ListEntry, TouchOutcome,
};
pub use app::cli::run as cli;
pub use domain::error::AppError;
