# Scenario Coverage v52: Playtest and Fun-Feel Gate Regression Suite

Issue: #1861  
Anchor: #1 Era J Milestone 58 (Human Playtest Harness and Fun-Feel Gate v1)

Scenario Coverage v52 locks the playtest capture and human fun-feel gate
contracts with deterministic state/shape checks only. It covers structured
playtest capture, the release-readiness block without a human verdict, the
release-readiness unblock after a recorded human verdict, a no-auto-score drift
guard, the fixture-scoped demo, and a backward-compatibility golden for the
existing evaluator gate aggregation.

The suite is local and fixture-scoped. It does not run a live browser, use the
network, assert timing, mutate trusted sources, auto-apply fixes, auto-merge,
self-approve, or claim automated fun, quality, release, production readiness,
market demand, or Godot parity. Browser/Studio surfaces remain read-only.
Generated runs/artifacts remain untracked unless fixture-scoped. Issues #1 and
#23 remain open.

## Matrix

`examples/playtest-funfeel-v1/scenario-coverage-v52/matrix.fixture.json`
enumerates these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V52.capture.shape` | #1858 structured playtest capture | capture validates as evidence-only and has no trusted write or release authority. |
| `V52.gate.no_verdict_blocks` | #1859 human gate | missing human verdict reports `needs-human-review` and blocks release-readiness. |
| `V52.gate.recorded_verdict_unblocks` | #1859 human gate | recorded human verdict reports `approved-by-human` for the scoped evidence. |
| `V52.gate.no_auto_score` | #1859 drift guard | automated or non-human fun verdict attempts remain blocked. |
| `V52.demo.smoke` | #1860 demo | deterministic `capture-valid -> needs-human-review -> approved-by-human` sequence. |
| `V52.evaluator_aggregation.backcompat` | existing evaluator aggregation | prior gate field names and evaluator writer ownership remain covered. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v52_playtest_funfeel
```

The runner reads committed fixtures, recomputes gate states, and checks the
existing evaluator aggregation compatibility contract is still present. It
intentionally avoids flaky assertions and subjective fun, quality, release, or
market judgments.
