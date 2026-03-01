mod adapters;
mod app;
mod domain;

#[cfg(test)]
pub(crate) mod testing;

pub use app::api::{
    add_snippet, cat_context, checkout_snippets, clean_context, copy_snippet, create_command,
    list_snippets, remove_snippet, touch_context, AddOutcome, CheckoutOutcome, CleanOutcome,
    CopyOutcome, CreateCommandOutcome, ListEntry, RemoveOutcome, TouchOutcome,
};
pub use app::cli::run as cli;
pub use domain::error::AppError;
