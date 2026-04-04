use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

#[test]
fn touch_creates_context_files() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("test content");

    ctx.cli()
        .arg("touch")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file created"));

    ctx.cli().args(["cat", "tk"]).assert().success().stdout(predicate::eq("test content"));
}

#[test]
fn touch_force_overwrites() {
    let ctx = TestContext::new();
    let tasks_md = ctx.work_dir().join(".mx/tasks.md");
    fs::create_dir_all(tasks_md.parent().unwrap()).unwrap();
    fs::write(&tasks_md, "original content").unwrap();

    let clipboard_content = "new clipboard content";
    ctx.setup_clipboard(clipboard_content);

    ctx.cli()
        .arg("t")
        .arg("tk")
        .arg("-f")
        .assert()
        .success()
        .stdout(predicate::str::contains("✅ Context file overwritten"));

    ctx.cli().args(["cat", "tk"]).assert().success().stdout(predicate::eq("new clipboard content"));
}
