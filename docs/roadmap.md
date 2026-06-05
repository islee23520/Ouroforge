# Ouroforge Roadmap

## Current status

Ouroforge is a local, evidence-native MVP. It now supports a small project
workspace loop in addition to the original run-centered demo, with hardened
run evidence fidelity, a completed Agentic Review & Regression Promotion v1
loop around proposal rationale, review decisions, review-gated scene
application, rerun comparison, regression promotion, Journal v2, and Studio
review cockpit state, a completed Agentic Loop Orchestration v1 control
layer for data-only plans, dry-run sequencing, CLI-only step execution,
recovery preflight, evidence bundles, agent handoffs, and Studio loop cockpit
inspection, a completed Engine Expressiveness v2 playable-authoring
surface for richer scene components, deterministic collision/triggers/HUD
state, collect-and-exit demo evidence, regression coverage, and read-only
Studio inspection, a completed Asset Pipeline v1 content-authoring foundation
for local asset manifests, atlas/tilemap metadata, reference integrity, runtime
loading evidence, preview/read-model evidence, Studio asset inspection, demo asset
refresh, and asset regression coverage, and a completed Visual Authoring v1 safe
local edit-draft cockpit for source-like drafts, Rust validation/preflight,
transaction previews, visual diff previews, review-gated apply evidence, demo
smoke ids, Scenario Coverage v5, read-only Studio/dashboard wording, and a
completed Production 2D Engine Core v1 milestone for bounded local 2D engine
evidence: render queue/camera/layers/tilemap integration, collision/physics,
input actions/replay, save-load/runtime-state digests, animation/VFX/audio
evidence, frame-budget profiling, a vertical-slice demo, Scenario Coverage v7,
and read-only Studio inspection, and a completed Multi-Agent Production Pipeline
v1 milestone for local evidence-gated collaboration/accountability artifacts:
role models, task boards, ownership, work packages, handoffs, shared state,
review/critic gates, QA queues, performance/regression lanes, decision ledgers,
production evidence bundles, read-only Studio/dashboard/cockpit inspection, a
deterministic demo, and Scenario Coverage v12, and a completed 3D Capability Gate
v1 milestone for bounded local 3D evidence: scene graph/transform validation,
camera/projection contracts, mesh/material local refs, renderer smoke evidence,
collision/trigger evidence, animation/probe/evaluator compatibility, deterministic
3D demo/regression fixtures, normalized dashboard read models, read-only Studio
3D inspection, and a conservative governance refresh, and a completed Agentic
Scene and Level Designer v1 milestone for evidence-gated level/scene authoring:
intent and constraint models, generation plans, layout constraints, tilemap and
terrain drafts, entity/objective/encounter placement drafts, reachability and
objective proof evidence, difficulty/pacing heuristic evidence, visual/semantic
diffs, review-gated level apply records, read-only Studio inspection, a
deterministic demo, Scenario Coverage v10, and a conservative governance
refresh, and a completed Gameplay Scripting / Logic System v1 milestone for
structured data-only gameplay logic: behavior models, deterministic
event/signal contracts, state-machine and ability/action contracts, script and
sandbox design gates, runtime integration, scenario assertions, behavior drafts,
review-gated apply records, evidence/journal integration, escaped read-only
Studio/dashboard inspection, a deterministic demo, Scenario Coverage v9, and a
conservative governance refresh, a completed GDD-to-Playable Prototype v1
milestone for bounded evidence-gated prototype generation from a GDD: design
brief validation, requirement extraction, mechanics/core-loop mapping,
feasibility, scaffold/scene/behavior/asset/scenario plans, prototype task graph,
review-gated draft/apply records, run/evidence/journal bundles, read-only Studio
planning inspection, deterministic demo evidence, Scenario Coverage v11, and a
conservative governance refresh, a completed Evaluator Depth v1 milestone
that adds bounded visual and semantic evaluator gates, additive four-category
verdict/Journals, read-only inspection, deterministic demo evidence, Scenario
Coverage v19, and conservative governance, a completed Evolve Loop Depth v1 milestone
that consumes the four-gate evaluator foundation so failed mechanical, runtime,
visual, and semantic evidence can produce evidence-linked, failure-classified,
bounded data/scene/scenario proposals, compare before/after reruns, surface the
loop read-only in Studio, prove the lifecycle with deterministic demo fixtures,
and lock Scenario Coverage v20, and a completed Build / Export / Packaging v1
local export foundation: export target matrix, export profile, dry-run export
plan, deterministic staging, local web bundle assembly, asset manifest/path
rewriting, runtime probe preservation, checksums/provenance, export verification
evidence, fixture-scoped Scenario Coverage v15, read-only inspection, and
explicit release/publish blockers, and a completed Foundation Hardening v1
architecture hygiene milestone that extracted `ouroforge-ledger`,
`ouroforge-evidence`, and `ouroforge-evaluator` behind the existing
`ouroforge-core` public facade while preserving golden verdict parity, and a
completed Godot-Plus Demo Game v1 milestone for the Signal Gate local vertical
slice: a small playable 2D vertical slice, Studio inspect/draft workflow,
agentic iteration evidence, Safe Source Apply chain evidence, QA swarm evidence
and QA/playtest swarm evidence, local web export/package verification, plugin
descriptors used as metadata, Godot-plus comparison matrix,
documentation/reproducibility notes, performance budget evidence, regression
suite, and conservative governance refresh:

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> proposal/review/application -> regression promotion/matrix -> loop plan/dry-run/step/handoff -> expressive scene/demo regression -> asset manifest/loading/preview/regression -> visual edit draft/preflight/diff/review evidence -> Production 2D runtime/replay/profiling/regression -> multi-agent role/task/handoff/review/QA/regression/evidence bundle -> journal/Studio inspection -> bounded 3D capability evidence/read-only inspection -> evidence-gated agentic scene/level design -> structured gameplay logic -> bounded GDD-to-prototype planning/draft/apply/evidence -> four-gate evaluator depth/evidence -> evidence-linked bounded evolve depth proposals/rerun deltas -> local web export/package evidence -> Foundation Hardening v1 crate seams with golden parity -> Signal Gate Godot-Plus Demo Game v1 evidence/governance
```

The trusted boundary remains Rust and the local filesystem. Browser examples
read exported JSON and show copyable commands; they do not execute commands,
write trusted files, accept mutations, or act as a production editor. Browser
and CDP observations are evidence inputs, not trusted authority.

Evaluator Depth v1 (#1279/#1283-#1288) is now complete as the Milestone 4.1
four-gate evaluator foundation. The merged evidence chain added bounded visual
gate fixtures, declared semantic invariant fixtures, additive four-category
`verdict.json` and Journal summaries, read-only dashboard/Studio inspection, a
deterministic demo, and Scenario Coverage v19. This completion is mechanical and
declared-invariant evaluation only: visual judgment is bounded screenshot diff
evidence rather than aesthetic quality, semantic judgment is over declared
invariants rather than general AI correctness, and no auto-fix/apply/merge,
trusted mutation, command bridge, release-readiness claim, or engine-replacement
claim is introduced.

Evolve Loop Depth v1 (#1290/#1292-#1298) is now complete as the Milestone 5.1
roadmap-alignment milestone. The merged evidence chain added per-gate,
evidence-linked mutation proposal rationale; failure-classification-driven
proposal selection; bounded data/scene/scenario mutation type mapping;
four-gate before/after rerun deltas and Evolve Journal v2; read-only Studio
inspection; deterministic demo fixtures; Scenario Coverage v20; and this
governance refresh. This completion consumes the corrected Evaluator Depth v1
four-gate foundation and #215 scene-only apply path, but it does not authorize
arbitrary source patching, auto-accept, auto-apply, auto-merge, self-approval,
reviewer bypass, command bridges, production-readiness claims, or current Godot
replacement positioning. Confidence remains an evidence-derived bounded signal,
not a correctness or quality guarantee.

Build / Export / Packaging v1 (#720-#736) is now complete as a local-only export
foundation. The merged evidence chain added the export target matrix, export
profile schema, dry-run export plan, deterministic generated-state staging,
local web bundle assembly, asset manifest/path rewriting, runtime probe
preservation, checksum/provenance primitives, package metadata, verification
coverage, read-only inspection contracts, release/publish blockers, and Scenario
Coverage v15. This completion means a bounded local web package can be assembled
and inspected as evidence; it does not mean public release, deployment, signing,
store publishing, desktop/mobile/native export, production distribution,
commercial readiness, secure distribution, multi-platform parity, or Godot
replacement status. Generated package outputs, checksums, verification logs,
screenshots, temp servers, dashboard exports, and local tool state remain
ignored unless a future issue explicitly scopes a source-like fixture.

Foundation Hardening v1 (#1301-#1306) is now complete as Milestone A.H from #1's
Roadmap Alignment Addendum. The merged chain established the golden parity
baseline (#1302), extracted `ouroforge-ledger` (#1303), extracted
`ouroforge-evidence` (#1304), extracted `ouroforge-evaluator` (#1305), and
recorded this governance refresh (#1306). The realized dependency direction is
`ouroforge-ledger <- ouroforge-evidence <- ouroforge-evaluator <-
ouroforge-core <- ouroforge-cli`; `ouroforge-core` preserves public API
compatibility through re-exports and thin facades. `crates/ouroforge-core/src/lib.rs`
was reduced from approximately 89k lines at milestone start to 89,047 lines
after #1305, with the new extracted crates measuring 96 (`ouroforge-ledger`),
130 (`ouroforge-evidence`), and 2,960 (`ouroforge-evaluator`) lines at the
#1306 audit. The `refactor_parity_golden` gate remained byte-identical across
the milestone, and full workspace tests/clippy stayed green. This was mechanical
hygiene only: no feature, behavior, serialization, release, production-readiness,
plugin/runtime, hosted/cloud, native-export, source-apply, or Godot-replacement
claim changed. Mutation, evolve, runtime, behavior, and seed remain in
`ouroforge-core` as explicit A.H2 candidates, not accepted work.

Godot-Plus Demo Game v1 (#778-#797) is now complete as a bounded local workflow
demonstration under #1. The merged evidence chain proved Signal Gate as a small
playable demo game and 2D vertical slice with deterministic gameplay evidence,
read-only or draft-only Studio inspect/draft workflow, agentic iteration
evidence, Safe Source Apply review-gated source mutation chain evidence, QA
swarm evidence and QA/playtest swarm evidence, local web export/package
verification, plugin descriptors used as metadata, a scoped Godot-plus
comparison matrix, documentation/reproducibility notes, performance budget
evidence, regression suite coverage, and this governance refresh. The
remaining gaps stay explicit: full Godot parity, mature editor tooling,
native/mobile export, large game production, real marketplace, executable plugin
ecosystem, production collaboration, commercial release readiness, production
engine maturity, secure sandbox guarantees, hosted/cloud operation, and broad
support promises. This completion does not authorize direct Studio trusted
writes, browser command bridges, arbitrary shell execution, dependency install,
network install/update, credentialed operation, unreviewed source mutation,
auto-apply, auto-merge, self-approval, reviewer bypass, public deployment,
native/mobile/console/store publishing, production-readiness claims, or current
Godot replacement positioning. Generated demo outputs, exports, QA runs,
evidence bundles, screenshots, videos, temp servers, package bundles, and local
tool state remain ignored unless a later issue explicitly scopes deterministic
source-like fixtures. #1 remains open. #23 remains open.

## Completed evidence-native milestones

The current implementation has completed these documented milestone surfaces:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`)
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`)
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`)
- Evolve Loop Depth v1 / evidence-linked proposals and four-gate rerun deltas
  (`docs/evolve-loop-depth-v1.md`,
  `docs/evolve-loop-depth-v1-demo.md`,
  `docs/scenario-coverage-v20.md`,
  `examples/evolve-loop-depth-v1/`)
