use crate::domain::context_file::alias_registry::resolve_alias;
use std::path::PathBuf;

pub fn resolve_context_path(key: &str) -> PathBuf {
    let mut current_key = key;
    let mut prefix_path = PathBuf::new();

    while let Some(remainder) = current_key.strip_prefix("pd-") {
        prefix_path.push("pending");
        current_key = remainder;
    }

    if let Some(mapped) = resolve_alias(current_key) {
        return prefix_path.join(mapped);
    }

    if let Some(remainder) = current_key.strip_prefix("tk") {
        if !remainder.is_empty() && remainder.chars().all(char::is_numeric) {
            return prefix_path.join(format!("tasks/tasks{}.md", remainder));
        }
    }

    let mut path = prefix_path.join(current_key);
    if path.extension().is_none() {
        let file_name = path.file_name().and_then(|segment| segment.to_str()).unwrap_or("");
        if !file_name.starts_with('.') {
            path.set_extension("md");
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_alias_dynamic_and_pending_prefix() {
        assert_eq!(resolve_context_path("tk"), PathBuf::from("tasks.md"));
        assert_eq!(resolve_context_path("tk12"), PathBuf::from("tasks/tasks12.md"));
        assert_eq!(resolve_context_path("pd-tk"), PathBuf::from("pending/tasks.md"));
        assert_eq!(resolve_context_path("docs/spec"), PathBuf::from("docs/spec.md"));
    }
}
