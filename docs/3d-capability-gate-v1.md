# 3D Capability Gate v1 Scope Contract

Issue: #596 — 3D Capability Gate v1 Scope and Contract. This document is the
canonical scope contract for #1 Milestone 9.

3D Capability Gate v1 defines the smallest safe path for Ouroforge to represent,
render, observe, and evaluate bounded local 3D scenes through the existing
evidence-native loop. It is a capability gate, not a full 3D engine, production-
ready engine, broad 3D compatibility promise, secure sandbox, native export
platform, plugin runtime, hosted/cloud product, autonomous launch system, or
Godot replacement claim.

## Purpose

The gate exists to coordinate future implementation issues and preserve the
trust, evidence, and generated-state boundaries already used by the 2D runtime.
This control document adds no 3D runtime, renderer, physics, animation, Studio,
export, plugin, hosted, or source-apply behavior by itself.

## Bounded 3D target

The initial target is a small deterministic local 3D demo scene with:

- a source-like scene graph and transform hierarchy;
- one bounded camera/projection model and viewport contract;
- local mesh/material references through existing source-like asset policies;
- a simple renderer smoke path that can produce deterministic render evidence;
- basic collision/trigger evidence sufficient for scenario assertions;
- simple animation playback evidence for bounded transforms or clips;
- runtime probe fields that distinguish 2D and 3D observations;
- scenario/evaluator compatibility for 3D verdicts without weakening 2D runs;
- dashboard and Studio inspection surfaces that remain escaped and read-only;
- generated run, screenshot, dashboard, and temp output kept ignored unless a
  later issue explicitly scopes a tiny deterministic fixture.

The target is deliberately smaller than a production 3D engine. It does not
include broad model import, advanced lighting/PBR/material graphs, skeletal
animation authoring, editor tooling, native export, marketplace/plugins, hosted
services, or arbitrary untrusted code execution.

## Dependency order

Follow-up issues should proceed in this order unless a later live issue audit
proves a safer dependency order:

1. 3D scene graph and transform hierarchy.
2. 3D camera, viewport, and projection evidence.
3. Mesh, material, and local asset reference contracts.
4. 3D renderer smoke path and render evidence.
5. 3D collision/physics capability and trigger evidence.
6. 3D animation playback evidence.
7. 3D runtime probe contract.
8. 3D scenario assertions and evaluator compatibility.
9. 3D demo scene.
10. Scenario Coverage v8 regression suite.
11. Studio 3D inspection surface.
12. Roadmap/#1 governance refresh after the 3D capability evidence exists.

Each follow-up issue should use the smallest safe PR units with focused tests,
post-merge verification, generated-state audit, and conservative wording audit.
Do not combine unrelated 3D systems when independent verification would be
clearer.

## Trusted boundary

- Rust/local code owns trusted validation, persistence, source-like fixture
  validation, generated evidence artifact writing, project/run binding, and CLI
  behavior.
- Browser runtime code may execute local demo logic and browser-local probes, but
  it does not gain trusted filesystem persistence, shell command execution,
  source mutation authority, or local server bridge authority.
- Dashboard and Studio surfaces display exported evidence and may prepare drafts
  only when explicitly scoped. They are not trusted writers and must not contain
  hidden command bridges, auto-apply, auto-merge, unrestricted source mutation,
  publish, export, or plugin controls.
- Generated 3D screenshots, runs, previews, dashboard data, temp projects,
  browser profiles, and local tool state stay untracked unless a follow-up issue
  explicitly scopes a tiny deterministic source-like fixture.

## 2D compatibility expectations

3D capability work must remain additive and must not regress existing 2D Seeds,
scenes, project manifests, run artifacts, dashboard exports, Studio read models,
or source-like fixtures unless an explicit migration note and compatibility test
is included in the specific follow-up issue.

Every implementation PR after this scope contract should include the narrow 3D
test for the behavior it changes and enough existing 2D verification to prove
backward compatibility for the touched path. Existing runtime/scenario/dashboard
schemas should prefer additive versioned fields or explicit read-model branches
over ambiguous shape changes.

## Verification and closure gates

Every 3D Capability Gate follow-up issue should include:

- live issue checks for the current issue, #1, and #23;
- focused tests/smokes proving the exact 3D capability behavior changed;
- 2D compatibility checks for existing Seeds, scenes, project manifests, runs,
  dashboard exports, Studio read models, and source-like fixtures touched by the
  change;
- generated-state audit showing only ignored local/runtime outputs are present;
- conservative wording audit: no Godot replacement, production-ready, broad 3D
  compatibility, secure-sandbox, native export, plugin runtime, hosted/cloud, or
  autonomous launch claims;
- final latest-main gate before issue closure:

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

3D browser/runtime issues should add relevant `examples/game-runtime/*.test.cjs`
coverage once the first 3D runtime fixtures exist. Dashboard/Studio issues should
add read-only rendering, missing/malformed state, and XSS escaping coverage.

## Non-goals for Milestone 9

Milestone 9 does not authorize:

- a full 3D editor or production 3D renderer;
- broad 3D engine parity, broad GLTF/import pipeline, advanced lighting/PBR, or
  material graph behavior;
- native export, platform packaging, signing, notarization, or release/publish
  automation;
- plugin runtime, plugin marketplace, remote asset hosting, hosted/cloud/server,
  auth/account systems, or collaboration infrastructure;
- browser-side trusted file writes, command bridges, local server bridges, hidden
  command execution, auto-merge, auto-apply, or unrestricted source mutation;
- dependency, CI, workflow, or build-script mutation unless a separate explicit
  governance issue authorizes it;
- current Godot replacement, broad production-ready, shipped-game maturity,
  secure-sandbox, or broad compatibility-stable claims.

## Governance anchors

- #1 remains the roadmap/final-goal anchor and must stay open unless a separate
  explicit governance decision changes it.
- #23 remains the memory/governance anchor and must stay open unless a separate
  explicit governance decision changes it.
- This document may be revised only by an explicit follow-up governance issue or
  roadmap refresh. Implementation issues should cite it rather than weakening its
  boundaries locally.
