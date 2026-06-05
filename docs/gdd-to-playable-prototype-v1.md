# GDD to Playable Prototype v1

Issue: #644  
Milestone: #1 Milestone 12 — Game Design Document to Playable Prototype  
Status: implementation/demo/coverage evidence complete through #660; #661 records the roadmap and #1 governance refresh

## Purpose

GDD to Playable Prototype v1 defines how a bounded game design brief can become a validated, reviewable, evidence-backed playable prototype. The v1 target is a small scoped 2D prototype first. Optional 3D compatibility is limited to prior 3D gate surfaces and must not imply a full 3D editor, production renderer, native export, or Godot replacement.

This document is a control contract for #644-#661. It does not add product behavior, GDD parsing, generation, prototype apply, Studio UI, command bridges, source/script mutation, asset generation, or runtime plugin behavior.

## Completion evidence

GDD to Playable Prototype v1 is complete as a bounded evidence-gated prototype
path after the merged issue/PR chain #644-#660. The completed slices added the
scope contract, GDD/design-brief schema, requirement extraction, mechanics/core
loop mapping, feasibility gate, scaffold plan, scene/level plan, behavior plan,
asset placeholder/reference plan, scenario/acceptance plan, prototype task graph,
draft bundle, review-gated apply contract, run/evidence/journal bundle,
read-only Studio planning inspection, deterministic demo manifest, and Scenario
Coverage v11 regression matrix.

The milestone remains intentionally narrow: generated GDD-derived output is
untrusted until Rust/local validation and review-gated apply; browser/dashboard/
Studio surfaces remain read-only or draft-only; generated prototype plans,
drafts, reviews, applies, runs, evidence, screenshots, dashboard exports, temp
projects, and local tool state stay ignored unless explicitly fixture-scoped.
Completion does not authorize autonomous unrestricted game creation, arbitrary
source/script mutation, browser trusted writes, command bridges, auto-apply,
auto-merge, uncontrolled/generated proprietary assets, production-game or
production-ready claims, current Godot replacement claims, native export, plugin
runtime, hosted/cloud behavior, public launch, or release automation.

## Bounded v1 target

The v1 prototype path is intentionally narrow:

1. A human-authored or fixture-scoped GDD/design brief states the target audience, core loop, win/loss goals, input model, constraints, and acceptance criteria.
2. Local validation turns the GDD into explicit requirements and rejects unsupported, ambiguous, unsafe, or untraceable scope.
3. Follow-up artifacts map requirements to mechanics, scene/level plans, behavior plans, placeholder assets, scenarios, and task graph entries.
4. Agent-authored prototype drafts remain untrusted proposals until Rust/local validation, independent review, rollback metadata, and review-gated apply accept them.
5. Prototype run evidence and journal/dashboard/Studio read models inspect the result without giving browser code trusted write or command authority.

The intended demo scale is a deterministic local fixture/prototype that can be reproduced in CI and reviewed from evidence. It is not autonomous unrestricted game creation.

## Dependency order for follow-up issues

Follow-up work should proceed in this order unless a later governance issue records a safer reordering:

