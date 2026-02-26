use crate::harness::TestContext;

pub fn install_sample_catalog(ctx: &TestContext) {
    ctx.write_snippet("w/wc.md", "# /wc\nPlan critically\n");
    ctx.write_snippet("sdd/sdd-0-rq.md", "Requirements prompt\n");
    ctx.write_config(
        r#"---
commands:
  wc:
    title: "Work on Tasks"
    description: "Critical planning workflow"
    prompt-file: "commands/w/wc.md"
  sdd-0-rq:
    title: "SDD Step 0"
    description: "Requirements capture"
    prompt-file: "commands/sdd/sdd-0-rq.md"
"#,
    );
}
