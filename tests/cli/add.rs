use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

#[test]
fn add_subcommand_saves_snippet_from_clipboard() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("my snippet content\n");

    ctx.cli()
        .args(["add", ".mx/commands/test-snippet.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added snippet"));

    ctx.cli().args(["copy", "test-snippet"]).assert().success();
    ctx.cli().args(["touch", "tk"]).assert().success();
    ctx.cli().args(["cat", "tk"]).assert().success().stdout(predicate::eq("my snippet content\n"));
}

#[test]
fn add_alias_a_works() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("body\n");

    ctx.cli().args(["a", ".mx/commands/alias-test.md"]).assert().success();
    assert!(ctx.commands_root().join("alias-test.md").exists());
}

#[test]
fn add_with_title_and_description_creates_frontmatter() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("body\n");

    ctx.cli()
        .args(["add", ".mx/commands/fm-test.md", "--title", "My Title", "--description", "My desc"])
        .assert()
        .success();

    ctx.cli().args(["copy", "fm-test"]).assert().success();
    ctx.cli().args(["touch", "tk"]).assert().success();
    ctx.cli().args(["cat", "tk"]).assert().success().stdout(predicate::str::contains("body\n"));
}

#[test]
fn add_force_overwrites_existing() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("v1\n");
    ctx.cli().args(["add", ".mx/commands/force-test.md"]).assert().success();

    ctx.setup_clipboard("v2\n");
    ctx.cli().args(["add", ".mx/commands/force-test.md", "--force"]).assert().success();

    ctx.cli().args(["copy", "force-test"]).assert().success();
    ctx.cli().args(["touch", "tk"]).assert().success();
    ctx.cli().args(["cat", "tk"]).assert().success().stdout(predicate::eq("v2\n"));
}

#[test]
fn add_then_copy_roundtrip() {
    let ctx = TestContext::new();
    let clip = ctx.clipboard_file("roundtrip_clip.txt");
    fs::write(&clip, "roundtrip content\n").unwrap();

    ctx.cli().args(["add", ".mx/commands/roundtrip.md"]).assert().success();
    ctx.cli().args(["copy", "roundtrip"]).assert().success();

    let copied = fs::read_to_string(&clip).unwrap();
    assert_eq!(copied, "roundtrip content\n");
}

#[test]
fn add_fails_on_duplicate_without_force() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("v1\n");
    ctx.cli().args(["add", ".mx/commands/dup.md"]).assert().success();

    ctx.setup_clipboard("v2\n");
    ctx.cli()
        .args(["add", ".mx/commands/dup.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn add_rejects_path_outside_mx_commands() {
    let ctx = TestContext::new();
    ctx.setup_clipboard("content\n");

    ctx.cli()
        .args(["add", "outside.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Path must be under .mx/commands/"));
}
