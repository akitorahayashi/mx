use crate::harness::{install_sample_catalog, TestContext};
use predicates::prelude::*;

#[test]
fn list_command_prints_snippets() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("wc").and(predicate::str::contains("sdd/sdd-0-rq")));
}

#[test]
fn list_alias_ls_works() {
    let ctx = TestContext::new();
    install_sample_catalog(&ctx);

    ctx.cli()
        .arg("ls")
        .assert()
        .success()
        .stdout(predicate::str::contains("wc").and(predicate::str::contains("sdd/sdd-0-rq")));
}
