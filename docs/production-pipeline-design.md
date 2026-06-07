# Multi-Agent Production Pipeline v1 Scope and Design Gate

Status: **design gate only**. This document is the canonical scope and design
gate for the Multi-Agent Production Pipeline under #1 Era H Milestone 42. It
realizes the role-specialized agent collaboration originally scoped by
Milestone 13, now that the underlying production functions (generation,
review/apply, trust gradient, QA swarm, provenance, asset manifest, evolve
campaigns) exist. It defines a GO/DEFER decision and the role-agent model,
artifact ownership, handoff and conflict-resolution contracts, shared state,
approvals, observability, and reviewer/critic promotion gates. It adds no
executable behavior, no new engine or runtime, and no new orchestration system.
Role agents never perform a direct trusted write.

This milestone refines, and does not replace, the prior Multi-Agent Production
Pipeline v1 evidence chain (#664-#680, documented in
`docs/multi-agent-production-pipeline-v1.md`, `docs/agent-role-model-v1.md`,
`docs/agent-handoff-v2.md`, `docs/file-artifact-ownership-conflict-policy-v1.md`,
`docs/review-critic-gate-v1.md`, and the related contracts). Every contract here
reuses those surfaces plus the Milestone 22 trust gradient
(`docs/trust-gradient-design.md`) and the safe source-apply path
(`docs/safe-source-apply-governance-handoff.md`). It does not introduce a
parallel orchestration engine.

## Decision

