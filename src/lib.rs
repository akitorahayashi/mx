pub mod adapters;
mod app;
pub mod domain;

#[cfg(test)]
pub(crate) mod testing;

pub use app::api::{
    add_snippet, cat_context, checkout_snippets, clean_context, copy_snippet, list_snippets,
    remove_snippet, touch_context, AddOutcome, CheckoutOutcome, CleanOutcome, CopyOutcome,
    ListEntry, RemoveOutcome, TouchOutcome,
};
pub use app::cli::run as cli;
pub use domain::error::AppError;
