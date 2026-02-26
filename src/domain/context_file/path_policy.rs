use crate::domain::error::AppError;
use std::path::Path;

pub fn validate_path(key: &str, resolved: &Path) -> Result<(), AppError> {
    for component in Path::new(key).components() {
        match component {
            std::path::Component::Normal(_) | std::path::Component::CurDir => {}
            _ => {
                return Err(AppError::path_traversal(
                    "Invalid path. Cannot create files outside of .mx directory.",
                ));
            }
        }
    }

    for component in resolved.components() {
        match component {
            std::path::Component::Normal(_) | std::path::Component::CurDir => {}
            _ => {
                return Err(AppError::path_traversal(
                    "Invalid path. Cannot create files outside of .mx directory.",
                ));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn validate_path_blocks_parent_and_absolute_paths() {
        assert!(validate_path("../hack", &PathBuf::from("../hack.md")).is_err());
        assert!(validate_path("/abs", &PathBuf::from("/abs.md")).is_err());
    }

    #[test]
    fn validate_path_accepts_nested_relative_path() {
        assert!(validate_path("docs/spec", &PathBuf::from("docs/spec.md")).is_ok());
        assert!(validate_path("notes..v2", &PathBuf::from("notes..v2.md")).is_ok());
    }
}
