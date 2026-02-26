use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn cat_displays_file_contents() {
    let temp = tempdir().unwrap();
    let mx_dir = temp.path().join(".mx");
    fs::create_dir_all(&mx_dir).unwrap();

    let expected_content = "# Tasks\n\n- Task 1\n- Task 2\n";
    fs::write(mx_dir.join("tasks.md"), expected_content).unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .arg("cat")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::eq(expected_content));
}

#[test]
fn cat_alias_ct_works() {
    let temp = tempdir().unwrap();
    let mx_dir = temp.path().join(".mx");
    fs::create_dir_all(&mx_dir).unwrap();

    let content = "Requirements document";
    fs::write(mx_dir.join("requirements.md"), content).unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .arg("ct")
        .arg("rq")
        .assert()
        .success()
        .stdout(predicate::eq(content));
}

#[test]
fn cat_with_touch_integration() {
    let temp = tempdir().unwrap();
    let clipboard_file = temp.path().join("clipboard.txt");
    let content = "Content from clipboard";
    fs::write(&clipboard_file, content).unwrap();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .env("MX_CLIPBOARD_FILE", &clipboard_file)
        .arg("touch")
        .arg("tk")
        .assert()
        .success();

    Command::cargo_bin("mx")
        .unwrap()
        .current_dir(&temp)
        .arg("cat")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::eq(content));
}
