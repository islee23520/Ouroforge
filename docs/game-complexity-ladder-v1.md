# Game Complexity Ladder v1

Game Complexity Ladder v1 is a scope and contract document for growing
Ouroforge game capability only through loop-produced evidence. It defines a
conservative sequence of game classes, the evidence required before claiming
each class, and the rule that engine growth must be justified by a specific
rung gate.

This document adds no executable behavior, fixtures, runtime features, Studio
surfaces, browser authority, or engine capability. It is a governance contract
for future implementation issues.

## Scope

The ladder is a capability gate sequence for small game classes that may be
claimed only after a loop-produced demo proves the class with evidence. Each
rung describes the minimum observable game shape and the evidence required to
claim it. A rung does not authorize implementation by itself; follow-up issues
must still scope concrete Rust/local changes and verification.

The contract applies to game-class claims, roadmap wording, and future engine
growth requests involving renderer, physics, audio, animation, runtime state,
scenario coverage, dashboard summaries, or Studio read-only inspection.

## Non-goals

Game Complexity Ladder v1 does not authorize:

- new renderer, physics, audio, animation, runtime, editor, or Studio behavior;
- browser trusted writes or a browser command bridge;
- hosted/cloud/server/database/auth infrastructure;
- source-code mutation or source patch application;
- native export implementation;
- plugin runtime, marketplace, dynamic loading, or extension API;
- production editor, production engine, public launch, or broad compatibility
  claims;
- closing or modifying #1 or #23 without a separate explicit governance
  decision.

## Ladder rungs

The ladder is ordered from the smallest game class to broader game structure.
Each rung is a gate, not a promise that the capability already exists.

1. **Collect-and-exit** — a single bounded play space with player movement,
   collectible or objective state, exit/win condition, deterministic scenario
   observation, and evidence that the objective can be completed.
2. **One-screen platformer** — a single bounded play space with platformer-style
   movement, gravity or jump-relevant collision behavior, hazards or obstacles,
   objective completion, and evidence that failure and success states are
   observable.
3. **Top-down objective game** — a single bounded top-down play space with
   navigation, collision or blocking geometry, pickups or interactions,
   objective state, and evidence that scenario assertions can distinguish
   progress, blocked paths, and completion.
4. **Multi-scene objective game** — multiple manifest-declared scenes or areas
   with bounded transitions, persistent or transferred objective state, a
   complete objective path, and evidence that transitions and state continuity
   are observable.

Future work may refine a rung or add an intermediate rung only through a
separate scoped governance decision. Refinement must preserve the rule that a
later rung cannot be claimed before the prior rung has passing evidence.

## Rung rule

Each rung requires a loop-produced, evidence-backed demo before the next rung is
claimed.

The minimum claim package for a rung is:

- a loop-produced demo created through the local authoring loop rather than an
  ad hoc manual assertion;
- four-gate evidence showing the demo passed the issue's required validation,
  runtime/scenario behavior, review/governance, and generated-state/trust-boundary
  checks;
- a loop-coverage verdict from the Milestone 20 loop coverage surface showing
  that the relevant loop path is covered and not merely hand-inspected;
- reproducible local commands and artifact paths sufficient for a reviewer to
  inspect the evidence;
- explicit missing, partial, or malformed evidence reporting where a gate cannot
  be satisfied.

If any required evidence is missing, stale, malformed, or generated outside the
allowed loop path, the rung remains unclaimed. Passing one rung does not imply
the next rung, and roadmap wording must not describe future rungs as complete or
pre-authorized.

## Evidence gates

Follow-up implementation issues must define focused verification for the rung
they affect. At minimum, a rung claim must include:

- issue and dependency state checks for the active follow-up issue;
- Rust/local validation for any trusted schema, runtime, scenario, or loop
  artifact touched by that issue;
- Node static checks/tests only for browser read-only surfaces touched by that
  issue;
- generated-state audit proving run outputs, dashboard exports, local tool
  state, and build outputs remain untracked unless explicitly fixture-scoped;
- guardrail audit for no browser trusted writes, no command bridge, no hidden
  execution, no auto-apply, no auto-promote, no auto-merge, and no broad engine
  claim;
- latest-main verification before issue closure.

Evidence must cite concrete artifacts. The absence of a failure is not evidence
of a pass, and a hand-authored screenshot or prose summary is not sufficient to
claim a rung.

## Engine-growth demand rule

Any new engine capability request must cite the specific rung gate it is needed
to satisfy. The citation must explain why the current implemented surface cannot
produce the required loop-backed evidence for that rung without the requested
capability.

Allowed demand shape:

```text
Requested capability: <bounded renderer/physics/audio/animation/runtime change>
Rung gate: <specific ladder rung>
Evidence gap: <which required demo assertion cannot pass today>
Minimal implementation: <smallest Rust/local change needed for that gap>
Non-goals: <engine breadth still out of scope>
```

Not allowed:

- adding broad renderer, physics, audio, animation, or runtime breadth because it
  may be useful later;
- implementing features for multiple future rungs without a current rung gate;
- describing the roadmap as pre-authorization for engine capability expansion;
- converting browser read-only surfaces into trusted writers or command
  launchers;
- treating a demo idea, mockup, or manual playthrough as sufficient engine-growth
  justification.

## Dependency and closure order

Milestone 24 follow-up work should remain in this order unless a later issue
documents a concrete blocker and replacement ordering:

1. #1493 — Game Complexity Ladder v1 Scope and Contract.
2. #1494 — first follow-up implementation issue after this contract is merged.
3. #1495 — next follow-up implementation issue after #1494 satisfies its closure
   gates.
4. #1496 — next follow-up implementation issue after #1495 satisfies its closure
   gates.
5. #1497 — final authorized follow-up in this contract sequence after #1496
   satisfies its closure gates.
6. Stop before #1498. This contract does not authorize implementing, modifying,
   closing, or claiming #1498.

Closure gates for each follow-up issue:

- the prior issue in the dependency order is merged or explicitly documented as
  superseded by a maintainer-approved governance decision;
- the issue scopes one rung or one bounded prerequisite for a rung;
- any engine growth cites the specific rung gate and evidence gap;
- four-gate evidence and loop-coverage verdict are present for any rung claim;
- generated-state and trust-boundary audits pass;
- #1 and #23 are checked and remain open anchors unless a separate explicit
  governance decision exists.

## Governance audit for #1 and #23

#1 remains the broad vision and roadmap anchor. #23 remains the repo-memory and
design context anchor. This contract preserves both as open anchors and does not
modify, close, replace, or narrow either issue.

Future work that proposes to change either anchor must be a separate explicit
governance decision. That decision must identify the replacement source of
truth, record maintainer approval, and avoid implying that this ladder contract
itself authorized the change.

