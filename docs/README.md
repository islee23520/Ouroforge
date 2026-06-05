# Ouroforge documentation index

This index is the public-alpha navigation layer for the `docs/` directory. It is
not a replacement for the milestone contracts; it points readers to the smallest
useful starting set, then preserves deeper milestone documents as references.

Ouroforge is a local-first, pre-release MVP. These docs do not authorize a public
launch, release publication, repository visibility change, source apply,
browser trusted writes, command bridges, secure sandboxing, or production support
commitment.

## Start here

| Reader goal | Start with | Why |
| --- | --- | --- |
| Understand the project quickly | [`../README.md`](../README.md) | Public-alpha overview, quickstart, safety model, non-goals, and repository map. |
| Understand the loop architecture | [`architecture.md`](architecture.md) | Explains Seed → Run → Evidence → Evaluation → Journal → Mutation. |
| See what is complete and what is next | [`roadmap.md`](roadmap.md) | Canonical roadmap and completed milestone references. |
| Run or verify the public-alpha demo | [`fresh-clone-onboarding-command-audit-v1.md`](fresh-clone-onboarding-command-audit-v1.md), [`fresh-clone-smoke-v1.md`](fresh-clone-smoke-v1.md), [`fresh-clone-troubleshooting-cleanup-v1.md`](fresh-clone-troubleshooting-cleanup-v1.md), [`public-demo-evidence.md`](public-demo-evidence.md), [`public-demo-smoke-evidence-policy-v1.md`](public-demo-smoke-evidence-policy-v1.md) | Reproducible local demo evidence and generated-state expectations. |
| Check safety/trust boundaries | [`evidence-fidelity-trust-boundary-v1.md`](evidence-fidelity-trust-boundary-v1.md), [`public-alpha-security-trust-boundary-v1.md`](public-alpha-security-trust-boundary-v1.md), [`public-alpha-disclosure-and-sandbox-limitations-v1.md`](public-alpha-disclosure-and-sandbox-limitations-v1.md) | Browser read-only, generated-state, disclosure, sandbox/dry-run, and no-source-apply boundaries. |
| Prepare or review a contribution | [`../CONTRIBUTING.md`](../CONTRIBUTING.md), [`artifact-write-policy-v1.md`](artifact-write-policy-v1.md), [`public-wording-guardrail-v1.md`](public-wording-guardrail-v1.md) | Review workflow, trusted-write categories, generated-state rules, and conservative wording checks. |

## Safety/trust boundaries

