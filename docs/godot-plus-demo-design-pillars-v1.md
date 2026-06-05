# Godot-Plus Demo Game v1 Design Pillars

Issue: #779  
Status: **GPD12.2.2 playable-success contract**. This document chooses the bounded demo genre, mechanics, objective, feel, difficulty ramp, playable success criteria, and implementation boundaries for the Godot-Plus Demonstration Game v1 track. It does not implement gameplay, add assets, run QA, export/package builds, mutate source through Studio, create executable plugins, publish a release, or change #1/#23 governance anchors.

## Genre choice

The v1 demo is a **single-screen top-down action-puzzle escape** called **Signal Gate**.

The player enters a compact arena, collects a signal key, avoids a patrolling hazard drone, routes the key into a gate switch, and exits through the opened gate. The design is 2D-first and intentionally small: one tutorial-style level plus room for later fixture variants, no campaign, no procedural generation, no inventory system, no dialogue tree, and no network or account behavior.

This genre is selected because it exercises existing Ouroforge contracts without needing new engine/editor surfaces:

- tilemap and sprite-backed level readability;
- movement, collision, trigger flags, hazard contact, and simple state transitions;
- structured behavior/state-machine evidence for the drone and gate;
- HUD/status feedback for objective clarity;
- deterministic scenario assertions for success, failure, and regression checks;
- visual/Studio draft workflows that can propose small level or behavior adjustments;
- local web package verification without native/mobile/store export.

## Design pillars

1. **Readable local vertical slice** — every gameplay state must be inspectable from local source fixtures, run evidence, dashboard, and Studio read-only/draft-only surfaces.
2. **Evidence-native agentic loop** — the demo must show an agent-created draft, deterministic QA failure evidence, explicit review decision, review-gated apply, rerun comparison, and final verification.
3. **Tiny complete game loop** — success means collect the signal key, open the gate, and reach the exit; failure means hazard contact or blocked objective state is visible and reproducible.
4. **Scoped Godot-plus proof, not parity** — claims are limited to this evidence-backed workflow slice and must not imply broad Godot replacement, full editor parity, production readiness, secure sandboxing, or commercial release readiness.
5. **Trusted boundary preservation** — Rust/local code owns validation, persistence, evidence writing, source-apply/export contracts, and CLI behavior; browser/Studio surfaces remain read-only or draft-only and never become command bridges or trusted write paths.

## Core mechanics

- **Move and collide:** the player moves on a bounded 2D tilemap with walls, a gate, key pickup, hazard contact, and an exit trigger.
- **Signal key and gate:** collecting the key sets a visible objective flag; the gate opens only after the key flag is present and records evidence of the transition.
- **Hazard drone:** a simple deterministic patrol or state machine creates pressure without adding arbitrary scripting or executable plugin behavior.
- **HUD feedback:** the HUD shows key state, gate state, hazard/failure state, and exit readiness in enough detail for scenario and Studio inspection.
- **Review-gated iteration:** a later agentic edit can adjust a tile, gate position, hazard route, or behavior parameter only through existing review-gated Safe Source Apply evidence.

## Player objective and target feel

Objective: **collect the signal key, open the gate, and exit the arena while avoiding the hazard drone**.

Target feel:

- 30–90 second local smoke-play session;
- readable grid/arena layout with obvious objective route;
- one meaningful hazard timing decision;
- immediate feedback when key/gate/exit state changes;
- failure is recoverable in later scenarios by reset/rerun evidence, not by hidden state.

## Difficulty ramp

The vertical slice uses a three-step ramp that later issues can map to levels/scenarios:

1. **Learn:** key and exit are visible; no hazard pressure required to complete.
2. **Time:** a deterministic drone patrol requires waiting or taking a safer route.
3. **Verify:** a regression scenario intentionally catches a blocked gate, missing key evidence, stale behavior, or unsafe draft/apply attempt.

The ramp is deliberately small. It does not add boss fights, skill trees, procedural content, large asset packs, physics complexity, networking, or broad AI behavior.

## Playable success criteria

GPD12.2.2 fixes the vertical-slice completion contract that later implementation issues must satisfy. The criteria are intentionally measurable and local; they do not authorize gameplay implementation in this PR unit.

### Completion criteria

1. **Loadable local project fixture** — a source-controlled Signal Gate project fixture loads through existing project/runtime validation paths without generated-state or dependency-install requirements.
2. **Deterministic win path** — a local scenario can prove the player collected the signal key, opened the gate, reached the exit trigger, and recorded a passing verdict with evidence refs.
3. **Deterministic failure path** — at least one local scenario can prove a blocked gate, missing key, hazard contact, stale behavior, or unsafe draft/apply attempt and records useful journal/dashboard context.
4. **Visible objective state** — key, gate, exit readiness, hazard/failure, and reset/rerun state are visible to scenario evidence plus dashboard/Studio read-only or draft-only surfaces.
5. **Review-gated iteration proof** — any source-changing agentic fix must be represented as draft/preview evidence, independent review decision, source-apply transaction, rollback metadata, rerun comparison, and final verification.
6. **Reproducible local package check** — later export/package issues may record a local web bundle smoke/checksum artifact, but native/mobile/console/store export, signing, publishing, deployment, and commercial release claims remain out of scope.
7. **Governance confirmation** — final issue evidence confirms #1 and #23 remain open and repeats the no-overclaim/no-production/no-Godot-replacement boundary.

