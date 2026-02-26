use crate::domain::error::AppError;
use crate::domain::snippet::query::{candidate_key, normalize_query, path_to_string};
use crate::domain::snippet::SnippetEntry;
use crate::ports::SnippetCatalog;
use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct FilesystemSnippetCatalog {
    commands_root: PathBuf,
}

impl FilesystemSnippetCatalog {
    pub fn from_env() -> Result<Self, AppError> {
        if let Ok(custom) = env::var("MX_COMMANDS_ROOT") {
            return Self::from_root(PathBuf::from(custom));
        }

        let home = env::var("HOME")
            .map_err(|_| AppError::config_error("HOME environment variable not set"))?;
        let root = PathBuf::from(home).join(".config").join("mx");
        Self::from_root(root)
    }

    pub fn from_root<P: AsRef<Path>>(root: P) -> Result<Self, AppError> {
        Ok(Self { commands_root: root.as_ref().join("commands") })
    }

    fn join_paths(snippets: &[SnippetEntry]) -> String {
        snippets.iter().map(|snippet| snippet.relative_path.clone()).collect::<Vec<_>>().join(", ")
    }
}

impl SnippetCatalog for FilesystemSnippetCatalog {
    fn enumerate_snippets(&self) -> Result<Vec<SnippetEntry>, AppError> {
        if !self.commands_root.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        for entry in WalkDir::new(&self.commands_root) {
            let entry = entry.map_err(|err| AppError::config_error(err.to_string()))?;
            if !entry.file_type().is_file() {
                continue;
            }
            if entry.path().extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }

            let relative = entry
                .path()
                .strip_prefix(&self.commands_root)
                .map_err(|_| AppError::config_error("Unable to derive relative snippet path"))?;
            let relative_without_ext = relative.with_extension("");
            let relative_path = path_to_string(&relative_without_ext)?;
            let key = entry
                .path()
                .file_stem()
                .and_then(|stem| stem.to_str())
                .ok_or_else(|| AppError::config_error("Snippet names must be valid UTF-8"))?
                .to_string();

            files.push(SnippetEntry { key, relative_path, absolute_path: entry.into_path() });
        }

        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(files)
    }

    fn resolve_snippet(&self, raw_query: &str) -> Result<SnippetEntry, AppError> {
        let normalized = normalize_query(raw_query)?;
        let query_key = candidate_key(&normalized);

        let mut exact_matches = Vec::new();
        let mut key_matches = Vec::new();

        for snippet in self.enumerate_snippets()? {
            if snippet.relative_path == normalized {
                exact_matches.push(snippet);
            } else if snippet.key == normalized || snippet.key == query_key {
                key_matches.push(snippet);
            }
        }

        if exact_matches.len() == 1 {
            return Ok(exact_matches.remove(0));
        }

        if exact_matches.len() > 1 {
            return Err(AppError::config_error(format!(
                "Multiple snippets match '{raw_query}': {}",
                Self::join_paths(&exact_matches)
            )));
        }

        if key_matches.is_empty() {
            return Err(AppError::not_found(format!(
                "No snippet named '{raw_query}' under {}",
                self.commands_root.display()
            )));
        }

        if key_matches.len() > 1 {
            return Err(AppError::config_error(format!(
                "Multiple snippets share the name '{raw_query}': {}",
                Self::join_paths(&key_matches)
            )));
        }

        Ok(key_matches.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn create_catalog(files: &[&str]) -> (FilesystemSnippetCatalog, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let commands = dir.path().join("commands");
        fs::create_dir_all(&commands).unwrap();

        for file in files {
            let path = commands.join(file);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(path, "content").unwrap();
        }

        (FilesystemSnippetCatalog { commands_root: commands }, dir)
    }

    #[test]
    fn resolves_exact_and_key_queries() {
        let (catalog, _dir) = create_catalog(&["w/wc.md"]);
        assert_eq!(catalog.resolve_snippet("w/wc").unwrap().relative_path, "w/wc");
        assert_eq!(catalog.resolve_snippet("wc").unwrap().relative_path, "w/wc");
    }
}
