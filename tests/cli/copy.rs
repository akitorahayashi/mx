use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;
use std::fs;

#[test]
fn copy_subcommand_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);
    let clipboard = ctx.clipboard_file("clipboard.txt");

    ctx.cli()
        .args(["copy", "wc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied 'wc'"));

    let captured = fs::read_to_string(&clipboard).expect("clipboard file should exist");
    assert!(captured.contains("/wc"), "clipboard should hold snippet contents");
}

#[test]
fn copy_alias_c_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);
    let clipboard = ctx.clipboard_file("clipboard_alias.txt");

    ctx.cli().args(["c", "wc"]).assert().success().stdout(predicate::str::contains("Copied 'wc'"));

    let captured = fs::read_to_string(&clipboard).expect("clipboard file should exist");
    assert!(captured.contains("/wc"), "clipboard should hold snippet contents");
}

