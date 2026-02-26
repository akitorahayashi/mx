use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;

#[test]
fn copy_fails_for_unknown_snippet() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .args(["copy", "unknown"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("No snippet named"));
}

#[test]
fn list_displays_catalog_entries() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("w/wc").and(predicate::str::contains("sdd/sdd-0-rq")));
}
