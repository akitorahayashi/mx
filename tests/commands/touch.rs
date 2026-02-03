//! Integration tests for the `mx touch` command

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

/// Helper to setup clipboard file for tests
fn setup_clipboard(temp: &tempfile::TempDir, content: &str) -> std::path::PathBuf {
    let clipboard_file = temp.path().join("clipboard.txt");
    fs::write(&clipboard_file, content).unwrap();
    clipboard_file
}

#[test]
fn touch_creates_context_files() {
    let temp = tempdir().unwrap();
    let clipboard_file = setup_clipboard(&temp, "test content");

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .env("MX_CLIPBOARD_FILE", &clipboard_file)
        .arg("touch")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let mx_dir = temp.path().join(".mx");
    assert!(mx_dir.exists());
    assert!(mx_dir.join(".gitignore").exists());
    assert!(mx_dir.join("tasks.md").exists());
}

#[test]
fn touch_force_overwrites() {
    let temp = tempdir().unwrap();
    let tasks_md = temp.path().join(".mx/tasks.md");
    fs::create_dir_all(tasks_md.parent().unwrap()).unwrap();
    fs::write(&tasks_md, "original content").unwrap();

    let clipboard_content = "new clipboard content";
    let clipboard_file = setup_clipboard(&temp, clipboard_content);

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .env("MX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("tk")
        .arg("-f")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file overwritten"));

    let content = fs::read_to_string(&tasks_md).unwrap();
    assert_eq!(content, clipboard_content);
}
