use crate::harness::TestContext;
use predicates::prelude::*;

#[test]
fn cat_displays_file_contents() {
    let ctx = TestContext::new();
    let expected_content = "# Tasks\n\n- Task 1\n- Task 2\n";
    ctx.setup_clipboard(expected_content);

    ctx.cli().arg("touch").arg("tk").assert().success();

    ctx.cli().arg("cat").arg("tk").assert().success().stdout(predicate::eq(expected_content));
}

#[test]
fn cat_alias_ct_works() {
    let ctx = TestContext::new();
    let content = "Requirements document";
    ctx.setup_clipboard(content);

    ctx.cli().arg("touch").arg("rq").assert().success();

    ctx.cli().arg("ct").arg("rq").assert().success().stdout(predicate::eq(content));
}

#[test]
fn cat_with_touch_integration() {
    let ctx = TestContext::new();
    let content = "Content from clipboard";
    ctx.setup_clipboard(content);

    ctx.cli().arg("touch").arg("tk").assert().success();

    ctx.cli().arg("cat").arg("tk").assert().success().stdout(predicate::eq(content));
}
