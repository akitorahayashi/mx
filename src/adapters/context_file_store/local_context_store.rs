use crate::domain::error::{AppError, NotFoundError, PathTraversalError};
use crate::domain::ports::{ContextFileStore, ContextWriteStatus};
use crate::domain::SafePath;
use std::ffi::OsStr;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

const GITIGNORE_FILE: &str = ".gitignore";

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
        relative_path: &SafePath,
        force: bool,
    ) -> Result<ContextWriteStatus, AppError> {
        let mx_dir = self.mx_dir();
        if !mx_dir.exists() {
            fs::create_dir(&mx_dir)?;
        }

        let gitignore = mx_dir.join(GITIGNORE_FILE);
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
            return Err(AppError::PathTraversal(PathTraversalError::Detected(
                "Invalid path. Cannot create files outside of .mx directory.".to_string(),
            )));
        }
        fs::write(absolute_path, contents)?;
        Ok(())
    }

    fn read_context_contents(&self, relative_path: &SafePath) -> Result<String, AppError> {
        let full_path = self.mx_dir().join(relative_path);

        if !full_path.is_file() {
            if full_path.exists() {
                return Err(AppError::NotFound(NotFoundError::ContextFile(format!(
                    "⚠️ Path is not a file: {}",
                    relative_path.display()
                ))));
            }

            return Err(AppError::NotFound(NotFoundError::ContextFile(format!(
                "⚠️ Context file not found: {}",
                relative_path.display()
            ))));
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
            for entry in fs::read_dir(&mx_dir)? {
                let entry = entry?;
                if entry.file_name() == OsStr::new(GITIGNORE_FILE) {
                    continue;
                }

                let file_type = entry.file_type()?;
                let path = entry.path();
                if file_type.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }

            return Ok(true);
        }

        Ok(false)
    }

    fn remove_context_file(&self, relative_path: &SafePath) -> Result<PathBuf, AppError> {
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

        Err(AppError::NotFound(crate::domain::error::NotFoundError::File(format!(
            "File not found: {}",
            target_path.display()
        ))))
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
        let status = store.prepare_context_file(&SafePath::try_from_path(Path::new("tasks.md")).unwrap(), true).unwrap();

        assert!(status.existed);
        assert!(status.overwritten);
        assert_eq!(fs::read_to_string(&target).unwrap(), "original");
    }

    fn adapter_rejects_writes_outside_mx() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let outside = workspace.path().join("outside.md");

        let result = store.write_context_contents(&outside, "content");
        assert!(matches!(result, Err(AppError::PathTraversal(PathTraversalError::Detected(_)))));
    }

    #[test]
    fn remove_context_root_preserves_gitignore() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();
        fs::write(mx_dir.join(".gitignore"), "*").unwrap();
        fs::write(mx_dir.join("tasks.md"), "task").unwrap();
        let nested_dir = mx_dir.join("nested");
        fs::create_dir_all(&nested_dir).unwrap();
        fs::write(nested_dir.join("notes.md"), "note").unwrap();

        let removed = store.remove_context_root().unwrap();

        assert!(removed);
        assert!(mx_dir.exists());
        assert!(mx_dir.join(".gitignore").exists());
        assert!(!mx_dir.join("tasks.md").exists());
        assert!(!mx_dir.join("nested").exists());
    }
}
