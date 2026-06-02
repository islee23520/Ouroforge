# Engine Expansion v1 Scope and Contract

Engine Expansion v1 is the bounded upgrade path from Ouroforge's proven private
MVP harness into a **small-2D-game-capable evidence-native engine foundation**.
It is a planning/control contract for issues #157 through #170. It does not make
Ouroforge a Godot replacement, production engine, public compatibility target,
native export product, plugin platform, or distributed system.

Issue #1 remains the top-level vision and roadmap source of truth. This document
links Engine Expansion v1 to that roadmap by defining only the milestone scope,
execution order, evidence expectations, verification policy, language/runtime
boundary, and drift guardrails. Each follow-up issue remains the implementation
contract for its own PR unit(s).

Related context:

- `docs/architecture.md` — current evidence-native architecture and language
  boundary.
- `docs/roadmap.md` — current roadmap and non-goal framing.
- `docs/runtime-v1.md` and `docs/runtime-v1-demo.md` — completed Runtime v1
  browser-game foundation.
- `docs/scenario-evaluator-v1.md` and `docs/scenario-evaluator-v1-demo.md` —
  completed deterministic scenario/evaluator foundation.
- `docs/evolve-loop-v1.md` and `docs/evolve-loop-v1-demo.md` — completed
  journal-to-mutation loop foundation.
- `docs/studio-v1.md` and `docs/studio-v1-demo.md` — completed local authoring
  and evidence-review foundation.
- `docs/distributed-elixir-design.md` — prior distributed design review; it does
  not authorize Engine Expansion v1 distributed implementation.

## Current Baseline: MVP Harness vs. Engine Expansion v1

The completed MVP harness proves the Ouroboros loop shape:

1. a Seed is validated;
2. a local run executes through browser workers;
3. evidence artifacts are produced;
4. deterministic checks create verdicts;
5. journals explain failures and decisions;
6. mutation proposals can cite the evidence.

Engine Expansion v1 builds on that baseline by making the local browser runtime
and authoring surface capable enough for a small 2D game slice. The milestone is
not a rewrite. It expands the same Seed -> Run -> Observe -> Verify -> Journal ->
Evolve loop with bounded renderer, tilemap, asset, animation, audio,
physics/collision, hot reload, scene composition, studio authoring, playable
template, and integration-demo capabilities.

## What "Small 2D Game Capable" Means Here

For this milestone, "small 2D game capable" means Ouroforge can locally author,
run, inspect, and verify a browser-based 2D game slice with:

- sprite rendering with deterministic layer, camera, and debug state;
- tilemap data sufficient for simple platformer or top-down levels;
- a local asset manifest/pipeline that validates and reports asset references;
- deterministic animation state exposed to probes and evidence;
- observable audio intent/events without requiring production audio mixing;
- deterministic physics/collision behavior suitable for small AABB-based demos;
- runtime hot reload for bounded local development feedback;
- scene composition that assembles entities, maps, assets, animation, audio, and
  collision contracts without a general engine rewrite;
- local Studio authoring v2 surfaces that use Rust validation for trusted
  persistence boundaries;
- a playable game template proving the completed capabilities work together;
- design-gated native export and plugin decisions with no implementation in the
  gate issues;
- an integration demo with evidence, verdicts, and journal/evolve links.

It does **not** mean production maturity, broad engine API stability, public
launch readiness, Godot compatibility, native app packaging, marketplace/plugin
extensibility, cloud collaboration, database persistence, or distributed scale.

## Issue Order and Dependency Graph

Engine Expansion v1 must be implemented in strict issue order and one PR unit at
a time unless a later issue explicitly changes the dependency graph. Do not stack
multiple unmerged PRs.

```text
#157 Engine Expansion v1 Scope and Contract (this document; design only)
  └─ #158 Renderer v1 Sprite, Layer, Camera, and Debug State
       └─ #159 Tilemap v1
            └─ #160 Asset Pipeline v1
                 └─ #161 Animation v1
                      └─ #162 Audio v1
                           └─ #163 Physics and Collision v2
                                └─ #164 Runtime Hot Reload v0
                                     └─ #165 Scene Composition v2
                                          └─ #166 Studio Runtime Authoring v2
                                               └─ #167 Playable Game Template v1
                                                    ├─ #168 Native Export Design Gate (design only)
                                                    ├─ #169 Plugin System Design Gate (design only)
                                                    └─ #170 Engine Expansion v1 Integration Demo
```

Dependency meaning:

- #157 is the sequencing and guardrail contract; it authorizes no product code.
- #158 establishes the visible renderer/probe vocabulary that later features can
  inspect.
- #159 uses renderer state to display tile-based game space.
- #160 validates and reports local asset references used by renderer, tilemap,
  animation, audio, and demos.
