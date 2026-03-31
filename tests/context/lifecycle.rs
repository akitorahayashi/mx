use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

#[test]
fn touch_cat_clean_lifecycle_is_consistent() {
    let ctx = TestContext::new();
    let clipboard = ctx.clipboard_file("clipboard.txt");
    fs::write(&clipboard, "workflow content").unwrap();

    ctx.cli()
        .args(["touch", "tk"])
        .assert()
        .success();

    ctx.cli()
        .args(["cat", "tk"])
        .assert()
        .success()
        .stdout(predicate::eq("workflow content"));

    ctx.cli()
        .args(["clean", "tk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));
}
