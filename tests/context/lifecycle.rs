use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn touch_cat_clean_lifecycle_is_consistent() {
    let dir = tempdir().unwrap();
    let clipboard = dir.path().join("clipboard.txt");
    fs::write(&clipboard, "workflow content").unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .env("MX_CLIPBOARD_FILE", &clipboard)
        .args(["touch", "tk"])
        .assert()
        .success();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .args(["cat", "tk"])
        .assert()
        .success()
        .stdout(predicate::eq("workflow content"));

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&dir)
        .args(["clean", "tk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));
}
