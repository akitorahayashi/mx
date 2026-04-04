use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn clean_full_directory() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("test content");

    ctx.cli().arg("touch").arg("tk").assert().success();

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
    ctx.setup_clipboard("test content");

    ctx.cli().args(["touch", "tk"]).assert().success();

    ctx.cli().arg("cl").assert().success().stdout(predicate::str::contains("Cleared"));
}
