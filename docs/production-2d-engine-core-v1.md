# Production-Grade 2D Engine Core v1 Scope Contract

This document is the canonical scope contract for #1 Milestone 8 and issue #581.
It defines a bounded engineering target for a small local 2D vertical-slice game
class. It is not a claim that Ouroforge is currently production-ready, a Godot
replacement, broadly compatibility-stable, a secure sandbox, a native export
platform, a plugin runtime, a hosted/cloud product, or an autonomous launch
system.

## Purpose

Production-Grade 2D Engine Core v1 moves Ouroforge from a minimal
evidence-native runtime and expressive fixtures toward a scoped 2D engine core
that can support small playable browser/local games through agentic,
evidence-backed workflows.

The milestone exists to coordinate implementation issues. This control document
adds no renderer, physics, input, save/load, Studio, export, plugin, hosted, or
source-apply behavior by itself.

## Completed baseline

The milestone builds on these existing contracts and governance surfaces:

- Runtime v1: local static browser runtime, scene loading, simple rendering,
  input, collision/trigger events, snapshots, and runtime probe evidence.
- Scenario/Evaluator v1: deterministic scenario packs, bounded assertions, run
  artifacts, verdicts, and evidence indexes.
- Evolve Loop v1: local generated proposals, review-gated mutation flows, and
  evidence-backed journals.
- Engine Expressiveness v2: scene components, tilemaps, animation/audio intent,
  physics/collision metadata, HUD values, transitions, and runtime inspection
  read models.
- Asset Pipeline v1: local manifest validation, asset integrity/loading evidence,
  previews, sprite atlas/tilemap authoring contracts, and generated-state policy.
- Visual Authoring v1: review-gated visual edit drafts, transaction previews,
  visual diff summaries, and read-only Studio/dashboard compatibility.
- Source Mutation Preview v1: source patch preview, file-class validation,
  sandbox dry-run evaluation, review decisions, evidence bundles, and display-only
  apply-transaction evidence.
- Public Alpha Governance: conservative wording, local-first boundaries,
  evidence-native claims, generated-state hygiene, and #1/#23 governance anchors.

## Bounded target game class

The target is small local 2D vertical-slice games with:

- Multiple bounded scenes and project manifests.
- Local assets: sprites, sprite atlases, tilemaps/tilesets, simple audio, and
  manifest-backed references.
- Deterministic 2D rendering: render queue/graph, layers, camera, viewport,
  sprite/tilemap/HUD/debug ordering, frame summaries, and inspection evidence.
- Camera, parallax/layer semantics, and viewport behavior sufficient for a small
  platformer/adventure-style vertical slice.
- Collision/physics suitable for simple kinematic/dynamic entities, triggers,
  hazards, goals, and scenario assertions.
- Input actions and replayable input evidence, not only raw key states.
- Animation, lightweight VFX, and audio-event evidence with deterministic
  summaries.
- Runtime state, save/load, and deterministic replay evidence for short sessions.
- Profiling/frame-budget evidence for local runtime smoke tests.
- Scenario coverage and regression suites tied to project manifests and generated
  run artifacts.
- Studio/dashboard inspection surfaces that remain read-only or draft-only unless
  a separate trusted Rust/local API explicitly owns persistence.

## Dependency order

Follow-up implementation issues should proceed in this order unless a later live
issue audit proves a safer dependency order:

1. Renderer architecture/render queue contract and evidence.
2. Camera, layers, parallax, and viewport semantics.
3. Sprite, sprite-atlas, and tilemap rendering integration.
4. 2D physics/collision solver and trigger evidence.
5. Input abstraction and action mapping.
6. Runtime state, save/load, and deterministic replay.
7. Animation, particles, and lightweight VFX.
8. Audio runtime and bus evidence.
9. Runtime debug, profiling, and frame-budget evidence.
10. Production 2D vertical-slice demo.
11. Scenario coverage/regression suite for the vertical slice.
12. Studio 2D engine inspection surfaces.
13. Roadmap/#1 governance refresh after the milestone evidence exists.

Each issue should use small PR units with focused tests and post-merge evidence.
Do not combine unrelated systems when independent verification would be clearer.

## Trusted boundary

- Rust/local code owns trusted validation, trusted persistence, source-like
  fixture validation, generated evidence artifact writing, project/run binding,
  and CLI behavior.
