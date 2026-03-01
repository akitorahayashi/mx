use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

#[test]
fn creates_template_with_frontmatter_stubs() {
    let ctx = TestContext::new();

    ctx.cli()
        .args(["create-command", ".mx/commands/my-cmd.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created command template: my-cmd"));

    let saved = ctx.commands_root().join("my-cmd.md");
    assert!(saved.exists(), "template file should be created");
    let content = fs::read_to_string(&saved).unwrap();
    assert!(content.starts_with("---\ntitle:"), "should start with front matter");
    assert!(content.contains("description:"), "should contain description field");
}

#[test]
fn cc_alias_works() {
    let ctx = TestContext::new();

    ctx.cli().args(["cc", ".mx/commands/alias-test.md"]).assert().success();

    assert!(ctx.commands_root().join("alias-test.md").exists());
}

#[test]
fn creates_nested_directory() {
    let ctx = TestContext::new();

    ctx.cli().args(["create-command", ".mx/commands/sub/nested-cmd.md"]).assert().success();

    assert!(ctx.commands_root().join("sub/nested-cmd.md").exists());
}

#[test]
fn fails_on_duplicate_without_force() {
    let ctx = TestContext::new();
    ctx.write_snippet("dup.md", "existing content\n");

    ctx.cli()
        .args(["create-command", ".mx/commands/dup.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn force_overwrites_existing() {
    let ctx = TestContext::new();
    ctx.write_snippet("dup.md", "old content\n");

    ctx.cli().args(["create-command", "--force", ".mx/commands/dup.md"]).assert().success();

    let content = fs::read_to_string(ctx.commands_root().join("dup.md")).unwrap();
    assert!(content.starts_with("---\n"), "template should replace old content");
}

