# Full Studio Editor v1 Scope and Contract

Issue: #757
Roadmap anchor: #1 (Full Studio Editor milestone).
Status: scope contract only; no executable behavior.

Full Studio Editor v1 is a local-first, evidence-backed Studio authoring cockpit. It integrates project, scene, entity, asset, scenario, evidence, export, and plugin inspection with draft-only authoring and Safe Source Mutation Apply handoff. It does not authorize direct trusted source writes from Studio, full Godot editor parity, a native desktop editor, arbitrary command execution, publish/deploy, executable editor plugins, or marketplace behavior.

This document is the canonical contract for all follow-up Full Studio Editor issues. It adds no trusted source-editing behavior; each follow-up issue implements one bounded slice against the boundaries defined here.

## Bounded target

The milestone covers the following bounded capabilities, each implemented and verified independently:

- Project overview: a read-only overview of trusted project state.
- Scene tree inspector: read-only scene tree inspection.
- Entity/component inspector: read-only entity and component inspection.
- Draft authoring model: draft-only operations that never write trusted source directly.
- Safe Source Apply handoff: handoff of drafts to the Safe Source Mutation Apply review gates.
- Visual scene canvas: a read-only/draft-only canvas surface.
- Asset browser: read-only asset inspection.
- Scenario/playtest panel: read-only scenario and playtest evidence panel.
- Evidence timeline/comparison: read-only evidence timeline and comparison.
- Export/package panel: read-only export/package inspection.
- Plugin panel integration: read-only plugin inspection (declarative descriptors only).
- Workspace layout persistence: draft-only workspace UI state.
- Command palette: navigation/inspection commands only, no trusted mutation.
- Accessibility/keyboard navigation, performance budget, diagnostics, integrated demo, regression suite, and roadmap governance refresh.

## Trusted boundary

- Studio may inspect trusted project state and create draft operations, but it must not directly write trusted source files.
- Trusted source mutation must go through Safe Source Mutation Apply review gates: validated preview, sandbox evidence, accepted independent review, stale-target checks, rollback metadata, allowlisted verification, and post-apply comparison.
- There is no browser trusted write, command bridge, arbitrary shell execution, dependency install, CI/workflow mutation, publish/deploy/signing/upload, executable plugin runtime, marketplace behavior, network install/update, or credentialed operation.
- Rust/local validation owns trusted validation, persistence, source-apply handoff, evidence writing, trusted file boundaries, and CLI contracts.
- TypeScript/JavaScript owns Studio UI rendering, draft interaction, read-only inspection, and draft-only browser state where explicitly scoped. Project/plugin/evidence data is rendered as inert data, never as executable code.

## Generated-state policy

Generated workspace state, drafts, previews, panel data, demo outputs, validation reports, evidence artifacts, temp servers, and local tool state remain ignored unless explicitly fixture-scoped. Each follow-up PR includes a generated-state audit (`git status --short --ignored`).

## Dependency order for follow-up issues

1. This scope and contract issue (#757) lands first.
2. Project overview, scene tree inspector, and entity/component inspector land as read-only foundations.
3. The draft authoring model and Safe Source Apply handoff build on the inspectors.
4. The visual scene canvas, asset browser, scenario/playtest panel, evidence timeline/comparison, export/package panel, and plugin panel integrate the surfaces above.
5. Workspace layout persistence, the command palette, accessibility/keyboard navigation, the performance budget, and diagnostics harden the cockpit.
6. The integrated demo and regression suite follow.
7. A roadmap and #1 governance refresh closes the milestone.

Each follow-up issue must verify its slice independently and must not combine overview, scene tree, inspector, draft model, source-apply handoff, canvas, assets, evidence, export, plugin, workspace, command palette, accessibility, performance, diagnostics, demo, and regression behavior into a single PR when they can be verified separately.

## Verification and closure gates

Every follow-up PR must pass the standard repository gates (`cargo fmt --check`, `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, the dashboard/cockpit node smokes, `git diff --check`, and a clean `git status --short --ignored`) and must add focused tests/smokes for the exact Studio editor behavior it implements. Closure evidence must include generated-state, no-direct-trusted-write, no-command-execution, no-publish/deploy, Safe Source Apply handoff, evidence, conservative-wording, and #1/#23 governance audits.

## Explicit non-goals

- No direct trusted source writes from Studio.
- No autonomous apply, auto-merge, self-approval, reviewer bypass, or hidden trusted writes.
- No browser command bridge, arbitrary shell execution, dependency install, CI/workflow mutation, credentialed operation, network install/update, publish/deploy/sign/upload, or release automation.
- No executable plugin runtime, marketplace, native desktop editor, advanced visual scripting, full asset import pipeline, full Godot editor parity, or production-ready collaborative editor.
- No generated workspace/editor/evidence artifacts committed unless explicitly fixture-scoped.
- No unrelated native/mobile/store export, plugin marketplace, executable editor tools, or Godot-plus demo implementation.

## Governance anchors

#1 remains the broad roadmap/final-goal anchor. #23 remains the repo-memory/design context anchor. Both remain open through this contract issue; this milestone does not close or modify either without a separate explicit governance decision.
