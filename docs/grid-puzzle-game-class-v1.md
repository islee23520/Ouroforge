# Grid-Puzzle Game Class v1 Scope and Design Gate

Status: **Design gate — scope and contracts only; no executable behavior**

Issue: #1573 — Grid-Puzzle Game Class v1 Scope and Design Gate
Anchor: #1 Era F Milestone 27 (Grid-Puzzle Game Class)

This document is the canonical Grid-Puzzle Game Class v1 design artifact. It is
a scope/design-gate document, not an implementation milestone. It selects the
Era F beachhead genre on evidence, defines the deterministic grid-puzzle
game-class contract, defines the PuzzleScript-compatible DSL ingest contract,
records how the rung sits under the Milestone 24 complexity ladder, and orders
the follow-up implementation issues. It adds no executable behavior, fixtures,
runtime features, Studio surfaces, browser authority, or engine capability.

This gate follows the project idiom for scope/design-gate work: the ADR #92
Distributed/Elixir gate, the Native Export Design Gate (#168), the Game
Complexity Ladder v1 gate (#1493, `docs/game-complexity-ladder-v1.md`), and the
Trust Gradient v1 gate (#1476, `docs/trust-gradient-design.md`). No grid-puzzle
behavior is implemented in #1573; it only decides and bounds. Implementation is
authorized only through the gated follow-up issues listed below.

## Scope

This gate covers three contracts and two governance records:

1. the genre-selection decision for the Era F beachhead and its
   machine-checkable acceptance shape;
2. the deterministic, probe-exposed grid-puzzle game-class contract;
3. the PuzzleScript-compatible DSL ingest contract (validate-then-load);
4. how the grid-puzzle rung records under the Milestone 24 complexity ladder;
5. the dependency order and closure gates for the follow-up issues, preserving
   #1 and #23 as open anchors.

The contract applies to grid-puzzle game-class claims, the DSL ingest path,
roadmap wording, scenario-coverage numbering, and any future engine growth
requested to satisfy this rung. A documented contract here does not authorize
implementation by itself; each follow-up issue must still scope concrete
Rust/local changes and verification.

## Non-goals

Grid-Puzzle Game Class v1 does not authorize:

- a new engine, runtime, writer, solver service, or any parallel system that
  duplicates the existing runtime, probe, evaluator, evolve/campaign, compare,
  provenance, dashboard, cockpit, source-apply, or CLI surfaces;
- direct trusted writes from generation or from any browser/Studio surface;
  proposals flow only through the existing review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden
  trusted writes;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- hosted/cloud/paid capability, a marketplace transaction layer, or
  distributed/Elixir orchestration (Layer-3; DEFER per #1508, NO-GO per ADR #92,
  `docs/distributed-elixir-design.md`);
- renderer, 3D, physics, audio, or animation breadth beyond what the grid-puzzle
  rung's gate justifies;
- committing generated runs/genre/evidence/registry artifacts unless explicitly
  fixture-scoped;
- natural-language generation (that is Milestone 30); this milestone only adds
  and ingests the game class;
- closing, modifying, replacing, or narrowing #1 or #23 without a separate
  explicit governance decision.

## Genre selection: grid puzzle (PuzzleScript-compatible)

The Era F Milestone 27 beachhead genre is the **grid puzzle**, expressed in a
PuzzleScript-compatible form. The selection is made on the following evidence,
not on preference:

1. **Machine-checkable acceptance is intrinsic, not asserted.** A grid puzzle
   has a discrete, finite state space and a deterministic transition function.
   Whether a level is *solvable*, whether it is solvable by the author's
   *intended solution*, and a conservative *difficulty* signal (shortest
   solution length / branching) can all be computed by a deterministic search
   over the discrete state space. Acceptance is a property the loop can verify,
   not a claim a human must trust.

2. **It reuses the existing deterministic loop with the least engine growth.**
   The genre needs a fixed-step discrete update, a small bounded world state,
   and a win/lose predicate — all of which the existing runtime, probe, and
   evaluator already model for the complexity-ladder rungs. No renderer, 3D,
   physics, audio, or animation depth is required to make the genre's acceptance
   checkable. This satisfies the Milestone 24 demand rule: engine capability
   grows only to satisfy this rung's gate.

3. **PuzzleScript compatibility gives a real, small, well-specified DSL.** A
   PuzzleScript-compatible subset is a published, deterministic, grid-and-rules
   format with a finite object/rule/legend/win-condition model. Compatibility
   gives the ingest path a real specification to validate against and a real
   corpus shape to target, while keeping the surface bounded and local.

The rejected alternatives are: (a) a real-time/action genre — rejected because
acceptance would depend on timing, physics, or feel that the loop cannot
machine-check without large engine growth and subjective judgement; and (b) a
bespoke ad-hoc puzzle format — rejected because it would invent an unspecified
DSL with no external corpus or specification to validate against.

### Machine-checkable acceptance shape

A grid-puzzle level's acceptance shape is the triple
`(solvability, intended-solution, difficulty)`:

- **solvability** — there exists at least one input sequence from the initial
  grid state that reaches a win state under the level's rules, found by a
  deterministic bounded search over the discrete state space; or a proof of
  unsolvability within the bound. Reported as a verdict with the witnessing
  sequence (or the searched bound), never asserted.
- **intended-solution** — the author-declared solution sequence, when replayed
  deterministically from the initial state, reaches a win state and does not hit
  a lose/blocked state first. A mismatch is reported as a failing verdict with
  the divergent step, not silently accepted.
- **difficulty** — a conservative, deterministic signal derived from the search
  (for example shortest-solution length and a branching/effort measure),
  reported as evidence rather than as a quality or fun claim.

This shape is the genre's gate. It is computed by Rust/local trusted logic over
the deterministic runtime's exposed world state and surfaced through the
existing evaluator four-gate aggregation (`declared-gate-and`) and `compare`
operators; it is never a hand-authored assertion or a prose summary.

## Grid-puzzle game-class contract

The grid-puzzle game class is a deterministic, probe-exposed game shape that
reuses the existing runtime and `window.__OUROFORGE__` probe. It introduces no
parallel runtime. The contract is:

- **Grid state model.** The world state is a bounded 2-D grid of cells. Each
  cell holds a finite, declared set of object layers (for example background,
  movable, player, target). The grid dimensions, the object vocabulary, and the
  legend are declared up front and fixed for a level; the runtime holds no
  hidden or unbounded state.

- **Rule model.** Transitions are expressed as deterministic
  pattern→replacement rules over local grid neighbourhoods (the
  PuzzleScript-compatible rule shape). Rules are pure functions of the current
  grid plus the current input; given the same grid and input, the same rules
  fire in the same declared order and produce the same next grid. No rule may
  reach outside the declared grid/object vocabulary.

- **Win/lose condition.** Win and lose are deterministic predicates over the
  grid state (for example "every target cell is covered" for win). The predicate
  result is a pure function of the grid; it is evaluated each step and exposed,
  never inferred from rendering.

- **Fixed-step update.** The game advances by discrete steps driven by the
  existing runtime's fixed-step update (`step`). One input maps to one
  deterministic grid transition; there is no real-time, frame-rate-dependent, or
  wall-clock-dependent behavior. Replaying the same input sequence from the same
  initial grid reproduces the same trajectory exactly.

- **Full world-state exposure.** The complete grid state (cell contents, object
  layers, tick, win/lose predicate results, and the applied-rule trace
  sufficient to reconstruct each transition) is exposed through the existing
  read-only `window.__OUROFORGE__` world-state probe. Exposure is observation
  only: the probe carries the existing `read-only` browser/Studio mode and its
  `disallowedActions` (no trusted writes, no command bridge, no live mutation).
  Trusted verification reads this exposed state; the browser never writes back.

Ownership follows the language boundary: Rust/local owns the trusted solver,
intended-solution replay, difficulty computation, gate aggregation, evidence and
provenance writing, and any persistence; the deterministic runtime and the
probe are JavaScript and read-only in the browser.

## PuzzleScript-compatible DSL ingest contract

Levels and rules enter the loop as a PuzzleScript-compatible DSL document
through a **validate-then-load** path:

1. **Validate (Rust/local, trusted).** A DSL document is first parsed and
   validated by Rust/local trusted logic against the compatible-subset grammar:
   the object/legend vocabulary resolves, rules reference only declared objects,
   the grid/levels are well-formed and within declared bounds, and the
   win-condition is expressible over the declared vocabulary. Validation **fails
   closed**: a missing, malformed, ambiguous, or out-of-subset document is
   rejected with a structured reason and does not load. Validation reports
   missing/partial/malformed inputs explicitly rather than guessing.

2. **Load (deterministic runtime, read-only browser).** Only a document that
   passed validation is loaded into the deterministic runtime for stepping and
   probe exposure. The browser runs the validated level read-only; it neither
   validates trusted inputs nor performs trusted writes. The browser is a
   runtime/probe surface, never the source of trust.

Ingest is additive and proposal-only. A DSL document produced by generation is a
proposal that flows through the existing review/apply/trust-gradient path; it is
never a direct trusted write and never auto-applied. The ingest path introduces
no new dependency, language, or runtime: validation is Rust/local and loading
reuses the existing JavaScript runtime and probe.

## Milestone 24 complexity-ladder linkage

The grid puzzle records as a rung claim under the Game Complexity Ladder v1
contract (`docs/game-complexity-ladder-v1.md`). The ladder's rule governs this
genre: a rung is claimed only after a **loop-produced**, evidence-backed demo
proves the class, with four-gate evidence and a loop-coverage verdict; a
hand-authored screenshot or prose summary is not sufficient.

Concretely, the grid-puzzle rung's claim package (delivered by the demo issue,
not by this gate) must include:

- a loop-produced grid-puzzle demo created through the local authoring loop;
- four-gate evidence (required validation; runtime/scenario behavior;
  review/governance; generated-state/trust-boundary) showing the demo passed;
- a loop-coverage verdict from the Milestone 20 loop-coverage surface showing
  the relevant loop path is covered, not merely hand-inspected;
- reproducible local commands and artifact paths for a reviewer;
- explicit missing/partial/malformed evidence reporting where a gate cannot be
  satisfied.

Engine-growth demand rule (inherited from the ladder): any renderer, runtime, or
other engine capability requested while building this rung must cite the
specific grid-puzzle gate it is needed to satisfy and explain why the current
implemented surface cannot produce the required loop-backed evidence without it.
This rung does not authorize 3D, physics, audio, or renderer breadth.

## Dependency and closure order

Milestone 27 follow-up work should remain in this order unless a later issue
documents a concrete blocker and replacement ordering:

1. #1573 — Grid-Puzzle Game Class v1 Scope and Design Gate (this issue).
2. #1574 — Grid-Puzzle Game Class and Runtime v1, after this contract is merged.
3. #1575 — PuzzleScript-Compatible DSL Ingest v1, after #1574 satisfies its
   closure gates.
4. #1576 — Grid-Puzzle Game Class Demo v1, after #1575 satisfies its closure
   gates.
5. #1577 — Scenario Coverage v27: Grid-Puzzle Game Class Regression Suite, after
   #1576 satisfies its closure gates.
6. #1578 — Roadmap and #1 Governance Refresh after Grid-Puzzle Game Class v1,
   after #1577 satisfies its closure gates. This ordering records the governed
   follow-up chain only; it does not authorize implementing, modifying, closing,
   or claiming #1578 from this contract alone.

```text
#1573 scope -> #1574 -> #1575 -> #1576 -> #1577 -> #1578
```

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly documented as
  superseded by a maintainer-approved governance decision;
- the issue scopes one bounded prerequisite for the rung (model/runtime, ingest,
  demo, coverage, or governance) and does not combine independently verifiable
  behavior into one PR;
- any engine growth cites the specific rung gate and evidence gap;
- four-gate evidence and a loop-coverage verdict are present for any rung claim;
- generated-state and trust-boundary audits pass (run outputs, dashboard
  exports, local tool state, and build outputs remain untracked unless
  explicitly fixture-scoped);
- public wording stays conservative (no auto-merge/quality/fun/production/Godot
  -replacement claim);
- Scenario Coverage numbering continues from v26 (Era E) onward; the regression
  suite for this rung is v27;
- #1 and #23 are checked and remain open anchors unless a separate explicit
  governance decision exists.

## Governance audit for #1 and #23

#1 remains the broad vision and roadmap anchor for Era F. #23 remains the
repo-memory and design-context anchor. This contract preserves both as open
anchors and does not modify, close, replace, or narrow either issue. The
grid-puzzle genre is a layer on the existing loop — generation is the front door
and the deterministic verification loop is the engine room; they are layers, not
alternatives.

Future work that proposes to change either anchor must be a separate explicit
governance decision. That decision must identify the replacement source of
truth, record maintainer approval, and avoid implying that this design gate
itself authorized the change.
