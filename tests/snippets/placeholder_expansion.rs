use crate::harness::TestContext;
use predicates::prelude::*;
use std::fs;

#[test]
fn copy_expands_workspace_placeholders() {
    let ctx = TestContext::new();
    ctx.write_snippet("w/wc.md", "header\n{{.mx/info.md}}\nfooter\n");
    let clipboard = ctx.clipboard_file("clipboard.txt");

    let mx_dir = ctx.work_dir().join(".mx");
    fs::create_dir_all(&mx_dir).unwrap();
    fs::write(mx_dir.join("info.md"), "dynamic info").unwrap();

    ctx.cli()
        .args(["copy", "wc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Copied 'wc'"));

    let copied = fs::read_to_string(clipboard).unwrap();
    assert!(copied.contains("dynamic info"));
}
