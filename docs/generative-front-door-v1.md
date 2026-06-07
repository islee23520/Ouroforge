# Generative Front Door v1

Generative Front Door v1 is a scope/design-gate milestone, not an implementation
milestone. It defines the decision, trust boundaries, promotion rules, and
follow-up sequence under #1 Era F Milestone 30 that let an author describe intent
(a brief or natural-language request) and receive a **verified game proposal**,
without weakening the safety model.

Generation is the **front door**. The deterministic verification loop — four
gates, solver, and over-solution detection — is the **engine room** that makes
output non-slop. They are layers, not alternatives: the front door lets a
non-developer express intent; the engine room is what any proposal must pass
before it can be promoted. Generation never replaces verification and never
bypasses it.

This gate exists because a generation entry point is tempting to wire as a
shortcut — a path that writes trusted artifacts directly, auto-applies its own
output, or claims its games are good. Ouroforge keeps trusted persistence in
Rust and keeps browser/Studio surfaces read-only. A generative front door must
stay inside those boundaries: it emits proposals only, routed through the
existing review/apply/trust-gradient path, and no proposal is promoted unless it
passes the engine room.

## Gate outcome: GO (bounded)

The decision for Generative Front Door v1 is **GO**, bounded strictly to the
scope below. The default for any capability not enumerated here remains
**DEFER**.

GO is justified only because the front door reuses surfaces that already exist
and are already governed:

- the review/apply/trust-gradient path (Milestone 15 / Milestone 22) already
  carries proposals through human review before any trusted write;
- the four-gate evaluator aggregation (`declared-gate-and`), solver, and
  over-solution detection (Milestone 28) already define the engine-room
  promotion guard;
- the deterministic runtime (`examples/game-runtime/runtime.js`) and the
  `window.__OUROFORGE__` probe already provide read-only browser inspection; and
- Rust already owns trusted validation, persistence, evidence, and provenance.

GO authorizes **only** the design contracts and the follow-up issue sequence in
this document. It does not authorize a new engine, a new runtime, a new writer,
a new language, or any executable generation behavior. Each follow-up issue is
separately scoped and must reuse the named surfaces above rather than build a
parallel system.

### GO/DEFER criteria

A capability is **GO** under Generative Front Door v1 only if all of the
following hold; otherwise it is **DEFER** by default:

1. **Proposal-only.** It emits a proposal routed through the existing
   review/apply/trust-gradient path. It never performs a direct trusted write.
2. **Engine-room gated.** No proposal it produces can be promoted unless it
   passes the engine room (four gates + solver + over-solution).
3. **Surface reuse.** It extends an existing surface (runtime/probe, evaluator,
   review/apply, source-apply, provenance, dashboard/cockpit, CLI) rather than
   adding a parallel engine, runtime, or writer.
4. **Boundary preserving.** Rust/local owns the new trusted logic; browser/Studio
   stays read-only; the change is additive and backward-compatible.
5. **Conservative wording.** It makes no auto-merge, quality, fun,
   production-ready, shippable, or Godot-replacement claim.
6. **Governance preserving.** It does not close, narrow, or modify #1 or #23.

Anything that fails any criterion is DEFER and requires a separate explicit
governance decision before it may be scoped.

## Non-slop is a process guarantee

"Verified" and "non-slop" in this milestone are **process guarantees, not
quality or fun claims**. A proposal is *verified* exactly when it has passed the
engine room: the four-gate evaluator aggregation, the solver (the proposal is
demonstrably solvable), and over-solution detection (the proposal is not
trivially or degenerately solvable). "Non-slop" means "passed the engine room."

It does **not** mean the generated game is good, fun, shippable, balanced, or
production-ready. No public wording in this milestone may imply otherwise.

## Proposals-only contract

Generation emits **proposals only**. The contract is:

- A generated artifact is a proposal, identical in trust to any other proposal
  entering the review/apply/trust-gradient path (Milestone 15 / Milestone 22).
- Generation never performs a trusted write. It cannot apply, auto-apply,
  auto-merge, self-approve, or bypass review. The same review-gated apply path
  and trust-gradient that govern hand-authored proposals govern generated ones.
- The browser/Studio read-only boundary is preserved. No browser command bridge,
  no browser-side trusted write, no Studio-initiated apply. Studio may display a
  generated proposal, its provenance, and its engine-room status; it may not
  promote it.
- Generation provenance (that the proposal was machine-generated, from what
  intent, through what path) is attached to the proposal so reviewers can see it
  originated from the front door.

## Engine-room promotion guard

A generated proposal **cannot be promoted** unless it passes the engine room
(Milestone 28):

- **Four gates.** The proposal must pass the evaluator's four-gate aggregation
  (`declared-gate-and`); a single failed gate fails closed.
- **Solver.** The proposal must be demonstrably solvable by the solver.
- **Over-solution.** The proposal must pass over-solution detection: it must not
  be trivially, degenerately, or unintentionally over-solvable.

The guard fails closed: a proposal that has not passed the engine room, or whose
engine-room evidence is missing or stale, is not promotable. Promotion is the
existing review/apply/trust-gradient action; the guard adds a precondition, it
does not add a new write path. This guard is the boundary between the front door
and a direct write: generation can produce a candidate, but only the engine room
plus human review can let it through.

