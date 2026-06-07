# Scenario Coverage v37 — Long-Form Game Systems Regression Suite

Issue: **#1663** (Era G Milestone 39). Part of Long-Form Game Systems v1
(#1656); locks the behavior of meta-progression/unlocks (#1657),
economy/currency (#1658), save/profile + run-history at scale (#1659), UI/UX
flow + accessibility (#1660), and the optional narrative/dialogue/event system
(#1661).

Scenario Coverage v37 is an enumerated, fixture-scoped regression suite. It
asserts **states and shapes only** — no flaky or timing-based assertions — so a
breaking change to a long-form system fails CI.

## What is covered

The matrix is
`examples/long-form-systems-v1/scenario-coverage-v37/matrix.fixture.json`,
driven by
`crates/ouroforge-core/tests/scenario_coverage_v37_long_form_systems.rs`. It
reuses the merged demo definitions under `examples/long-form-systems-v1/demo/`.

- **Meta-progression** — counters accrue and threshold unlocks gate; replay is
  deterministic; a restored state with an unlock not justified by its counters is
  rejected (fail-closed).
- **Economy** — earn/spend balances are deterministic; an overspend that would
  drive a balance negative is rejected (non-negative invariant, fail-closed).
- **Save/profile and run-history at scale** — multi-profile isolation (a write to
  one profile never alters another); a large (2000-entry) run-history round-trips
  with chained-digest integrity; a tampered history fails the integrity check
  (fail-closed).
- **UI/UX flow** — the flow contract validates with reachable screens and
  declared accessibility options; an unreachable screen is rejected (fail-closed).
- **Narrative** — a dialogue advances and its events fire deterministically from
  flag conditions; a restored state with an inconsistent node/event is rejected
  (fail-closed).
- **Backward compatibility** — an existing single-run `save-profile-v0` document
  still migrates into the v1 store and verifies, and a single-run v1 store still
  round-trips unchanged.

## Reproduce

```bash
cargo test -p ouroforge-core --test scenario_coverage_v37_long_form_systems
```

The suite reuses the Long-Form Game Systems v1 data systems
(`meta_progression`, `economy_system`, `save_profile_scale`, `uiux_flow`,
`narrative_system`) — it is a regression suite, not a new engine, runtime, or
writer. Scenario Coverage numbering continues from v36 (Era G) onward; #1 and #23
remain open anchors.