| Issue | Slice | Purpose |
| --- | --- | --- |
| #644 | Scope and Contract | This milestone contract, dependency order, trusted boundary, asset boundary, verification gates, and non-goals. |
| #645 | Game Design Brief Schema v1 | Typed GDD/brief schema and fixture examples. |
| #646 | Design Requirement Extraction v1 | Extract traceable requirements from the brief without inventing unbounded scope. |
| #647 | Mechanics and Core Loop Mapping v1 | Map requirements to explicit mechanics, inputs, objectives, and loop states. |
| #648 | Prototype Scope and Feasibility Gate v1 | Decide ready/blocked/partial feasibility before scaffold or draft work. |
| #649 | Project Scaffold from GDD v1 | Generate/review a local project scaffold plan, not arbitrary source mutation. |
| #650 | Scene and Level Plan from GDD v1 | Produce bounded scene/level plans that reuse existing scene and level contracts. |
| #651 | Gameplay Behavior Plan from GDD v1 | Map behavior needs to structured gameplay behavior contracts, not executable scripts. |
| #652 | Asset Placeholder and Reference Plan v1 | Declare placeholders, local fixtures, manifest refs, and license/source notes. |
| #653 | Scenario and Acceptance Criteria Generation v1 | Generate scenario/acceptance artifacts tied to requirements and expected evidence. |
| #654 | Prototype Implementation Task Graph v1 | Split implementation into reviewable tasks with ownership and verification evidence. |
| #655 | Agent-Generated Prototype Draft Bundle v1 | Collect untrusted drafts with provenance, generated-state policy, and blocked states. |
| #656 | Review-Gated Prototype Apply v1 | Apply only after accepted review, rollback metadata, stale checks, and local validation. |
| #657 | Prototype Run, Evidence, and Journal Bundle v1 | Capture run/evidence/journal output without source or browser authority drift. |
| #658 | Studio Prototype Planning Inspection Surface v1 | Read-only/draft-only Studio inspection for plans, drafts, reviews, and evidence. |
| #659 | GDD-to-Prototype Demo v1 | Deterministic local demo composed from accepted prior slices. |
| #660 | Scenario Coverage v11: GDD-to-Prototype Regression Suite | Regression coverage for valid, missing, malformed, stale, unsupported, blocked, generated-state, and wording cases. |
| #661 | Roadmap and #1 Governance Refresh | Evidence-based roadmap/#1 handoff after the milestone is complete. |

## Artifact separation

GDD-to-prototype artifacts must remain separate so review can identify drift:

- GDD/design brief;
- extracted requirements;
- mechanics/core-loop mapping;
- feasibility gate;
- scaffold plan;
- scene and level plan;
- gameplay behavior plan;
- asset placeholder/reference plan;
- scenarios and acceptance criteria;
- implementation task graph;
- prototype draft bundle;
- review and apply records;
- run evidence and journal bundle;
- dashboard/Studio read models.

A later issue may define schemas for these artifacts, but #644 only names the boundaries and dependency order.

## Trusted boundary

GDD-derived output remains untrusted until Rust/local validation and review-gated apply. Rust/local code owns trusted validation, persistence, prototype draft/apply validation, evidence artifact writing, run/project binding, and CLI behavior.

Browser, dashboard, and Studio surfaces are read-only or draft-only for trusted state unless an explicitly scoped Rust/local trusted API owns persistence. They must not install dependencies, execute commands, mutate source, write trusted files, run hidden local servers, accept their own drafts, merge changes, or publish/deploy output.

## Asset boundary

V1 may use placeholders, local fixtures, or manifest references only. Every asset reference must record enough source/license context for review when it becomes fixture-scoped. Uncontrolled asset generation is out of scope, and generated copyrighted/proprietary assets must not be committed or claimed.

Generated prototype drafts, plans, reviews, applies, runs, evidence, screenshots, dashboard exports, temporary projects, and local tool state remain ignored unless explicitly fixture-scoped by a later issue.

## Verification and closure gates

Every follow-up issue must recheck its own issue plus #1 and #23 before work, before merge, and before closure. Required local verification remains:

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

Issue-specific evidence must include generated-state, asset-license/source, no-autonomous-game-generation, wording, compatibility, and #1/#23 governance audits.

## Explicit non-goals

GDD to Playable Prototype v1 does not authorize:

- autonomous unrestricted game creation;
- arbitrary source mutation, arbitrary script execution, visual scripting implementation, dynamic code loading, plugin loading, or runtime plugin behavior;
- browser trusted writes, command bridge, local server bridge, hidden command execution, auto-apply, auto-merge, or self-approval;
- uncontrolled asset generation or generated copyrighted/proprietary assets;
- production game, shipped-game, commercial readiness, broad compatibility, secure sandbox, current Godot replacement, or production-ready engine claims;
- native export, platform packaging, marketplace, hosted/cloud/server/auth/account behavior, public launch, or release automation;
- changes to #1 or #23 closure state.

## Governance anchors

#1 remains the broad vision and implementation-roadmap anchor. #23 remains the repo-memory/design context anchor. Both must stay open unless a separate explicit governance decision exists.
