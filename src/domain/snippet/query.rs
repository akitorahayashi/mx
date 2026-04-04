use crate::domain::error::{AppError, ConfigError};
use crate::domain::SafePath;
use std::path::{Component, Path};

pub fn normalize_query(raw: &str) -> Result<SafePath, AppError> {
    let trimmed = raw.trim().trim_start_matches('/');
    if trimmed.is_empty() {
        return Err(AppError::ConfigError(ConfigError::EmptySnippetName));
    }

    let mut normalized = trimmed.replace('\\', "/");
    if let Some(stripped) = normalized.strip_prefix("commands/") {
        normalized = stripped.to_string();
    }
    if let Some(stripped) = normalized.strip_suffix(".md") {
        normalized = stripped.to_string();
    }

    let safe_path = SafePath::try_from_path(Path::new(&normalized)).map_err(|_| {
        AppError::ConfigError(ConfigError::Other(
            "Snippet paths cannot contain empty, absolute, or traversal segments".to_string(),
        ))
    })?;

    Ok(safe_path)
}

pub fn candidate_key(normalized_query: &str) -> String {
    normalized_query.rsplit('/').next().unwrap_or(normalized_query).to_string()
}

pub fn path_to_string(path: &Path) -> Result<String, AppError> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(segment) => parts.push(
                segment
                    .to_str()
                    .ok_or(AppError::ConfigError(ConfigError::InvalidUtf8))?
                    .to_string(),
            ),
            Component::CurDir => continue,
            _ => {
                return Err(AppError::ConfigError(ConfigError::Other(
                    "Snippet paths cannot include traversal components".to_string(),
                )));
            }
        }
    }

    Ok(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query_rejects_empty_and_traversal() {
        assert!(normalize_query("  ").is_err());
        assert!(normalize_query("../secret").is_err());
        assert!(normalize_query("foo//bar").is_err());
    }

    #[test]
    fn normalize_query_strips_prefix_and_extension() {
        assert_eq!(normalize_query("commands/w/wc.md").unwrap().to_string(), "w/wc");
        assert_eq!(normalize_query("/foo").unwrap().to_string(), "foo");
    }
}
