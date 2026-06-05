# Full Studio Editor Regression Suite v17

Issue: #775

This directory contains the fixture-scoped Scenario Coverage v17 matrix for Full
Studio Editor v1. It locks successful read-only/draft-only Studio surfaces and
fail-closed unsafe cases without adding new editor authority.

Run from the repository root:

```bash
node --check examples/full-studio-editor-regression-v17/coverage-smoke.test.cjs
node examples/full-studio-editor-regression-v17/coverage-smoke.test.cjs
node examples/authoring-cockpit/cockpit.test.cjs
```

Generated outputs remain ignored under roots such as `runs/`, `target/`, `.omx/`,
`.omc/`, and `.openchrome/`. The tracked fixture is deterministic and small.
