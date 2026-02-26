use mx::domain::context_file::{resolve_context_path, validate_path};
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
