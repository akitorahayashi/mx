use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

fn setup_clipboard(ctx: &TestContext, content: &str) -> std::path::PathBuf {
    let clipboard_file = ctx.clipboard_file("clipboard.txt");
    fs::write(&clipboard_file, content).unwrap();
    clipboard_file
}

#[test]
fn clean_full_directory() {
    let ctx = TestContext::new();
    let _ = setup_clipboard(&ctx, "test content");

    ctx.cli()
        .arg("touch")
        .arg("tk")
        .assert()
        .success();

    ctx.cli()
        .arg("clean")
        .assert()
        .success()
        .stdout(predicate::eq("✅ Cleared .mx directory contents\n"));

    ctx.cli()
        .arg("cat")
        .arg("tk")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Context file not found"));
}

#[test]
fn clean_alias_cl_works() {
    let ctx = TestContext::new();
    let _ = setup_clipboard(&ctx, "test content");

    ctx.cli()
        .args(["touch", "tk"])
        .assert()
        .success();

    ctx.cli()
        .arg("cl")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cleared"));
}
