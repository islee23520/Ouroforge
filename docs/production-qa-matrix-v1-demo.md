# Production-Scale QA Matrix v1 demo

Issue: #1670
Roadmap anchor: #1 Era G Milestone 40 (Production-Scale QA Matrix).

This fixture-scoped demo records a deterministic Production-Scale QA Matrix run
for a small `collect-and-exit` game build. It demonstrates the matrix catching a
planted cross-variant regression with replayable evidence, and a consolidated
production-QA verdict that fails closed because of it. It reuses the existing
regression-matrix and consolidated-verdict contracts; it adds no new engine,
runtime, or writer. The demo claims gate mechanics only: it does not claim fun,
aesthetic quality, production readiness, current Godot replacement, source
mutation authority, or any trusted browser action. The verdict is descriptive
evidence, not a quality guarantee.

## Fixtures

- Regression matrix: `examples/production-qa-matrix-v1/demo/demo.matrix.fixture.json`
- Consolidated verdict: `examples/production-qa-matrix-v1/demo/demo.verdict.fixture.json`
- Smoke test: `crates/ouroforge-core/tests/production_qa_matrix_demo_contract.rs`

The demo reproduces deterministically from committed fixture JSON. It requires no
network access and no live browser.

## What the demo shows

The build is exercised across a `content variant x seed x target` matrix:

| contentVariant | seed   | target  | verdict |
| -------------- | ------ | ------- | ------- |
| base           | seed-1 | web     | passed  |
| base           | seed-1 | desktop | passed  |
| candidate      | seed-1 | web     | failed  |
| candidate      | seed-1 | desktop | passed  |

The `base` content variant is the regression baseline. The `candidate` variant
regresses on the `(seed-1, web)` coordinate: the baseline passed but the
candidate failed. The matrix read model surfaces exactly one detected
cross-variant regression, carrying both the baseline and the candidate evidence
refs so the regression is replayable.

The consolidated verdict composes the per-check results via the evaluator's
`declared-gate-and` aggregation. The `regressionMatrix` check is declared and
fails (it references the demo matrix above); the visual, performance/soak, and
crash checks pass; the accessibility check is undeclared and therefore neutral.
Because one declared check fails, the consolidated production-QA verdict is
`fail` — a single failing check propagates and fails closed.

## Expected outcomes

- `demo.matrix.fixture.json`: status `complete`, exactly one detected
  cross-variant regression at `candidate / seed-1 / web`, replayable from the
  recorded evidence refs.
- `demo.verdict.fixture.json`: consolidated verdict `fail`, with the
  `regressionMatrix` check as the failing check; aggregation operator
  `declared-gate-and`, undeclared-gate policy `neutral`.

## Boundaries

- Synthetic, deterministic, fixture-scoped; no network, no live browser, no
  real-player data.
- Rust/local owns the matrix and verdict logic; browser/Studio surfaces remain
  read-only; the demo performs no trusted mutation, auto-fix, auto-apply, or
  auto-merge.
- The verdict is descriptive evidence only, never a quality/fun guarantee or a
  release authority. #1 and #23 remain open.
