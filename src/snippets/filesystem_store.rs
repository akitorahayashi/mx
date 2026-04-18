use crate::error::{AppError, ConfigError, NotFoundError};
use crate::project_fs::SafePath;
use crate::snippets::SnippetStore;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FilesystemSnippetStore {
    commands_root: PathBuf,
}

impl FilesystemSnippetStore {
    pub fn from_env() -> Result<Self, AppError> {
        if let Ok(custom) = env::var("MX_COMMANDS_ROOT") {
            let custom_path = PathBuf::from(custom);
            let legacy_commands_root = custom_path.join("commands");
            let commands_root =
                if legacy_commands_root.is_dir() { legacy_commands_root } else { custom_path };
            return Ok(Self { commands_root });
        }

        let home = env::var("HOME")
            .map_err(|_| AppError::ConfigError(ConfigError::MissingEnvVar("HOME".to_string())))?;
        let root = PathBuf::from(home).join(".config").join("mx");
        Ok(Self { commands_root: root.join("commands") })
    }

    pub fn from_root<P: AsRef<Path>>(root: P) -> Self {
        Self { commands_root: root.as_ref().join("commands") }
    }
}

impl SnippetStore for FilesystemSnippetStore {
    fn write_snippet(&self, relative_path: &SafePath, contents: &str) -> Result<PathBuf, AppError> {
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

    fn snippet_exists(&self, relative_path: &SafePath) -> bool {
        let target = if relative_path.extension().is_some() {
            self.commands_root.join(relative_path)
        } else {
            self.commands_root.join(relative_path).with_extension("md")
        };
        target.exists()
    }

    fn remove_snippet(&self, relative_path: &SafePath) -> Result<PathBuf, AppError> {
        let target = if relative_path.extension().is_some() {
            self.commands_root.join(relative_path)
        } else {
            self.commands_root.join(relative_path).with_extension("md")
        };

        if !target.exists() {
            return Err(AppError::NotFound(NotFoundError::Snippet(format!(
                "Snippet file not found: {}",
                target.display()
            ))));
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
    use std::ffi::OsString;
    use tempfile::tempdir;

    struct EnvGuard {
        key: &'static str,
        original: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: &Path) -> Self {
            let original = env::var_os(key);
            env::set_var(key, value);
            Self { key, original }
        }

        fn remove(key: &'static str) -> Self {
            let original = env::var_os(key);
            env::remove_var(key);
            Self { key, original }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.original {
                env::set_var(self.key, value);
            } else {
                env::remove_var(self.key);
            }
        }
    }

    #[test]
    #[serial]
    fn from_env_default_resolves_to_home_config_mx_commands() {
        let _env_remove_root = EnvGuard::remove("MX_COMMANDS_ROOT");
        let dir = tempdir().unwrap();
        let _env_home = EnvGuard::set("HOME", dir.path());

        let store = FilesystemSnippetStore::from_env().unwrap();
        assert_eq!(store.commands_root, dir.path().join(".config").join("mx").join("commands"));
    }

    #[test]
    #[serial]
    fn from_env_with_mx_commands_root_resolves_to_custom_path() {
        let dir = tempdir().unwrap();
        let custom_root = dir.path().join("my_custom_root");
        let _env_root = EnvGuard::set("MX_COMMANDS_ROOT", &custom_root);

        let store = FilesystemSnippetStore::from_env().unwrap();
        assert_eq!(store.commands_root, custom_root);
    }

    #[test]
    #[serial]
    fn from_env_with_mx_commands_root_resolves_to_legacy_commands_subdir() {
        let dir = tempdir().unwrap();
        let custom_root = dir.path().join("my_custom_root");
        let legacy_dir = custom_root.join("commands");
        fs::create_dir_all(&legacy_dir).unwrap();
        let _env_root = EnvGuard::set("MX_COMMANDS_ROOT", &custom_root);

        let store = FilesystemSnippetStore::from_env().unwrap();
        assert_eq!(store.commands_root, legacy_dir);
    }

    #[test]
    #[serial]
    fn from_env_fails_when_home_not_set() {
        let _env_remove_root = EnvGuard::remove("MX_COMMANDS_ROOT");
        let _env_remove_home = EnvGuard::remove("HOME");

        let result = FilesystemSnippetStore::from_env();
        assert!(matches!(
            result,
            Err(AppError::ConfigError(ConfigError::MissingEnvVar(ref key))) if key == "HOME"
        ));
    }
}
