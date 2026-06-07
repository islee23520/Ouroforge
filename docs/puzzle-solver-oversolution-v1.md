# Puzzle Solver and Over-Solution Detection v1

Puzzle Solver and Over-Solution Detection v1 is a scope and contract document
for Era F Milestone 28 under #1. It defines how Ouroforge proves that an
authored grid-puzzle level has exactly its intended solution: a deterministic
solver contract, designer intent capture, an over-solution detector contract, a
difficulty-metric artifact, and a design-integrity gate that composes with the
existing evaluator gates. Solvability is table stakes; the load-bearing
deliverable is over-solution detection, which is structurally possible only
because the runtime is deterministic and fully observable.

This document adds no executable behavior, fixtures, runtime features, Studio
surfaces, browser authority, or engine capability. It is a governance contract
for the follow-up implementation issues #1580–#1586.

## Scope

The contract applies to authored grid-puzzle levels that run on the existing
deterministic runtime (`examples/game-runtime/runtime.js`) exposed through the
`window.__OUROFORGE__` probe, as established by the Grid-Puzzle Game Class and
Runtime v1 line (#1573/#1574). It defines the contracts the follow-up issues
must satisfy and the boundaries they must respect; it does not authorize any
implementation by itself.

This milestone only *verifies* authored levels. It does not generate or author
levels (that is Milestone 30) and does not auto-fix detected over-solutions
(detection, measurement, and gating only).

## Non-goals

Puzzle Solver and Over-Solution Detection v1 does not authorize:

- a new engine, runtime, writer, or evaluator parallel to existing surfaces;
- direct trusted writes from generation or any browser/Studio surface; all
  proposals flow only through the existing review/apply/trust-gradient path
  (`source_apply_*`, `trust_gradient_*`);
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden
  trusted writes;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- hosted/cloud/paid capability, marketplace transaction layer, or
  distributed/Elixir orchestration (Layer-3; DEFER per #1508, NO-GO per
  ADR #92, `docs/distributed-elixir-design.md`);
- engine breadth (renderer/3D/audio/physics depth) beyond what this rung's gate
  justifies;
- committing generated runs/genre/evidence/registry artifacts unless explicitly
  fixture-scoped;
- any claim of a production-ready engine, Godot replacement or parity, or that
  generated games are good, fun, or shippable;
- generation or authoring (Milestone 30);
- auto-fix of detected over-solutions;
- closing or modifying #1 or #23 without a separate explicit governance
  decision.

## Contracts

### Deterministic solver contract (#1580)

The solver performs a bounded search over the grid-puzzle world state exposed by
the existing runtime probe. It is a trusted Rust/local component that consumes
the probe-observable state and the deterministic fixed-step transition; it does
not embed or fork the runtime.

- **Input**: an authored level (grid state model and rule model) and a bounded
  search budget (maximum explored states and/or maximum solution depth).
- **Output**: a solvability verdict (`solvable` / `unsolvable` / `unknown` when
  the bound is exhausted before a verdict) plus a *witness* — a replayable
  action sequence that reaches the win condition, recorded so the runtime can
  replay it deterministically and the FNV1a64 replay digest matches.
- **Determinism**: identical inputs and budget yield an identical verdict and
  witness; tie-breaking in the search frontier is total and deterministic.
- **Fail-closed**: malformed levels, missing win conditions, or exceeded bounds
  produce an explicit diagnostic and never a silent or optimistic pass.

### Designer intent capture (#1581)

Designer intent capture is an authored, validated artifact that records what the
level is supposed to require:

- the **intended solution path** (a canonical witness the designer asserts), and
  / or
- the **mechanic the level teaches** (the rule or interaction the intended path
  must exercise).

Intent is supplied by the author and validated by Rust/local; it is never
inferred as a trusted fact by a browser or Studio surface. Browser/Studio may
display captured intent read-only.

### Over-solution detector contract (#1581)

The detector exhaustively searches (within the declared bound) for solutions
that reach the win condition while bypassing the captured intent — for example a
shorter solution, or one that does not exercise the taught mechanic.

- **Input**: the authored level, the captured designer intent, and a bounded
  search budget.
- **Output**: each unintended solution as a *replayable counterexample trace*
  (an action sequence the runtime can replay deterministically, digest-stable),
  classified by how it bypasses intent (e.g. shorter-than-intended, or
  intent-mechanic-not-exercised).
- **Empty result is meaningful**: zero counterexamples within the declared bound
  is the passing shape; an exhausted bound is reported explicitly as
  `bound-exhausted`, never silently treated as "no over-solutions".

### Difficulty-metric artifact (#1582)

A read-only artifact derived from the solver search, recording at least:

- **solution length** (steps in the intended/shortest witness),
- **branching factor** (average legal moves per reachable state),
- **dead-end density** (fraction of reachable states with no progress toward the
  win condition), and
- **mechanic-introduction order** (the order in which taught mechanics first
  become required along the intended path).

The artifact is descriptive measurement only. It carries no quality, fun, or
production judgement and gates nothing by itself.

### Design-integrity gate (#1583)

The design-integrity gate passes only when **intent is satisfied AND no
unintended over-solutions exist** within the declared bound. It is a declared
gate evaluated through the existing evaluator aggregation
(`declared-gate-and`, `undeclaredGatePolicy: neutral`) — it is one more declared
gate alongside the existing mechanical, runtime, visual, and semantic gates, not
a parallel evaluator. When the level declares no design-integrity evidence the
gate stays neutral, exactly as the existing four gates do for undeclared
evidence.

The gate is implemented (#1583) as
`ouroforge_evaluator::design_integrity_gate`: a declared-evidence gate that
consumes the over-solution detector's result (intent-satisfaction,
over-solution count, and whether the bounded search was exhausted) and emits a
verdict ANDed into the existing aggregation via
`compose_design_integrity_into_categories`. Because `ouroforge-core` depends on
the evaluator crate, the detector cannot live in the evaluator; the gate instead
consumes the detector's declared result evidence, mirroring how the visual gate
consumes a precomputed `compare` artifact. The gate **fails closed**: malformed
evidence, an unsatisfiable captured intent, any over-solution, or a bounded
search that was exhausted before the shorter-solution space was fully explored
(`Inconclusive`) all block — an exhausted search is never coerced into a pass.
The gate performs no trusted write, auto-apply, or auto-fix of detected
over-solutions; detection, measurement, and gating only.

## Evidence and provenance

All solver verdicts, detector counterexamples, difficulty artifacts, and gate
outcomes are written by Rust/local as evidence and bound into the existing
provenance model (`provenance_bundle.rs`). Counterexample traces reuse the
existing replay/digest contract so any reviewer can replay them. No parallel
evidence store is introduced. Where these contracts are exercised inside an
evolution loop, they reuse the existing `evolve_campaign.rs` surface rather than
a new loop.

## Language boundary

- **Rust/local** owns the solver, intent validation, detector, difficulty
  metric, the design-integrity gate, evidence and provenance writing, and the
  review/apply/trust-gradient path.
- **TypeScript/JavaScript** owns the deterministic runtime, the
  `window.__OUROFORGE__` probe, and browser-local read-only inspection and
  static dashboard/cockpit display of captured intent and gate outcomes.
- **Python** may be used only for temporary local tooling or smoke helpers and
  must not own any Era F contract.
- No new language or runtime is introduced; distributed/Elixir remains NO-GO per
  ADR #92.

## Dependency order and closure gates

Follow-up issues are implemented and merged in order, each as its own PR with
green CI:

```text
#1579 scope -> #1580 -> #1581 -> #1582 -> #1583 -> #1584 -> #1585 -> #1586
```

1. **#1579 — Scope and Contract** (this document).
2. **#1580 — Deterministic Grid-Puzzle Solver v1** (depends on the grid-puzzle
   runtime #1574 being merged).
3. **#1581 — Designer Intent Capture and Over-Solution Detector v1**.
4. **#1582 — Difficulty Metric Artifact v1**.
5. **#1583 — Design-Integrity Gate v1** (composes via `declared-gate-and`).
6. **#1584 — Solver and Over-Solution Detection Demo v1**.
7. **#1585 — Scenario Coverage v28: Solver and Over-Solution Regression Suite**
   (continues Scenario Coverage numbering from v26 onward).
8. **#1586 — Roadmap and #1 Governance Refresh after Solver and Over-Solution
   v1**.

Each follow-up issue's Definition of Done must reconfirm that #1 and #23 remain
open. Generated runs and artifacts stay untracked unless explicitly
fixture-scoped. Public wording stays conservative: no auto-merge, quality, fun,
production-ready, or Godot-replacement claim.
