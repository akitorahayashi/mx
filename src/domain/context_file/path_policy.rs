use crate::domain::error::AppError;
use std::path::Path;

const PATH_TRAVERSAL_MESSAGE: &str = "Invalid path. Cannot create files outside of .mx directory.";

pub(crate) fn validate_relative_components(path: &Path) -> Result<(), AppError> {
    for component in path.components() {
        match component {
            std::path::Component::Normal(_) | std::path::Component::CurDir => {}
            _ => {
                return Err(AppError::path_traversal(PATH_TRAVERSAL_MESSAGE));
            }
        }
    }

    Ok(())
}

pub fn validate_path(key: &str, resolved: &Path) -> Result<(), AppError> {
    validate_relative_components(Path::new(key))?;
    validate_relative_components(resolved)?;
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

    #[test]
    fn validate_relative_components_blocks_absolute_and_parent_paths() {
        assert!(validate_relative_components(Path::new("../escape")).is_err());
        assert!(validate_relative_components(Path::new("/escape")).is_err());
        assert!(validate_relative_components(Path::new("safe/path")).is_ok());
    }
}
