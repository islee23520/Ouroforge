# Deck-Roguelike Game Class v1 Demo

Status: **Loop-produced, fixture-scoped genre-rung demo**

Issue: #1602 — Deck-Roguelike Game Class Demo v1
Anchor: #1 Era F Milestone 31 (Deck-Roguelike Game Class), recorded under the
Game Complexity Ladder (`docs/game-complexity-ladder-v1.md`).

This document is the canonical demo artifact for the Deck-Roguelike Game Class
v1 (#1601). It records a deterministic, seed-reproducible single-encounter deck
run with passing four-gate and Milestone 20 loop-coverage evidence, and it
records the deck-roguelike rung under the Milestone 24 ladder. It builds on the
existing runtime, probe, and replay-digest surfaces and the prior Signal Gate /
genre demo trees; it adds no new engine, runtime, or writer.

## Demo tree

The demo lives under `examples/deck-roguelike-game-class-v1/demo/`:

- `ouroforge.project.json` — project manifest (validated by `ProjectManifest`).
- `seeds/deck-roguelike-demo.yaml` — Seed (validated by `Seed`).
- `scenarios/deck-roguelike-demo.json` — scenario pack (validated by `ScenarioPack`).
- `scenes/deck-roguelike-demo.scene.json` — scene carrying the `deckRoguelike`
  spec, run seed `12345`.
- `fixtures/loop/deck-roguelike-loop-run.fixture.json` — six-stage loop run.
- `fixtures/evidence/four-gate-verdict.fixture.json` — passing mechanical,
  runtime, visual, and semantic gates.
- `fixtures/evidence/loop-coverage.fixture.json` — Milestone 20 loop-coverage
  attribution (`coverageFraction` 1.0, `computed`).
- `fixtures/journal/deck-roguelike-journal.fixture.md` — run journal.
- `fixtures/generalization/comparable-pass.fixture.json` — loop-shape comparison
  with the prior genre rung.
- `rung-demo.fixture.json` — the Milestone 24 ladder rung record.
- `README.md`, `demo-smoke.test.cjs`.

## What the demo proves

- **Seed reproducibility (live).** `demo-smoke.test.cjs` loads the demo scene
  into the existing runtime, drives the seeded run twice, and asserts an
  identical replay-state digest, then asserts a divergent digest for a different
  seed. All randomness comes from the seeded `mulberry32` stream carried on the
  deck state (the Milestone 31 seeded stochastic determinism contract); no
  wall-clock, host entropy, or `Math.random`.
- **Four-gate evidence.** The verdict fixture records passing mechanical,
  runtime, visual, and semantic gates with refs to the source inputs and the
  runtime module.
- **Loop coverage.** The loop-coverage fixture attributes all four loop inputs
  to loop-produced / loop-verified provenance.
- **Ladder rung.** `rung-demo.fixture.json` records the deck-roguelike rung as
  `satisfied` on the `seeded-stochastic-state` capability axis, backed by the
  loop-produced contracts and smoke tests.

## Trusted validation

- `crates/ouroforge-core/tests/deck_roguelike_demo_contract.rs` validates the
  demo source inputs through the existing manifest/seed/scenario contracts and
  machine-checks the evidence-fixture shapes, gate states, rung linkage, and
  guardrails as part of `cargo test`.
- `crates/ouroforge-core/tests/deck_roguelike_game_class_contract.rs` (#1601) is
  the trusted Rust mirror of the deck-roguelike semantics and seed
  reproducibility.

## Boundary

This is a fixture-scoped deck-roguelike genre proof. It is deterministic and
runs with no network and no live browser. It makes no production-readiness,
quality, fun, or Godot-replacement/parity claim, performs no trusted writes from
generation or any browser/Studio surface (browser/Studio surfaces stay
read-only), and does not authorize renderer, physics, audio, animation, or 3D
breadth. It does not modify or close #1 or #23. Generated run digests remain
ephemeral evidence and are not committed.
