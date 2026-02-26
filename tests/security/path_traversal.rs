use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn cat_rejects_path_traversal() {
    let temp = tempdir().unwrap();
    fs::create_dir_all(temp.path().join(".mx")).unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .arg("cat")
        .arg("../etc/passwd")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid path"));
}

#[test]
fn touch_rejects_path_traversal() {
    let temp = tempdir().unwrap();
    let clipboard = temp.path().join("clipboard.txt");
    fs::write(&clipboard, "content").unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .env("MX_CLIPBOARD_FILE", &clipboard)
        .arg("touch")
        .arg("../etc/passwd")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid path"));
}