- Studio v1 (`docs/studio-v1.md`, `docs/studio-v1-demo.md`)
- Engine Expansion v1 (`docs/engine-expansion-v1.md`,
  `docs/engine-expansion-v1-demo.md`)
- Authoring Loop v2 (`docs/authoring-loop-v2.md`,
  `docs/scene-edit-transactions.md`, `docs/run-comparison-v2.md`,
  `docs/scene-only-mutation-v2.md`, `docs/studio-v2-cockpit.md`)
- Project Workspace Loop v1 (`docs/project-workspace-loop-v1.md`,
  `docs/project-manifest-v1.md`, `docs/project-scaffold-v1.md`,
  `docs/scenario-pack-v1.md`, `docs/project-run-v1.md`,
  `docs/project-comparison-v1.md`, `docs/project-mutation-loop-v1.md`,
  `docs/studio-v3-project-workspace-cockpit.md`)
- Evidence Fidelity & Trust Boundary Hardening v1
  (`docs/evidence-fidelity-trust-boundary-v1.md`,
  `docs/runtime-probe-contract-v2.md`,
  `docs/input-replay-evidence-v2.md`,
  `docs/openchrome-cdp-evidence-fidelity-v2.md`,
  `docs/reproducible-run-command-context-v1.md`,
  `docs/studio-evidence-fidelity-surfaces.md`)
