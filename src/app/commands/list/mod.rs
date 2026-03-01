use crate::domain::error::AppError;
use crate::domain::ports::SnippetCatalog;
use crate::domain::snippet::parse_frontmatter_metadata;
use std::fs;

#[derive(Debug, Clone)]
pub struct ListEntry {
    pub snippet: String,
    pub relative_path: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

pub fn execute(catalog: &dyn SnippetCatalog) -> Result<Vec<ListEntry>, AppError> {
    let snippets = catalog.enumerate_snippets()?;

    let mut entries: Vec<ListEntry> = snippets
        .into_iter()
        .map(|snippet| {
            let (title, description) = fs::read_to_string(&snippet.absolute_path)
                .ok()
                .and_then(|content| {
                    let fm = parse_frontmatter_metadata(&content)?;
                    Some((fm.title, fm.description))
                })
                .unwrap_or((None, None));

            ListEntry {
                snippet: snippet.key,
                relative_path: snippet.relative_path,
                title,
                description,
            }
        })
        .collect();

    entries.sort_by(|a, b| a.snippet.cmp(&b.snippet));
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::snippet::SnippetEntry;
    use crate::testing::InMemoryCatalog;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn entry_with_file(key: &str, rel: &str, content: &str) -> (SnippetEntry, TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(format!("{key}.md"));
        fs::write(&path, content).unwrap();
        (
            SnippetEntry {
                key: key.to_string(),
                relative_path: rel.to_string(),
                absolute_path: path,
            },
            dir,
        )
    }

    #[test]
    fn execute_sorts_entries_by_snippet_name() {
        let catalog = InMemoryCatalog::new(vec![
            SnippetEntry {
                key: "wc".to_string(),
                relative_path: "w/wc".to_string(),
                absolute_path: PathBuf::from("commands/w/wc.md"),
            },
            SnippetEntry {
                key: "aa".to_string(),
                relative_path: "a/aa".to_string(),
                absolute_path: PathBuf::from("commands/a/aa.md"),
            },
        ]);

        let entries = execute(&catalog).expect("list command should succeed");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].snippet, "aa");
        assert_eq!(entries[1].snippet, "wc");
    }

    #[test]
    fn execute_reads_title_and_description_from_frontmatter() {
        let (e, _dir) = entry_with_file(
            "wc",
            "w/wc",
            "---\ntitle: Work on Tasks\ndescription: Critical planning\n---\nbody\n",
        );
        let catalog = InMemoryCatalog::new(vec![e]);

        let entries = execute(&catalog).unwrap();
        assert_eq!(entries[0].title.as_deref(), Some("Work on Tasks"));
        assert_eq!(entries[0].description.as_deref(), Some("Critical planning"));
    }

    #[test]
    fn execute_returns_none_title_when_no_frontmatter() {
        let (e, _dir) = entry_with_file("wc", "w/wc", "plain body\n");
        let catalog = InMemoryCatalog::new(vec![e]);

        let entries = execute(&catalog).unwrap();
        assert!(entries[0].title.is_none());
        assert!(entries[0].description.is_none());
    }

    #[test]
    fn execute_handles_mix_of_frontmatter_and_plain_snippets() {
        let (e1, _d1) = entry_with_file("aa", "a/aa", "---\ntitle: AA\n---\nbody\n");
        let (e2, _d2) = entry_with_file("zz", "z/zz", "no frontmatter\n");
        let catalog = InMemoryCatalog::new(vec![e1, e2]);

        let entries = execute(&catalog).unwrap();
        assert_eq!(entries[0].snippet, "aa");
        assert_eq!(entries[0].title.as_deref(), Some("AA"));
        assert_eq!(entries[1].snippet, "zz");
        assert!(entries[1].title.is_none());
    }
}

use crate::adapters::snippet_catalog::FilesystemSnippetCatalog;
use crate::app::api;

#[derive(clap::Args)]
pub struct Cli {}

pub fn run(_args: Cli) -> Result<(), crate::domain::error::AppError> {
    let storage = FilesystemSnippetCatalog::from_env()?;
    let entries = api::list_snippets(&storage)?;
    if entries.is_empty() {
        println!("(no snippets found)");
        return Ok(());
    }

    println!("📚 Available snippets:\n");
    for api::ListEntry { snippet, relative_path, title, description } in entries {
        println!("- {snippet} ({relative_path})");
        if let Some(title) = title {
            println!("  • {title}");
        }
        if let Some(description) = description {
            println!("    {description}");
        }
    }

    Ok(())
}
