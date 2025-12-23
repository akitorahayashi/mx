use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

/// Helper to create a clipboard file with content
fn setup_clipboard_file(dir: &std::path::Path, content: &str) -> std::path::PathBuf {
    let clipboard_file = dir.join("clipboard.txt");
    fs::write(&clipboard_file, content).expect("write clipboard file");
    clipboard_file
}

#[test]
fn test_touch_paste_creates_file_with_clipboard_content() {
    let temp = tempdir().unwrap();
    let clipboard_content = "This is the clipboard content\nWith multiple lines";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("tk")
        .arg("--paste")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let tasks_file = temp.path().join(".mix/tasks.md");
    assert!(tasks_file.exists());
    let content = fs::read_to_string(&tasks_file).expect("read tasks file");
    assert_eq!(content, clipboard_content);
}

#[test]
fn test_touch_paste_short_flag() {
    let temp = tempdir().unwrap();
    let clipboard_content = "Short flag test";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("rq")
        .arg("-p")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let requirements_file = temp.path().join(".mix/requirements.md");
    assert!(requirements_file.exists());
    let content = fs::read_to_string(&requirements_file).expect("read requirements file");
    assert_eq!(content, clipboard_content);
}

#[test]
fn test_touch_paste_does_not_overwrite_existing_file() {
    let temp = tempdir().unwrap();
    let original_content = "Original content that should not be overwritten";
    let clipboard_content = "New clipboard content";

    // Create existing file
    let tasks_file = temp.path().join(".mix/tasks.md");
    fs::create_dir_all(tasks_file.parent().unwrap()).unwrap();
    fs::write(&tasks_file, original_content).unwrap();

    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("tk")
        .arg("--paste")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file found"));

    // File should still have original content
    let content = fs::read_to_string(&tasks_file).expect("read tasks file");
    assert_eq!(content, original_content);
}

#[test]
fn test_touch_paste_with_dynamic_path() {
    let temp = tempdir().unwrap();
    let clipboard_content = "Dynamic path content";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("docs/spec")
        .arg("-p")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let spec_file = temp.path().join(".mix/docs/spec.md");
    assert!(spec_file.exists());
    let content = fs::read_to_string(&spec_file).expect("read spec file");
    assert_eq!(content, clipboard_content);
}

#[test]
fn test_touch_paste_with_nested_alias() {
    let temp = tempdir().unwrap();
    let clipboard_content = "Nested alias content";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("pdt")
        .arg("--paste")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let pending_tasks = temp.path().join(".mix/pending/tasks.md");
    assert!(pending_tasks.exists());
    let content = fs::read_to_string(&pending_tasks).expect("read pending tasks");
    assert_eq!(content, clipboard_content);
}

#[test]
fn test_touch_paste_with_different_extension() {
    let temp = tempdir().unwrap();
    let clipboard_content = r#"{"key": "value", "number": 42}"#;
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("config.json")
        .arg("-p")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let config_file = temp.path().join(".mix/config.json");
    assert!(config_file.exists());
    let content = fs::read_to_string(&config_file).expect("read config file");
    assert_eq!(content, clipboard_content);
}

#[test]
fn test_touch_paste_empty_clipboard() {
    let temp = tempdir().unwrap();
    let clipboard_file = setup_clipboard_file(temp.path(), "");

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("empty")
        .arg("-p")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let empty_file = temp.path().join(".mix/empty.md");
    assert!(empty_file.exists());
    let content = fs::read_to_string(&empty_file).expect("read empty file");
    assert_eq!(content, "");
}

#[test]
fn test_touch_without_paste_flag_does_not_write_clipboard() {
    let temp = tempdir().unwrap();
    let clipboard_content = "This should not be written";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let tasks_file = temp.path().join(".mix/tasks.md");
    assert!(tasks_file.exists());
    let content = fs::read_to_string(&tasks_file).expect("read tasks file");
    // File should exist but be empty (just created by touch)
    assert_eq!(content, "");
}

#[test]
fn test_touch_paste_with_numbered_alias() {
    let temp = tempdir().unwrap();
    let clipboard_content = "Numbered alias test";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("tk1")
        .arg("-p")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    let numbered_file = temp.path().join(".mix/tasks/tasks1.md");
    assert!(numbered_file.exists());
    let content = fs::read_to_string(&numbered_file).expect("read numbered file");
    assert_eq!(content, clipboard_content);
}

#[test]
fn test_touch_paste_multiline_content() {
    let temp = tempdir().unwrap();
    let clipboard_content = "Line 1\nLine 2\nLine 3\n\nLine 5 with blank line before";
    let clipboard_file = setup_clipboard_file(temp.path(), clipboard_content);

    let mut cmd = Command::cargo_bin("mix").unwrap();
    cmd.current_dir(&temp)
        .env("MIX_CLIPBOARD_FILE", &clipboard_file)
        .arg("t")
        .arg("multiline")
        .arg("-p")
        .assert()
        .success();

    let multiline_file = temp.path().join(".mix/multiline.md");
    let content = fs::read_to_string(&multiline_file).expect("read multiline file");
    assert_eq!(content, clipboard_content);
}