- Agentic Review & Regression Promotion v1
  (`docs/agentic-review-regression-promotion-v1.md`,
  `docs/mutation-proposal-quality-v2.md`,
  `docs/review-decision-ledger-v1.md`,
  `docs/project-mutation-loop-v1.md`,
  `docs/regression-run-matrix-v1.md`,
  `docs/evidence-backed-journal-v2.md`,
  `docs/studio-review-cockpit-v1.md`)
- Agentic Loop Orchestration v1
  (`docs/agentic-loop-orchestration-v1.md`,
  `docs/authoring-loop-plan-v1.md`,
  `docs/authoring-loop-dry-run-v1.md`,
  `docs/authoring-loop-execution-v1.md`,
  `docs/authoring-loop-recovery-v1.md`,
  `docs/authoring-loop-evidence-bundle-v1.md`,
  `docs/agent-handoff-contract-v1.md`,
  `examples/authoring-cockpit/README.md`)
- Engine Expressiveness v2 / Playable Game Authoring v1 implemented subset
  (`docs/engine-expressiveness-v2.md`,
  `docs/engine-expressiveness-v2-governance-handoff.md`,
  `docs/scene-component-model-v2.md`,
  `docs/collision-physics-v2.md`,
  `docs/gameplay-trigger-flags-v1.md`,
  `docs/scene-transitions-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/scenario-coverage-v3.md`,
  `docs/studio-authoring-surface-v2-expressive-inspection.md`)
- Asset Pipeline v1 / Content Authoring Foundation
  (`docs/asset-pipeline-v1.md`,
  `docs/asset-manifest-v1.md`,
  `docs/sprite-atlas-manifest-v1.md`,
  `docs/tileset-tilemap-authoring-v2.md`,
  `docs/asset-reference-integrity-v1.md`,
  `docs/runtime-asset-loading-evidence-v1.md`,
  `docs/asset-preview-evidence-v1.md`,
  `docs/studio-asset-inspector-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/scenario-coverage-v4-asset-pipeline.md`,
  `docs/asset-pipeline-v1-governance-handoff.md`)
- Visual Authoring v1 / Safe Local Edit Cockpit
  (`docs/visual-authoring-v1.md`,
  `docs/visual-edit-draft-model-v1.md`,
  `docs/edit-draft-transaction-cli-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/visual-authoring-v1-governance-handoff.md`)
- Production 2D Engine Core v1 / bounded local 2D vertical-slice core
  (`docs/production-2d-engine-core-v1.md`,
  `docs/runtime-state-save-v1.md`,
  `docs/runtime-frame-budget-v1.md`,
  `docs/playable-demo-v2-collect-and-exit.md`,
  `docs/scenario-coverage-v3.md`,
  `docs/production-2d-studio-inspection-v1.md`)
- Multi-Agent Production Pipeline v1 / evidence-gated local collaboration
  (`docs/multi-agent-production-pipeline-v1.md`,
  `docs/agent-role-model-v1.md`,
  `docs/file-artifact-ownership-conflict-policy-v1.md`,
  `docs/agent-work-package-v1.md`,
  `docs/agent-handoff-v2.md`,
  `docs/agent-shared-state-snapshot-v1.md`,
  `docs/review-critic-gate-v1.md`,
  `docs/qa-agent-work-queue-v1.md`,
  `docs/performance-regression-lane-v1.md`,
  `docs/production-evidence-bundle-v1.md`,
  `docs/studio-multi-agent-pipeline-inspection-v1.md`,
  `docs/multi-agent-prototype-production-demo-v1.md`,
  `docs/multi-agent-pipeline-coverage-matrix-v1.md`,
  `docs/multi-agent-production-pipeline-governance-handoff.md`)
- 3D Capability Gate v1 / bounded local 3D capability evidence
  (`docs/3d-capability-gate-v1.md`,
  `docs/3d-scene-graph-v1.md`,
  `docs/3d-camera-projection-v1.md`,
  `docs/3d-mesh-material-refs-v1.md`,
  `docs/3d-render-smoke-v1.md`,
  `docs/3d-collision-physics-v1.md`,
  `docs/3d-animation-playback-v1.md`,
  `docs/3d-runtime-probe-contract-v1.md`,
  `examples/3d-demo-scene-v1/README.md`,
  `examples/3d-capability-regression-v8/README.md`,
  `docs/studio-3d-inspection-surface-v1.md`)
- Agentic Scene and Level Designer v1 / evidence-gated level authoring
  (`docs/agentic-scene-level-designer-v1.md`,
  `docs/level-intent-v1.md`,
  `docs/scene-generation-plan-v1.md`,
  `docs/spatial-layout-constraint-solver-v1.md`,
  `docs/tilemap-terrain-generation-draft-v1.md`,
  `docs/entity-objective-encounter-placement-draft-v1.md`,
  `docs/reachability-pathing-evidence-v1.md`,
  `docs/objective-completion-proof-v1.md`,
  `docs/difficulty-pacing-heuristic-evidence-v1.md`,
  `docs/level-visual-semantic-diff-v1.md`,
  `docs/agent-generated-level-draft-v1.md`,
  `docs/review-gated-level-apply-v1.md`,
  `docs/studio-level-design-inspection-surface-v1.md`,
  `docs/agentic-level-design-demo-v1.md`,
  `docs/scenario-coverage-v10-agentic-level-design.md`,
  `docs/agentic-scene-level-designer-governance-handoff.md`)
