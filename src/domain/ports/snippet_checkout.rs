use crate::domain::error::AppError;
use crate::domain::snippet::SnippetEntry;
use std::path::{Path, PathBuf};

pub enum CheckoutStatus {
    Created(PathBuf),
    Skipped(PathBuf),
}

pub trait SnippetCheckout {
    fn checkout(
        &self,
        snippet: &SnippetEntry,
        target_root: &Path,
    ) -> Result<CheckoutStatus, AppError>;
}
