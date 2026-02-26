use crate::domain::error::AppError;
use std::path::{Component, Path};

pub fn normalize_query(raw: &str) -> Result<String, AppError> {
    let trimmed = raw.trim().trim_start_matches('/');
    if trimmed.is_empty() {
        return Err(AppError::config_error("Snippet name cannot be empty"));
    }

    let mut normalized = trimmed.replace('\\', "/");
    if let Some(stripped) = normalized.strip_prefix("commands/") {
        normalized = stripped.to_string();
    }
    if let Some(stripped) = normalized.strip_suffix(".md") {
        normalized = stripped.to_string();
    }

    ensure_safe_segments(&normalized)?;
    Ok(normalized)
}

pub fn candidate_key(normalized_query: &str) -> String {
    normalized_query.rsplit('/').next().unwrap_or(normalized_query).to_string()
}

pub fn ensure_safe_segments(value: &str) -> Result<(), AppError> {
    if value.split('/').any(|segment| segment.is_empty() || segment == "..") {
        return Err(AppError::config_error(
            "Snippet paths cannot contain empty or traversal segments",
        ));
    }
    Ok(())
}

pub fn path_to_string(path: &Path) -> Result<String, AppError> {
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(segment) => parts.push(
                segment
                    .to_str()
                    .ok_or_else(|| AppError::config_error("Snippet paths must be UTF-8"))?
                    .to_string(),
            ),
            Component::CurDir => continue,
            _ => {
                return Err(AppError::config_error(
                    "Snippet paths cannot include traversal components",
                ));
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
        assert_eq!(normalize_query("commands/w/wc.md").unwrap(), "w/wc");
        assert_eq!(normalize_query("/foo").unwrap(), "foo");
    }
}
