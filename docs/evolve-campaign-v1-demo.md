# Multi-Iteration Evolve Campaign demo v1

This document records the Multi-Iteration Evolve Campaigns v1 demo for issue
#1490, under #1 Era E Milestone 23. It composes the completed campaign
capabilities only — the campaign model and stop conditions (#1487), convergence
tracking and budgets (#1488), and the journal narrative (#1489) — over the
existing **playable-demo-v2 / collect-and-exit (Signal Gate)** demo game.

It is a **descriptive, read-only audit** of two pre-recorded, deterministic
campaigns. It does **not** run autonomous agents, apply patches to the primary
working tree, merge anything, create branches, open GitHub PRs, reach the
network, or drive a live browser. It is not a quality, correctness, or
production-readiness guarantee.

## What the demo shows

The demo enumerates two campaigns against the Signal Gate collect-and-exit game,
both bounded by an explicit iteration/cost budget and declared stop conditions:

1. **Converging campaign** — a failing game converges to passing acceptance over
   four bounded iterations, with a full per-iteration audit trail
   (hypothesis → mutation → four-gate verdict → rerun delta → evidence) and a
   final `converged` summary.
2. **Non-converging campaign** — a stalled game stops safely at the iteration
   budget with an evidence-linked `not-converged` diagnosis. It never loops
   unbounded.

Both campaigns honor the Milestone 22 trust gradient: every iteration is
`manual-review` unless its mutation falls within the bounded auto-apply budget.

## Fixtures

Deterministic, fixture-scoped, checked into the repository:

```text
examples/evolve-campaign-v1/demo/manifest.json            # enumerates both cases
examples/evolve-campaign-v1/demo/converging.fixture.json  # acceptance-reached / converged
examples/evolve-campaign-v1/demo/non-converging.fixture.json  # budget-exhausted / not-converged
```

The `runs/...` references inside each fixture are illustrative run-relative
evidence refs; no generated run state is tracked.

## Converging campaign trajectory

| Iteration | mechanical | runtime | visual | semantic | decision |
| --- | --- | --- | --- | --- | --- |
| 0 (baseline) | fail | fail | fail | fail | manual-review |
| 1 | pass | pass | fail | fail | manual-review |
| 2 | pass | pass | pass | fail | manual-review |
| 3 | pass | pass | pass | pass | auto-apply |

Termination: `acceptance-reached` at iteration 3 (`stop-acceptance`). Outcome:
`converged`.

## Non-converging campaign trajectory

| Iteration | mechanical | runtime | visual | semantic | decision |
| --- | --- | --- | --- | --- | --- |
| 0 (baseline) | fail | fail | fail | fail | manual-review |
| 1 | pass | fail | fail | fail | manual-review |
| 2 | pass | pass | fail | fail | manual-review |
| 3 | pass | pass | pass | fail | manual-review |

Termination: `budget-exhausted` (`stop-budget`) after the iteration budget is
reached with the semantic gate still failing. Outcome: `not-converged` with an
evidence-linked diagnosis.

## Reproducing the demo

From a fresh clone, the demo reproduces deterministically with no network and no
live browser:

```bash
cargo test -p ouroforge-core --test evolve_campaign_demo_contract
```

The smoke test asserts, for the recorded fixtures:

- the converging campaign reaches passing acceptance over N bounded iterations
  with a complete per-iteration audit trail (the `converged` outcome and journal
  narrative);
- the non-converging campaign stops safely at the iteration budget with an
  evidence-linked diagnosis (the `not-converged` outcome and journal summary);
- the manifest enumerates both cases and every enumerated fixture validates.

## Boundary

Descriptive read-only audit of bounded multi-iteration evolve campaigns;
iterations are manual-review unless within the Milestone 22 bounded auto-apply
budget; no auto-merge; not a quality or correctness guarantee. #1 and #23 remain
open.
