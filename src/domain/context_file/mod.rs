pub mod alias_registry;
pub mod key;
pub mod path_policy;

pub use key::{resolve_context_path, resolve_validated_context_path};
pub use path_policy::validate_path;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn resolves_alias_dynamic_and_pending_paths() {
        assert_eq!(resolve_context_path("tk"), PathBuf::from("tasks.md"));
        assert_eq!(resolve_context_path("tk2"), PathBuf::from("tasks/tasks2.md"));
        assert_eq!(resolve_context_path("pd-tk"), PathBuf::from("pending/tasks.md"));
        assert_eq!(resolve_context_path("docs/spec"), PathBuf::from("docs/spec.md"));
    }

    #[test]
    fn validation_rejects_traversal() {
        assert!(validate_path("../etc/passwd", &PathBuf::from("../etc/passwd")).is_err());
    }
}
