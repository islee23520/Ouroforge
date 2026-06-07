# Long-Form Game Systems v1 Demo

Status: **Fixture-scoped deterministic demo — no network, no live browser**

Issue: #1662 — Long-Form Game Systems Demo v1
Anchor: #1 Era G Milestone 39 (Long-Form Game Systems), scope #1656

This document is the canonical evidence record for the Long-Form Game Systems v1
demo: a longer deterministic slice that **composes** the milestone's systems —
meta-progression/unlocks (#1657), economy/currency (#1658), save/profile and
run-history at scale (#1659), and UI/UX flow with accessibility (#1660), with the
optional narrative/dialogue/event system (#1661) — and records each as a
Milestone 24 ladder rung with passing four-gate and loop-coverage evidence.

It adds no new engine or runtime. The demo reuses the existing trusted Rust data
systems and the Game Complexity Ladder v1 contract
(`docs/game-complexity-ladder-v1.md`). Generation stays proposal-only, browser
surfaces stay read-only, and #1/#23 remain open anchors.

## What the demo composes

The demo smoke test (`crates/ouroforge-core/tests/long_form_systems_demo_contract.rs`)
runs one deterministic profile (`demo-hero`) through a longer slice:

1. **Meta-progression** — three recorded run outcomes accrue progression
   counters; the `gold-shop` and `second-character` unlocks gate on their
   thresholds and the `starter-kit` zero-threshold unlock is present from the
   start.
2. **Economy** — the runs earn currencies and a spend is integrity-checked; the
   non-negative invariant holds and balances are deterministic.
3. **Save/profile and run-history at scale** — each run is appended to the
   profile's run-history with its replay-digest, and the store's chained
   per-profile history digest verifies end to end.
4. **UI/UX flow and accessibility** — the in-game flow navigates
   `title → onboarding → hud` and an accessibility option (`textScale`) is set;
   the flow is deterministic and probe-shaped.
5. **Narrative (optional)** — a short dialogue advances and its declared events
   fire deterministically from flag conditions.

All state is held in the trusted Rust data systems; the slice replays
deterministically and reproduces the same composed state.

## Rung records (four-gate + loop-coverage)

The demo records each system as a rung in
`examples/long-form-systems-v1/demo/complexity-ladder.json` under the Game
Complexity Ladder v1 contract. Each rung carries:

- a loop-produced demo reference (`loopProducedDemo: true`, a current `demoRef`);
- a four-gate verdict (mechanical / runtime / visual / semantic) — all `pass`,
  with a current `verdictRef`;
- a loop-coverage verdict — `pass`, with a current `verdictRef`.

`evaluate_complexity_ladder` evaluates the ladder and reports every rung
`satisfied`, in contiguous order, demonstrating the rung linkage. This is a
descriptive evidence record, not a quality, fun, or production-readiness claim.

## Reproduce locally

```bash
cargo test -p ouroforge-core --test long_form_systems_demo_contract
```

The demo is deterministic and requires no network or live browser. The
referenced fixtures live under `examples/long-form-systems-v1/demo/`.

## Governance

- Generated runs/assets/content/release artifacts are not committed; the only
  added data is the fixture-scoped demo definitions and ladder under
  `examples/long-form-systems-v1/demo/`.
- Wording is conservative: no auto-merge/quality/fun/production/Godot-replacement
  claim. The four-gate and loop-coverage records are descriptive evidence.
- Additive and backward-compatible; #1 and #23 remain open anchors.
