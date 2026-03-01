use crate::domain::error::AppError;
use crate::domain::ports::{CheckoutStatus, SnippetCatalog, SnippetCheckout};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CheckoutOutcome {
    pub created: Vec<PathBuf>,
    pub skipped: usize,
    pub gitignore_path: PathBuf,
}

pub fn execute(
    query: Option<&str>,
    all: bool,
    catalog: &dyn SnippetCatalog,
    checkout: &dyn SnippetCheckout,
    target_root: &Path,
) -> Result<CheckoutOutcome, AppError> {
    let snippets = if all || query.is_none() {
        catalog.enumerate_snippets()?
    } else {
        let snippet = catalog.resolve_snippet(query.unwrap())?;
        vec![snippet]
    };

    let mut created = Vec::new();
    let mut skipped = 0usize;

    for snippet in &snippets {
        match checkout.checkout(snippet, target_root)? {
            CheckoutStatus::Created(path) => created.push(path),
            CheckoutStatus::Skipped(_) => skipped += 1,
        }
    }

    let gitignore_path = write_gitignore(target_root)?;

    Ok(CheckoutOutcome { created, skipped, gitignore_path })
}

fn write_gitignore(target_root: &Path) -> Result<PathBuf, AppError> {
    fs::create_dir_all(target_root)?;
    let path = target_root.join(".gitignore");
    if !path.exists() {
        fs::write(&path, "*\n")?;
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::snippet::SnippetEntry;
    use crate::testing::{InMemoryCatalog, InMemoryCheckout};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn entry(key: &str, rel: &str) -> SnippetEntry {
        SnippetEntry {
            key: key.to_string(),
            relative_path: rel.to_string(),
            absolute_path: PathBuf::from(format!("/fake/commands/{rel}.md")),
        }
    }

    #[test]
    fn checkout_all_records_each_snippet() {
        let catalog = InMemoryCatalog::new(vec![entry("wc", "w/wc"), entry("aa", "a/aa")]);
        let checkout = InMemoryCheckout::new();
        let dir = tempdir().unwrap();

        let outcome = execute(None, true, &catalog, &checkout, dir.path()).unwrap();
        assert_eq!(outcome.created.len(), 2);
        assert_eq!(outcome.skipped, 0);
        assert!(outcome.gitignore_path.exists());
    }

    #[test]
    fn checkout_individual_snippet() {
        let catalog = InMemoryCatalog::new(vec![entry("wc", "w/wc"), entry("aa", "a/aa")]);
        let checkout = InMemoryCheckout::new();
        let dir = tempdir().unwrap();

        let outcome = execute(Some("wc"), false, &catalog, &checkout, dir.path()).unwrap();
        assert_eq!(outcome.created.len(), 1);
    }

    #[test]
    fn skips_duplicate_checkout() {
        let catalog = InMemoryCatalog::new(vec![entry("wc", "w/wc")]);
        let checkout = InMemoryCheckout::new();
        let dir = tempdir().unwrap();

        execute(None, true, &catalog, &checkout, dir.path()).unwrap();
        let outcome2 = execute(None, true, &catalog, &checkout, dir.path()).unwrap();
        assert_eq!(outcome2.created.len(), 0);
        assert_eq!(outcome2.skipped, 1);
    }

    #[test]
    fn gitignore_is_written_to_target_root() {
        let catalog = InMemoryCatalog::new(vec![]);
        let checkout = InMemoryCheckout::new();
        let dir = tempdir().unwrap();

        let outcome = execute(None, true, &catalog, &checkout, dir.path()).unwrap();
        let contents = std::fs::read_to_string(&outcome.gitignore_path).unwrap();
        assert_eq!(contents, "*\n");
    }
}
