# Playtest and Fun-Feel Gate Demo v1

This fixture-scoped demo shows the Human Playtest Harness and Fun-Feel Gate v1
flow from #1860 without network access or a live browser:

1. `examples/playtest-funfeel-v1/demo/playtest-session-demo-v1.json` captures a
   bounded local human playtest session as evidence only.
2. `examples/playtest-funfeel-v1/demo/funfeel-gate-no-verdict-demo-v1.json`
   evaluates the same scoped candidate with no human verdict and therefore
   reports `needs-human-review`; release-readiness remains blocked.
3. `examples/playtest-funfeel-v1/demo/funfeel-gate-recorded-verdict-demo-v1.json`
   records a human reviewer verdict for the same candidate/capture refs and
   reports `approved-by-human` as a release-readiness precondition.

The demo asserts behavior and gate states, not subjective fun. It does not
produce an automated fun score, quality score, market-demand score, production
readiness claim, Godot replacement claim, trusted write, release button, or
browser/Studio write authority. Browser, dashboard, and Studio surfaces may only
inspect this evidence read-only unless a later governance issue scopes a trusted
Rust/local writer.

Generated playtest runs, logs, and artifacts remain untracked unless explicitly
fixture-scoped. Issues #1 and #23 remain open governance anchors.

Run the smoke contract with:

```bash
cargo test -p ouroforge-core --test playtest_funfeel_demo_contract
```