See the safety/trust rows in [Start here](#start-here), plus the source-preview
and sandbox boundary section below.

## Public-alpha readiness and governance

Use these when reviewing public-readiness and launch-governance changes. They are governance/evidence
references, not launch approval. As of the #387 roadmap refresh, the launch-governance
sequence records manual decision inputs and conservative communication material;
repository visibility, publication, releases, and support commitments remain
separate maintainer actions:

- [`public-alpha-readiness-v1.md`](public-alpha-readiness-v1.md) — Public Alpha Readiness v1 scope, dependency order, boundaries, and closure gates.
- [`public-alpha-readiness-gate-v1.md`](public-alpha-readiness-gate-v1.md) — public-alpha gate checklist and failure modes.
- [`public-readiness-audit.md`](public-readiness-audit.md) — readiness audit evidence.
- [`public-launch-checklist.md`](public-launch-checklist.md) — manual launch/visibility checklist; it does not change repository visibility.
- [`public-alpha-launch-governance-v1.md`](public-alpha-launch-governance-v1.md) — launch governance and decision boundaries.
- [`public-alpha-launch-governance-final-handoff-v1.md`](public-alpha-launch-governance-final-handoff-v1.md) — final roadmap/#1 handoff source, manual hold outcome, protected anchors, and next candidate audit.
- [`public-alpha-launch-hold-criteria-v1.md`](public-alpha-launch-hold-criteria-v1.md) — manual hold criteria for launch blockers, evidence, owner, and cadence.
- [`public-alpha-launch-response-rollback-v1.md`](public-alpha-launch-response-rollback-v1.md) — manual response and rollback options after launch-governance blockers.
- [`public-pr-intake-policy-v1.md`](public-pr-intake-policy-v1.md) — public-alpha PR checklist, forbidden-scope gates, dependency/Lore expectations, and merge-readiness criteria.
- [`public-alpha-communication-pack-v1.md`](public-alpha-communication-pack-v1.md) — conservative public-alpha summary, demo/reporting pointers, non-goals, and forbidden-overclaim checklist.
- Public Alpha Launch Governance v1 is complete as governance/docs only; it did
  not publish an announcement, change repository visibility, release packages,
  automate launch/rollback, add product behavior, or replace #1/#23.
- [`public-alpha-readiness-governance-handoff-v1.md`](public-alpha-readiness-governance-handoff-v1.md) — handoff evidence for later governance.
- [`public-alpha-readiness-final-audit-v1.md`](public-alpha-readiness-final-audit-v1.md) and [`public-alpha-readiness-final-report-v1.md`](public-alpha-readiness-final-report-v1.md) — final public-alpha readiness evidence.
- [`release-versioning-policy-v1.md`](release-versioning-policy-v1.md) and [`release-artifact-policy-v1.md`](release-artifact-policy-v1.md) — versioning and release artifact policy without publication automation.

## Core loop and evidence contracts

Read these for the foundational local loop and generated artifact contracts:

- [`architecture.md`](architecture.md)
- [`runtime-v1.md`](runtime-v1.md) and [`runtime-v1-demo.md`](runtime-v1-demo.md)
- [`runtime-probe-contract-v2.md`](runtime-probe-contract-v2.md) and [`3d-runtime-probe-contract-v1.md`](3d-runtime-probe-contract-v1.md)
- [`scenario-evaluator-v1.md`](scenario-evaluator-v1.md) and [`scenario-evaluator-v1-demo.md`](scenario-evaluator-v1-demo.md)
- [`evidence-backed-journal-v2.md`](evidence-backed-journal-v2.md)
- [`mutation-proposal-quality-v2.md`](mutation-proposal-quality-v2.md)
- [`run-comparison-v2.md`](run-comparison-v2.md)
- [`regression-run-matrix-v1.md`](regression-run-matrix-v1.md)
- [`reproducible-run-command-context-v1.md`](reproducible-run-command-context-v1.md)

## Project, scene, asset, and gameplay contracts

Use these for source-like models and fixture-backed engine capability references:

- [`project-manifest-v1.md`](project-manifest-v1.md), [`project-scaffold-v1.md`](project-scaffold-v1.md), [`project-run-v1.md`](project-run-v1.md), [`project-workspace-loop-v1.md`](project-workspace-loop-v1.md), [`project-mutation-loop-v1.md`](project-mutation-loop-v1.md)
- [`scene-component-model-v2.md`](scene-component-model-v2.md), [`scene-edit-transactions.md`](scene-edit-transactions.md), [`scene-only-mutation-v2.md`](scene-only-mutation-v2.md), [`scene-transitions-v1.md`](scene-transitions-v1.md)
- [`asset-manifest-v1.md`](asset-manifest-v1.md), [`asset-pipeline-v1.md`](asset-pipeline-v1.md), [`asset-preview-evidence-v1.md`](asset-preview-evidence-v1.md), [`asset-reference-integrity-v1.md`](asset-reference-integrity-v1.md), [`sprite-atlas-manifest-v1.md`](sprite-atlas-manifest-v1.md), [`tileset-tilemap-authoring-v2.md`](tileset-tilemap-authoring-v2.md)
- [`collision-physics-v2.md`](collision-physics-v2.md), [`gameplay-trigger-flags-v1.md`](gameplay-trigger-flags-v1.md), [`engine-expansion-v1.md`](engine-expansion-v1.md), [`engine-expressiveness-v2.md`](engine-expressiveness-v2.md)
- [`gameplay-scripting-logic-system-v1.md`](gameplay-scripting-logic-system-v1.md) — Milestone 10 scope contract for structured gameplay behavior, event/signal, state-machine, ability/action, draft/apply, evidence, and Studio inspection work; no arbitrary script execution, plugin loader, command bridge, or production-stable scripting API claim.
- [`gdd-to-playable-prototype-v1.md`](gdd-to-playable-prototype-v1.md) — Milestone 12 scope contract for bounded GDD-to-prototype planning, artifact separation, review-gated apply, asset/source boundaries, and evidence/Studio inspection; no autonomous unrestricted game creation, uncontrolled asset generation, command bridge, or Godot replacement claim.
- [`gdd-design-brief-v1.md`](gdd-design-brief-v1.md) — structured GDD/design brief schema and fixtures for bounded input validation before prototype generation; no generation authority, command bridge, source/script mutation, uncontrolled assets, or engine replacement claim.
- [`scenario-coverage-v16-plugin-extension.md`](scenario-coverage-v16-plugin-extension.md) — Plugin / Extension System v1 regression scenario matrix for valid declarative descriptors and blocked unsafe plugin drift; no executable plugins, network install/update, command execution, publish/deploy, marketplace, or production-ready plugin ecosystem claim.
- [`gameplay-scripting-logic-system-governance-handoff.md`](gameplay-scripting-logic-system-governance-handoff.md) — Governance handoff for completed #611-#625 structured gameplay logic evidence, generated-state and no-arbitrary-script audits, and the next GDD-to-Playable Prototype recommendation.
- [`gameplay-behavior-model-v1.md`](gameplay-behavior-model-v1.md) — data-first behavior artifact schema for patrol, collect, contact damage, door, win-condition, timed hazard, and ability-trigger examples; no executable scripts, plugin loaders, or command bridges.
- [`gameplay-event-signal-system-v1.md`](gameplay-event-signal-system-v1.md) — deterministic event/signal artifact schema for collision/contact, trigger, item, flag, timer, input, scene, state, and behavior events; no executable scripts, plugin loaders, or command bridges.
- [`gameplay-state-machine-v1.md`](gameplay-state-machine-v1.md) — structured state-machine artifact schema for player dash readiness, guard alert, door, hazard, and progression states; no executable scripts, plugin loaders, or command bridges.
- [`gameplay-ability-action-v1.md`](gameplay-ability-action-v1.md) — structured ability/action artifact schema for player dash, enemy alert, door, hazard, and win-state actions with cooldown/cost/runtime status validation; no executable scripts, plugin loaders, or command bridges.
- [`gameplay-state-ability-evidence-compatibility-v1.md`](gameplay-state-ability-evidence-compatibility-v1.md) — read-only state-machine and ability/action evidence/read-model compatibility for scenario, dashboard, Studio, and probe consumers; no runtime dispatch, executable scripts, plugin loaders, or command bridges.
- [`script-module-interface-design-gate-v1.md`](script-module-interface-design-gate-v1.md) — design-only future script module interface gate defining allowed metadata/capabilities, forbidden APIs, review/sandbox/evidence requirements, and deterministic expectations; no executable runtime, dynamic import, plugin loader, command bridge, or browser trusted writes.
- [`safe-script-sandbox-trust-boundary-v1.md`](safe-script-sandbox-trust-boundary-v1.md) — design-only sandbox/trust-boundary policy for future script proposals, covering allowed/blocked operations, deterministic limits, failure evidence, restrictions, review/rollback/dry-run gates, and generated evidence path; no runtime execution, command bridge, plugin loader, dynamic import, or browser trusted writes.
- [`3d-capability-gate-v1.md`](3d-capability-gate-v1.md), [`3d-scene-graph-v1.md`](3d-scene-graph-v1.md), [`3d-camera-projection-v1.md`](3d-camera-projection-v1.md), [`3d-mesh-material-refs-v1.md`](3d-mesh-material-refs-v1.md), [`3d-render-smoke-v1.md`](3d-render-smoke-v1.md), [`3d-collision-physics-v1.md`](3d-collision-physics-v1.md), [`3d-animation-playback-v1.md`](3d-animation-playback-v1.md)

## Authoring, Studio, and review surfaces

These docs describe local authoring flows and read-only/review-gated inspection
surfaces. They do not create browser trusted writes or source apply authority:

- [`agentic-loop-orchestration-v1.md`](agentic-loop-orchestration-v1.md), [`authoring-loop-plan-v1.md`](authoring-loop-plan-v1.md), [`authoring-loop-dry-run-v1.md`](authoring-loop-dry-run-v1.md), [`authoring-loop-execution-v1.md`](authoring-loop-execution-v1.md), [`authoring-loop-recovery-v1.md`](authoring-loop-recovery-v1.md), [`authoring-loop-evidence-bundle-v1.md`](authoring-loop-evidence-bundle-v1.md)
- [`agent-handoff-contract-v1.md`](agent-handoff-contract-v1.md), [`agent-role-model-v1.md`](agent-role-model-v1.md), [`review-decision-ledger-v1.md`](review-decision-ledger-v1.md)
- [`studio-v1.md`](studio-v1.md), [`studio-v2-cockpit.md`](studio-v2-cockpit.md), [`studio-v3-project-workspace-cockpit.md`](studio-v3-project-workspace-cockpit.md), [`studio-review-cockpit-v1.md`](studio-review-cockpit-v1.md), [`studio-evidence-fidelity-surfaces.md`](studio-evidence-fidelity-surfaces.md), [`studio-asset-inspector-v1.md`](studio-asset-inspector-v1.md)
- [`studio-behavior-inspection-surface-v1.md`](studio-behavior-inspection-surface-v1.md) — escaped read-only Studio behavior/event/state/ability/draft/review-apply inspection; no arbitrary script execution, command bridge, browser trusted writes, auto-apply, self-approval, plugin runtime, or production-stable scripting API claim.
- [`visual-authoring-v1.md`](visual-authoring-v1.md), [`visual-edit-draft-model-v1.md`](visual-edit-draft-model-v1.md)
- [`studio-3d-inspection-surface-v1.md`](studio-3d-inspection-surface-v1.md) — read-only escaped Studio 3D evidence inspection; no 3D editor, trusted write, command bridge, viewport persistence, production 3D, or Godot replacement claim.
- [`agentic-scene-level-designer-v1.md`](agentic-scene-level-designer-v1.md) — Agentic Scene and Level Designer v1 scope contract for evidence-gated level/scene authoring; no autonomous full game generation, production editor, visual scripting, browser trusted write, command bridge, native export, plugin runtime, hosted/cloud behavior, or Godot replacement claim.
- [`agentic-level-design-demo-v1.md`](agentic-level-design-demo-v1.md) — deterministic Agentic Scene and Level Designer v1 demo chain; no autonomous full game generation, browser trusted write, command bridge, production editor, or Godot replacement claim.
- [`scenario-coverage-v10-agentic-level-design.md`](scenario-coverage-v10-agentic-level-design.md) — Agentic Scene and Level Designer v1 regression coverage matrix for valid, malformed, missing, stale, unsupported, blocked, Studio, and generated-state cases.
- [`agentic-scene-level-designer-governance-handoff.md`](agentic-scene-level-designer-governance-handoff.md) — #642 roadmap/#1 governance handoff after Agentic Scene and Level Designer v1; keeps #1/#23 open and records conservative next-candidate guidance.

## Source preview, sandbox, and apply-boundary references

These documents are easy to misread as source-write authorization. Treat them as
preview, review, sandbox, threat-model, or later-governance references unless a
specific document says a trusted operation is implemented and bounded. Public
alpha still forbids browser command bridges, hidden command execution, and source
apply from browser surfaces.

- [`source-mutation-threat-model-v1.md`](source-mutation-threat-model-v1.md), [`source-apply-threat-model-refresh-v1.md`](source-apply-threat-model-refresh-v1.md)
- [`source-mutation-design-gate-v1.md`](source-mutation-design-gate-v1.md), [`source-mutation-design-gate-governance-handoff.md`](source-mutation-design-gate-governance-handoff.md)
- [`source-mutation-preview-v1.md`](source-mutation-preview-v1.md), [`patch-preview-artifact-v1.md`](patch-preview-artifact-v1.md), [`patch-diff-integrity-v1.md`](patch-diff-integrity-v1.md)
- [`source-mutation-file-classes-v1.md`](source-mutation-file-classes-v1.md), [`source-file-class-validator-v1.md`](source-file-class-validator-v1.md)
- [`source-mutation-sandbox-boundary-v1.md`](source-mutation-sandbox-boundary-v1.md), [`source-patch-sandbox-dry-run-evaluator-v1.md`](source-patch-sandbox-dry-run-evaluator-v1.md), [`source-patch-test-command-allowlist-v1.md`](source-patch-test-command-allowlist-v1.md)
- [`source-patch-review-gate-v1.md`](source-patch-review-gate-v1.md), [`source-patch-stale-target-guard-v1.md`](source-patch-stale-target-guard-v1.md), [`source-patch-apply-transaction-v1.md`](source-patch-apply-transaction-v1.md), [`safe-source-mutation-apply-v1.md`](safe-source-mutation-apply-v1.md)

## Optional/future capability references

These are design or roadmap references, not current public-alpha capability
claims:

- [`production-2d-engine-core-v1.md`](production-2d-engine-core-v1.md)
- [`native-export-design.md`](native-export-design.md)
- [`plugin-system-design.md`](plugin-system-design.md)
- [`distributed-elixir-design.md`](distributed-elixir-design.md)
- [`godot-plus-demo-game-v1.md`](godot-plus-demo-game-v1.md)
- [`godot-plus-demo-design-pillars-v1.md`](godot-plus-demo-design-pillars-v1.md)
- [`post-launch-roadmap-triage-v1.md`](post-launch-roadmap-triage-v1.md) and [`post-launch-roadmap-response-snippets-v1.md`](post-launch-roadmap-response-snippets-v1.md)

## Generated-state and wording audits

Before publishing public-facing doc changes, check:

```bash
grep -RInE "Godot replacement|Godot parity|production-ready|production ready|commercial-release ready|ship-ready|compatibility-stable|stable public engine API|secure sandbox|sandbox guarantee|source apply ready|auto-apply|auto-merge|autonomous repair|browser trusted write|command bridge|local server bridge|native export ready|desktop/mobile export|installer|app-store ready|plugin runtime ready|extension marketplace|third-party code loading|hosted service|cloud runtime|multi-user auth|autonomous launch|public release automation|go-live automation|support SLA|guaranteed support|security guarantee" README.md CONTRIBUTING.md SECURITY.md docs examples .github || true
git status --short --ignored
```

Matches are acceptable only when they are conservative boundary statements,
explicit negations, non-goals, or wording-audit examples. Generated local state
should remain ignored/untracked.

## Documents

- [Fresh Clone Onboarding Command Audit v1](fresh-clone-onboarding-command-audit-v1.md) — PA1.2.1 quickstart command audit, expected generated state, and cleanup boundary notes.
- [Contributor Template Audit v1](contributor-template-audit-v1.md) — PA1.7.3 audit of contribution guidance, PR checklist, issue templates, generated-state boundaries, and conservative wording.
- [Fresh Clone Smoke v1](fresh-clone-smoke-v1.md) — PA1.2.2 isolated fresh-clone-style smoke wrapper, generated-output boundaries, and evidence summary.
- [Fresh Clone Troubleshooting and Cleanup v1](fresh-clone-troubleshooting-cleanup-v1.md) — PA1.2.3 prerequisite checks, common failures, cleanup commands, and generated-state policy.
- [Canonical Demo Script v1](canonical-demo-script-v1.md) — non-destructive local demo command sequence, smoke wrapper, and command audit.
- [Canonical Demo Readiness Evidence v1](canonical-demo-readiness-evidence-v1.md) — PA1.3.3 cleanup, failure-mode, generated-state, and closure evidence.
- [Runtime State Invariant Checker v1](runtime-invariant-checker-v1.md) — QA14.5.1 invariant schema, statuses, fixtures, and guardrails.
