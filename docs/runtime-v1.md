# Runtime v1 Scope and Contract

Runtime v1 is the bounded upgrade path from Ouroforge's current demo runtime to a **small-game-capable browser runtime**. It is not a full engine rewrite, a Godot replacement, or a public compatibility promise.

This document is planning/control only. Implementation authority remains in the individual GitHub issues linked below.

Related context:

- `docs/architecture.md` — current evidence-native architecture and language boundary.
- `docs/roadmap.md` — product roadmap context and milestone sequencing.
- `docs/public-readiness-audit.md` — readiness constraints that must not be interpreted as launch commitments.

## Current MVP Baseline

The current MVP proves that Ouroforge can validate seeds, run a deterministic local workflow, produce evidence artifacts, evaluate runs, create journals, propose mutations, and show local evidence in browser-facing examples.

Runtime v1 builds on that baseline by making runtime behavior richer while preserving the same evidence-native shape:

1. runtime state is observable;
2. observed state can be captured as run evidence;
3. scenarios/evaluators can assert against that evidence;
4. journals/mutations can reason over the resulting artifacts.

## Runtime v1 Capability List

Runtime v1 permits only these runtime data categories:

- **scene schema v1** — richer, validated scene data for small browser demos;
- **entity/component model** — small explicit entity/component data separation inside that scene schema;
- **deterministic input replay** — replayable input streams with stable timing semantics;
- **AABB collision events** — simple axis-aligned collision detection and observable events;
- **in-memory snapshot/restore** — local runtime state capture and restore for replay/debug flows;
- **local static image assets** — bounded local static image references for browser demos;
- **deterministic animation state** — animation clocks/state that can be inspected and replayed;
- **observable audio events** — audio intent/events visible in evidence without requiring production audio mixing;
- **playable demo evidence** — sample runtime output that proves the capabilities work together.

Anything outside this list belongs in a later issue or milestone.

## Issue Order and Dependency Graph

Runtime v1 must be implemented in GitHub issue order unless an issue body explicitly updates the dependency graph. The actual open issue sequence is:

```text
#59 Runtime v1 Scope and Contract (this document; design only)
  └─ #60 Scene Schema and Entity/Component Model
       ├─ #61 Deterministic Input Replay
       │    └─ #63 Snapshot and Restore
       ├─ #62 Collision System
       ├─ #64 Local Asset Loading
       ├─ #65 Animation
       └─ #66 Audio
            └─ #67 Playable Demo Integration
```

Dependency meaning:

- #60 is the Runtime v1 data foundation and should land before behavior-specific runtime features.
- #61 needs the #60 scene/entity vocabulary so replay can target stable runtime entities and inputs.
- #62 uses #60 collider declarations and entity transforms.
- #63 depends on deterministic state from #60 and replay semantics from #61.
- #64, #65, and #66 attach local assets, animation state, and audio events to the bounded #60 runtime data model.
- #67 composes the completed Runtime v1 capabilities into a playable demo; it must not backfill unfinished feature work.

Sequential execution rule: even when the dependency graph allows conceptual parallelism, implementation PRs must follow the active workflow's one-PR-at-a-time merge discipline unless a later issue explicitly relaxes that rule.

## Follow-Up Issue Contracts

### #60 — Scene Schema and Entity/Component Model

Unlocks a small explicit scene schema with entities, components, transforms, sprites, collider declarations, tags, metadata, and runtime queryability.

Must not implement a full ECS framework, plugin-defined schema extensions, database persistence, editor visual scripting, production ECS optimization, or compatibility guarantees.

### #61 — Deterministic Input Replay

Unlocks replayable frame-based inputs that scenarios can use to drive runtime behavior deterministically.

Must not implement recording UX, cloud replay storage, Playwright-based browser control, or non-deterministic wall-clock replay semantics.

### #62 — Collision System

Unlocks minimal deterministic 2D AABB collision detection and evidence for small platformer/top-down demos.

Must not implement a physics engine, continuous collision detection, rigid bodies, broad-phase optimization beyond the issue contract, or editor tooling.

### #63 — Snapshot and Restore

Unlocks in-memory deterministic world snapshot/restore for scenario branching and replayable QA.

Must not implement durable save games, database persistence, cloud sync, binary snapshot formats, or rollback networking.