- #161 builds deterministic animation state on the renderer/assets foundation.
- #162 adds observable audio intent/events after asset references are bounded.
- #163 upgrades collision/physics after sprite, tilemap, and animation state are
  inspectable.
- #164 hot reloads bounded runtime inputs after core runtime feature contracts
  are stable enough to reinitialize deterministically.
- #165 composes the engine feature set into scene contracts without jumping into
  editor or plugin scope.
- #166 adds local Studio authoring v2 only after scene composition and Rust
  validation boundaries exist.
- #167 creates the playable template after the runtime and authoring contracts
  are present.
- #168 and #169 are design gates only; a GO decision may create follow-up issues
  but does not implement native export or plugins.
- #170 composes the completed milestone into integration evidence and must not
  backfill unfinished feature work.

## Follow-Up Issue Contracts

### #158 — Renderer v1 Sprite, Layer, Camera, and Debug State

Adds bounded renderer state for sprites, draw layers, camera, and debug/probe
visibility. Evidence should prove deterministic render-facing state, not visual
perfection or a full rendering engine.

### #159 — Tilemap v1

Adds bounded tilemap contracts for small 2D levels. Evidence should show tilemap
loading, deterministic map state, and probe/verdict visibility without adding a
map editor unless the issue explicitly authorizes it.

### #160 — Asset Pipeline v1

Adds a local asset manifest/pipeline contract for validating, resolving, and
reporting asset references. It must stay local-first and must not become uploads,
remote hosting, CDN behavior, compression/transcoding infrastructure, or a
marketplace/package system.

### #161 — Animation v1

Adds deterministic animation state that can be replayed, inspected, and asserted
through probes/evidence. It must not become skeletal animation, blending graphs,
imported animation tooling, or a timeline editor.

### #162 — Audio v1

Adds observable audio intent/events and evidence. Acceptance depends on event
state and artifact links, not speaker output, production mixing, streaming,
device selection, native audio, or authoring tools.

### #163 — Physics and Collision v2

Adds the next bounded physics/collision increment for small 2D gameplay. It must
stay deterministic and evidence-linked, and must not become a general physics
engine, networking/rollback system, or production performance project.

### #164 — Runtime Hot Reload v0

Adds bounded local hot reload behavior for supported runtime inputs. It must not
add a server, database, cloud sync, arbitrary browser file writes, or broad HMR
infrastructure outside the issue contract.

### #165 — Scene Composition v2

Adds composition contracts that connect renderer, tilemap, assets, animation,
audio, collision, and runtime data. It must not create a plugin system, visual
scripting platform, or full ECS rewrite.

### #166 — Studio Runtime Authoring v2

Adds bounded local authoring improvements connected to runtime composition and
Rust validation. Direct browser writes to trusted persisted project state remain
out of bounds.

### #167 — Playable Game Template v1

Adds a playable template proving the completed Engine Expansion v1 runtime path.
It must not be used to hide missing feature implementation, public launch work,
production claims, or compatibility promises.

### #168 — Native Export Design Gate

Produces a design document and GO/NO-GO decision only. It must not add native
export code, Tauri/Electron/native shell scaffolding, desktop/mobile packaging,
or any native export implementation. A GO decision may create follow-up issues.

### #169 — Plugin System Design Gate

Produces a design document and GO/NO-GO decision only. It must not add plugin
runtime code, dynamic loading, marketplace UI, plugin manager UI, or arbitrary
source mutation mechanisms. A GO decision may create follow-up issues.

### #170 — Engine Expansion v1 Integration Demo

Composes the completed milestone into evidence-native demo artifacts. It must
link run evidence, verdicts, probes, and journal/evolve outputs where authorized,
and must not backfill unfinished earlier issue scope.

## Evidence and Probe Expectations

Every Engine Expansion v1 feature must preserve the evidence-native loop:

```text
Seed -> Run -> Observe -> Verify -> Journal -> Evolve
```

Feature claims must be backed by deterministic probes, artifacts, or verdict
links. At minimum, the active implementation issue should define how its behavior
appears in one or more of:

- browser runtime probe/world-state excerpts;
- scenario results and deterministic evaluator verdicts;
- run artifact paths and evidence references;
- dashboard or cockpit read models when the issue changes those surfaces;
- journal entries or mutation proposals only when the issue explicitly connects
  that feature to journal/evolve behavior;
- focused Node tests for browser runtime behavior;
- Rust fixtures/tests for schema, model, CLI, artifact, or validation behavior;
- design documents and no-code/no-scaffold audits for design-only issues.

Evidence must describe the exact PR unit being proved. Broad smoke checks are
not a substitute for focused tests when a PR introduces schema, runtime, UI, or
artifact behavior.

## PR Unit Policy

For every issue and PR unit, implementers must:

1. read the full issue body and treat it as the active contract;
2. identify the current issue and PR unit before coding;
3. identify expected changed files;
4. identify focused tests/fixtures proving only that PR unit;
5. identify generated artifacts that must remain untracked;
6. identify guardrail, over-engineering, and drift-prevention items that would
   fail if later scope slips in;
