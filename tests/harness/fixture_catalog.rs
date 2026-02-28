use crate::harness::TestContext;

pub fn install_sample_catalog(ctx: &TestContext) {
    ctx.write_snippet("w/wc.md", "# /wc\nPlan critically\n");
    ctx.write_snippet("sdd/sdd-0-rq.md", "Requirements prompt\n");
}