- Gameplay Scripting / Logic System v1 / structured data-only gameplay behavior
  (`docs/gameplay-scripting-logic-system-v1.md`,
  `docs/gameplay-behavior-model-v1.md`,
  `docs/gameplay-event-signal-system-v1.md`,
  `docs/gameplay-state-machine-v1.md`,
  `docs/gameplay-ability-action-v1.md`,
  `docs/script-module-interface-design-gate-v1.md`,
  `docs/safe-script-sandbox-trust-boundary-v1.md`,
  `docs/gameplay-state-ability-evidence-compatibility-v1.md`,
  `docs/behavior-draft-v1.md`,
  `docs/behavior-apply-transaction-v1.md`,
  `docs/studio-behavior-inspection-surface-v1.md`,
  `examples/gameplay-logic-demo-v1/README.md`,
  `examples/gameplay-logic-regression-v9/README.md`)
- Build / Export / Packaging v1 / local export package foundation
  (`docs/build-export-packaging-v1.md`,
  `docs/export-target-matrix-v1.md`,
  `docs/export-staging-policy-v1.md`,
  `docs/scenario-coverage-v15-build-export-packaging.md`,
  `examples/build-export-packaging-regression-v15/coverage-matrix.fixture.json`,
  `examples/export-profile-v1/export-profile.valid.fixture.json`,
  `examples/export-bundle-v1/export-profile.fixture.json`,
  `examples/export-asset-manifest-v1/asset-manifest.valid.fixture.json`,
  `examples/export-probe-v1/`)
- Evolve Loop Depth v1 / evidence-linked bounded mutation loop
  (`docs/evolve-loop-depth-v1.md`,
  `docs/evolve-loop-depth-v1-demo.md`,
  `docs/scenario-coverage-v20.md`,
  `examples/evolve-loop-depth-v1/demo/`,
  `examples/evolve-loop-depth-v1/scenario-coverage-v20/`,
  `examples/evolve-loop-depth-v1/scenario-coverage-v20-evolve-depth.test.cjs`)
- Foundation Hardening v1 / mechanical crate decomposition
  (`docs/foundation-hardening-v1.md`,
  `docs/refactor-parity-golden-baseline-v1.md`,
  `crates/ouroforge-ledger/`,
  `crates/ouroforge-evidence/`,
  `crates/ouroforge-evaluator/`)
- Godot-Plus Demo Game v1 / Signal Gate local workflow demonstration
  (`docs/godot-plus-demo-game-v1.md`,
  `docs/godot-plus-demo-design-pillars-v1.md`,
  `docs/autonomous-qa-playtest-swarm-v1.md`,
  `docs/safe-source-mutation-apply-v1.md`,
  `docs/scenario-coverage-v15-build-export-packaging.md`,
  `docs/scenario-coverage-v16-plugin-extension.md`)

These milestones are still MVP contracts, not public compatibility promises.
Generated run evidence remains ignored local state unless an issue explicitly
scopes a tiny deterministic fixture as tracked source-like data.

Engine Expressiveness v2 completion covers the implemented local playable demo,
component, collision, trigger, HUD, animation/audio event, manifest-declared
transition, regression, and Studio-inspection surfaces. These are bounded local
evidence contracts, not broad editor/runtime compatibility claims.

Asset Pipeline v1 completion covers local source-like asset manifests, sprite
atlas metadata, tileset/tilemap authoring metadata, scene/reference integrity,
runtime loading evidence, preview/read-model evidence, Studio asset inspection,
asset-backed playable-demo refresh, Scenario Coverage v4, and the post-milestone
governance handoff. It remains local-first and Rust-trusted; it does not add
remote asset hosting, browser uploads/writes, marketplace/plugin behavior,
native export, production editor claims, source mutation apply, or public launch
automation.

Visual Authoring v1 completion covers local source-like visual edit draft
fixtures, Rust-owned draft validation/preflight, transaction preview, visual diff
summary evidence, review-gated apply lifecycle evidence, ignored generated demo
smoke ids, Scenario Coverage v5, and read-only Studio/dashboard compatibility
wording. It remains a bounded Safe Local Edit Cockpit milestone: it does not add
browser trusted writes, command bridges, production editor behavior, visual
scripting, source mutation apply, plugin runtime, native export, hosted/cloud
services, public launch automation, or Godot replacement claims.

Production 2D Engine Core v1 completion covers the #581-#593 evidence chain:
scope/contract, renderer architecture, camera/layer/viewport behavior,
sprite/atlas/tilemap rendering, 2D physics/collision, input actions/replay,
runtime state save/load and deterministic replay digests, animation/VFX, audio
runtime evidence, frame-budget profiling, a bounded vertical-slice demo,
Scenario Coverage v7, and read-only Studio inspection. It remains local-first
and Rust-trusted for validation, persistence, generated evidence, source-like
fixture validation, save/load artifacts, and CLI behavior. Browser/dashboard/
Studio surfaces remain read-only or draft-only for trusted state. This completion
does not add 3D implementation, native export, plugin runtime, hosted/cloud
behavior, unrestricted source apply, command bridges, public launch automation,
production-ready status, broad compatibility-stable API promises, shipped-game
claims, secure-sandbox guarantees, or Godot replacement positioning.


Multi-Agent Production Pipeline v1 completion covers the #664-#679 evidence
chain: scope/contract, role model, task board, ownership, work packages,
handoffs, shared state snapshots, review/critic gates, QA queue,
performance/regression lane, build/release design gate, decision ledger,
production evidence bundle, read-only Studio/dashboard/cockpit inspection, demo,
and Scenario Coverage v12. It remains a local-first accountability framework: it
does not add hidden/background agents, unbounded spawning, hosted/cloud
orchestration, browser command bridges, trusted browser writes, auto-apply,
auto-merge, self-approval, reviewer bypass, dependency/CI/workflow mutation,
release automation, production readiness, public-launch approval, or current
Godot replacement positioning.