7. branch from latest `main`;
8. keep the diff small, reversible, and limited to the active PR unit;
9. verify, open a PR, merge it, update `main`, and run post-merge verification
   before moving to the next PR unit or issue;
10. audit the issue checklist, comment with evidence, and close only when every
    success criterion, verification method, guardrail, non-goal, precision
    requirement, over-engineering check, drift-prevention check, and Definition
    of Done passes.

Design-only issues (#157, #168, and #169) require explicit no-code/no-scaffold
audits plus standard Rust verification.

## Verification Policy

Issue-specific verification commands are authoritative when stricter. Baseline
Engine Expansion v1 verification includes:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue list --repo shaun0927/Ouroforge --limit 160
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Runtime/browser changes should also run the relevant focused JavaScript checks
from the active issue, such as:

```bash
node --check examples/game-runtime/runtime.js
node examples/game-runtime/renderer.test.cjs
node examples/game-runtime/tilemap.test.cjs
node examples/game-runtime/assets.test.cjs
node examples/game-runtime/animation.test.cjs
node examples/game-runtime/audio.test.cjs
node examples/game-runtime/collision.test.cjs
node examples/game-runtime/reload.test.cjs
node examples/game-runtime/composition.test.cjs
```

Dashboard or cockpit changes should run the dashboard/cockpit export and Node
checks required by the active issue. Runtime/scenario/evidence changes should run
the platformer seed command when applicable:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

PR evidence should include summarized command outputs, run IDs/evidence refs when
runtime behavior changes, and known gaps if any.

## Language and Runtime Boundary

Engine Expansion v1 keeps the existing local-first language split:

- **Rust** owns schema validation, CLI behavior, artifact contracts, evaluator
  integration, deterministic core logic, and trusted persistence validation.
- **JavaScript** owns browser runtime/demo/probe behavior and local browser UI
  surfaces.
- Browser JavaScript must not write trusted persisted project state directly.
  Persistence that requires trust must flow through Rust validation.
- **Documentation/design** owns gate decisions for native export and plugins.
- **No Elixir, native shell/export implementation, server, database, cloud,
  auth, plugin runtime, marketplace, or dynamic loading** is authorized unless a
  later follow-up issue explicitly authorizes it after the relevant gate.

## Generated Artifact Policy

Generated and local runtime state must remain untracked and out of commits,
including:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- `.openchrome/`
- `.omc/`
- `.omx/`
- `.claude/`

PRs should mention any generated artifacts used as verification evidence and
confirm they were not committed.

## Explicit Non-Goals and Drift Risks

Engine Expansion v1 does not authorize:

- Godot replacement claims;
- production maturity claims;
- public launch or public compatibility promises;
- native export implementation in #168;
- plugin runtime, marketplace, dynamic loading, or plugin manager UI in #169;
- distributed or Elixir implementation;
- server, database, cloud, or auth infrastructure;
- Playwright adoption;
- direct browser writes to trusted persisted state;
- broad engine rewrites, full ECS replacement, visual scripting, or generalized
  platform extensibility unless a later issue explicitly authorizes a bounded
  increment.

Primary drift risks and countermeasures:

- **Planning becomes implementation permission.** Countermeasure: #157 is
  documentation/control only; each later issue body remains the implementation
  contract.
- **Integration demo becomes a dumping ground.** Countermeasure: #170 may only
  compose already-merged capabilities and evidence.
- **Design gates become scaffolds.** Countermeasure: #168 and #169 produce design
  decisions only; GO creates follow-up issues rather than code.
- **Evidence is replaced by claims.** Countermeasure: every feature needs focused
  tests, probes, artifacts, or verdict links.
- **Runtime/UI bypasses Rust validation.** Countermeasure: trusted persistence
  boundaries stay Rust-owned.

## PR Decomposition Summary for #158-#170

Each follow-up issue controls its own PR decomposition. The expected Engine
Expansion v1 shape is:

1. add or adjust the minimal Rust contract/schema/test surface required by the
   active issue;
2. add or adjust the minimal JavaScript runtime/probe/UI behavior authorized by
   the active issue;
3. connect behavior to deterministic evidence, scenario verdicts, dashboards,
   cockpit read models, journals, or mutation proposals only when authorized;
4. for design gates, write decision documents and follow-up issue proposals only;
5. stop before later issue scope, generated artifacts, public-launch claims,
   native/plugin/distributed implementations, or infrastructure drift.

## Definition of Done for #157

Issue #157 is done when this document defines Engine Expansion v1 scope, issue
order, dependencies, PR expectations, evidence/probe expectations, verification
policy, language boundaries, generated artifact policy, non-goals, guardrails,
and drift risks with no product implementation code.
