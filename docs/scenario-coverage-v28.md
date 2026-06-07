# Scenario Coverage v28: Solver and Over-Solution Regression Suite

Issue: #1585

Scenario Coverage v28 locks deterministic, fixture-scoped regression coverage
for Puzzle Solver and Over-Solution Detection v1
(`docs/puzzle-solver-oversolution-v1.md`, Era F Milestone 28). It enumerates the
solver verdicts, the over-solution detector verdicts, difficulty-metric
computation, and the design-integrity gate verdicts, plus a
backward-compatibility golden proving the existing four-gate aggregation is
unchanged. It asserts states and shapes only — no flaky or timing-based
assertions.

## Fixture Layout

- `examples/puzzle-solver-oversolution-v1/scenario-coverage-v28/matrix.fixture.json`
  enumerates the regression cases and wording audit.
- `compatibility/four-gate-aggregation.golden.json` pins the existing
  `declared-gate-and` four-gate aggregation shape.
- The level and evidence specs are reused from the fixtures committed by
  #1580/#1581/#1582 (`examples/game-runtime/grid-puzzle-*.json`), referenced by
  repo-relative path — no duplicated levels.

## Coverage

| Case | Subject | Expected state |
| --- | --- | --- |
| `V28.solver.solvable` | solver over `grid-puzzle-scene-v1` | `solvable` with a witness that replays to win |
| `V28.solver.unsolvable` | solver over `grid-puzzle-unsolvable-v1` | `unsolvable` after full exploration |
| `V28.solver.exhausted` | solver over `grid-puzzle-scene-v1`, `maxStates: 2` | `exhausted` (explicit, not a false negative) |
| `V28.detector.detected` | detector over `grid-puzzle-oversolution-v1` | `over-solution` with a replayable counterexample trace |
| `V28.detector.none` | detector over `grid-puzzle-scene-v1` (unique solution) | `clean` — no false positive |
| `V28.detector.missing-intent` | detector with empty intent | fail-closed |
| `V28.difficulty.known` | difficulty over `grid-puzzle-scene-v1` evidence | `computed` (solution length 4, mechanic order `["push","move"]`) |
| `V28.difficulty.stale` | difficulty over stale evidence | fail-closed |
| `V28.gate.fail` | design-integrity gate over `grid-puzzle-oversolution-v1` | `fail` (an unintended over-solution exists) |
| `V28.gate.pass` | design-integrity gate over `grid-puzzle-scene-v1` | `pass` (intent satisfied, no over-solution) |
| `V28.compat.four-gate` | `evaluation_gate_categories` golden | unchanged `declared-gate-and` / `neutral` aggregation |

The design-integrity gate verdict is composed from the merged solver/detector
surfaces per the scope contract (#1579: *intent satisfied AND no over-solution*);
the formal evaluator `declared-gate-and` integration is tracked by #1583.

## Runner

Run:

```bash
cargo test --test scenario_coverage_v28_puzzle_solver
```

The runner reads `matrix.fixture.json`, dispatches each case to the trusted
Rust solver (#1580), detector (#1581), or difficulty metric (#1582), asserts the
enumerated verdict, and compares the four-gate aggregation to the committed
golden. A breaking change to any of these behaviors fails the suite.

## Boundaries

- Rust/local owns the trusted logic; browser/Studio surfaces are read-only.
- Asserts deterministic states and shapes only — no quality, fun,
  production-readiness, or Godot-replacement/parity claim, and no auto-fix of
  detected over-solutions.
- All artifacts are fixture-scoped; no generated runs are committed. Scenario
  Coverage numbering continues from v26 (Era E). #1 and #23 remain open.
