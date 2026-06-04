# Agentic Level Design Regression Suite v1

This fixture package is the #641 Scenario Coverage v10 source index for
Agentic Scene and Level Designer v1. The demo in #640 proves composition; this
matrix proves each feature area has focused regression anchors.

Run:

```bash
node examples/agentic-level-design-regression-suite-v1/coverage-smoke.test.cjs
```

The smoke checks that every matrix row has valid, edge/stale/missing, malformed,
and runnable test anchors, and that generated roots remain absent.

Generated `runs/`, `dashboard-data/`, and `tmp/` directories under this fixture
remain untracked local evidence only.
