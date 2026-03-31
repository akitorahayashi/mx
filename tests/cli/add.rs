use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

fn setup_clipboard(ctx: &TestContext, content: &str) -> std::path::PathBuf {
    let file = ctx.clipboard_file("add_clipboard.txt");
    fs::write(&file, content).unwrap();
    file
}

#[test]
fn add_subcommand_saves_snippet_from_clipboard() {
    let ctx = TestContext::new();
    setup_clipboard(&ctx, "my snippet content\n");

    ctx.cli()
        .args(["add", ".mx/commands/test-snippet.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Added snippet"));

    let saved = ctx.commands_root().join("test-snippet.md");
    assert!(saved.exists(), "snippet file should be created");
    ctx.cli().args(["copy", "test-snippet"]).assert().success();
}

#[test]
fn add_alias_a_works() {
    let ctx = TestContext::new();
    setup_clipboard(&ctx, "body\n");

    ctx.cli().args(["a", ".mx/commands/alias-test.md"]).assert().success();
    assert!(ctx.commands_root().join("alias-test.md").exists());
}

#[test]
fn add_with_title_and_description_creates_frontmatter() {
    let ctx = TestContext::new();
    setup_clipboard(&ctx, "body\n");

    ctx.cli()
        .args(["add", ".mx/commands/fm-test.md", "--title", "My Title", "--description", "My desc"])
        .assert()
        .success();

    let content = fs::read_to_string(ctx.commands_root().join("fm-test.md")).unwrap();
    assert!(content.starts_with("---\ntitle: My Title\n"));
    assert!(content.contains("description: My desc\n"));
    assert!(content.contains("body\n"));
}

#[test]
fn add_force_overwrites_existing() {
    let ctx = TestContext::new();
    setup_clipboard(&ctx, "v1\n");
    ctx.cli().args(["add", ".mx/commands/force-test.md"]).assert().success();

    setup_clipboard(&ctx, "v2\n");
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
