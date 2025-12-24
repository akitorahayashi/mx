mod common;

use common::TestContext;
use predicates::prelude::*;
use serial_test::serial;

#[test]
#[serial]
fn list_command_prints_snippets() {
    let ctx = TestContext::new();
    ctx.install_sample_catalog();

    ctx.cli()
        .arg("list")
        .assert()
        .success()
        // Metadata titles removed, just checking for keys/paths
        .stdout(predicate::str::contains("wc").and(predicate::str::contains("sdd/sdd-0-rq")));
}

#[test]
#[serial]
fn list_alias_ls_works() {
    let ctx = TestContext::new();
    ctx.install_sample_catalog();

    ctx.cli()
        .arg("ls")
        .assert()
        .success()
        // Metadata titles removed, just checking for keys/paths
        .stdout(predicate::str::contains("wc").and(predicate::str::contains("sdd/sdd-0-rq")));
}

#[test]
#[serial]
fn version_flag_works() {
    let ctx = TestContext::new();

    ctx.cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
#[serial]
fn clean_alias_cl_works() {
    let ctx = TestContext::new();

    // Create a context file
    ctx.cli().current_dir(ctx.work_dir()).arg("touch").arg("tk").assert().success();

    // Clean with alias
    ctx.cli()
        .current_dir(ctx.work_dir())
        .arg("cl")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));
}
