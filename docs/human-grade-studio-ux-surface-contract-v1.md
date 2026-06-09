# Human-Grade Studio UX Surface Contract v1

Era N, Milestone 85 fixes the local Phoenix LiveView Studio surface contract before implementation. The Studio composes Era M observation, intervention, and authoring surfaces into one local-first read + gated-write control/presentation plane while preserving the autonomous CLI loop as the default path.

## Goal

Provide a single contract for a human-grade Studio UX surface that can render Rust-owned run state, evidence, diagnoses, interventions, authoring drafts, journals, and verdicts, then route any operator action as intervention-as-evidence through existing gates. The Studio is a local single-user convenience surface; it is not a new artifact authority, data store, validator, hosted service, or mandatory human checkpoint.

## Surface Inventory

The M85 Studio surface may compose these panels from existing Rust-backed read models and gate contracts:

| Surface | Read model | Gated-write action | Required gate |
| --- | --- | --- | --- |
| Diagnosis console | Current diagnosis, suspected bottleneck, failed gate, source evidence refs, confidence, and next autonomous action. | Submit a diagnosis correction or added context as a recorded proposal/constraint/directive. | Evaluator, evidence/provenance, review/apply or source/scene-apply when an artifact can change. |
| Steering console | Campaign stage, budget, retry/rollback status, active blockers, and safe next commands as copyable CLI references. | Submit a bounded steering directive, priority hint, or constraint. | Existing live campaign steering directive and evaluator gates. |
| Amendment panel | Proposal metadata, target artifact class, base hash/ref, intended effect, and risk tier. | Amend a proposal without applying it. | Proposal amendment reverify, evidence/provenance, and downstream review/apply. |
| Constraint panel | Human-authored constraint text, target scope, reason, expiration/staleness, and evidence refs. | Record a style/budget/mechanic/directive constraint. | Human constraint gate and evaluator aggregation. |
| Correction panel | Incorrect diagnosis/verdict/source mapping and proposed correction with actor/reason. | Record correction evidence. | Diagnosis correction/intervention feedback contract and Rust evaluator gates. |
| Takeover/handback | Stage ownership, handback package, unresolved blockers, stale refs, and rollback handles. | Record takeover or handback intent. | Stage takeover/handback contract plus evidence/provenance. |
| Authoring surface | Draft scene/source/asset/prototype/read-model previews, stale status, unsupported operations, and validation diagnostics. | Submit an authoring draft or review decision; never directly write trusted artifacts. | Scene/source-apply, review/apply, evaluator, evidence/provenance. |
| Evidence view | Evidence bundle index, provenance chain, run refs, replay keys, hashes, malformed states, and freshness status. | None except copy refs or attach refs to a gated proposal. | Read-only; attachment is validated by the target gate. |
| Journal view | Run journal, decision ledger, attribution, friction summaries, and campaign history. | None except creating a proposal/constraint/directive that cites journal refs. | Read-only; target gate validates cited refs. |
| Verdict view | Four-gate verdicts, design/fun/release status, readiness blockers, and human Ring 2 go/no-go records where already present. | Record optional human fun/taste or release go/no-go evidence only when explicitly requested by the operator. | Existing fun/taste and release readiness contracts; never automated by Studio. |

## Read + Gated-Write Invariant

Studio surfaces are read + gated-write:

1. **Read:** Phoenix LiveView renders Rust-owned evidence, read models, journals, verdicts, proposals, directives, and generated-state diagnostics.
2. **Capture:** An operator action is captured as a bounded proposal, constraint, directive, correction, amendment, takeover, handback, review decision, or fun/release record with actor, reason, target refs, base hashes, and intended effect.
3. **Validate:** Rust data-plane gates validate shape, hashes, target class, provenance, freshness, risk, determinism, and policy. Elixir/Phoenix does not validate artifact semantics.
4. **Record:** Accepted and rejected interventions are recorded as evidence/provenance. Rejected or blocked records remain visible without stopping unrelated autonomous work.
5. **Apply only through existing gates:** Any trusted artifact effect must pass review/apply, scene/source-apply, evaluator, evidence/provenance, or another existing Rust-owned gate. The Studio never performs raw artifact writes.

## Real-Time, Local-First, and CLI Fallback

- Phoenix PubSub may broadcast local run-state updates, panel refreshes, gate status, and evidence notifications inside the local single-user Studio process.
- PubSub events are presentation/control signals only; they do not carry trusted artifact semantics or apply authority.
- A fresh checkout can run the full autonomous loop through the CLI without starting Phoenix, opening a browser, or waiting for human input.
- Copyable CLI references remain available for every important action so the terminal path stays canonical.
- Hosted, multi-user, collaborative, real-time remote Studio, accounts, auth, cloud state, and shared workspaces are Layer-3 DEFER.

## Two-Plane Boundary

