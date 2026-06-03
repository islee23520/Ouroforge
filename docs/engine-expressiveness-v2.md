# Engine Expressiveness v2 / Playable Game Authoring v1

Engine Expressiveness v2 was the bounded milestone after Agentic Loop
Orchestration v1. The authoring loop can now plan, dry-run, execute trusted
steps, recover, bundle evidence, hand off to agents, and display loop state in a
read-only Studio cockpit. The next bottleneck was the small 2D game expression
surface: scenes, runtime rules, scenario coverage, and Studio inspection needed
enough structure to author and verify a tiny playable game without changing the
project's local-first trust boundary.

Status: the implemented subset is complete through the expressive Studio
inspection work and this #322 governance refresh. Completion covers additive
scene components, deterministic collision/trigger/HUD evidence, the
collect-and-exit playable demo fixture, regression scenario coverage, and
read-only Studio inspection. Animation/audio gameplay events (#317) and
multi-scene transitions (#318) remain separate design-blocked candidates and are
not included in the completed milestone claim.

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

## Completed implementation evidence

The implemented Engine Expressiveness v2 evidence is recorded in these source
contracts and fixtures:

- `docs/scene-component-model-v2.md` for additive component summaries and
  compatibility expectations.
- `docs/collision-physics-v2.md` for deterministic movement/collision rules and
  explicit missing/malformed reporting.
- `docs/gameplay-trigger-flags-v1.md` for trigger/flag state used by goals,
  pickups, exits, and scenario evidence.
- `docs/playable-demo-v2-collect-and-exit.md` plus
  `examples/playable-demo-v2/collect-and-exit/` for the local collect-and-exit
  fixture.
- `docs/scenario-coverage-v3.md` plus
  `examples/engine-expressiveness-v2-regression/` for regression coverage over
  expressive runtime behavior.
- `docs/studio-authoring-surface-v2-expressive-inspection.md` and
  `examples/authoring-cockpit/` for escaped, read-only inspection of expressive
  component, collision, trigger, HUD, transition, and event state.

The milestone remains an MVP contract, not a public compatibility promise or a
production editor/runtime claim.

## Target outcome

The milestone enables a small 2D playable authoring loop for the implemented
surfaces while keeping design-blocked surfaces explicit:

```text
richer scene components
  -> gameplay triggers and flags
  -> HUD/state feedback
  -> collision/physics rules
  -> animation/audio gameplay events (design-blocked #317)
  -> manifest-declared scene transitions (design-blocked #318)
  -> playable collect-and-exit demo
  -> scenario regression coverage
  -> Studio read-only inspection
```

The target is not a production engine. The completed subset provides enough
expressive structure to make small playable game states inspectable, testable,
and reviewable through the existing evidence-native loop while leaving #317 and
#318 out of completion claims.

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
5. **Animation and Audio Gameplay Events v2** — design-blocked (#317); add
   bounded event declarations and evidence hooks only after its design gate is
   resolved.
6. **Multi-Scene and Level Transition v1** — design-blocked (#318); allow
   manifest-declared scene transitions only after deterministic validation and
   scenario-evidence design are resolved.
7. **Playable Demo v2** — create a one-screen collect-and-exit demo that exercises
   the new component/rule surface.
8. **Scenario Coverage v3** — add regression scenarios for expressiveness
   features and demo behavior.
9. **Studio Authoring Surface v2** — extend read-only Studio inspection for the
   new scene/runtime state and inert Rust command text only.
10. **Roadmap and #1 Governance Refresh** — update #1/top-level docs after the
    implemented milestone surfaces are verified, while leaving #1/#23 open.

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
