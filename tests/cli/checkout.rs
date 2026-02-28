use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;
use std::fs;

#[test]
fn checkout_all_creates_symlinks_in_mx_commands() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .args(["checkout", "--all"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Checked out"));

    let mx_commands = ctx.work_dir().join(".mx").join("commands");
    assert!(mx_commands.exists(), ".mx/commands should be created");

    let gitignore = mx_commands.join(".gitignore");
    assert!(gitignore.exists(), ".gitignore should be created");
    let gi_content = fs::read_to_string(&gitignore).unwrap();
    assert_eq!(gi_content, "*\n");

    // The sample catalog has wc and sdd-0-rq
    let wc_link = mx_commands.join("w").join("wc.md");
    assert!(wc_link.exists(), "wc symlink should exist");
    assert!(wc_link.is_symlink(), "wc.md should be a symlink");
}

#[test]
fn checkout_alias_co_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli().args(["co", "--all"]).assert().success();
}

#[test]
fn checkout_individual_snippet_by_key() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .args(["checkout", "wc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Checked out 1 snippet(s)"));

    let link = ctx.work_dir().join(".mx").join("commands").join("w").join("wc.md");
    assert!(link.exists(), "symlink for wc should exist");
    assert!(link.is_symlink(), "wc path should be a symlink");
}

#[test]
fn checkout_symlink_points_to_actual_file() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli().args(["checkout", "wc"]).assert().success();

    let link = ctx.work_dir().join(".mx").join("commands").join("w").join("wc.md");
    let content = fs::read_to_string(&link).expect("symlink should be readable");
    assert!(content.contains("# /wc"), "symlink should point to actual snippet content");
}
