# Puzzle Solver and Over-Solution Detection v1 — Demo

This is a deterministic, fixture-scoped demo for Era F Milestone 28
(`docs/puzzle-solver-oversolution-v1.md`). It shows the moat end to end over two
authored grid-puzzle levels, with no network and no live browser:

- the **solver** (#1580) proves each level solvable and returns a replayable
  witness;
- the **over-solution detector** (#1581) finds a strictly shorter unintended
  solution on the dirty level and surfaces it as a replayable counterexample
  trace, and finds none on the clean level;
- the **design-integrity gate** verdict — defined by the scope contract (#1579)
  as *intent satisfied AND no unintended over-solution* — **fails** the dirty
  level and **passes** the clean level.

Solvability is table stakes; the load-bearing result is that an unintended
over-solution is caught with a concrete, replayable bypass rather than a "trust
me".

## Fixtures

Both levels are fixture-scoped under
`examples/puzzle-solver-oversolution-v1/demo/` and reuse the existing
`ouroforge.grid-puzzle.v1` state model — no new runtime or engine.

### Dirty level — `dirty-level.json` (gate FAILS)

```
#####
#@*P#
#...#
#####
```

The crate (`*`) sits one push from the target (`@`), so the level can be won in
a single move (`["left"]`). The designer's captured intent declares a longer
3-step path (`["down","up","left"]`), so a strictly shorter solution bypasses
the intent.

- Solver: solvable; shortest witness `["left"]`.
- Detector: one counterexample, `shorter-than-intended`, trace `["left"]`
  (length 1 < intended length 3). Replay `["left"]` on the runtime to watch the
  bypass.
- Design-integrity gate: **FAIL** (an unintended over-solution exists).

### Clean level — `clean-level.json` (gate PASSES)

```
######
#@...#
#.*P.#
#....#
######
```

The captured intent `["left","down","left","up"]` is the unique shortest
solution.

- Solver: solvable; shortest witness equals the intended 4-step path.
- Detector: no counterexample.
- Design-integrity gate: **PASS** (intent satisfied, no over-solution).

## Reproduce

From a fresh clone, the demo is a Rust smoke test — deterministic, no network,
no browser:

```bash
cargo test --test puzzle_solver_oversolution_demo
```

The test asserts solvability for both levels, the dirty level's replayable
over-solution trace, and the gate verdicts (FAIL for dirty, PASS for clean).

## Boundaries

- The gate verdict shown here is composed from the merged solver/detector
  surfaces to demonstrate the gate's semantics end to end; the formal evaluator
  `declared-gate-and` integration is tracked separately by #1583.
- Rust/local owns the trusted logic; browser/Studio surfaces are read-only.
- Descriptive behavior and gate states only — no difficulty, quality, fun,
  production-readiness, or Godot-replacement/parity claim. No auto-fix of
  detected over-solutions.
- All artifacts are fixture-scoped; no generated runs are committed. #1 and #23
  remain open.
