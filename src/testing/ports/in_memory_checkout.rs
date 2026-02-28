use crate::domain::error::AppError;
use crate::domain::ports::{CheckoutStatus, SnippetCheckout};
use crate::domain::snippet::SnippetEntry;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct InMemoryCheckout {
    checked_out: Mutex<HashSet<PathBuf>>,
}

impl InMemoryCheckout {
    pub fn new() -> Self {
        Self { checked_out: Mutex::new(HashSet::new()) }
    }
}

impl Default for InMemoryCheckout {
    fn default() -> Self {
        Self::new()
    }
}

impl SnippetCheckout for InMemoryCheckout {
    fn checkout(
        &self,
        snippet: &SnippetEntry,
        target_root: &Path,
    ) -> Result<CheckoutStatus, AppError> {
        let target = target_root.join(format!("{}.md", snippet.relative_path));
        let mut set = self.checked_out.lock().unwrap();
        if set.contains(&target) {
            return Ok(CheckoutStatus::Skipped(target));
        }
        set.insert(target.clone());
        Ok(CheckoutStatus::Created(target))
    }
}
