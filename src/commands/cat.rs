use crate::commands::touch::{find_project_root, resolve_path, validate_path};
use crate::error::AppError;
use std::fs;

/// Internal implementation for displaying context file contents.
/// Use the public `cat_context` function from the library root instead.
pub fn cat(key: &str) -> Result<String, AppError> {
    // Find the project root directory (where .mx/ directory is or should be)
    let root = find_project_root()?;

    // Resolve the key to a relative path (e.g., "tk" -> "tasks.md")
    let relative_path = resolve_path(key);

    // Validate the path to prevent traversal attacks
    validate_path(key, &relative_path)?;

    // Build the full path to the file
    let mx_dir = root.join(".mx");
    let full_path = mx_dir.join(&relative_path);

    // Check if it's a file (is_file() already implies existence)
    if !full_path.is_file() {
        // If it's not a file, it could be a directory or not exist at all
        if full_path.exists() {
            return Err(AppError::not_found(format!(
                "⚠️ Path is not a file: {}",
                relative_path.display()
            )));
        } else {
            return Err(AppError::not_found(format!(
                "⚠️ Context file not found: {}",
                relative_path.display()
            )));
        }
    }

    // Read and return the file contents
    fs::read_to_string(&full_path).map_err(|e| {
        AppError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to read {}: {}", relative_path.display(), e),
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn cat_reads_existing_file() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        // Create a context file with known content
        let mx_dir = temp.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();
        let tasks_path = mx_dir.join("tasks.md");
        let expected_content = "# Test Tasks\n\n- Task 1\n- Task 2\n";
        fs::write(&tasks_path, expected_content).unwrap();

        // Read it back using cat
        let result = cat("tk").unwrap();
        assert_eq!(result, expected_content);
    }

    #[test]
    #[serial]
    fn cat_returns_error_for_missing_file() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        // Ensure .mx directory exists but file doesn't
        fs::create_dir_all(temp.path().join(".mx")).unwrap();

        let result = cat("tk");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("⚠️"));
        assert!(err_msg.contains("not found"));
    }

    #[test]
    #[serial]
    fn cat_rejects_path_traversal() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        let result = cat("../etc/passwd");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::PathTraversal(_)));
    }

    #[test]
    #[serial]
    fn cat_handles_empty_file() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        // Create an empty file
        let mx_dir = temp.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();
        let empty_path = mx_dir.join("empty.md");
        fs::write(&empty_path, "").unwrap();

        let result = cat("empty").unwrap();
        assert_eq!(result, "");
    }

    #[test]
    #[serial]
    fn cat_resolves_aliases_correctly() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        // Create files for different aliases
        let mx_dir = temp.path().join(".mx");
        fs::create_dir_all(&mx_dir).unwrap();

        // Standard alias
        let content = "requirements content";
        fs::write(mx_dir.join("requirements.md"), content).unwrap();
        assert_eq!(cat("rq").unwrap(), content);

        // Nested alias
        fs::create_dir_all(mx_dir.join("pending")).unwrap();
        let nested_content = "pending tasks";
        fs::write(mx_dir.join("pending/tasks.md"), nested_content).unwrap();
        assert_eq!(cat("pdt").unwrap(), nested_content);
    }

    #[test]
    #[serial]
    fn cat_errors_on_directory() {
        let temp = tempdir().unwrap();
        env::set_current_dir(&temp).unwrap();

        // Create a directory with .md extension to simulate the edge case
        let mx_dir = temp.path().join(".mx");
        fs::create_dir_all(mx_dir.join("somedir.md")).unwrap();

        let result = cat("somedir.md");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("⚠️"));
        assert!(err_msg.contains("not a file"));
    }
}