3D Capability Gate v1 completion covers the #596-#608 evidence chain: scope
contract, scene graph and transform hierarchy, camera/projection, mesh/material
local references, render smoke evidence, collision/trigger evidence, bounded
animation playback, runtime probe contract, scenario/evaluator compatibility, a
small deterministic 3D demo scene, Scenario Coverage v8, normalized dashboard
read models, read-only Studio 3D inspection, and this roadmap/#1 governance
refresh. It remains a bounded local capability gate. It does not add a full 3D
editor, production 3D renderer, broad 3D compatibility promise, broad
GLTF/import pipeline, advanced lighting/PBR/material graph, skeletal authoring,
native export, plugin runtime, hosted/cloud/server/auth behavior, browser
trusted writes, command bridges, unrestricted source apply, public launch
automation, production-ready status, secure-sandbox guarantees, shipped-game
maturity, or current Godot replacement positioning. Generated 3D runs,
screenshots, previews, dashboard exports, temp projects, and local tool outputs
remain ignored generated state unless a later issue explicitly scopes a tiny
deterministic source-like fixture.

Agentic Scene and Level Designer v1 completion covers the #627-#642 evidence
chain: scope/contract, level intent, scene generation plan, spatial layout
constraints, tilemap/terrain draft, entity/objective/encounter placement draft,
reachability/pathing evidence, objective completion proof, difficulty/pacing
heuristic evidence, level visual/semantic diff, agent-generated level draft,
review-gated level apply record, read-only Studio inspection, deterministic demo,
Scenario Coverage v10, and this roadmap/#1 governance refresh. It remains a
bounded local level-authoring milestone: it does not add autonomous unrestricted
game creation, a production editor, a full visual level editor, visual scripting,
browser trusted writes, command bridges, auto-apply, auto-merge, self-approval,
unrestricted source mutation apply, native export, plugin runtime, hosted/cloud
behavior, production-ready status, secure-sandbox guarantees, shipped-game
maturity, or current Godot replacement positioning. Generated level drafts,
previews, screenshots, runs, dashboard exports, temp projects, and local tool
outputs remain ignored generated state unless a later issue explicitly scopes a
deterministic source-like fixture.

After #625, Gameplay Scripting / Logic System v1 (#611-#625) is complete as a
structured data-only gameplay behavior/evidence milestone. It covers behavior
models, deterministic events/signals, state machines, ability/action contracts,
script and sandbox design gates, runtime integration, behavior scenario
assertions, agent-generated behavior drafts, review-gated behavior apply,
evidence/journal integration, Studio behavior inspection, demo evidence,
Scenario Coverage v9, and this roadmap/#1 governance refresh. This completion
stays bounded to local behavior/evidence contracts and is not arbitrary
third-party code loading, plugin runtime, browser command bridge, hosted
execution, production scripting, source-apply authority, public-launch approval,
or Godot replacement scope.


