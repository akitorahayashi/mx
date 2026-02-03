use crate::error::AppError;
use std::env;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub(crate) struct SnippetStorage {
    commands_root: PathBuf,
}

impl SnippetStorage {
    pub fn new_default() -> Result<Self, AppError> {
        if let Ok(custom) = env::var("MX_COMMANDS_ROOT") {
            return Self::from_root(PathBuf::from(custom));
        }

        let home = env::var("HOME")
            .map_err(|_| AppError::config_error("HOME environment variable not set"))?;
        let root = PathBuf::from(home).join(".config").join("mx");
        Self::from_root(root)
    }

    pub fn from_root<P: AsRef<Path>>(root: P) -> Result<Self, AppError> {
        let root = root.as_ref().to_path_buf();
        Ok(Self { commands_root: root.join("commands") })
    }

    pub fn enumerate_snippets(&self) -> Result<Vec<SnippetFile>, AppError> {
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

            files.push(SnippetFile { key, relative_path, absolute_path: entry.into_path() });
        }

        files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        Ok(files)
    }

    pub fn resolve_snippet(&self, raw_query: &str) -> Result<SnippetFile, AppError> {
        let normalized = normalize_query(raw_query)?;
        let mut exact_matches = Vec::new();
        let mut key_matches = Vec::new();
        let candidate_key =
            normalized.rsplit('/').next().unwrap_or(normalized.as_str()).to_string();

        for snippet in self.enumerate_snippets()? {
            if snippet.relative_path == normalized {
                exact_matches.push(snippet);
            } else if snippet.key == normalized || snippet.key == candidate_key {
                key_matches.push(snippet);
            }
        }

        if !exact_matches.is_empty() {
            if exact_matches.len() == 1 {
                return Ok(exact_matches.into_iter().next().unwrap());
            }
            return Err(AppError::config_error(format!(
                "Multiple snippets match '{raw_query}': {}",
                join_paths(&exact_matches)
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
                join_paths(&key_matches)
            )));
        }

        Ok(key_matches.into_iter().next().unwrap())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SnippetFile {
    pub key: String,
    pub relative_path: String,
    pub absolute_path: PathBuf,
}

fn path_to_string(path: &Path) -> Result<String, AppError> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(segment) => parts.push(
                segment
                    .to_str()
                    .ok_or_else(|| AppError::config_error("Snippet paths must be UTF-8"))?
                    .to_string(),
            ),
            Component::CurDir => continue,
            _ => {
                return Err(AppError::config_error(
                    "Snippet paths cannot include traversal components",
                ));
            }
        }
    }

    Ok(parts.join("/"))
}

fn normalize_query(raw: &str) -> Result<String, AppError> {
    let trimmed = raw.trim().trim_start_matches('/');
    if trimmed.is_empty() {
        return Err(AppError::config_error("Snippet name cannot be empty"));
    }

    let mut normalized = trimmed.replace('\\', "/");
    if let Some(stripped) = normalized.strip_prefix("commands/") {
        normalized = stripped.to_string();
    }
    if let Some(stripped) = normalized.strip_suffix(".md") {
        normalized = stripped.to_string();
    }

    ensure_safe_segments(&normalized)?;
    Ok(normalized)
}

fn ensure_safe_segments(value: &str) -> Result<(), AppError> {
    if value.split('/').any(|segment| segment.is_empty() || segment == "..") {
        return Err(AppError::config_error(
            "Snippet paths cannot contain empty or traversal segments",
        ));
    }
    Ok(())
}

fn join_paths(snippets: &[SnippetFile]) -> String {
    snippets.iter().map(|s| s.relative_path.clone()).collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_ensure_safe_segments_valid() {
        assert!(ensure_safe_segments("foo").is_ok());
        assert!(ensure_safe_segments("foo/bar").is_ok());
        assert!(ensure_safe_segments("foo-bar_123").is_ok());
    }

    #[test]
    fn test_ensure_safe_segments_traversal() {
        assert!(ensure_safe_segments("..").is_err());
        assert!(ensure_safe_segments("../foo").is_err());
        assert!(ensure_safe_segments("foo/..").is_err());
        assert!(ensure_safe_segments("foo/../bar").is_err());
    }

    #[test]
    fn test_ensure_safe_segments_empty() {
        assert!(ensure_safe_segments("foo//bar").is_err());
        assert!(ensure_safe_segments("/foo").is_err());
    }

    fn create_test_storage(files: &[&str]) -> (SnippetStorage, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let commands_dir = dir.path().join("commands");
        fs::create_dir(&commands_dir).unwrap();

        for path in files {
            let full_path = commands_dir.join(path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(full_path, "content").unwrap();
        }

        let storage = SnippetStorage { commands_root: commands_dir };
        (storage, dir)
    }

    #[test]
    fn test_resolve_snippet_exact() {
        let (storage, _dir) = create_test_storage(&["foo/bar.md"]);
        let snippet = storage.resolve_snippet("foo/bar").unwrap();
        assert_eq!(snippet.relative_path, "foo/bar");
    }

    #[test]
    fn test_resolve_snippet_key() {
        let (storage, _dir) = create_test_storage(&["foo/bar.md"]);
        let snippet = storage.resolve_snippet("bar").unwrap();
        assert_eq!(snippet.relative_path, "foo/bar");
    }

    #[test]
    fn test_resolve_snippet_ambiguous_key() {
        let (storage, _dir) = create_test_storage(&["a/foo.md", "b/foo.md"]);
        let err = storage.resolve_snippet("foo").unwrap_err();
        match err {
            AppError::ConfigError(msg) => assert!(msg.contains("Multiple snippets share the name 'foo'")),
            _ => panic!("Expected ConfigError, got {:?}", err),
        }
    }

    #[test]
    fn test_resolve_snippet_not_found() {
        let (storage, _dir) = create_test_storage(&[]);
        let err = storage.resolve_snippet("foo").unwrap_err();
        match err {
            AppError::NotFound(_) => {},
            _ => panic!("Expected NotFound, got {:?}", err),
        }
    }

    #[test]
    fn test_resolve_snippet_traversal_attack() {
        let (storage, _dir) = create_test_storage(&[]);
        let err = storage.resolve_snippet("../secret").unwrap_err();
        match err {
            AppError::ConfigError(msg) => assert!(msg.contains("Snippet paths cannot contain empty or traversal segments")),
            _ => panic!("Expected ConfigError for traversal, got {:?}", err),
        }
    }

    #[test]
    fn test_resolve_snippet_normalization() {
        let (storage, _dir) = create_test_storage(&["foo.md"]);

        let s1 = storage.resolve_snippet("commands/foo").unwrap();
        assert_eq!(s1.relative_path, "foo");

        let s2 = storage.resolve_snippet("foo.md").unwrap();
        assert_eq!(s2.relative_path, "foo");

        let s3 = storage.resolve_snippet("/foo").unwrap();
        assert_eq!(s3.relative_path, "foo");
    }
}