### Acceptance matrix

| Criterion | Later evidence source | Pass signal | Boundary |
| --- | --- | --- | --- |
| Loadable project | Project validation artifact and runtime smoke | Fixture loads locally with stable project metadata | No generated demo artifact committed unless fixture-scoped |
| Win path | Scenario result, input replay, runtime probe, journal/dashboard refs | Key, gate, exit, and pass verdict are linked | No manual-only or hosted evidence required |
| Failure path | Scenario failure result plus journal/dashboard context | Blocker is visible and actionable | Failure evidence does not auto-apply a fix |
| Objective visibility | HUD/status read model plus Studio/dashboard surfaces | Key/gate/exit/hazard states are inspectable | Browser surfaces remain read-only or draft-only |
| Review-gated iteration | Draft, review decision, source-apply transaction, rollback, rerun comparison | Independent review precedes trusted apply; rerun proves improvement | No self-approval, auto-apply, auto-merge, or hidden trusted writes |
| Package reproducibility | Local web package verification/checksum evidence | Bundle smoke passes locally | No native/mobile/console/store export, signing, publishing, or deployment |
| Governance | Issue comments and verification commands | #1/#23 remain open; claims stay scoped | No broad Godot parity, production readiness, secure sandbox, or commercial release claim |

### Non-acceptance examples

The vertical slice is not accepted by screenshots alone, a broad design document without scenario evidence, a manually played run without reproducible inputs, an unreviewed source mutation, a browser-side trusted write, an executable plugin, a hosted deployment, a native/mobile/store package, or language claiming full Godot replacement/parity.

## Godot-plus capability mapping

| Demo moment | Evidence-native proof point | Boundary |
| --- | --- | --- |
| Draft a level or behavior tweak | Generated draft/preview artifact links to source fixture and expected scenario impact | Draft-only; no browser trusted write or auto-apply |
| QA catches a blocked gate or hazard regression | Scenario result, assertion failure, journal entry, and dashboard read model identify the failure | Deterministic local evidence only; no hosted QA service |
| Review approves a safe fix | Review decision and source-apply transaction record reviewer, target hashes, and rollback metadata | Explicit review gate; no self-approval or reviewer bypass |
| Rerun proves improvement | Before/after run comparison links scenario verdicts and evidence refs | Local generated evidence; no public deployment |
| Studio walkthrough explains the loop | Studio renders source, evidence, draft, review, and comparison surfaces | Read-only/draft-only browser UI; no command bridge |
| Plugin descriptor appears in demo | Plugin registry/descriptor evidence is displayed as inert metadata | No executable plugin runtime, marketplace, install/update, or dynamic loading |
| Package verification is recorded | Later local web package smoke proves reproducibility | No native/mobile/console/store export or commercial release claim |

## Explicit boundaries

This document does **not** authorize:

- gameplay implementation in this PR unit;
- broad Godot parity, current Godot replacement, production-ready engine/editor, secure sandbox, or commercial release claims;
- public launch, app-store/Steam/itch publishing, signing, native/mobile/console export, hosted/cloud/server/auth/account behavior, or credentialed operation;
- direct Studio trusted source writes, browser command bridges, local server command bridges, hidden command execution, arbitrary shell execution, dependency install, network install/update, CI/workflow mutation, auto-apply, auto-merge, self-approval, or reviewer bypass;
- executable plugins, arbitrary JavaScript/native extensions, plugin marketplace, or network plugin install/update;
- tracking generated demo outputs, exports, QA runs, screenshots, videos, package bundles, temp servers, browser profiles, or local tool state unless a later issue explicitly scopes a fixture artifact.

## Implementation handoff for later issues

- #780 should turn these pillars and playable success criteria into a concise GDD and acceptance criteria.
- #781 should scaffold the project without expanding the trusted boundary.
- #782–#787 should implement mechanics, level, behavior, UI, assets, and scenario matrix in small verifiable PR units.
- #788–#797 should demonstrate QA, agentic iteration, Studio walkthrough, package/export evidence, plugin descriptor usage, comparison matrix, performance budget, docs, regression coverage, and roadmap refresh.

If later implementation finds the loop too large, prefer deleting scope before adding systems. The vertical slice is complete when the evidence-backed workflow is reproducible, not when it resembles a full commercial game or full Godot replacement.
