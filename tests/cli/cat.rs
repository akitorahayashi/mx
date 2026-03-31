use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

fn setup_clipboard(ctx: &TestContext, content: &str) -> std::path::PathBuf {
    let clipboard_file = ctx.clipboard_file("clipboard.txt");
    fs::write(&clipboard_file, content).unwrap();
    clipboard_file
}

#[test]
fn cat_displays_file_contents() {
    let ctx = TestContext::new();
    let expected_content = "# Tasks\n\n- Task 1\n- Task 2\n";
    let _ = setup_clipboard(&ctx, expected_content);

    ctx.cli()
        .arg("touch")
        .arg("tk")
        .assert()
        .success();

    ctx.cli()
        .arg("cat")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::eq(expected_content));
}

#[test]
fn cat_alias_ct_works() {
    let ctx = TestContext::new();
    let content = "Requirements document";
    let _ = setup_clipboard(&ctx, content);

    ctx.cli()
        .arg("touch")
        .arg("rq")
        .assert()
        .success();

    ctx.cli()
        .arg("ct")
        .arg("rq")
        .assert()
        .success()
        .stdout(predicate::eq(content));
}

#[test]
fn cat_with_touch_integration() {
    let ctx = TestContext::new();
    let content = "Content from clipboard";
    let _ = setup_clipboard(&ctx, content);

    ctx.cli()
        .arg("touch")
        .arg("tk")
        .assert()
        .success();

    ctx.cli()
        .arg("cat")
        .arg("tk")
        .assert()
        .success()
        .stdout(predicate::eq(content));
}
