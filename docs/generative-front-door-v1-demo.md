# Generative Front Door v1 — Demo

This is a deterministic, fixture-scoped demo for Era F Milestone 30
(`docs/generative-front-door-v1.md`, #1596). It shows the generative front door
over the verification engine room end to end, with no network and no live
browser:

- the **intake** (#1593) turns a plain authoring brief into a grid-puzzle
  *proposal* — proposal-only, never a trusted write;
- the **engine room** (the promotion guard, #1594) verifies each proposal — the
  deterministic solver (#1580) and over-solution detector (#1581) produce the
  facts, the design-integrity gate (#1583) turns them into a verdict, and that
  verdict is ANDed into the existing four-gate `declared-gate-and` aggregation;
- a **blocked** proposal (an unintended over-solution exists) is withheld from
  promotion with a replayable, evidence-linked reason;
- a **promotable** proposal (intent satisfied, no over-solution) is routed
  **unchanged** (`proposed`/`pending`/`unverified`) into the existing
  review/apply/trust-gradient path.

Generation is the front door and the deterministic verification loop is the
engine room — layers, not alternatives. "Promotable" means only that the
proposal passed the engine room; it is **not** a claim that the generated game
is good, fun, shippable, or a Godot replacement, and it is **not** an apply,
auto-merge, self-approval, or reviewer bypass.

## Fixtures

Both briefs are fixture-scoped under
`examples/generative-front-door-v1/demo/` and reuse the existing
`ouroforge.generative-intake.v1` brief model and `ouroforge.grid-puzzle.v1`
state model — no new runtime or engine.

### Promotable brief — `brief-promotable.json` (engine room PASSES)

```
######
#@...#
#.*P.#
#....#
######
```

The intended solution `["left","down","left","up"]` is the unique shortest
solution.

- Intake: a valid grid-puzzle proposal (`status: proposed`, `verdict_status:
  pending`, `confidence: unverified`) with generation provenance linking it to
  the brief.
- Engine room: **PASS** (intent satisfied, no over-solution). The proposal is
  promotable and is routed unchanged into the existing
  review/apply/trust-gradient path.

### Blocked brief — `brief-blocked.json` (engine room BLOCKS)

```
#####
#@*P#
#...#
#####
```

The crate (`*`) sits one push from the target (`@`), so the level can be won in
a single move (`["left"]`), but the brief's intended solution declares a longer
3-step path (`["down","up","left"]`), so a strictly shorter solution bypasses
the intent.

- Intake: a valid grid-puzzle proposal (proposal-only, just like the promotable
  brief — intake never runs the solver or promotes).
- Engine room: **BLOCKED** (an unintended over-solution exists). The proposal is
  withheld from the apply path with a replayable counterexample reference; it is
  never promoted.

## Reproduce

From a fresh clone, the demo is a Rust smoke test — deterministic, no network,
no browser:

```bash
cargo test --test generative_front_door_demo
```

The test asserts brief → proposal for both briefs, the blocked brief's
not-promotable verdict with evidence, the promotable brief's routed-unchanged
proposal, and that the verdicts are deterministic across runs.

## Boundaries

- Generation is proposal-only; promotion stays gated by the engine room and
  flows through the existing review/apply/trust-gradient path (`source_apply_*`,
  `trust_gradient_*`). No trusted write, auto-apply, auto-merge, self-approval,
  or reviewer bypass.
- Rust/local owns the trusted logic; browser/Studio surfaces are read-only.
- Descriptive behavior and gate states only — no difficulty, quality, fun,
  production-readiness, or Godot-replacement/parity claim.
- All artifacts are fixture-scoped; no generated runs are committed. #1 and #23
  remain open.
