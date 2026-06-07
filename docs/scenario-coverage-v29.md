# Scenario Coverage v29 — Design Regression Harness Regression Suite

Status: **fixture-scoped regression coverage**

Issue: #1590 — Scenario Coverage v29: Design Regression Harness Regression Suite
Anchor: #1 Era F Milestone 29 (Design Regression Harness v1)

Locks Design Regression Harness v1 (#1588) behavior: newly-broken / improved / unchanged
classification, replayable trace linkage for regressions, stale baseline fail-closed as
inconclusive, and backward compatibility proving single-run solver (#1580) remains valid
outside the harness. Asserts states and shapes only; no flaky or timing assertions.

## Suite

- Matrix: `examples/design-regression-harness-v1/scenario-coverage-v29/matrix.fixture.json`
- Runtime runner: `examples/design-regression-harness-v1/scenario-coverage-v29-design-regression.test.cjs`
- CI-gated Rust mirror: `crates/ouroforge-core/tests/scenario_coverage_v29_design_regression.rs`

## Boundary

fixture-scoped, deterministic, no network, browser/Studio read-only, no trusted writes,
no production-ready or Godot-replacement/parity claim. Generated runs remain untracked.
#1 and #23 remain open.
