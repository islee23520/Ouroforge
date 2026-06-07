# Design Regression Harness v1 — Demo

Issue: **#1589** (Era F Milestone 29, under #1). Builds on the Milestone 29 harness
model and diff (#1588) and the Milestone 28 solver / over-solution detector / difficulty
suite (#1580–#1582).

This is a **deterministic, fixture-scoped demo** of design regression as CI for game
design. It reproduces from a fresh clone with **no network and no live browser**. It shows
the harness re-running the existing solver, over-solution detector, and difficulty suite
across the affected levels, diffing against the recorded baseline, and:

1. **flagging a regression with a replayable trace** when a shared-rule edit opens a new
   over-solution **elsewhere**, and
2. **passing a clean edit** with no false regression.

The demo asserts **behavior and gate state**, never subjective quality. It performs no
trusted write: detection only, human-in-the-loop. It adds no new engine — it composes the
existing surfaces through the `design-regression-harness-v1` artifact.

## Fixtures

- [`examples/design-regression-harness-v1/demo/regression-edit.json`](../examples/design-regression-harness-v1/demo/regression-edit.json)
- [`examples/design-regression-harness-v1/demo/clean-edit.json`](../examples/design-regression-harness-v1/demo/clean-edit.json)

Both are `design-regression-harness-v1` artifacts: an `editRef`, an untracked
`generatedOutputRoot`, and the affected grid-puzzle levels (each carrying the current
post-edit `ouroforge.grid-puzzle.v1` spec, the captured designer `intent`, and the design
status recorded at `baseline`).

## Scenario 1 — a rule edit opens a new over-solution *elsewhere* (regression)

`regression-edit.json` re-proves one shared-rule edit across two affected levels:

| Level | Role | Baseline | After the edit | Outcome |
| --- | --- | --- | --- | --- |
| `rule-edited-target` | the directly-edited level | clean (solvable, no over-solution) | still clean | `unchanged` |
| `shared-mechanic-elsewhere` | another level sharing the edited rule | clean (solvable, no over-solution) | a 1-push shortcut now wins, bypassing the intended 3-step path | **`newly-broken`** |

The edited level is fine — but the **same rule edit** opens a brand-new unintended
over-solution **elsewhere**. The harness re-runs every affected level, so it catches the
regression in `shared-mechanic-elsewhere`:

- overall verdict: **`regressed`**, `regressionCount = 1`, promotion **blocked**;
- the flagged level carries a **replayable counterexample trace** `["left"]`
  (`traceKind = shorter-than-intended`). Replaying `["left"]` on the deterministic runtime
  pushes the crate straight onto its target and wins in a single move — *watch the bypass*,
  not "trust me". The demo smoke test replays the trace on the trusted stepper and confirms
  it reaches the win state (trace linkage).

The grid for `shared-mechanic-elsewhere`:

```
#####
#@*P#   @ target · * crate · P player
#...#   designer intent: ["down","up","left"] (3 steps)
#####   over-solution: ["left"] (1 step) — strictly shorter, flagged
```

## Scenario 2 — a clean edit passes

`clean-edit.json` re-proves a non-structural edit across two clean levels. Solvability, the
over-solution set, and difficulty are all equivalent to baseline, so every level classifies
`unchanged`:

- overall verdict: **`clean`**, `regressionCount = 0`, promotion **not** blocked;
- no level is `newly-broken`; no false trace is produced.

## Reproduce

```bash
cargo test -p ouroforge-core --test design_regression_harness_demo
```

The smoke test
[`crates/ouroforge-core/tests/design_regression_harness_demo.rs`](../crates/ouroforge-core/tests/design_regression_harness_demo.rs)
asserts the regression classification, the replayable-trace linkage (the trace genuinely
wins on the trusted stepper), and the clean edit passing.

## Boundaries

- Deterministic and fixture-scoped; no network, no live browser, no non-deterministic state.
- Trusted logic is Rust/local; browser/Studio surfaces are read-only.
- Detection only: no auto-fix, no auto-apply, no auto-merge, no self-approval, no reviewer
  bypass. Any trusted write stays on the existing review/apply/trust-gradient path.
- No production-ready, Godot-replacement/parity, or quality/fun claim. A regression verdict
  blocks promotion; it does not change content.
- Generated re-run artifacts stay untracked under `generatedOutputRoot`.
- #1 and #23 remain open governance anchors.
