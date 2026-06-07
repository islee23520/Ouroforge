# Scenario Coverage v31 — Deck-Roguelike Game Class Regression Suite

Status: **Fixture-scoped regression coverage**

Issue: #1603 — Scenario Coverage v31: Deck-Roguelike Game Class Regression Suite
Anchor: #1 Era F Milestone 31 (Deck-Roguelike Game Class)

This coverage version locks the behavior of the Deck-Roguelike Game Class v1
(#1601) and its seeded stochastic determinism. It reuses the existing runtime,
probe, and replay-digest surfaces and the existing test/coverage harness; it
adds a regression suite, not a new engine. All cases assert states, shapes, and
digest goldens only — there are no flaky or timing-based assertions. Scenario
Coverage numbering continues from v26 (Era E) onward.

## Suite

- Runtime runner (authoritative, live):
  `examples/deck-roguelike-game-class-v1/scenario-coverage-v31-deck-roguelike.test.cjs`
- CI-gated Rust mirror:
  `crates/ouroforge-core/tests/scenario_coverage_v31_deck_roguelike.rs`
- Enumerated cases:
  `examples/deck-roguelike-game-class-v1/scenario-coverage-v31/cases.fixture.json`
- Backward-compatibility golden:
  `examples/deck-roguelike-game-class-v1/scenario-coverage-v31/non-stochastic-digest.golden.json`

## Covered regressions

1. **Seeded determinism (same seed).** A seeded run reproduces its opening hand,
   its win/survival outcome, and its replay-state digest golden. A change to the
   recorded digest fails CI.
2. **Seeded divergence (different seed).** A different seed shuffles to a
   different opening hand and a divergent replay-state digest.
3. **Snapshot across a draw.** Snapshot/restore returns the run (including the
   seeded random-stream position and deck/hand state) to the snapshot point, and
   the continuation reproduces the identical digest.
4. **Run reproducibility.** Two identical-seed runs driven by the same action
   sequence produce an identical replay-state digest.
5. **Backward compatibility.** The deck-roguelike digest key is additive: prior
   non-stochastic classes (the grid-puzzle game class) keep byte-identical
   replay-state digests. The golden records the initial and solved grid-puzzle
   digests; a change is a backward-incompatible regression and fails CI.

## Boundary

This is fixture-scoped regression coverage. It is deterministic and runs with no
network and no live browser, asserts only states/shapes and digest goldens,
performs no trusted writes from generation or any browser/Studio surface
(browser/Studio surfaces stay read-only), and authorizes no renderer, physics,
audio, or 3D breadth. It makes no production-ready, quality, fun, shippable, or
Godot-replacement/parity claim. Generated run digests remain ephemeral evidence
and are not committed. #1 and #23 remain open.
