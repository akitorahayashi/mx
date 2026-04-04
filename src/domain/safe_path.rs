use crate::domain::error::{AppError, PathTraversalError};
use std::path::{Component, Path, PathBuf};

/// A strongly typed wrapper around a path that has been validated
/// to contain only safe, relative components, preventing path traversal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SafePath {
    inner: PathBuf,
}

impl SafePath {
    /// Attempts to create a SafePath from a given Path.
    /// Fails if the path contains empty, absolute, or traversal segments.
    pub fn try_from_path(path: &Path) -> Result<Self, AppError> {
        let mut inner = PathBuf::new();
        for component in path.components() {
            match component {
                Component::Normal(segment) => inner.push(segment),
                Component::CurDir => {} // Skip current directory segments for normalization
                _ => {
                    return Err(AppError::PathTraversal(PathTraversalError::Detected(
                        "Invalid path. Cannot create or access files outside of the allowed directory."
                            .to_string(),
                    )));
                }
            }
        }

        Ok(Self { inner })
    }
}

impl AsRef<Path> for SafePath {
    fn as_ref(&self) -> &Path {
        &self.inner
    }
}

impl AsRef<std::ffi::OsStr> for SafePath {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.inner.as_os_str()
    }
}

impl std::ops::Deref for SafePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::fmt::Display for SafePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut components = self.inner.components().filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy()),
            _ => None,
        });

        if let Some(first) = components.next() {
            write!(f, "{}", first)?;
            for component in components {
                write!(f, "/{}", component)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safepath_blocks_parent_and_absolute_paths() {
        assert!(SafePath::try_from_path(Path::new("../hack")).is_err());
        assert!(SafePath::try_from_path(Path::new("/abs")).is_err());
    }

    #[test]
    fn safepath_accepts_nested_relative_path() {
        assert!(SafePath::try_from_path(Path::new("docs/spec")).is_ok());
        assert!(SafePath::try_from_path(Path::new("notes..v2")).is_ok());
    }
}
