# Studio Packaging and Local Delivery Contract v1

Era N, Milestone 86 fixes the contract for packaging and locally delivering the Studio before implementation. The capability packages a local single-user Phoenix LiveView Studio surface and supporting Rust CLI/data-plane artifacts for local use only. It does not create a hosted product, release channel, app store path, remote collaboration system, or new trusted write surface.

## Goal

Define how Studio packaging and local delivery preserve Ouroforge's agent-first default, read + gated-write invariant, two-plane architecture, and CLI fallback while making the local Studio easier to start, inspect, and hand off as a reproducible developer artifact.

## Final Implementation Scope

- Local packaging descriptors for the Studio control/presentation plane and its Rust CLI/data-plane prerequisites.
- Local delivery instructions for a fresh checkout: build Rust, fetch/compile the Mix app, run the CLI loop, and optionally start the local Studio.
- Intervention points exposed by the packaged Studio: observation, steering, amendment, constraints, diagnosis correction, takeover/handback, authoring drafts, evidence, journal, verdicts, and demo flows.
- Gated path for every write-affecting action: review/apply, scene/source-apply, evaluator, evidence/provenance, and the existing Rust-owned gate families introduced before M86.
- Packaging evidence that records versions, commands, hashes/refs, generated-state boundaries, and unsupported hosted/collaborative/release targets.

## Read + Gated-Write Invariant

Packaging does not change Studio authority:

1. **Read:** the packaged Studio renders Rust-owned evidence, diagnoses, journals, verdicts, proposals, directives, authoring drafts, and generated-state diagnostics.
2. **Capture:** optional human actions are captured as proposal/constraint/directive/correction/amendment/takeover/handback/review evidence envelopes.
3. **Validate:** Rust data-plane gates validate shape, refs, hashes, provenance, freshness, determinism, and target class.
4. **Record:** accepted, rejected, stale, and blocked intervention records remain audit-visible.
5. **Apply:** trusted artifact effects occur only through existing review/apply, scene/source-apply, evaluator, and evidence/provenance gates. The packaged Studio never writes trusted artifacts directly.

## Local-First CLI Fallback

A fresh checkout must remain sufficient without Studio:

```bash
cargo build --workspace --jobs 2
cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1)
[ -n "$STU" ] && (cd "$(dirname "$STU")" && mix deps.get && mix compile)
```

The autonomous loop can run and complete through the CLI with zero human input. Starting the packaged Studio is optional and must not become a prerequisite for validation, evidence generation, artifact writes, review/apply, source/scene-apply, or release readiness.

## Two-Plane Boundary

- **Rust data plane:** artifact truth, validation, determinism, evidence, provenance, review/apply, scene/source-apply, evaluator verdicts, generated-state policy, and artifact semantics.
- **Elixir/OTP + Phoenix LiveView control/presentation plane:** local supervision, PubSub refresh, routing envelopes, forms, accessibility/i18n/theme/keyboard presentation, and display feedback.
- Packaging scripts/descriptors may assemble local developer conveniences, but they do not grant Elixir artifact semantics, trusted writes, command bridge authority, or validator ownership.

## Success Criteria

- The M86 contract specifies the gated path reused by every write-affecting Studio action.
- The intervention-as-evidence invariant is explicit for packaging and delivery.
- The local-first CLI fallback is documented and remains sufficient.
- Packaging is local single-user only and does not introduce hosted, multi-user, collaborative, release, publish, deploy, signing, app-store, or cloud behavior.
- Governance anchors #1 and #23 remain open.

## Guardrails

- Agent-first default preserved: human intervention is opt-in and never required for the autonomous loop.
- Every human intervention is a validated, recorded proposal, constraint, directive, correction, amendment, takeover/handback, or review record through existing gates.
- No raw human write bypasses gates, evidence, determinism, provenance, or audit.
- No new data store, ledger, validator, artifact writer, command bridge, release pipeline, deployment credential, signing path, or hosted collaboration surface.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions and are not automated by packaging.
- Conservative wording only: local delivery is not a no-code product, public release, production editor maturity claim, or hosted service.

## Explicit Non-Goals

- Hosted, multi-user, collaborative, or real-time remote Studio.
- Store, installer, updater, code-signing, deploy, publish, release, or cloud synchronization workflows.
- Elixir/Phoenix artifact validation, artifact semantics, trusted writes, or evidence truth ownership.
- A new persistence store or parallel write path.
- Mandatory human involvement or a Studio-required autonomous loop.
- Automating fun/taste verdicts or release go/no-go.

## Implementation Approach

1. Package only local descriptors and startup/read-model wiring that reference existing Rust and Mix build steps.
2. Surface packaging status as generated evidence/read models, not trusted source edits.
3. Reuse the M85 Studio shell, intervention panels, demo, and v75 coverage assertions.
4. Keep copyable CLI commands visible as the canonical fallback.
5. Fail closed on any descriptor that requests direct artifact writes, command bridges, hosted/collab mode, release credentials, signing, deploy, or a new store.

## PR Decomposition

- PR 1: this contract document.
- Downstream PRs may add local packaging descriptors, a local delivery demo, scenario coverage, and governance handoff, all referencing this contract.

## Over-Engineering Checklist

- [x] No capability beyond M86 local packaging and delivery scope.
- [x] Existing gates are reused; no parallel write path is introduced.
- [x] Phoenix LiveView scope remains local and single-user.
- [x] No hosted/collab, cloud, auth, signing, installer, app-store, or release pipeline behavior.
- [x] No new data store; Rust kernel ledger/evidence remains source of truth.

## Drift-Prevention Checklist

- [x] Agent-first default preserved; intervention opt-in and never required.
- [x] Every write-affecting action routes through existing gates as read + gated-write.
- [x] Rust = data plane; Elixir/Phoenix LiveView = local control + presentation.
- [x] Hosted/multi-user collaborative Studio deferred; CLI fallback intact.
- [x] Fun/taste and release go/no-go remain human; #1 and #23 remain open.

## Language Boundary

Documentation only for this issue. This contract records M86 decisions for later implementation. It adds no code, no package artifact, no release process, no new store, and no write authority.

## Critical Risk Review

- **Raw-bypass risk:** mitigated by requiring packaging to preserve intervention-as-evidence and existing Rust gates for every write-affecting action.
- **Autonomy regression:** mitigated by requiring a fresh-checkout CLI fallback and zero-human autonomous completion.
- **Presentation-plane leakage:** mitigated by forbidding Elixir/Phoenix artifact semantics, validation, evidence truth ownership, and trusted writes.
- **Scope creep to hosted/release:** mitigated by explicitly deferring hosted/collab and rejecting release, signing, deploy, publish, app-store, and cloud behavior.

## Definition of Done

- This contract exists in docs and is referenceable by downstream M86 issues.
- The issue verification passes.
- #1 and #23 remain open.
