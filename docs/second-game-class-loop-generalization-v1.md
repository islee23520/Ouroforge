# Second Game Class Loop Generalization v1

Milestone 21 validates one additional bounded game class through the same local authoring loop shape already used by collect-and-exit:

Seed -> Build -> Observe -> Verify -> Journal -> Evolve.

The scoped second class is `signal-gate-platformer`, a one-screen obstacle-and-signal platformer fixture. It is intentionally narrow: one player avatar, one jumpable hazard lane, one timed signal gate, one exit, deterministic replay inputs, and four-gate evidence over mechanical, runtime, visual, and semantic checks. This is not a broad genre claim and does not replace engine runtime validation.

## Contract

- The second class lives under `examples/signal-gate-platformer/`.
- The GDD lives in `docs/signal-gate-platformer-gdd-v1.md`.
- The example reuses the existing Seed, scenario pack, project manifest, loop coverage, four-gate verdict, journal, and comparison evidence shapes.
- No per-game evaluator escape hatch is allowed. Any missing generalization behavior is represented as an explicit structured gap finding with evidence refs.
- Evidence remains comparable with collect-and-exit: both classes expose source refs, seed refs, scenario refs, verdict gate ids, loop stage refs, journal refs, and loop coverage refs.
- Generated run output remains untracked. The committed artifacts are deterministic fixtures and are labeled `fixtureScoped: true`.
- Rust/local validation remains the trusted source for source fixture parsing and comparison shape tests.

## Evidence Map

| Requirement | Evidence |
| --- | --- |
| Distinct second game class | `examples/signal-gate-platformer/seeds/signal-gate-platformer.yaml`, `docs/signal-gate-platformer-gdd-v1.md` |
| Same loop shape | `examples/signal-gate-platformer/fixtures/loop/signal-gate-loop-run.fixture.json` |
| Four-gate evaluator shape | `examples/signal-gate-platformer/fixtures/evidence/four-gate-verdict.fixture.json` |
| Loop coverage metric shape | `examples/signal-gate-platformer/fixtures/evidence/loop-coverage.fixture.json` |
| Collect-and-exit comparison | `examples/signal-gate-platformer/fixtures/generalization/comparable-pass.fixture.json` |
| Explicit gap findings | `examples/signal-gate-platformer/fixtures/generalization/gap-found.fixture.json` |
| Scenario Coverage v22 | `docs/scenario-coverage-v22.md`, `examples/signal-gate-platformer/scenario-coverage-v22-loop-generalization.test.cjs` |

## Boundaries

The committed fixtures are deterministic contract examples, not generated run products. The v1 result is evidence that a second bounded game class can use the same loop-shaped contracts as collect-and-exit. It does not claim broad platformer support, does not claim production readiness, and does not claim Godot replacement capability.
