use crate::domain::error::AppError;
use crate::domain::ports::{CheckoutStatus, SnippetCheckout};
use crate::domain::snippet::SnippetEntry;
use std::fs;
use std::path::Path;

pub struct SymlinkCheckout;

impl SnippetCheckout for SymlinkCheckout {
    fn checkout(
        &self,
        snippet: &SnippetEntry,
        target_root: &Path,
    ) -> Result<CheckoutStatus, AppError> {
        let target_path = target_root.join(format!("{}.md", snippet.relative_path));

        if target_path.exists() || target_path.symlink_metadata().is_ok() {
            return Ok(CheckoutStatus::Skipped(target_path));
        }

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }

        std::os::unix::fs::symlink(&snippet.absolute_path, &target_path)?;
        Ok(CheckoutStatus::Created(target_path))
    }
}

impl SymlinkCheckout {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SymlinkCheckout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::snippet::SnippetEntry;
    use std::fs;
    use tempfile::tempdir;

    fn make_entry(relative: &str, file: &Path) -> SnippetEntry {
        let key = relative.rsplit('/').next().unwrap_or(relative).to_string();
        SnippetEntry { key, relative_path: relative.to_string(), absolute_path: file.to_path_buf() }
    }

    #[test]
    fn creates_symlink_in_target_root() {
        let src_dir = tempdir().unwrap();
        let target_dir = tempdir().unwrap();

        let snippet_file = src_dir.path().join("w").join("wc.md");
        fs::create_dir_all(snippet_file.parent().unwrap()).unwrap();
        fs::write(&snippet_file, "content").unwrap();

        let entry = make_entry("w/wc", &snippet_file);
        let checkout = SymlinkCheckout::new();
        let status = checkout.checkout(&entry, target_dir.path()).unwrap();

        let created = match status {
            CheckoutStatus::Created(p) => p,
            CheckoutStatus::Skipped(_) => panic!("expected Created"),
        };
        assert!(created.exists());
        assert!(created.is_symlink());
        assert_eq!(fs::read_to_string(&created).unwrap(), "content");
    }

    #[test]
    fn skips_existing_symlink() {
        let src_dir = tempdir().unwrap();
        let target_dir = tempdir().unwrap();

        let snippet_file = src_dir.path().join("wc.md");
        fs::write(&snippet_file, "content").unwrap();

        let entry = make_entry("wc", &snippet_file);
        let target = target_dir.path().join("wc.md");
        std::os::unix::fs::symlink(&snippet_file, &target).unwrap();

        let checkout = SymlinkCheckout::new();
        let status = checkout.checkout(&entry, target_dir.path()).unwrap();

        assert!(matches!(status, CheckoutStatus::Skipped(_)));
    }
}
