# QA Swarm Regression Suite v1 fixtures

Issue: #697 — Scenario Coverage v13: QA Swarm Regression Suite.

`coverage.matrix.json` is validated by `ouroforge_core::qa_regression_coverage`.
It enumerates every QA/playtest area and records where its regression coverage
lives. For #697 closure, every required QA/playtest area is `in-repo`: either
re-validated directly by the regression suite or linked to its focused contract
test. Future unsupported areas must be listed as `documented-gap` with an honest
`knownGaps` entry.

## Valid fixture

- `coverage.matrix.json` — all fourteen areas, with known gaps stated honestly.

## Invalid fixtures (fail closed)

- `invalid/coverage.missing-area.json`
- `invalid/coverage.unsupported-status.json`
- `invalid/coverage.unsafe-ref.json`
- `invalid/coverage.missing-known-gaps.json`
- `invalid/coverage.unsafe-boundary.json`
