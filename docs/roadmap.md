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
and read-only Studio inspection:

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> proposal/review/application -> regression promotion/matrix -> loop plan/dry-run/step/handoff -> expressive scene/demo regression -> asset manifest/loading/preview/regression -> visual edit draft/preflight/diff/review evidence -> Production 2D runtime/replay/profiling/regression -> journal/Studio inspection
```

The trusted boundary remains Rust and the local filesystem. Browser examples
read exported JSON and show copyable commands; they do not execute commands,
write trusted files, accept mutations, or act as a production editor. Browser
and CDP observations are evidence inputs, not trusted authority.

## Completed evidence-native milestones

The current implementation has completed these documented milestone surfaces:

- Runtime v1 (`docs/runtime-v1.md`, `docs/runtime-v1-demo.md`)
- Scenario/Evaluator v1 (`docs/scenario-evaluator-v1.md`,
  `docs/scenario-evaluator-v1-demo.md`)
- Evolve Loop v1 (`docs/evolve-loop-v1.md`, `docs/evolve-loop-v1-demo.md`)
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
The conservative next technical milestone candidate is the 3D Capability Gate v1
sequence (#596-#608) because it is the next live dependency-ordered branch and
is framed as a capability gate rather than a broad engine-parity promise. That
candidate must remain issue-scoped and evidence-backed; it is not a production,
public-launch, native-export, plugin-runtime, source-apply, broad 3D parity, or
Godot replacement claim.

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

1. **3D Capability Gate v1 (#596-#608)** — the next dependency-ordered technical
   branch, starting with the #596 scope/contract in
   [`docs/3d-capability-gate-v1.md`](3d-capability-gate-v1.md) and then scene
   graph, camera, mesh/material local references, render smoke evidence, physics,
   animation, runtime probe, scenario compatibility, demo, Studio inspection,
   regression, and roadmap refresh issues. Keep it a scoped capability gate, not
   a broad 3D parity, production-engine, native-export, plugin-runtime, or Godot
   replacement claim.
2. **Gameplay Scripting / Logic System v1 (#611-#625)** — a later candidate for
   bounded behavior models and safe script/logic boundaries after the 3D gate or
   by explicit maintainer resequencing.
3. **Agentic Scene and Level Designer v1 (#627-#642)** or **GDD-to-Playable
   Prototype v1 (#644-#661)** — later agentic authoring candidates that require
   their own scoped contracts, review gates, generated-state audits, and
   conservative wording. The Agentic Scene and Level Designer contract starts in
   [`docs/agentic-scene-level-designer-v1.md`](agentic-scene-level-designer-v1.md).
4. **Manual public visibility review** — only if maintainers separately choose
   it, using the launch checklist, visibility decision records, communication
   pack, hold/rollback criteria, issue/PR intake policy, security-response
   routing, and fresh verification on the intended decision date.
5. **Deferral/hold** — if any launch-governance blocker or generated-state,
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
