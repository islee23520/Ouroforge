# Grid-Puzzle Game Class v1 Demo

Issue: #1576

This is a deterministic, fixture-scoped demo for the Grid-Puzzle Game Class v1
scope and design gate (#1573). It records one rung climbed on the Milestone 24
complexity ladder for the grid-puzzle game class, backed by four-gate evidence
over the sokoban-micro fixture — with no network and no live browser.

## Claim

- Rung: `grid-puzzle-game-class-v1.grid-puzzle-sokoban-micro-v1`
- Rung gate status: `satisfied`
- Demo fixture: `examples/grid-puzzle-game-class-v1/demo/run-manifest.json`
- Canonical scene: `examples/game-runtime/grid-puzzle-scene-v1.json`

The claim is intentionally narrow: one grid-puzzle rung is satisfied. This does
not claim later ladder rungs, full engine parity, replacement status for Godot,
production readiness, native export, hosted operation, executable plugins, or
browser-owned trusted writes.

## Evidence Package

The demo fixture links to source-controlled evidence for the required gates:

| Gate | Status | Evidence |
| --- | --- | --- |
| Mechanical | `pass` | Declared intended solution reaches the win state; blocked push does not falsely win. |
| Runtime | `pass` | Deterministic grid-puzzle state machine evaluates the declared solution to `won`. |
| Visual | `pass` | Scene includes wall, floor, player, crate, and target entities with valid legend mapping. |
| Semantic | `pass` | Grid-puzzle game-class contract verified: one player, one target, win condition `all-targets-covered`, intended solution replayable. |

Loop coverage is recorded as `satisfied` / `pass` for the grid-puzzle local
authoring loop. The fixture cites the existing contract tests and docs that
cover deterministic validation, intended-solution replay, blocked-push
non-winning, and malformed/missing-win fail-closed behavior.

## Ladder Rung

The rung record at `examples/grid-puzzle-game-class-v1/demo/rung-record.json`
links the run manifest to the verdict and documents the required capabilities:
deterministic grid state, pushable mechanics, and win-condition evaluation. Each
capability is backed by the existing Rust contract test in
`crates/ouroforge-core/tests/grid_puzzle_game_class_contract.rs`.

## Boundaries

- Rust/local owns the trusted validation logic; browser/Studio surfaces are
  read-only.
- Descriptive behavior and gate states only — no difficulty, quality, fun,
  production-readiness, or Godot-replacement/parity claim.
- All artifacts are fixture-scoped; no generated runs are committed.
- #1 and #23 remain open governance anchors.

## Verification

Run:

```bash
cargo test --test grid_puzzle_game_class_demo_contract
```

The smoke test loads the run manifest, verdict, and rung record, verifies all
four gates are passing, checks that artifact refs point to existing fixtures,
and asserts determinism.
