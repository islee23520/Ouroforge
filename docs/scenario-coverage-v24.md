# Scenario Coverage v24: Evolve Campaign Regression Suite

Scenario Coverage v24 locks the behavior of Multi-Iteration Evolve Campaigns v1
(#1486–#1489) under #1 Era E Milestone 23 (#23). It is a deterministic,
fixture-scoped regression suite: it asserts states and shapes only — no
subjective quality, no flaky or timing assertions — and a breaking change to the
campaign model, convergence outcome, or journal narrative fails CI.

This document is the control artifact for #1491. It adds no product behavior; it
enumerates the regression coverage and the boundary the suite enforces.

## Boundary

- Deterministic fixture coverage only; descriptive read-only.
- No subjective quality assertions; no auto-fix, no auto-apply, no auto-merge.
- Generated artifacts are untracked unless fixture-scoped.
- Public wording is conservative; no production-readiness or Godot-replacement
  claim. #1 and #23 remain open anchors.

## Coverage

The suite runs through two runners over the same fixtures:

- `crates/ouroforge-core/tests/scenario_coverage_v24_evolve_campaign.rs` — the
  Rust regression that exercises the real `validate_evolve_campaign`,
  `compute_evolve_campaign_outcome`, and `build_evolve_campaign_journal` APIs.
- `examples/evolve-campaign-v1/scenario-coverage-v24-evolve-campaign.test.cjs` —
  the JSON-shape runner that asserts the fixture matrix and shapes.

### Campaign termination

| Case | Fixture | Stop reason | Outcome |
| --- | --- | --- | --- |
| `v24.termination-acceptance-reached` | `acceptance.fixture.json` | `acceptance-reached` | `converged` |
| `v24.termination-budget-exhausted` | `budget.fixture.json` | `budget-exhausted` | `not-converged` |
| `v24.termination-no-progress` | `no-progress.fixture.json` | `no-progress` | `not-converged` |

Every campaign terminates on a declared stop condition; non-convergence stops
with an evidence-linked diagnosis and never loops unbounded.

### Convergence outcomes

Each termination case is checked for its converged/not-converged outcome, a
recorded per-iteration four-gate verdict delta trajectory, and — for
not-converged cases — a non-empty diagnosis with no claimed accepted iteration.

### Journal narrative

Each campaign produces a per-iteration narrative (baseline flagged, evidence
linked) and a final converged/not-converged summary consistent with the
termination.

### Backward compatibility

| Case | Fixture | Note |
| --- | --- | --- |
| `v24.backward-compat-single-shot-evolve` | `single-shot.fixture.json` | a one-iteration campaign equals a single-shot evolve and remains valid |

The single-shot golden ensures the campaign model is a superset of the existing
single-shot evolve path: a one-iteration campaign that reaches acceptance
validates, converges at the baseline iteration, and renders a baseline-only
narrative. The existing single-shot evolve mechanism (`evolve_run` /
`EvolveSummary`) is unchanged and remains covered by the workspace test suite.

## Verification

```bash
cargo test -p ouroforge-core --test scenario_coverage_v24_evolve_campaign
node examples/evolve-campaign-v1/scenario-coverage-v24-evolve-campaign.test.cjs
```
