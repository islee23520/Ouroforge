# Engine Expressiveness v2 / Playable Game Authoring v1

Engine Expressiveness v2 is the next bounded milestone after Agentic Loop
Orchestration v1. The authoring loop can now plan, dry-run, execute trusted
steps, recover, bundle evidence, hand off to agents, and display loop state in a
read-only Studio cockpit. The next bottleneck is the small 2D game expression
surface: scenes, runtime rules, scenario coverage, and Studio inspection need
enough structure to author and verify a tiny playable game without changing the
project's local-first trust boundary.

This document is a scope contract only. It does not implement runtime, scene,
scenario, dashboard, or Studio behavior.

## Completed baseline

Engine Expressiveness v2 builds on these completed MVP contracts:

- Project Workspace Loop v1: local manifests, scaffolded project fixtures,
  project-bound runs, project comparison, and project-scoped scene mutation
  context.
- Evidence Fidelity & Trust Boundary Hardening v1: Rust-trusted artifact writes,
  browser/CDP evidence separation, reproducible command context, and conservative
  Studio evidence surfaces.
- Agentic Review & Regression Promotion v1: proposal rationale, review decision
  ledger, review-gated scene application, rerun comparison, regression
  promotion, regression run matrix, Journal v2, and Studio review cockpit.
- Agentic Loop Orchestration v1: data-only loop plans, inert dry-run sequencing,
  CLI-only step execution, explicit recovery preflight, generated evidence
  bundles, advisory handoff contracts, and read-only Studio loop cockpit.

## Target outcome

The milestone should enable a small 2D playable authoring loop:

```text
richer scene components
  -> gameplay triggers and flags
  -> HUD/state feedback
  -> collision/physics rules
  -> animation/audio gameplay events
  -> manifest-declared scene transitions
  -> playable collect-and-exit demo
  -> scenario regression coverage
  -> Studio read-only inspection
```

The target is not a production engine. It is enough expressive structure to make
small playable game states inspectable, testable, and reviewable through the
existing evidence-native loop.

## Dependency order

Follow-up issues should stay in this order unless a later issue documents a
blocker and replacement ordering:

1. **Scene Component Model v2** — define bounded scene component schema additions
   for common small-game authoring state. Keep backward compatibility with
   existing scenes and explicit missing/malformed reporting.
2. **Collision and Physics Rules v2** — add deterministic, bounded collision and
   movement rules that scenario tests can observe. Avoid broad physics-engine
   claims.
3. **Gameplay Trigger and Flag System v1** — make scene-local triggers and flags
   explicit evidence inputs for goals, pickups, exits, and state changes.
4. **UI/HUD Entities v1** — expose small HUD/state feedback entities without
   turning Studio into a production UI editor.
5. **Animation and Audio Gameplay Events v2** — add bounded event declarations
   and evidence hooks for animation/audio state used by gameplay scenarios.
6. **Multi-Scene and Level Transition v1** — allow manifest-declared scene
   transitions with deterministic validation and scenario evidence.
7. **Playable Demo v2** — create a one-screen collect-and-exit demo that exercises
   the new component/rule surface.
8. **Scenario Coverage v3** — add regression scenarios for expressiveness
   features and demo behavior.
9. **Studio Authoring Surface v2** — extend read-only Studio inspection for the
   new scene/runtime state and inert Rust command text only.
10. **Roadmap and #1 Governance Refresh** — update #1/top-level docs after the
    milestone is implemented and verified.

## Compatibility policy

- Existing scene fixtures, seeds, scenario packs, project manifests, dashboard
  exports, and Studio panels must remain compatible unless an issue explicitly
  scopes a migration.
- New scene fields should be additive and validated by Rust-owned schema checks.
- Missing, legacy, or malformed data must surface as explicit warnings or empty
  states, not inferred passes.
- Runtime and scenario behavior must remain deterministic enough for local
  regression tests and evidence comparison.
- Dashboard and Studio changes must read exported JSON only; they must not write
  trusted files, execute local commands, start a bridge, or gain authority over
  review/apply/promote/merge decisions.

## Verification policy

Each follow-up implementation issue must define focused verification for its
surface and include at least:

- issue/#1/#23 state checks where governance is in scope;
- Rust unit/integration tests for schema, validation, runtime, and scenario
  behavior touched by that issue;
- Node static checks/tests for dashboard or Studio changes;
- generated-state audit proving `runs/`, `target/`, dashboard exports, and local
  tool state remain untracked;
- latest-main post-merge verification before issue closure;
- explicit guardrail audit for no browser trusted writes, no command bridge, no
  auto-run, no auto-apply, no auto-promote, and no auto-merge.

## Non-goals

Engine Expressiveness v2 does not authorize:

- 3D engine scope;
- native export implementation;
- plugin runtime, marketplace, dynamic loading, or extension API;
- source-code mutation or source patch application;
- hosted/cloud/server/database/auth infrastructure;
- browser trusted writes or local command bridges;
- visual scripting;
- production editor claims;
- public launch automation or repository visibility changes;
- broad compatibility-stable engine API, secure-sandbox, production-ready, or
  Godot replacement claims.

## Governance anchors

- #1 remains open as the broad vision and implementation-roadmap anchor until a
  separate explicit governance decision replaces it.
- #23 remains open as repo-memory/design context.
- Any issue that changes these anchors must document maintainer approval and a
  replacement source of truth before closure.
