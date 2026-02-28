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