## Accessibility contract

The non-developer accessibility path is the point of the front door:

- A non-developer can describe a puzzle in a brief or natural language and obtain
  a **verified-solvable proposal** — one that has passed the engine room — without
  writing code.
- The path is proposal-only end to end: the author's intent produces a proposal,
  the engine room verifies it, and a human reviews it through the existing
  apply/trust-gradient path. The author never performs and never triggers a
  trusted write.
- Generation provenance is attached so the author and reviewers can see what
  intent produced the proposal and that it came through the front door.
- Accessibility means *expressing intent without code*; it does not mean
  bypassing review, gates, or the read-only boundary. The non-developer gets a
  verified candidate, not an applied change.

## Language boundary

- **Rust** owns trusted validation, persistence, the solver/detector/gate/
  registry/telemetry logic, evidence and provenance writing, run/project binding,
  the review/apply/trust-gradient path, and CLI behavior.
- **TypeScript/JavaScript** owns the deterministic runtime, the
  `window.__OUROFORGE__` probe, browser-local read-only inspection, and static
  dashboard/cockpit behavior where explicitly scoped.
- **Python** may be used only for temporary local tooling or smoke helpers and
  must not own core Era F contracts.
- No new language or runtime is introduced without explicit issue-level
  authorization; distributed/Elixir remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`).

## Studio boundary

Studio's generative-front-door surface, when designed in a later issue, must
remain a **read-only evidence surface**. It may display generated proposals,
their generation provenance, engine-room status (gates/solver/over-solution),
review state, and copyable CLI commands. It must not generate proposals through a
trusted path, apply or promote proposals, auto-approve, execute commands, or
become a command bridge.

## Dependency order and closure gates

The follow-up issues stay scoped to reuse of existing surfaces and must be
completed in this order:

1. **Scope and Contract** — this issue (#1592).
2. **Brief/NL Intake and Proposal Model v1** — #1593. Define the intake and
   proposal model that turns a brief/NL request into a proposal on the existing
   review/apply path. Depends on Grid-Puzzle Game Class and Runtime v1 (#1574).
3. **Engine-Room Promotion Guard v1** — #1594. Implement the promotion
   precondition (four gates + solver + over-solution) on the existing path.
   Depends on Design-Integrity Gate v1 (#1583).
4. **Non-Developer Accessibility Path v1** — #1595. Wire the brief-to-verified
   proposal path for a non-developer, provenance attached.
5. **Generative Front Door Demo v1** — #1596. A deterministic demo of the front
   door over the engine room, reusing runtime/probe/evaluator/provenance.
6. **Scenario Coverage v30: Generative Front Door Regression Suite** — #1597.
   Continue Scenario Coverage numbering from the Era E baseline (v26 onward).
7. **Roadmap and #1 Governance Refresh after Generative Front Door v1** — #1598.
   Refresh roadmap/#1 context only after the above are complete, preserving #1
   and #23 as open anchors.

```text
#1592 scope -> #1593 -> #1594 -> #1595 -> #1596 -> #1597 -> #1598
```

Each follow-up issue must define the exact surface it extends, the files it
changes, the non-goals that stay blocked, the verification commands or checks
for closure, generated-state expectations, and proof that #1 and #23 remain
open. Issue closure requires all fixed PR units merged in order, latest `main`
pulled, issue-level verification run on latest `main`, the Definition of Done /
guardrail / drift-prevention / over-engineering / generated-state audits
recorded, a final evidence comment, and #1 and #23 confirmed open.

## Explicit non-goals

This gate does not authorize, now or implicitly through any follow-up:

- a direct trusted write from generation or any browser/Studio surface;
  proposals only, through the existing review/apply/trust-gradient path;
- autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden
  trusted writes;
- promotion of any proposal that has not passed the engine room (four gates +
  solver + over-solution);
- a new engine, runtime, or writer where an existing surface should be reused;
- a browser command bridge, arbitrary shell execution, dependency install,
  CI/workflow mutation, credentialed operation, network install/update, or
  publish/deploy/sign/upload;
- hosted/cloud/paid capability, marketplace transaction layer, or
  distributed/Elixir orchestration (Layer-3; DEFER per #1508, NO-GO per
  ADR #92);
- engine breadth (renderer/3D/audio/physics depth) beyond what this rung's gate
  justifies;
- generated runs/genre/evidence/registry artifacts committed unless explicitly
  fixture-scoped;
- any claim of a production-ready engine, Godot replacement/parity, or that
  generated games are good, fun, or shippable.

## Generated-state policy

Generated run state, caches, local worktrees, build outputs, and evidence
bundles remain untracked unless a future issue explicitly scopes a tiny
deterministic fixture. Closure audits should use `git status --short --ignored`
or an equivalent check to confirm generated/local artifacts remain ignored.

## #1 / #23 governance preservation

- #1 remains open as the broad roadmap/vision anchor.
- #23 remains open as the repo-memory/design context anchor.
- This design gate does not replace, close, or narrow either anchor. Any change
  to #1 or #23 requires a separate explicit governance decision.
