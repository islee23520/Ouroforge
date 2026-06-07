# Scenario Coverage v38 — Production-Scale QA Matrix Regression Suite

Issue: #1671
Roadmap anchor: #1 Era G Milestone 40 (Production-Scale QA Matrix).

Scenario Coverage v38 locks the behavior of Production-Scale QA Matrix v1 with an
enumerated regression suite. It asserts **states and shapes only** — no flaky or
timing-based assertions — so a breaking change to any composed surface fails CI.
It reuses the existing test/coverage harness; it is a regression suite, not a new
engine.

## Enumerated coverage

The suite matrix is `examples/production-qa-matrix-v1/scenario-coverage-v38/matrix.fixture.json`
and the runner is `crates/ouroforge-core/tests/scenario_coverage_v38_production_qa_matrix.rs`.

| system      | scenarios |
| ----------- | --------- |
| matrix      | aggregation (complete, full coverage), cross-variant regression (detected), malformed (fail-closed) |
| visual      | baseline match (no regression), planted diff (detected), missing baseline (surfaced) |
| performance | budget pass, planted regression (regressed), soak drift (unstable) |
| verdict     | consolidated pass, failure propagates (fail), inconclusive, crash/accessibility checks composed |
| backcompat  | existing per-artifact QA gates remain valid (golden) |

Each scenario re-exercises the merged module behavior:

- **Matrix** (#1666): aggregation roll-up, cross-variant regression detection, and
  fail-closed validation of the `production_qa_matrix` artifact.
- **Visual** (#1667): baseline match, planted visual diff, and explicit
  missing-baseline handling of the `visual_regression_scale` artifact.
- **Performance/soak** (#1668): budget pass, planted budget regression, and soak
  drift instability of the `performance_soak` artifact.
- **Verdict** (#1669): consolidated `pass`/`fail`/`inconclusive` outcomes via the
  `declared-gate-and` aggregation, including the crash and accessibility checks.

## Backward compatibility

The production-scale matrix composes existing per-artifact gates. The suite's
backward-compatibility golden re-validates two of those gates unchanged — the QA
swarm run matrix (`qa-swarm-run-matrix-v1`) and the QA performance budget
(`qa-performance-budget-v1`) — so the new milestone never regresses the
pre-existing per-artifact contracts.

## Boundaries

- Synthetic, deterministic, fixture-scoped; no network, no live browser, no
  real-player data.
- Coverage asserts states/shapes only; it is descriptive and makes no quality,
  fun, production-ready, or Godot-replacement claim.
- Rust/local owns the logic; browser/Studio surfaces remain read-only; additive
  and backward-compatible. No auto-merge or self-approval.
- #1 and #23 remain open.
