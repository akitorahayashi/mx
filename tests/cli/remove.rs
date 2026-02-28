use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;

#[test]
fn remove_deletes_existing_snippet() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .args(["remove", "wc"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Removed snippet 'wc'"));

    assert!(!ctx.commands_root().join("w").join("wc.md").exists());
}

#[test]
fn remove_alias_rm_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli().args(["rm", "wc"]).assert().success();
    assert!(!ctx.commands_root().join("w").join("wc.md").exists());
}

#[test]
fn remove_then_list_no_longer_shows_snippet() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli().args(["remove", "wc"]).assert().success();
    ctx.cli().args(["list"]).assert().success().stdout(predicate::str::contains("wc").not());
}

#[test]
fn remove_fails_for_nonexistent_snippet() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .args(["remove", "does-not-exist"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No snippet named"));
}

#[test]
fn remove_checkout_symlink_becomes_broken() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    // First checkout the snippet
    ctx.cli().args(["checkout", "wc"]).assert().success();
    let link = ctx.work_dir().join(".mx").join("commands").join("w").join("wc.md");
    assert!(link.exists(), "symlink should exist before remove");

    // Now remove the actual file
    ctx.cli().args(["remove", "wc"]).assert().success();

    // Symlink should now be broken (exists as symlink entry but target is gone)
    assert!(!link.exists(), "symlink target is gone");
    assert!(link.symlink_metadata().is_ok(), "broken symlink should still have metadata entry");
}