- Browser runtime code may execute local game logic and browser-local probes, but
  it does not gain trusted filesystem persistence, shell command execution, or
  source mutation authority.
- Dashboard and Studio surfaces display exported evidence and may prepare drafts
  only when explicitly scoped. They are not trusted writers and must not contain
  hidden command bridges, local server bridges, auto-apply, auto-merge, or
  unrestricted source mutation controls.
- Generated runs, previews, dashboard data, screenshots, temp projects, and local
  tool state stay untracked unless a follow-up issue explicitly scopes a tiny
  deterministic fixture as tracked source-like data.

## Verification and closure gates

Every Production 2D follow-up issue should include:

- Live issue checks for the current issue, #1, and #23.
- Focused tests/smokes proving the exact behavior changed.
- Backward-compatibility checks for existing seeds, scenes, manifests, run
  artifacts, dashboard exports, Studio read models, and source-like fixtures.
- Generated-state audit showing only ignored local/runtime outputs are present.
- Conservative wording audit: no Godot replacement, current production-ready,
  broad compatibility-stable, secure-sandbox, native export, plugin runtime,
  hosted/cloud, or autonomous launch claims.
- Final latest-main gate before closing the issue:

```bash
gh issue view <issue-number> --repo shaun0927/Ouroforge
gh issue view 1 --repo shaun0927/Ouroforge
gh issue view 23 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
git status --short --ignored
```

Runtime/browser issues should add relevant `examples/game-runtime/*.test.cjs` and
syntax checks. Dashboard/Studio issues should add read-only rendering and XSS
escaping coverage.

## Non-goals for Milestone 8

Milestone 8 does not authorize:

- 3D engine implementation.
- Native export, platform packaging, signing, notarization, or release/publish
  automation.
- Plugin runtime, plugin marketplace, remote asset hosting, hosted/cloud/server,
  auth/account systems, or collaboration infrastructure.
- WebGPU/native renderer migration or shader/material systems.
- Browser-side trusted file writes, command bridges, local server bridges, hidden
  command execution, auto-merge, auto-apply, or unrestricted source mutation.
- Dependency, CI, workflow, or build-script mutation unless a separate explicit
  governance issue authorizes it.
- A full Studio editor claim.
- Current Godot replacement, broad production-ready, or shipped-game maturity
  claims.

## Governance anchors

- #1 remains the roadmap/final-goal anchor and must stay open unless a separate
  explicit governance decision changes it.
- #23 remains the memory/governance anchor and must stay open unless a separate
  explicit governance decision changes it.
- This document may be revised only by an explicit follow-up governance issue or
  roadmap refresh. Implementation issues should cite it rather than weakening its
  boundaries locally.


## Completion status after #594

Production 2D Engine Core v1 is recorded complete after issues #581-#593 closed
with merged implementation, demo, regression, Studio inspection, and generated-
state evidence. The completed evidence chain covers:

- scope/contract (#581);
- renderer architecture and render graph (#582);
- camera, layers, parallax, and viewport behavior (#583);
- sprite, atlas, and tilemap rendering integration (#584);
- 2D physics/collision and trigger evidence (#585);
- input abstraction and action mapping (#586);
- runtime state save/load and deterministic replay digest evidence (#587);
- animation, particles, lightweight VFX, audio runtime, and bus evidence (#588,
  #589);
- runtime debug/profiling/frame-budget evidence (#590);
- bounded Production 2D vertical-slice demo (#591);
- Scenario Coverage v7 regression suite (#592);
- escaped read-only Studio 2D inspection surfaces (#593).

This completion remains a bounded engineering milestone for local, evidence-
native 2D vertical-slice games. It does not revise the non-goals above: no 3D
implementation, native export, plugin runtime, hosted/cloud/server/auth behavior,
browser trusted writes, command bridges, unrestricted source apply, auto-merge,
public launch automation, production-ready claim, broad compatibility-stable API,
secure-sandbox guarantee, shipped-game maturity claim, or Godot replacement
positioning is authorized.

The recommended next dependency-ordered technical branch is 3D Capability Gate v1
(#596-#608), starting with a scoped capability contract and preserving the same
live issue checks, generated-state audit, and #1/#23 anchor requirements.
