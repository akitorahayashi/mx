use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;

#[test]
fn copy_subcommand_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);
    let _ = ctx.clipboard_file("clipboard.txt");

    ctx.cli()
        .args(["copy", "wc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied 'wc'"));

    ctx.cli().args(["touch", "tk"]).assert().success();
    ctx.cli()
        .args(["cat", "tk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("/wc"));
}

#[test]
fn copy_alias_c_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);
    let _ = ctx.clipboard_file("clipboard_alias.txt");

    ctx.cli().args(["c", "wc"]).assert().success().stdout(predicate::str::contains("Copied 'wc'"));

    ctx.cli().args(["touch", "tk"]).assert().success();
    ctx.cli()
        .args(["cat", "tk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("/wc"));
}

#[test]
fn copy_subcommand_missing_snippet_fails() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);
    let _ = ctx.clipboard_file("clipboard_fail.txt");

    ctx.cli()
        .arg("copy")
        .arg("unknown")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No snippet named"));
}
