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
    let probe_name = format!(
        "{}_escape_probe.txt",
        temp.path().file_name().unwrap_or_default().to_string_lossy()
    );
    let escaped_target = temp.path().parent().unwrap().join(&probe_name);
    let payload = format!("../{probe_name}");
    let clipboard = temp.path().join("clipboard.txt");
    fs::write(&clipboard, "content").unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .env("MX_CLIPBOARD_FILE", &clipboard)
        .arg("touch")
        .arg(&payload)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid path"));

    assert!(!escaped_target.exists(), "path traversal must not create files outside the workspace");
}
