# Second Game Class Demo

The deterministic demo for Milestone 21 is `examples/signal-gate-platformer`.

## Demo Inputs

- Project manifest: `examples/signal-gate-platformer/ouroforge.project.json`
- Seed: `examples/signal-gate-platformer/seeds/signal-gate-platformer.yaml`
- Scenario pack: `examples/signal-gate-platformer/scenarios/signal-gate-platformer.json`
- Scene fixture: `examples/signal-gate-platformer/scenes/signal-gate-platformer.scene.json`

## Demo Evidence

- Loop run: `examples/signal-gate-platformer/fixtures/loop/signal-gate-loop-run.fixture.json`
- Four-gate verdict: `examples/signal-gate-platformer/fixtures/evidence/four-gate-verdict.fixture.json`
- Loop coverage: `examples/signal-gate-platformer/fixtures/evidence/loop-coverage.fixture.json`
- Journal: `examples/signal-gate-platformer/fixtures/journal/signal-gate-journal.fixture.md`
- Generalization comparison: `examples/signal-gate-platformer/fixtures/generalization/comparable-pass.fixture.json`

The committed demo records the same loop stages as collect-and-exit: Seed, Build, Observe, Verify, Journal, and Evolve. The smoke and Scenario Coverage v22 tests validate the artifact refs offline.

## Boundary

This demo is a fixture-scoped second-class proof. It does not claim broad genre support, does not claim production readiness, and does not claim replacement of any external engine.
