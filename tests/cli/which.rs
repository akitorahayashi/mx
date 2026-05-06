use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;

#[test]
fn which_without_argument_prints_commands_root_absolute_path() {
    let ctx = TestContext::new();

    let expected = ctx.commands_root().display().to_string();
    ctx.cli().arg("which").assert().success().stdout(predicate::eq(format!("{expected}\n")));
}

#[test]
fn which_with_snippet_prints_snippet_absolute_path() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    let expected = ctx.commands_root().join("w/wc.md").display().to_string();
    ctx.cli()
        .args(["which", "wc"])
        .assert()
        .success()
        .stdout(predicate::eq(format!("{expected}\n")));
}

#[test]
fn which_with_unknown_snippet_fails() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .args(["which", "unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No snippet named"));
}
