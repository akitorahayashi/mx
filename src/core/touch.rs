use std::path::{Path, PathBuf};
use crate::error::AppError;
use std::fs;
use std::io::Write;

pub struct TouchOutcome {
    pub key: String,
    pub path: PathBuf,
    pub existed: bool,
}

pub fn touch(key: &str) -> Result<TouchOutcome, AppError> {
    let root = find_project_root()?;
    let mix_dir = root.join(".mix");

    // 1. Create .mix directory
    if !mix_dir.exists() {
        fs::create_dir(&mix_dir).map_err(|e| AppError::Io(e))?;
    }

    // 2. Create .gitignore
    let gitignore = mix_dir.join(".gitignore");
    if !gitignore.exists() {
        let mut file = fs::File::create(&gitignore).map_err(|e| AppError::Io(e))?;
        writeln!(file, "*").map_err(|e| AppError::Io(e))?;
    }

    // 3. Map key to file path
    let relative_path = match key {
        "tk" => PathBuf::from("tasks.md"),
        "rq" => PathBuf::from("requirements.md"),
        "rv" => PathBuf::from("review.md"),
        "df" => PathBuf::from("diff.md"),
        "pdt" => PathBuf::from("pending/tasks.md"),
        "pdr" => PathBuf::from("pending/requirements.md"),
        _ => return Err(AppError::InvalidKey(key.to_string())),
    };

    let target_path = mix_dir.join(&relative_path);

    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| AppError::Io(e))?;
        }
    }

    // 4. Create file if not exists
    let existed = if target_path.exists() {
        true
    } else {
        fs::File::create(&target_path).map_err(|e| AppError::Io(e))?;
        false
    };

    Ok(TouchOutcome {
        key: key.to_string(),
        path: target_path,
        existed,
    })
}

fn find_project_root() -> Result<PathBuf, AppError> {
    // For now, assume current directory is root or we look for .git
    // But simplest is to use current directory.
    // If we want to be robust, we can look for .git up the tree.
    // Given the request "Creates .mix/ in the project root", implies CWD usually.
    std::env::current_dir().map_err(|e| AppError::Io(e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_touch_creates_mix_and_gitignore() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let outcome = touch("tk").unwrap();

        assert!(dir.path().join(".mix").exists());
        assert!(dir.path().join(".mix/.gitignore").exists());
        let gitignore_content = fs::read_to_string(dir.path().join(".mix/.gitignore")).unwrap();
        assert_eq!(gitignore_content.trim(), "*");
        assert_eq!(outcome.key, "tk");
        assert!(outcome.path.ends_with(".mix/tasks.md"));
        assert!(!outcome.existed);
    }

    #[test]
    #[serial]
    fn test_touch_nested_file() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let outcome = touch("pdt").unwrap();

        assert!(dir.path().join(".mix/pending/tasks.md").exists());
        assert!(!outcome.existed);
    }

    #[test]
    #[serial]
    fn test_touch_idempotency() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        touch("tk").unwrap();
        let outcome = touch("tk").unwrap();

        assert!(outcome.existed);
    }

    #[test]
    #[serial]
    fn test_invalid_key() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let result = touch("invalid");
        assert!(result.is_err());
    }
}