- **Rust data plane:** artifact truth, validation, determinism, evidence, provenance, gate evaluation, source/scene apply, review/apply, release readiness, and semantic ownership.
- **Elixir/OTP + Phoenix LiveView control/presentation plane:** local routing, PubSub refresh, form rendering, keyboard/a11y/i18n/theme presentation, local process supervision, and capture of operator intent for Rust-owned validation.
- Elixir never owns artifact semantics, never writes trusted artifacts, never bypasses gates, never records evidence as authoritative without Rust validation, and never becomes required for autonomous completion.

## Success Criteria

- The Studio surface inventory is explicit and covers diagnosis, steering, amendment, constraint, correction, takeover/handback, authoring, evidence, journal, and verdict views.
- Every write-affecting action is intervention-as-evidence and routes through existing gates.
- The contract preserves two-plane and local-first boundaries.
- Real-time behavior is limited to local PubSub presentation/control refresh.
- CLI fallback and zero-human autonomous completion remain intact.
- Governance anchors #1 and #23 remain open.

## Guardrails

- Agent-first default is preserved: human intervention is opt-in at defined points and never required.
- Every human intervention is a validated, recorded proposal, constraint, directive, correction, amendment, takeover, handback, review decision, or Ring 2 record through existing gates.
- No raw write bypasses review/apply, scene/source-apply, evaluator, evidence/provenance, determinism, or audit.
- No new data store, ledger, validator, artifact writer, browser command bridge, release/publish path, or Elixir data plane.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions; the Studio may display or record them through their existing gates but never automates them.
- Conservative wording only: this is not a no-code product, hosted collaborative editor, Godot replacement, public release system, or production maturity claim.

## Explicit Non-Goals

- Raw human writes to artifacts, ledgers, evidence, scenes, sources, release metadata, merge state, deployment targets, or generated state.
- Mandatory human intervention or a Studio-required autonomous loop.
- Hosted, multi-user, collaborative, or real-time remote Studio.
- Elixir/Phoenix artifact validation, artifact semantics, trusted writes, or ownership of evidence truth.
- New persistence stores, databases, auth/accounts, cloud synchronization, or browser command bridges.
- Automating fun/taste verdicts or release go/no-go.

## Implementation Approach

1. Compose existing Era M read-only and intervention surfaces into one LiveView navigation shell.
2. Feed every panel from Rust-exported read models, evidence bundles, ledgers, verdicts, and gate diagnostics.
3. Use Phoenix PubSub only for local refresh and gate-status notifications.
4. Normalize every operator action into a Rust-validated intervention-as-evidence envelope.
5. Reuse review/apply, scene/source-apply, evaluator, evidence/provenance, human-constraint, steering, correction, takeover/handback, fun/taste, and release readiness gates.
6. Keep copyable CLI fallback commands visible and sufficient for zero-Studio operation.

## PR Decomposition

- PR 1: this Studio UX surface contract.
- Downstream PRs may implement the minimal Phoenix shell, panel adapters, PubSub status refresh, and scenario coverage, but must reference this contract and preserve the same gate boundaries.

## Over-Engineering Checklist

- [x] No capability beyond M85 surface scope.
- [x] Reuses existing gates instead of creating a parallel write path.
- [x] Phoenix LiveView remains local single-user control/presentation.
- [x] No new data store, hosted service, collaboration system, or browser command bridge.
- [x] PubSub is presentation/control refresh only, not artifact authority.

## Drift-Prevention Checklist

- [x] Agent-first default preserved; intervention opt-in and never required.
- [x] Every intervention routes through existing gates as read + gated-write.
- [x] Rust = data plane; Elixir/Phoenix LiveView = local control + presentation.
- [x] Hosted/multi-user collaborative Studio deferred; CLI fallback intact.
- [x] Fun/taste and release go/no-go stay human; #1 and #23 remain open.

## Language Boundary

Documentation only for this issue. This contract records decisions for later Rust and Phoenix implementation. It does not add code, a new write path, a new store, or runtime authority.

## Critical Risk Review

- **Raw-bypass risk:** mitigated by requiring every operator action to become a validated and recorded proposal, constraint, directive, correction, amendment, takeover, handback, review decision, or Ring 2 record before any trusted effect.
- **Autonomy regression:** mitigated by preserving zero-human CLI completion and treating Studio use as opt-in.
- **Presentation-plane leakage:** mitigated by assigning validation, determinism, artifact semantics, evidence truth, and apply authority to Rust only.
- **Scope creep to hosted/collab:** mitigated by local-first single-user scope and Layer-3 DEFER for remote collaboration.
- **Overclaim risk:** mitigated by conservative wording and explicit non-goals for no-code, release, and production editor claims.

## Definition of Done

- This contract exists in docs and can be referenced by M85 implementation issues.
- The issue verification passes.
- #1 and #23 remain open.