GDD-to-Playable Prototype v1 (#644-#661) is complete after the merged evidence
chain: #644 defined the scope contract, #645 added the design-brief schema, #646
added requirement extraction, #647 added mechanics/core-loop mapping, #648 added
the feasibility gate, #649 added scaffold planning, #650 added scene/level
planning, #651 added behavior planning, #652 added asset placeholder/reference
planning, #653 added scenario/acceptance planning, #654 added the prototype task
graph, #655 added the draft bundle, #656 added review-gated apply, #657 added
run/evidence/journal bundles, #658 added read-only Studio planning inspection,
#659 added the deterministic demo, #660 added Scenario Coverage v11, and #661
records this roadmap/#1 governance refresh. This completion enables a bounded,
evidence-gated GDD-to-prototype path only. It does not authorize autonomous
unrestricted game creation, arbitrary source/script mutation, browser trusted
writes, command bridges, auto-apply, auto-merge, uncontrolled asset generation,
generated proprietary assets, native export, plugin runtime, hosted/cloud
behavior, public launch, production-ready or shipped-game claims, or current
Godot replacement positioning. GDD-derived outputs remain untrusted until
Rust/local validation and review-gated apply; browser/dashboard/Studio surfaces
remain read-only or draft-only for trusted state; generated prototype outputs and
local tool state remain ignored unless explicitly fixture-scoped.

Evaluator Depth v1 (#1279/#1283-#1288) is complete after the merged evidence
chain: #1300 defined the scope contract, #1308 added Visual Evaluator Gate v1,
#1311 added Semantic Evaluator Gate v1, #1318 added four-category verdict and
Journal summaries, #1320 added read-only evaluator-depth inspection surfaces,
#1323 added the deterministic demo, and #1325 added Scenario Coverage v19 plus
the legacy two-gate golden. The resulting verdict changes are additive: legacy
two-gate runs remain byte-compatible when visual/semantic gates are undeclared,
while declared gates expose `mechanical`, `runtime`, `visual`, and `semantic`
categories with evidence-linked reasons. Rust/local validation retains trusted
persistence, gate logic, and verdict serialization; browser/dashboard/Studio
surfaces remain read-only evidence viewers. Known gaps remain deliberately
outside this milestone: no aesthetic visual quality judgment, no general
semantic correctness guarantee, no source mutation authority, no command bridge,
no native export, no plugin runtime, no hosted/cloud behavior, no release
readiness, and no engine-replacement claim.

Evolve Loop Depth v1 (#1290/#1292-#1298) is complete after the merged
evidence chain: #1290 defined the scope contract, #1292 added evidence-linked
mutation proposal rationale, #1293 added failure-classification-driven bounded
selection, #1294 added four-gate before/after rerun comparison plus Journal v2
summaries, #1295 added read-only Studio/evidence dashboard inspection, #1296
added the deterministic fixture-scoped demo, #1297 added Scenario Coverage v20
with legacy evolve v0 compatibility, and #1298 records this governance refresh.
The milestone completes #1 Milestone 5.1 under the disambiguated name **Evolve
Loop Depth v1** rather than the closed #215 scene-only apply milestone it
consumes. Proposals remain manual-review inputs; confidence is an evidence-
derived bounded signal, not a correctness or quality guarantee; mutation types
remain bounded to data, scene, and scenario artifacts; Rust/local validation owns
trusted proposal/comparison persistence and serialization; browser/dashboard/
Studio surfaces remain read-only evidence displays. This completion does not add
arbitrary source patching, auto-accept, auto-apply, auto-merge, self-approval,
reviewer bypass, browser trusted writes, command bridges, native export, plugin
runtime, hosted/cloud behavior, release readiness, production readiness, or a
current Godot replacement claim. Generated runs, proposals, comparisons,
screenshots, and local tool outputs remain ignored unless explicitly fixture-
scoped.

Source Mutation Design Gate v1 is complete as a design/control milestone. Its
outcome keeps source mutation apply blocked: the gate produced threat model,
file-class, preview-artifact, review-gate, rollback/audit, sandbox/worktree, and
read-only Studio review designs, but it did not implement source patch
application, arbitrary patch apply, browser command bridges, or source-mutation
readiness.

Source Mutation Preview v1 is complete as an inert preview/evidence milestone
after #365/#366. Its implementation slices added file-class validation, bounded
diff integrity checks, preview artifacts, stale-target guards, allowlisted
sandbox dry-run evidence, review-decision/evidence bundles, read-only
dashboard/Studio display, and generated-state cleanup/audit coverage. This is
not source apply authorization: source patch application to the trusted
maintainer worktree, merge/rebase automation, dependency/CI mutation, arbitrary
shell/network/install commands, browser command bridges, public launch
automation, native export, plugin runtime, and Godot replacement claims remain
out of scope unless a separate later governance issue authorizes them.

Public Alpha Launch Governance v1 is complete as a governance/documentation
track. It produced manual visibility decision records, hold/rollback criteria,
public issue and PR intake policies, security-response routing, demo-stability
monitoring, a conservative communication pack, and post-launch roadmap triage
references. It did not launch Ouroforge, publish an announcement, change
repository visibility or GitHub settings, release packages, add product
behavior, create support/security SLAs, implement source apply, or replace the
#1/#23 governance anchors.

## Near-term governance and public-readiness work

### Evaluator Depth v1 governance refresh

Evaluator Depth v1 is recorded as **complete for Milestone 4.1**, closing the
four-gate evaluator drift identified in #1's Roadmap Alignment Addendum. The
completion evidence is the merged implementation chain #1300, #1308, #1311,
#1318, #1320, #1323, and #1325, with required local verification and CI evidence
recorded on the linked issues. #1 and #23 remain open governance anchors.

Milestone 5.1 is now complete as **Evolve Loop Depth v1** (#1290/#1292-#1298).
The recommended next technical sequence is to continue **Era C** (#660-#797) on
the corrected four-gate, evidence-linked evolve foundation, with Foundation
Hardening (A.H/A.H2 candidates) treated only as a parallel hygiene track when a
separate scope issue defines no-behavior-change parity gates. Layer-3 expansion
remains unchanged and deferred: distributed orchestration / Elixir per ADR #92,
native export, plugin runtime, and hosted/cloud scope still require separate
scoped issue sequences and governance approval.

### Evolve Loop Depth v1 governance refresh

Evolve Loop Depth v1 is recorded as **complete for Milestone 5.1**, closing the
Roadmap Alignment Addendum item that remained after #215. The completion
evidence is the merged issue/PR chain #1290, #1292, #1293, #1294, #1295, #1296,
#1297, and #1298, with final demo and Scenario Coverage v20 verification recorded
on #1296/#1297 and this governance issue. #1 and #23 remain open governance
anchors.

The recommended next technical sequence is to continue **Era C (#660-#797)** on
the corrected four-gate, evidence-linked evolve foundation. In practical order,
this means QA/playtest swarm and Full Studio Editor/Godot-Plus demo work can
consume bounded proposal confidence, data/scene/scenario mutation type mapping,
and four-gate rerun deltas without weakening review gates. Foundation Hardening
(A.H) remains a parallel hygiene track only; future A.H2 work still needs a
separate scope issue before any extraction. Layer-3 scope remains unchanged and
deferred: distributed orchestration / Elixir per ADR #92, native export, plugin
runtime, and hosted/cloud still require separate design gates and governance
approval.

### Foundation Hardening v1 governance refresh

Foundation Hardening v1 is recorded as **complete for Milestone A.H**, closing
the structural-drift item in #1's Roadmap Alignment Addendum. The completion
evidence is the merged issue/PR chain #1301, #1302, #1303, #1304, #1305, and
#1306, with zero-diff golden parity evidence and final full required gates
recorded on #1306. #1 and #23 remain open governance anchors.

The recommended next hardening direction is a later **A.H2** candidate focused
on one of the still-large `ouroforge-core` clusters: mutation/evolve, runtime,
behavior, or seed. This recommendation is not approval to start that work; an
A.H2 scope issue must define the exact crate seam, acyclic dependency direction,
re-export strategy, golden/parity gates, and no-behavior-change guardrails before
any extraction PR. Layer-3 scope remains unchanged and deferred: distributed
orchestration / Elixir per ADR #92, native export, plugin runtime, and
hosted/cloud still require separate design gates and governance approval.

### Godot-Plus Demo Game v1 governance refresh

Godot-Plus Demo Game v1 is recorded as **complete as a bounded local workflow
demonstration**, closing the Era C (#660-#797) governance chain without closing
#1 or #23. The completion evidence is the merged issue sequence #778-#797:
Signal Gate design and implementation, scenario/regression evidence,
QA swarm evidence, QA/playtest swarm evidence, agentic iteration and Safe
Source Apply chain evidence, read-only or draft-only Studio walkthrough
evidence, local web export/package verification, plugin descriptors used as
metadata, scoped comparison matrix, performance budget evidence,
documentation/reproducibility, and this roadmap refresh.

The recommended next technical sequence is not expanded by this completion.
Future work still needs separate scoped issues for any mature editor tooling,
native/mobile export, executable plugin ecosystem, real marketplace,
large-project production workflow, production collaboration, commercial release
readiness, production engine maturity, distributed orchestration / Elixir per
ADR #92, hosted/cloud behavior, or broader compatibility claim. #1 and #23
remain open governance anchors.

### Public Alpha Readiness handoff

The scope contract for this preparation milestone is [`docs/public-alpha-readiness-v1.md`](public-alpha-readiness-v1.md).

Public Alpha Readiness v1 is recorded as **prepared for manual public-visibility review, not launched**. The handoff lives in [`docs/public-alpha-readiness-governance-handoff-v1.md`](public-alpha-readiness-governance-handoff-v1.md). It summarizes readiness evidence, remaining manual launch boundaries, and conservative next milestone candidates without changing repository visibility, publishing packages, or approving public-launch messaging.

Actual visibility changes remain separate manual maintainer action after the launch checklist and decision-record process are rerun on the intended visibility date.


This roadmap/#1 governance refresh records the completed Visual Authoring v1 /
Safe Local Edit Cockpit milestone while preserving conservative public wording
and leaving #1/#23 open. Visual Authoring v1 (#343-#354) is now complete as a
local-first draft/preview/review evidence milestone; it is not a public-readiness
or production-editor claim.

After #387, Public Alpha Readiness (#367-#377) and Public Alpha Launch
Governance (#378-#387) are recorded as governance/readiness preparation tracks,
not launch execution. They produced readiness evidence, manual visibility
decision inputs, hold/rollback criteria, issue/PR intake policy, security and
demo-stability boundaries, conservative communication material, and this
roadmap/#1 refresh. They did not change repository visibility, publish packages,
announce launch, add support commitments, or implement product behavior.

The current governance outcome is **manual hold / ready for separate maintainer
decision**. Maintainers can either rerun the visibility checklist and decision
record on a chosen date, or continue technical roadmap work while keeping launch
actions manual. Production 2D Engine Core v1 (#581-#593) is now recorded as
complete, with this #594 roadmap/#1 refresh preserving the evidence chain and
conservative wording.
The 3D Capability Gate v1 sequence (#596-#608) and Agentic Scene and Level
Designer v1 sequence (#627-#642) are now recorded as complete after
implementation, demo, regression, Studio inspection, and governance evidence.
Gameplay Scripting / Logic System v1 (#611-#625) is now recorded as complete,
framed as bounded local behavior/evidence contracts rather than arbitrary
scripting, plugin loading, hosted execution, browser command bridges, source
apply, public launch, production readiness, or Godot replacement scope.


Other possible later governance topics remain Native Export Design Gate, Plugin
Design Gate, Source Mutation Apply Design Gate, and Visual Authoring v2. None is
authorized as implementation scope unless a separate fixed issue sequence opens
it with explicit non-goals, regression coverage, generated-state audits, and
manual launch boundaries.

The public-readiness and post-launch triage docs remain governance inputs, not
automated launch or roadmap-acceptance paths:

- `docs/public-readiness-audit.md`
- `docs/public-demo-evidence.md`
- `docs/public-launch-checklist.md`
- `docs/public-alpha-launch-governance-v1.md`
- `docs/public-alpha-communication-pack-v1.md`
- `docs/post-launch-roadmap-triage-v1.md`

Maintainers should use those documents for separate manual repository visibility
and post-launch roadmap decisions. Public launch and future milestone acceptance
remain governance actions, not automated code paths.

### Public Alpha Launch Governance outcome

Public Alpha Launch Governance v1 (#378-#387) is recorded as **governance
complete, launch not executed**. The completed artifacts define how maintainers
may make, hold, communicate, audit, and roll back a future public-alpha decision,
but the repository remains under the same manual visibility, release,
publication, and support boundaries described above.

Recommended next milestone candidates after this governance refresh are:

1. **Autonomous QA / Playtest Swarm v1 (#682-#698) is complete as bounded
   evidence-gated QA/playtest infrastructure**. The completed scope includes
   scenario candidates, adversarial fuzz plans, worker/budget policy, runtime
   invariants, route attempts, visual/performance/error evidence,
   flake/rerun policy, failure classification, backlog, run matrix, evidence
   bundle, Studio inspection, demo, Scenario Coverage v13, generated-state
   audits, and conservative wording. It remains evidence/backlog-only and must
   not become hidden workers, remote/cloud swarm orchestration, autonomous
   repair, browser trusted writes, command bridges, auto-fix, auto-apply,
   auto-merge, self-approval, production QA certification, public-launch
   approval, current Godot replacement, or production-ready claims.
2. **Resolve Safe Source Mutation Apply governance refresh #716 if it remains
   open**, then continue only through fixed scoped issue sequences for Full
   Studio Editor and Godot-Plus Demo. Preserve Rust/local trusted persistence,
   independent review, rollback, stale-target, sandbox, and allowlisted
   verification gates.
3. **Safe Executable Script Implementation Gate** — defer until maintainers open
   a separate governance issue with threat model, test matrix, rollback story,
   review-gate story, generated-state policy, and public wording; #611-#625 do
   not authorize arbitrary executable scripts.
4. **Bounded Production 3D expansion** — defer unless maintainers open a new
   evidence-backed scope contract after the 3D gate. The completed gate is not
   proof of broad 3D engine parity, advanced rendering, import pipelines, native
   export, production readiness, or Godot replacement status.
5. **Manual public visibility review** — only if maintainers separately choose
   it, using the launch checklist, visibility decision records, communication
   pack, hold/rollback criteria, issue/PR intake policy, security-response
   routing, and fresh verification on the intended decision date.
6. **Deferral/hold** — if any launch-governance blocker or generated-state,
   wording, demo-stability, security-reporting, or #1/#23 anchor concern is
   found during a future manual review.

None of these candidates is automatically accepted by this roadmap refresh.
Technical work still requires its own scoped issue/PR sequence; visibility,
announcement, release, package publication, support, and security guarantees
remain separate maintainer actions.

## Product direction

- Keep the evidence-native loop inspectable, file-based, and local-first.
- Use Rust-owned validation for trusted persistence, project resolution, run
  binding, comparison artifacts, and scene-only mutation application.
- Keep browser surfaces static/read-only for trusted state: display exported
  data, preview runtime state, and show copyable CLI commands only; Studio
  source patch review surfaces remain inert evidence displays and never apply,
  merge, write files, or execute commands.
- Expand runtime/scenario coverage only when backed by concrete issues and
  tests; keep each expressive surface tied to its bounded evidence contract and
  do not infer broader production-engine/editor capabilities from completed
  animation/audio or transition slices.
- Keep authoring actions connected to QA evidence, semantic comparison,
  journals, rollback metadata, explicit mutation review, and regression
  promotion/matrix context.
- Treat evidence fidelity and review governance as first-class contracts: every
  run-facing surface should distinguish Rust-trusted artifacts from browser/CDP
  observations, and should expose missing or malformed evidence/review state as
  warnings instead of inferred passes.
- Keep source mutation apply blocked until a separately scoped later milestone
  has an explicit implementation decision, sandbox/evidence enforcement, and
  review approval; the completed design gate, completed Source Mutation Preview
  v1 milestone, and completed Asset Pipeline v1 are not that authorization.
- Treat Source Mutation Preview v1 as complete but preview-only. Its artifacts
  are evidence inputs for review/governance, not authority to apply patches,
  merge branches, execute browser-originated commands, mutate dependencies/CI,
  publish, package native exports, enable plugins, or launch publicly.
- Treat Public Alpha Launch Governance v1 as complete but non-executing. Its
  artifacts are decision/checklist/response inputs, not authority to toggle
  visibility, publish announcements, release packages, accept public roadmap
  scope automatically, promise support/security response times, or close #1/#23.
- Treat Production 2D Engine Core v1 as complete but bounded. Its artifacts are
  local evidence contracts for a small 2D vertical-slice class, not authority to
  claim production readiness, shipped-game maturity, secure sandboxing, broad
  compatibility stability, native export, plugin runtime, hosted/cloud behavior,
  source apply, public launch, or Godot replacement status.
- Treat Agentic Scene and Level Designer v1 as complete but bounded. Its
  artifacts are local evidence contracts for reviewable level/scene authoring,
  not authority to claim autonomous full game generation, production editor/full
  visual editor maturity, visual scripting, browser trusted writes, command
  bridges, unrestricted source apply, native export, plugin runtime, hosted/cloud
  behavior, public launch, production readiness, or Godot replacement status.
- Treat Gameplay Scripting / Logic System v1 as complete but bounded. Its
  artifacts are structured data-only local behavior/evidence contracts, not
  arbitrary script execution, a production-stable scripting API, secure sandbox
  guarantee, plugin runtime, command bridge, browser trusted write path,
  unrestricted source apply, native export, hosted/cloud behavior, public-launch
  automation, production readiness, or Godot replacement status.
- Treat Evaluator Depth v1 as complete but bounded. Its artifacts are
  mechanical visual diff and declared-invariant semantic evaluator contracts,
  not aesthetic scoring, general semantic correctness guarantees, source
  mutation authority, command bridges, browser trusted writes, native export,
  plugin runtime, hosted/cloud behavior, public-launch automation, release
  readiness, or engine-replacement status. Use Milestone 5.1 to consume these
  verdicts for evidence-linked mutation proposals rather than weakening the
  evaluator boundary.
- Treat Evolve Loop Depth v1 as complete but bounded. Its artifacts authorize
  evidence-linked, manual-review proposal inputs, bounded confidence signals,
  data/scene/scenario mutation type mapping, and four-gate rerun delta evidence;
  they do not authorize arbitrary source patching, auto-accept, auto-apply,
  auto-merge, self-approval, reviewer bypass, command bridges, browser trusted
  writes, native export, plugin runtime, hosted/cloud behavior, production
  readiness, or current Godot replacement positioning.
- Treat Autonomous QA / Playtest Swarm v1 as complete but bounded. Its artifacts
  authorize deterministic local QA/playtest evidence, fixture-scoped fuzzing,
  explicit worker/rerun/time/output budgets, failure classification, backlog
  inputs, run matrices, evidence bundles, and read-only Studio/dashboard
  inspection; they do not authorize hidden workers, remote/cloud swarm
  orchestration, unbounded spawning, browser command bridges, browser trusted
  writes, auto-fix, auto-apply, auto-merge, self-approval, production QA
  certification, quality/fun/market/production safety guarantees, current Godot
  replacement positioning, or production-ready claims.
- Treat Build / Export / Packaging v1 as complete but local-only. Its artifacts
  authorize fixture-scoped local web package evidence, deterministic staging,
  profile/plan validation, asset manifests, runtime probe preservation,
  checksums/provenance, verification evidence, and read-only inspection; they do
  not authorize public release, deployment, signing, upload, app-store/Steam/itch
  publishing, desktop/mobile/native export, credentialed release flow,
  production-ready export claims, commercial distribution, secure distribution,
  multi-platform parity, or Godot replacement status.
- Treat Foundation Hardening v1 as complete but mechanical. Its artifacts
  authorize the realized `ouroforge-ledger`, `ouroforge-evidence`, and
  `ouroforge-evaluator` crate seams, re-export facades, and golden-parity audit;
  they do not authorize behavior changes, bug fixes, new user capability, source
  apply, native export, plugin runtime, hosted/cloud behavior, release
  readiness, production claims, or Godot replacement status.
- Treat Godot-Plus Demo Game v1 as complete but scoped. Its artifacts authorize
  claims about the Signal Gate local evidence-native workflow only: a small
  playable 2D vertical slice, Studio inspect/draft workflow, agentic iteration
  evidence, review-gated Safe Source Apply chain evidence, QA swarm evidence,
  QA/playtest swarm evidence, local web export/package verification, plugin
  descriptors used as metadata, scoped comparison evidence, performance budget
  evidence, documentation, and regression coverage. They do not authorize full
  Godot parity, mature editor tooling claims, native/mobile export, large game
  production, real marketplace, executable plugin ecosystem, production
  collaboration, commercial release readiness, production engine maturity,
  public deployment, direct Studio trusted writes, command bridges, unreviewed
  source mutation, auto-apply, auto-merge, self-approval, reviewer bypass,
  secure sandbox guarantees, or current Godot replacement positioning.

## Active anchors

- #1 remains the broad vision and implementation-roadmap anchor until a separate
  explicit governance decision replaces it.
- #23 remains open as the repo-memory/design context anchor.

## Non-goals

Ouroforge is not currently trying to be:

- a Godot replacement;
- a production-ready or compatibility-stable public engine API;
- a hosted/cloud engine;
- a native packaged editor or native export implementation;
- a general marketplace or plugin platform;
- a browser-side trusted file writer or command bridge;
- an autonomous public-launch automation system.

Any shift in those boundaries requires a design issue, explicit maintainer
approval, and evidence that the change belongs in the current roadmap.


## Current roadmap anchors

- Godot-Plus Demo Game v1 scope contract: [`docs/godot-plus-demo-game-v1.md`](godot-plus-demo-game-v1.md).
