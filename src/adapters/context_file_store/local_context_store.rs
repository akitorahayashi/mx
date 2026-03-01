use crate::domain::context_file::path_policy::validate_relative_components;
use crate::domain::error::AppError;
use crate::domain::ports::{ContextFileStore, ContextWriteStatus};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LocalContextFileStore {
    workspace_root: PathBuf,
}

impl LocalContextFileStore {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self { workspace_root }
    }

    fn mx_dir(&self) -> PathBuf {
        self.workspace_root.join(".mx")
    }
}

impl ContextFileStore for LocalContextFileStore {
    fn prepare_context_file(
        &self,
        relative_path: &Path,
        force: bool,
    ) -> Result<ContextWriteStatus, AppError> {
        validate_relative_components(relative_path)?;

        let mx_dir = self.mx_dir();
        if !mx_dir.exists() {
            fs::create_dir(&mx_dir)?;
        }

        let gitignore = mx_dir.join(".gitignore");
        if !gitignore.exists() {
            let mut file = OpenOptions::new().write(true).create_new(true).open(&gitignore)?;
            writeln!(file, "*")?;
        }

        let target_path = mx_dir.join(relative_path);
        if let Some(parent) = target_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let (existed, overwritten) = if force {
            let exists = target_path.exists();
            (exists, exists)
        } else {
            match OpenOptions::new().write(true).create_new(true).open(&target_path) {
                Ok(_) => (false, false),
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => (true, false),
                Err(err) => return Err(err.into()),
            }
        };

        Ok(ContextWriteStatus { path: target_path, existed, overwritten })
    }

    fn write_context_contents(&self, absolute_path: &Path, contents: &str) -> Result<(), AppError> {
        if !absolute_path.starts_with(self.mx_dir()) {
            return Err(AppError::path_traversal(
                "Invalid path. Cannot create files outside of .mx directory.",
            ));
        }
        fs::write(absolute_path, contents)?;
        Ok(())
    }

    fn read_context_contents(&self, relative_path: &Path) -> Result<String, AppError> {
        validate_relative_components(relative_path)?;
        let full_path = self.mx_dir().join(relative_path);

        if !full_path.is_file() {
            if full_path.exists() {
                return Err(AppError::not_found(format!(
                    "⚠️ Path is not a file: {}",
                    relative_path.display()
                )));
            }

            return Err(AppError::not_found(format!(
                "⚠️ Context file not found: {}",
                relative_path.display()
            )));
        }

        fs::read_to_string(&full_path).map_err(|err| {
            AppError::Io(std::io::Error::new(
                err.kind(),
                format!("Failed to read {}: {}", relative_path.display(), err),
            ))
        })
    }

    fn remove_context_root(&self) -> Result<bool, AppError> {
        let mx_dir = self.mx_dir();
        if mx_dir.exists() {
            fs::remove_dir_all(&mx_dir)?;
            return Ok(true);
        }

        Ok(false)
    }

    fn remove_context_file(&self, relative_path: &Path) -> Result<PathBuf, AppError> {
        validate_relative_components(relative_path)?;
        let mx_dir = self.mx_dir();
        let target_path = mx_dir.join(relative_path);

        if target_path.exists() {
            fs::remove_file(&target_path)?;

            if let Some(parent) = target_path.parent() {
                for candidate in parent.ancestors() {
                    if !candidate.starts_with(&mx_dir) || candidate == mx_dir {
                        break;
                    }
                    if fs::remove_dir(candidate).is_err() {
                        break;
                    }
                }
            }

            return Ok(target_path);
        }

        Err(AppError::not_found(format!("File not found: {}", target_path.display())))
    }

    fn read_workspace_file(&self, relative_path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(self.workspace_root.join(relative_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn force_prepare_does_not_truncate_existing_file() {
        let workspace = tempdir().unwrap();
        let mx_dir = workspace.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();
        let target = mx_dir.join("tasks.md");
        fs::write(&target, "original").unwrap();

        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let status = store.prepare_context_file(Path::new("tasks.md"), true).unwrap();

        assert!(status.existed);
        assert!(status.overwritten);
        assert_eq!(fs::read_to_string(&target).unwrap(), "original");
    }

    #[test]
    fn adapter_rejects_unsafe_relative_paths() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());

        let result = store.prepare_context_file(Path::new("../escape.md"), false);
        assert!(matches!(result, Err(AppError::PathTraversal(_))));
    }

    #[test]
    fn adapter_rejects_writes_outside_mx() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let outside = workspace.path().join("outside.md");

        let result = store.write_context_contents(&outside, "content");
        assert!(matches!(result, Err(AppError::PathTraversal(_))));
    }
}