**GO** for a bounded, additive, reuse-only contract scope implemented by the
follow-up issue sequence (#1675-#1681). The default posture remains **DEFER**
for anything outside that bounded scope.

GO authorizes only:

- a role-agent model and artifact-ownership contract that reuses existing agent,
  evidence, journal, and review surfaces;
- handoff artifacts and a conflict-resolution policy reusing the existing
  handoff and ownership contracts;
- reviewer/critic promotion gates reusing the existing review/critic gate and
  the Milestone 22 trust gradient;
- a deterministic local demo, a scenario-coverage regression suite, and a
  roadmap/#1 governance refresh.

GO does **not** authorize, and these remain **DEFER**:

- any new orchestration engine, runtime, writer, scheduler, or worker pool;
- any direct trusted write, auto-apply, auto-merge, self-approval, or reviewer
  bypass by any role agent, the producer, or any browser/Studio surface;
- promotion of any unlicensed, uncredited, or unverified-style generated asset,
  audio, or content;
- shipping (native/store export), hosted/cloud execution, real-player
  telemetry, or live-ops, which stay Layer-3 and DEFER per Milestone 26 / #1508;
- distributed/Elixir, which remains NO-GO per ADR #92
  (`docs/distributed-elixir-design.md`);
- any automated quality/fun/taste claim, production-ready claim, or Godot
  replacement/parity claim.

## Role-agent model

Each role is a specialized capability with its own verification gate. A role
agent proposes; it never promotes its own output. Roles map to existing
surfaces:

| Role | Responsibility (proposal scope) | Reused surface |
| --- | --- | --- |
| Designer | design brief and requirement proposals | GDD design-brief / requirement-extraction contracts |
| Gameplay / systems | mechanics, behavior, and scripting proposals | gameplay/mechanics mapping, scripting-logic contracts |
| Level / content | scene/level and content draft proposals | agentic scene/level designer, scene-draft contracts |
| Artist | asset proposals (license/provenance required) | asset manifest + provenance bundle |
| Audio | audio proposals (license/provenance required) | asset manifest + provenance bundle |
| UX | UX/flow proposals | design-brief / scenario-acceptance contracts |
| QA | function-specific verification evidence | QA swarm, run matrix, regression coverage |
| Build | local readiness evidence and blockers | build/release-candidate agent lane design gate (advisory only) |
| Reviewer | independent review evidence | review/critic gate |
| Critic | adversarial critique evidence | review/critic gate |

No role has trusted-write or merge authority. Generated asset/audio/content
proposals are inert until they carry license/provenance metadata and pass the
function-specific QA gate.

## Artifact ownership and shared state

- Each artifact class has a single owning role; ownership is recorded as
  evidence, not enforced by a runtime lock.
- Shared project state is read through the existing shared-state snapshot
  contract; it is never mutated by a role agent directly.
- Ownership conflicts are recorded and surfaced, never silently merged; the
  existing file/artifact ownership-conflict policy is reused.
- Generated runs, exports, temp projects, and bundles remain ignored unless
  explicitly fixture-scoped.

## Handoff, conflict resolution, and approvals

- Handoffs reuse the existing agent handoff artifact; a handoff carries exact
  commit/PR/issue/evidence refs and the owning/receiving roles.
- Conflicts (overlapping ownership, stale state, missing evidence) produce a
  blocked/needs-fix outcome and preserve the conflicting evidence; they are
  never auto-resolved by promotion.
- Approval to promote any proposal flows only through the existing
  review/apply/trust-gradient path. A human maintainer retains the release
  go/no-go.

## Reviewer/critic promotion gates

Promotion of any role proposal requires an independent review/critic gate that
reuses the existing review/critic gate contract and the Milestone 22 trust
gradient:

- the implementer role and the reviewer/critic roles must be distinct actors;
- the trust gradient determines the required scrutiny: higher-risk and
  source-affecting changes require stronger review and are never auto-applied;
- accepted gates require an explicit `promote` recommendation with no hidden
  blocked reasons or required fixes;
- rejected/deferred/needs-fix/blocked gates keep their risk, fix, stale-state,
  and blocker evidence visible instead of repairing or bypassing it.

The gate is inert local evidence. It does not execute commands, spawn agents,
apply changes, merge, publish, sign, deploy, or write trusted browser state.

## Proposals-only boundary

Role agents, the producer, generation, and any browser/Studio surface emit
proposals only, through the existing review/apply/trust-gradient path. There are
no direct trusted writes, no hidden state, and no unreviewed writes between
roles. Browser/Studio surfaces are read-only and display escaped read-only
summaries; they do not execute commands, bridge to local commands, spawn agents,
auto-apply, auto-merge, self-approve, release, publish, sign, upload, or change
visibility.

## Language boundary

- Rust/local owns trusted validation, persistence, generation-proposal,
  asset-QA, curation, orchestration logic, provenance/compliance, evidence
  writing, run/project binding, the review/apply/trust-gradient path, and CLI
  behavior.
- TypeScript/JavaScript owns the deterministic runtime (including in-game
  UI/HUD/menus), the `window.__OUROFORGE__` probe, browser-local read-only
  inspection, and static dashboard/cockpit behavior where explicitly scoped.
- Python may be used only for temporary local tooling or smoke helpers and must
  not own core Era G/H contracts.
- No new language or runtime is introduced; distributed/Elixir remains NO-GO per
  ADR #92.

## Dependency order and closure gates

```text
#1674 scope -> #1675 roles -> #1676 handoff -> #1678 gates -> #1679 demo
       -> #1680 coverage -> #1681 governance
```

1. Multi-Agent Production Pipeline v1 Scope and Design Gate (this issue, #1674)
2. Role Agent Model and Artifact Ownership v1 — #1675
3. Handoff Artifacts and Conflict Resolution v1 — #1676
4. Reviewer/Critic Promotion Gates v1 — #1678
5. Multi-Agent Production Pipeline Demo v1 — #1679
6. Scenario Coverage v39: Multi-Agent Production Pipeline Regression Suite — #1680
7. Roadmap and #1 Governance Refresh after Multi-Agent Production Pipeline v1 — #1681

Each follow-up is independently verifiable (model, gate, demo, coverage,
governance) and must not be combined into a single PR. Scenario Coverage
numbering continues from the Era F sequence. Issues #1 and #23 remain open as
governance anchors and are not closed or modified by this milestone unless a
separate explicit governance decision exists.

## Definition of done for this design gate

This issue is complete when this document exists with conservative wording and a
focused audit verifies that it:

- records a GO/DEFER decision with DEFER as the default outside the bounded
  GO scope;
- defines the role-agent model, artifact ownership, handoff/conflict-resolution,
  shared state, approvals, and observability contracts;
- defines reviewer/critic promotion gates that reuse the existing review/critic
  gate and the Milestone 22 trust gradient;
- states that role agents emit proposals only and never perform a direct trusted
  write;
- reuses existing agent/evidence/journal/review surfaces with no parallel
  orchestration engine and no executable behavior;
- preserves generated-state policy and conservative public wording;
- reconfirms that #1 and #23 remain open.