### #64 — Local Asset Loading

Unlocks bounded local static image loading for browser runtime demos.

Must not implement an asset pipeline, remote asset hosting, uploads, compression/transcoding, CDN behavior, or marketplace/package behavior.

### #65 — Animation

Unlocks minimal fixed-timestep animation state that can be inspected, replayed, and scenario-verified.

Must not implement a timeline editor, skeletal animation, blending graphs, imported animation formats, or production animation tooling.

### #66 — Audio

Unlocks minimal local audio event support that can be triggered and observed as evidence.

Must not implement production audio mixing, streaming, device selection, WebAudio authoring tools, native audio backends, or acceptance based on speaker output.

### #67 — Playable Demo Integration

Unlocks an integrated small playable browser demo that proves completed Runtime v1 features work together beyond the original MVP.

Must not become a catch-all for unfinished feature work, editor work, public launch work, or production-engine claims.

## Probe API Expectations

Every Runtime v1 capability must be observable through the browser runtime probe API at the level needed for deterministic tests and evidence capture. At minimum, the probe-facing state should expose:

- loaded scene identity and validated schema version;
- entity/component summaries relevant to the active issue;
- deterministic tick/frame counters used by replay and animation;
- input replay status and currently applied input events;
- collision event summaries for the current or most recent tick;
- snapshot/restore status when that capability is active;
- local asset resolution status without exposing direct browser writes;
- animation state summaries;
- audio event summaries;
- errors/warnings that affect scenario verdicts.

The probe API is for observability and deterministic verification. It must not become a remote control server, database API, plugin boundary, or editor persistence layer.

## Evidence and Verification Expectations

Runtime v1 implementation issues should keep evidence connected to the existing Seed/Scenario/run artifact flow. When a runtime feature affects behavior, its issue should specify how it appears in one or more of:

- scenario results;
- evaluator verdicts;
- run evidence artifacts;
- journals or mutation proposals when the later issue explicitly authorizes that connection;
- sample runtime/demo evidence.

Baseline verification for Runtime v1 documentation and implementation PRs should include, when applicable:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

Browser example changes should also run the relevant JavaScript syntax and test commands named by the active issue, such as:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Issue-specific verification commands override this document when stricter.

## Language and Runtime Boundary

Runtime v1 keeps the existing local-first language split:

- **Rust** owns schema validation, CLI behavior, artifact contracts, evaluator integration, deterministic core logic, and any persistence that must be trusted.
- **JavaScript** owns browser runtime/demo/probe behavior and local UI examples.
- **No Elixir or distributed orchestration** is authorized by Runtime v1. Distributed design remains gated by the later design issue.
- Browser JavaScript must not write trusted persisted project state directly. Persisted state that requires validation must flow through Rust validation.

## Non-Goals and Drift Risks

Runtime v1 does not authorize:

- full engine architecture rewrite;
- physics engine integration;
- Godot replacement claims;
- production maturity claims;
- public compatibility promises;
- native export;
- public launch automation;
- server, database, cloud, auth, marketplace, or plugin systems;
- editor/UI implementation unless the active issue explicitly authorizes it;
- Playwright adoption;
- Elixir or OTP implementation.

Primary drift risk: using this planning document as permission to implement a full engine. Countermeasure: each implementation PR must cite the active issue's exact PR unit and reject later issue scope.

## PR Decomposition Summary for #60–#67

Each Runtime v1 implementation issue controls its own PR decomposition. The expected shape is:

1. add or adjust the minimal Rust contract/schema/test surface needed by that issue;
2. add or adjust the minimal browser runtime/probe/demo behavior authorized by that issue;
3. connect the behavior to evidence/scenario verification only when the issue requires it;
4. stop before editor, distributed, plugin, production, or public-launch concerns.

No Runtime v1 PR should combine separate follow-up issues unless the issue body explicitly authorizes that combination.

## Closing Checklist for Runtime v1 Planning

Before closing #59:

- this file exists as the canonical Runtime v1 scope contract;
- #60–#67 are linked with bounded acceptance targets;
- probe, evidence, verification, and language boundaries are documented;
- no product implementation code was added;
- Runtime v1 remains framed as a small-game-capable browser runtime, not a mature engine.
