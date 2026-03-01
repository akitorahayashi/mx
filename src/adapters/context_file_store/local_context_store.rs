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

    #[test]
    fn read_context_contents_returns_not_found_when_missing() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());

        let result = store.read_context_contents(Path::new("missing.md"));

        match result {
            Err(AppError::NotFound(msg)) => {
                assert!(msg.contains("Context file not found"));
                assert!(msg.contains("missing.md"));
            }
            _ => panic!("Expected NotFound error for missing file"),
        }
    }

    #[test]
    fn read_context_contents_returns_not_found_when_directory() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");
        let dir_path = mx_dir.join("some_dir");
        fs::create_dir_all(&dir_path).unwrap();

        let result = store.read_context_contents(Path::new("some_dir"));

        match result {
            Err(AppError::NotFound(msg)) => {
                assert!(msg.contains("Path is not a file"));
                assert!(msg.contains("some_dir"));
            }
            _ => panic!("Expected NotFound error for directory path"),
        }
    }

    #[test]
    fn remove_context_root_returns_true_when_removed() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();
        let file_path = mx_dir.join("some_file.md");
        fs::write(&file_path, "content").unwrap();

        assert!(mx_dir.exists());
        let result = store.remove_context_root().unwrap();

        assert!(result);
        assert!(!mx_dir.exists());
    }

    #[test]
    fn remove_context_root_returns_false_when_missing() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");

        assert!(!mx_dir.exists());
        let result = store.remove_context_root().unwrap();

        assert!(!result);
    }

    #[test]
    fn remove_context_root_propagates_io_error() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();

        // We'll make the directory read-only so that it cannot be deleted.
        // On Unix, lack of write permission on the directory stops deletion of items inside it,
        // and lack of write on parent stops deletion of the directory itself.
        // Let's create a file inside and make the `.mx` directory read-only so `remove_dir_all` fails.
        let file_path = mx_dir.join("some_file.md");
        fs::write(&file_path, "content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&mx_dir).unwrap().permissions();
            // Remove write permissions (read-execute only)
            perms.set_mode(0o555);
            fs::set_permissions(&mx_dir, perms).unwrap();

            let result = store.remove_context_root();
            match result {
                Err(AppError::Io(err)) => {
                    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
                }
                _ => panic!("Expected Io error for permission denied when removing root"),
            }

            // Restore permissions so tempdir cleanup succeeds
            let mut perms = fs::metadata(&mx_dir).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&mx_dir, perms).unwrap();
        }
    }

    #[test]
    fn remove_context_file_removes_empty_ancestor_directories_but_keeps_mx() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");

        // Structure: .mx/a/b/c/file.md
        let c_dir = mx_dir.join("a").join("b").join("c");
        fs::create_dir_all(&c_dir).unwrap();
        let file_path = c_dir.join("file.md");
        fs::write(&file_path, "content").unwrap();

        let removed_path = store.remove_context_file(Path::new("a/b/c/file.md")).unwrap();
        assert_eq!(removed_path, file_path);

        assert!(!file_path.exists());
        assert!(!c_dir.exists());
        assert!(!mx_dir.join("a").join("b").exists());
        assert!(!mx_dir.join("a").exists());
        assert!(mx_dir.exists()); // .mx should be kept
    }

    #[test]
    fn remove_context_file_stops_removing_ancestors_when_not_empty() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");

        // Structure:
        // .mx/a/b/c/file.md
        // .mx/a/b/other.md
        let c_dir = mx_dir.join("a").join("b").join("c");
        fs::create_dir_all(&c_dir).unwrap();
        let file_path = c_dir.join("file.md");
        fs::write(&file_path, "content").unwrap();

        let other_path = mx_dir.join("a").join("b").join("other.md");
        fs::write(&other_path, "other").unwrap();

        let removed_path = store.remove_context_file(Path::new("a/b/c/file.md")).unwrap();
        assert_eq!(removed_path, file_path);

        assert!(!file_path.exists());
        assert!(!c_dir.exists()); // c should be removed
        assert!(mx_dir.join("a").join("b").exists()); // b should be kept because of other.md
        assert!(other_path.exists());
    }

    #[test]
    fn remove_context_file_returns_not_found_when_missing() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();

        let result = store.remove_context_file(Path::new("missing.md"));
        match result {
            Err(AppError::NotFound(msg)) => {
                assert!(msg.contains("File not found"));
                assert!(msg.contains("missing.md"));
            }
            _ => panic!("Expected NotFound error for missing file"),
        }
    }

    #[test]
    fn read_context_contents_propagates_io_error() {
        let workspace = tempdir().unwrap();
        let store = LocalContextFileStore::new(workspace.path().to_path_buf());
        let mx_dir = workspace.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();
        let file_path = mx_dir.join("unreadable.md");
        fs::write(&file_path, "secret").unwrap();

        // Make the file unreadable (owner gets write/execute, but no read)
        // Works on unix, on windows permissions might not map perfectly to read denial,
        // but let's try 0o222 (write-only).
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o222);
            fs::set_permissions(&file_path, perms).unwrap();

            let result = store.read_context_contents(Path::new("unreadable.md"));
            match result {
                Err(AppError::Io(err)) => {
                    assert_eq!(err.kind(), std::io::ErrorKind::PermissionDenied);
                }
                _ => panic!("Expected Io error for permission denied"),
            }
        }
    }
}
