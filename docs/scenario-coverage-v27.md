# Scenario Coverage v27 — Grid-Puzzle Game Class Regression Suite

Status: **fixture-scoped regression coverage**

Issue: #1577 — Scenario Coverage v27: Grid-Puzzle Game Class Regression Suite
Anchor: #1 Era F Milestone 27 (Grid-Puzzle Game Class v1)

Locks grid-puzzle determinism, PuzzleScript DSL ingest (valid/malformed/unsupported),
win/lose detection, and backward compatibility for existing platformer/collect-and-exit
classes. Asserts states and shapes only.

## Suite

- Matrix: `examples/grid-puzzle-game-class-v1/scenario-coverage-v27/matrix.fixture.json`
- Runner: `examples/grid-puzzle-game-class-v1/scenario-coverage-v27-grid-puzzle.test.cjs`
- Rust mirror: `crates/ouroforge-core/tests/scenario_coverage_v27_grid_puzzle.rs`

## Boundary

fixture-scoped, deterministic, browser/Studio read-only, no production-ready or
Godot-replacement/parity claim. #1 and #23 remain open.
