# Engine-Builder Balance Demo v1

Status: **Fixture-scoped deterministic demo; no executable authority beyond tests**

Issue: #1815 — Engine-Builder Balance Demo v1
Anchor: #1 Era I Milestone 50 (Engine-Builder Balance Verification v1)

This demo records a deterministic, local-only Engine-Builder Balance Verification
v1 walkthrough. It reuses the already scoped analyzer surfaces:

- Combo-Explosion and Degenerate-Build Detector v1 (#1812) over fixture-scoped
  synthetic run telemetry.
- Dominant-Build Analyzer v1 (#1813) over pick-rate/win-rate evidence.
- Fairness and Daily-Seed Solvability Verifier v1 (#1814) over seeded winning
  witnesses and loss-attribution evidence.

The demo is a fixture manifest and a Rust smoke test. It does not add a new
engine, run a live browser, contact the network, mutate trusted sources, promote
content, no auto-apply fixes, no auto-merge, or make a fun/quality/production/Godot
claim.

## Demo fixtures

`examples/engine-builder-balance-v1/demo/demo-manifest.json` points to the three
fixture-scoped inputs used by the smoke test:

| Surface | Fixture | Expected evidence |
| --- | --- | --- |
| Degenerate combo | `examples/engine-builder-balance-v1/combo-detector/degenerate-combo.fixture.json` | `overcharger` + `reactor-loop` is flagged with replay seed `310` and persona `smith`. |
| Dominant build | `examples/engine-builder-balance-v1/dominant-build/dominant-build.fixture.json` | `loop-engine` is dominant with replay seed `510` and persona `smith`; `rusty-bearing` is a dead modifier. |
| Fairness / daily seed | `examples/engine-builder-balance-v1/fairness/fairness.fixture.json` | daily seed `7202` is flagged as unfair/unwinnable; seed `7201` remains skilled-winnable. |

The smoke test recomputes these facts from evidence. The metrics are computed,
not asserted as raw claims, and the replay seeds are carried from the underlying
reports.

## Reproducibility

From a fresh clone, run:

```bash
cargo test -p ouroforge-core --test engine_builder_balance_demo_contract
```

The test is deterministic and local. It does not require network access, a live
browser, generated run directories, or credentials. Dashboard and cockpit
surfaces may display exported results read-only, but trusted validation and any
future source changes remain Rust/local and review-gated.

## Boundaries and generated state

Generated runs/artifacts remain untracked unless fixture-scoped. The checked-in
manifest and tiny telemetry fixtures are fixture-scoped regression data. Any
future generated balance report, dashboard export, cockpit export, replay bundle,
or temporary run output remains ignored/local unless a follow-up issue explicitly
scopes it as a deterministic fixture.

This demo is descriptive balance evidence only. Balance verdicts are not a fun
guarantee; the fun/feel verdict remains the human Era J gate. Fairness means a
skilled player can attribute losses to decisions, not luck. Issues #1 and #23 remain open.
