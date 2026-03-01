use crate::domain::error::AppError;
use crate::domain::ports::SnippetStore;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FilesystemSnippetStore {
    commands_root: PathBuf,
}

impl FilesystemSnippetStore {
    pub fn from_env() -> Result<Self, AppError> {
        if let Ok(custom) = env::var("MX_COMMANDS_ROOT") {
            let custom_path = PathBuf::from(custom);
            let legacy = custom_path.join("commands");
            let commands_root = if legacy.is_dir() { legacy } else { custom_path };
            return Ok(Self { commands_root });
        }

        let home = env::var("HOME")
            .map_err(|_| AppError::config_error("HOME environment variable not set"))?;
        let root = PathBuf::from(home).join(".config").join("mx");
        Ok(Self { commands_root: root.join("commands") })
    }

    pub fn from_root<P: AsRef<Path>>(root: P) -> Self {
        Self { commands_root: root.as_ref().join("commands") }
    }
}

impl SnippetStore for FilesystemSnippetStore {
    fn write_snippet(&self, relative_path: &Path, contents: &str) -> Result<PathBuf, AppError> {
        let target = if relative_path.extension().is_some() {
            self.commands_root.join(relative_path)
        } else {
            self.commands_root.join(relative_path).with_extension("md")
        };

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target, contents)?;
        Ok(target)
    }

    fn snippet_exists(&self, relative_path: &Path) -> bool {
        let target = if relative_path.extension().is_some() {
            self.commands_root.join(relative_path)
        } else {
            self.commands_root.join(relative_path).with_extension("md")
        };
        target.exists()
    }

    fn remove_snippet(&self, relative_path: &Path) -> Result<PathBuf, AppError> {
        let target = if relative_path.extension().is_some() {
            self.commands_root.join(relative_path)
        } else {
            self.commands_root.join(relative_path).with_extension("md")
        };

        if !target.exists() {
            return Err(AppError::not_found(format!(
                "Snippet file not found: {}",
                target.display()
            )));
        }

        fs::remove_file(&target)?;

        // Remove empty parent directories up to commands_root
        let mut dir = target.parent();
        while let Some(parent) = dir {
            if parent == self.commands_root {
                break;
            }
            let is_empty = fs::read_dir(parent).map(|mut d| d.next().is_none()).unwrap_or(false);
            if is_empty {
                let _ = fs::remove_dir(parent);
                dir = parent.parent();
            } else {
                break;
            }
        }

        Ok(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::tempdir;

    #[test]
    fn test_remove_snippet_cleans_up_empty_directories() {
        let dir = tempdir().unwrap();
        let store = FilesystemSnippetStore::from_root(dir.path());

        let snippet_path = PathBuf::from("a/b/c.md");
        store.write_snippet(&snippet_path, "content").unwrap();

        let removed_path = store.remove_snippet(&snippet_path).unwrap();
        assert_eq!(removed_path, dir.path().join("commands").join("a/b/c.md"));

        assert!(!removed_path.exists());
        assert!(!dir.path().join("commands").join("a/b").exists());
        assert!(!dir.path().join("commands").join("a").exists());
        assert!(dir.path().join("commands").exists());
    }

    #[test]
    fn test_remove_snippet_leaves_non_empty_directories() {
        let dir = tempdir().unwrap();
        let store = FilesystemSnippetStore::from_root(dir.path());

        let snippet_path1 = PathBuf::from("a/b/c1.md");
        let snippet_path2 = PathBuf::from("a/b/c2.md");
        store.write_snippet(&snippet_path1, "content1").unwrap();
        store.write_snippet(&snippet_path2, "content2").unwrap();

        store.remove_snippet(&snippet_path1).unwrap();

        assert!(!dir.path().join("commands").join("a/b/c1.md").exists());
        assert!(dir.path().join("commands").join("a/b/c2.md").exists());
        assert!(dir.path().join("commands").join("a/b").exists());
        assert!(dir.path().join("commands").join("a").exists());
    }

    #[test]
    #[serial]
    fn test_from_env_missing_home() {
        // Save current environment variables to restore them later.
        let old_mx_commands_root = env::var_os("MX_COMMANDS_ROOT");
        let old_home = env::var_os("HOME");

        env::remove_var("MX_COMMANDS_ROOT");
        env::remove_var("HOME");

        let result = FilesystemSnippetStore::from_env();

        assert!(result.is_err());
        if let Err(AppError::ConfigError(message)) = result {
            assert_eq!(message, "HOME environment variable not set");
        } else {
            panic!("Expected AppError::ConfigError, got something else");
        }

        // Restore environment variables
        if let Some(val) = old_mx_commands_root {
            env::set_var("MX_COMMANDS_ROOT", val);
        }
        if let Some(val) = old_home {
            env::set_var("HOME", val);
        }
    }

    #[test]
    #[serial]
    fn test_from_env_with_mx_commands_root() {
        let old_mx_commands_root = env::var_os("MX_COMMANDS_ROOT");

        env::set_var("MX_COMMANDS_ROOT", "/custom/path");

        let store = FilesystemSnippetStore::from_env().unwrap();
        assert_eq!(store.commands_root, PathBuf::from("/custom/path"));

        // Restore
        if let Some(val) = old_mx_commands_root {
            env::set_var("MX_COMMANDS_ROOT", val);
        } else {
            env::remove_var("MX_COMMANDS_ROOT");
        }
    }

    #[test]
    #[serial]
    fn test_from_env_with_home() {
        let old_mx_commands_root = env::var_os("MX_COMMANDS_ROOT");
        let old_home = env::var_os("HOME");

        env::remove_var("MX_COMMANDS_ROOT");
        env::set_var("HOME", "/home/user");

        let store = FilesystemSnippetStore::from_env().unwrap();
        assert_eq!(store.commands_root, PathBuf::from("/home/user/.config/mx/commands"));

        // Restore
        if let Some(val) = old_mx_commands_root {
            env::set_var("MX_COMMANDS_ROOT", val);
        }
        if let Some(val) = old_home {
            env::set_var("HOME", val);
        } else {
            env::remove_var("HOME");
        }
    }

    #[test]
    fn test_remove_snippet_not_found() {
        let dir = tempdir().unwrap();
        let store = FilesystemSnippetStore::from_root(dir.path());

        let snippet_path = PathBuf::from("nonexistent.md");
        let result = store.remove_snippet(&snippet_path);

        assert!(result.is_err());
        if let Err(AppError::NotFound(message)) = result {
            assert!(message.contains("Snippet file not found"));
        } else {
            panic!("Expected AppError::NotFound, got {:?}", result);
        }
    }
}
