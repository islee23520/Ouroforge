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
inspection, and a completed Engine Expressiveness v2 playable-authoring
surface for richer scene components, deterministic collision/triggers/HUD
state, collect-and-exit demo evidence, regression coverage, and read-only
Studio inspection:

```text
project manifest -> scaffold/scene/seed/scenario pack -> run -> evidence -> compare -> proposal/review/application -> regression promotion/matrix -> loop plan/dry-run/step/handoff -> expressive scene/demo regression -> journal/Studio inspection
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

These milestones are still MVP contracts, not public compatibility promises.
Generated run evidence remains ignored local state unless an issue explicitly
scopes a tiny deterministic fixture as tracked source-like data.

Engine Expressiveness v2 completion covers the implemented local playable demo,
component, collision, trigger, HUD, animation/audio event, manifest-declared
transition, regression, and Studio-inspection surfaces. These are bounded local
evidence contracts, not broad editor/runtime compatibility claims.

Source Mutation Design Gate v1 is complete as a design/control milestone. Its
outcome keeps source mutation apply blocked: the gate produced threat model,
file-class, preview-artifact, review-gate, rollback/audit, sandbox/worktree, and
read-only Studio review designs, but it did not implement source patch
application, arbitrary patch apply, browser command bridges, or source-mutation
readiness. Source Mutation Preview v1 remains scoped as a later preview-only
implementation contract in `docs/source-mutation-preview-v1.md`; it may only
produce inert preview/evidence surfaces unless a separate later governance issue
authorizes more.

## Near-term governance and public-readiness work

This roadmap/#1 governance refresh records the completed Source Mutation Design
Gate v1 control milestone while preserving conservative public wording and
leaving #1/#23 open. The recommendation after the gate is to keep source
mutation implementation blocked, complete Asset Pipeline v1 (#332-#342) and
Visual Authoring v1 (#343-#354) first, then revisit Source Mutation Preview v1
implementation slices (#356-#366) as inert preview/evidence work only.

After #331, the next milestone candidates should stay inside the same
local-first, Rust-trusted, browser-read-only boundary. Recommended sequence is
Asset Pipeline v1 (#332-#342), Visual Authoring v1 (#343-#354), Source Mutation
Preview v1 implementation slices (#356-#366), Public Alpha Readiness
(#367-#377), and Public Alpha Launch Governance (#378-#387), only when each is
backed by fixed PR units, regression coverage, generated-state audits, and
explicit non-goals.

The public-readiness docs remain governance inputs, not automated launch paths:

- `docs/public-readiness-audit.md`
- `docs/public-demo-evidence.md`
- `docs/public-launch-checklist.md`

Maintainers should use those documents for a separate manual repository
visibility decision. Public launch remains a governance action, not an automated
code path.

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
  review approval; the completed design gate is not that authorization.

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
