//! Tests for CLI-level features (version, help, etc.)

use super::super::common::TestContext;
use predicates::prelude::*;

#[test]
fn version_flag_works() {
    let ctx = TestContext::new();

    ctx.cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
