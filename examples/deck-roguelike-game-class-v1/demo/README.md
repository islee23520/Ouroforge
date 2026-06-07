# Deck-Roguelike Game Class Demo (Milestone 31, #1602)

A deterministic, fixture-scoped demo that exercises the Deck-Roguelike Game
Class v1 (#1601) through the existing game-runtime, probe, and replay-digest
surfaces. It is a genre-rung proof, not a shipped game: it records a
seed-reproducible single-encounter deck run with passing four-gate and
loop-coverage evidence and a Milestone 24 ladder rung record.

## Demo inputs

- Project manifest: `ouroforge.project.json`
- Seed: `seeds/deck-roguelike-demo.yaml`
- Scenario pack: `scenarios/deck-roguelike-demo.json`
- Scene fixture: `scenes/deck-roguelike-demo.scene.json` (carries the
  `deckRoguelike` spec, seed `12345`)

## Demo evidence

- Loop run: `fixtures/loop/deck-roguelike-loop-run.fixture.json`
- Four-gate verdict: `fixtures/evidence/four-gate-verdict.fixture.json`
- Loop coverage: `fixtures/evidence/loop-coverage.fixture.json`
- Journal: `fixtures/journal/deck-roguelike-journal.fixture.md`
- Generalization comparison: `fixtures/generalization/comparable-pass.fixture.json`
- Ladder rung record: `rung-demo.fixture.json`

The committed demo records the same six loop stages as the prior genre rungs —
Seed, Build, Observe, Verify, Journal, and Evolve. Seed reproducibility is shown
live: `demo-smoke.test.cjs` loads the demo scene into the runtime, drives the
seeded run twice, and asserts an identical replay-state digest (and a divergent
digest for a different seed). The Rust contract
`crates/ouroforge-core/tests/deck_roguelike_demo_contract.rs` validates the
fixture evidence offline as part of `cargo test`.

## Boundary

This demo is a fixture-scoped deck-roguelike genre proof. All randomness is
seeded and replay-stable; nothing is driven by network or a live browser. It
makes no production-readiness, quality, fun, or Godot-replacement/parity claim,
performs no trusted writes from generation or any browser/Studio surface, and
does not modify or close #1 or #23. Generated run digests remain ephemeral
evidence and are not committed.
