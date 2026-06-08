# Scenario Coverage v45: Engine-Builder Balance Regression Suite

Issue: #1816
Anchor: #1 Era I Milestone 50 (Engine-Builder Balance Verification v1)

Scenario Coverage v45 locks the Engine-Builder Balance Verification v1 surfaces
with deterministic state/shape checks only. It covers combo-explosion detection,
dominant-build and dead-modifier metrics, fairness/daily-seed solvability, the
fixture-scoped demo, and a Milestone 32 synthetic-balance backward-compatibility
report.

The suite is fixture-scoped and local. It does not run a live browser, use the
network, assert timing, mutate trusted sources, auto-apply fixes, auto-merge,
self-approve, or claim fun/quality/production/Godot parity. Browser/Studio
surfaces remain read-only. Generated runs/artifacts remain untracked unless
fixture-scoped. Issues #1 and #23 remain open.

## Matrix

`examples/engine-builder-balance-v1/scenario-coverage-v45/matrix.fixture.json`
enumerates these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V45.combo.detect` | #1812 combo detector | planted `overcharger` + `reactor-loop` finding with replay seed. |
| `V45.combo.balanced` | #1812 combo detector | balanced fixture has no finding. |
| `V45.dominant.detect` | #1813 dominant analyzer | `loop-engine` dominant build and `rusty-bearing` dead modifier. |
| `V45.dominant.balanced` | #1813 dominant analyzer | balanced build mix has no dominant or dead flags. |
| `V45.fairness.unfair` | #1814 fairness verifier | daily seed `7202` is unfair/unwinnable; seed `7201` passes. |
| `V45.demo.smoke` | #1815 demo | manifest remains local, deterministic, and recomputable. |
| `V45.m32.backcompat` | Milestone 32 synthetic balance | existing `ouroforge.balance-report.v1` shape remains valid. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v45_engine_builder_balance
```

The runner recomputes analyzer states from fixtures and checks the M32 report
shape. It intentionally avoids flaky assertions and subjective balance/fun
judgments.
