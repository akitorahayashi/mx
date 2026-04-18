use mx::context_files::resolve_context_path;
use mx::project_fs::SafePath;
use std::path::{Path, PathBuf};

#[test]
fn resolves_alias_dynamic_and_pending_paths() {
    assert_eq!(resolve_context_path("tk"), PathBuf::from("tasks.md"));
    assert_eq!(resolve_context_path("tk2"), PathBuf::from("tasks/tasks2.md"));
    assert_eq!(resolve_context_path("pd-tk"), PathBuf::from("pending/tasks.md"));
    assert_eq!(resolve_context_path("docs/spec"), PathBuf::from("docs/spec.md"));
}

#[test]
fn validation_rejects_traversal() {
    assert!(SafePath::try_from_path(Path::new("../etc/passwd")).is_err());
}
