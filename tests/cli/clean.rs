use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

fn setup_clipboard(temp: &tempfile::TempDir, content: &str) -> std::path::PathBuf {
    let clipboard_file = temp.path().join("clipboard.txt");
    fs::write(&clipboard_file, content).unwrap();
    clipboard_file
}

#[test]
fn clean_full_directory() {
    let dir = tempdir().unwrap();
    let clipboard_file = setup_clipboard(&dir, "test content");

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .env("MX_CLIPBOARD_FILE", &clipboard_file)
        .arg("touch")
        .arg("tk")
        .assert()
        .success();

    assert!(dir.path().join(".mx").exists());

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .arg("clean")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::eq("✅ Removed .mx directory\n"));

    assert!(!dir.path().join(".mx").exists());
}

#[test]
fn clean_alias_cl_works() {
    let dir = tempdir().unwrap();
    let clipboard_file = setup_clipboard(&dir, "test content");

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .env("MX_CLIPBOARD_FILE", &clipboard_file)
        .args(["touch", "tk"])
        .assert()
        .success();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .arg("cl")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));
}

#[test]
fn clean_without_force_aborts_non_interactive() {
    let dir = tempdir().unwrap();
    let clipboard_file = setup_clipboard(&dir, "test content");

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .env("MX_CLIPBOARD_FILE", &clipboard_file)
        .arg("touch")
        .arg("tk")
        .assert()
        .success();

    assert!(dir.path().join(".mx").exists());

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .arg("clean")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Operation cancelled by user"));

    assert!(dir.path().join(".mx").exists());
}
