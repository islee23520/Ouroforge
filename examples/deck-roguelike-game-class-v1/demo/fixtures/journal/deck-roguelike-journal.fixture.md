# Deck-Roguelike Journal Fixture

Run: `m31-deck-roguelike-loop-demo`
Seed: `12345`

The deck-roguelike game class reused the existing runtime, probe, and replay-digest
surfaces without introducing a per-game evaluator bypass or a parallel engine. The
run's randomness comes only from the seeded `mulberry32` stream carried on the deck
state (the Milestone 31 seeded stochastic determinism contract), so the run is
digest-stable: an identical seed and action sequence reproduce the same replay-state
digest, and a different seed diverges detectably through `compareReplayDigest`.

Source fixtures, the four-gate verdict, and the loop-coverage attribution are all
fixture-scoped. Generated run digests remain ephemeral evidence and are not committed.

Generalization evidence comparing the deck-roguelike loop shape with the prior
genre rungs is recorded in
`examples/deck-roguelike-game-class-v1/demo/fixtures/generalization/comparable-pass.fixture.json`,
keeping cross-class comparison descriptive rather than a silent broad-genre claim.
