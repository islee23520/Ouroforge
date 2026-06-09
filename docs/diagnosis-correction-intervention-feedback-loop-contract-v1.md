# Diagnosis Correction and Intervention Feedback Loop Scope & Contract v1

Era M, Milestone 79 defines how a local operator may correct a diagnosis or add feedback to an intervention loop without breaking Ouroforge's agent-first default.

## Goal

Fix the contract for diagnosis correction and intervention feedback before implementation. The capability is an intervention-as-evidence path over existing gates, not a new diagnosis authority, artifact writer, or mandatory human checkpoint.

## Final Implementation Scope

- Diagnosis correction requests are captured as validated, recorded proposals, constraints, or directives with actor, reason, target diagnosis, base evidence refs, and intended effect.
- Intervention feedback is attached as evidence to the existing loop and may influence future scheduling or evaluation only after the reused gated path passes.
- The gated path is the existing review/apply, scene/source-apply, evaluator, evidence, and provenance chain.
- Studio surfaces may read current diagnoses, evidence, and loop state, then submit gated-write requests only.
- CLI fallback remains the canonical local-first path: a fresh checkout can run the autonomous loop and accept correction artifacts through the Rust-owned gates without Studio.

## Success Criteria

- Every diagnosis correction or intervention feedback write path is specified as read + gated-write.
- The intervention-as-evidence invariant is explicit and machine-testable downstream.
- The two-plane boundary is preserved: Rust owns data-plane validation, artifact semantics, determinism, evidence, and provenance; Elixir/OTP and Phoenix LiveView own only local control and presentation.
- The autonomous loop can still complete with zero human input; human correction is opt-in and never required.
- Governance anchors #1 and #23 remain open.

## Verification Method

```bash
set -euo pipefail
grep -RIlqi "read \+ gated-write\|intervention-as-evidence\|two-plane\|local-first" docs/ || true
cargo build --workspace --jobs 2
```

## Guardrails

- Agent-first default preserved: diagnosis correction and feedback are opt-in at defined points and never required for the loop to progress.
- Every human intervention is a validated, recorded proposal, constraint, or directive routed through review/apply, scene/source-apply, evaluator, evidence, and provenance gates.
- No raw write bypasses gates, determinism, provenance, or audit.
- Studio surfaces are read + gated-write only.
- Two-plane: Rust is the data plane for artifact truth, validation, determinism, diagnosis semantics, evidence, and provenance; Elixir/OTP plus Phoenix LiveView are the local control and presentation plane.
- Local-first: a fresh checkout can run the full loop through the CLI without Studio.
- Hosted, multi-user, collaborative, or real-time remote Studio remains Layer-3 DEFER.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions; this contract does not automate them.

## Explicit Non-Goals

- Raw human writes to artifacts, ledgers, evidence, diagnoses, scenes, sources, releases, merges, or deploy surfaces.
- Making human correction mandatory or blocking autonomous progress on human input.
- A hosted, multi-user, collaborative, or real-time remote Studio.
- Elixir/Phoenix ownership of diagnosis or artifact semantics, validation, or trusted writes.
- A new data store, write path, validator, ledger, or provenance plane.
- Automating fun/taste verdicts or release go/no-go.

## Implementation Approach

1. Render the current diagnosis, evidence bundle, and loop state in the local Studio from existing Rust-backed read models.
2. Capture a diagnosis correction or intervention feedback item as a proposal, constraint, or directive with actor, reason, base refs, target diagnosis id, and requested correction.
3. Validate and record that intervention through existing Rust gates before it can change scheduling, evaluator outcomes, source/scene state, or diagnosis status.
4. Route accepted corrections through review/apply, scene/source-apply, evaluator, evidence, and provenance gates as applicable to the affected artifact class.
5. Record rejected corrections with fail-closed evidence so the loop can continue autonomously without waiting for a human.
6. Keep the CLI path sufficient for both zero-human autonomous runs and optional correction submission with no Studio process.

## PR Decomposition

- PR 1: this contract document.
- Downstream PRs: Rust correction/evaluator data-plane capability, Elixir local Studio surface, demo, scenario coverage, and governance handoff must reference this contract.

## Over-Engineering Checklist

- [x] No capability beyond M79 diagnosis correction and intervention feedback scope.
- [x] No speculative cluster, hosted, collaborative, or multi-user behavior.
- [x] Existing gates are reused; no parallel write path is introduced.
- [x] Phoenix LiveView scope remains minimal, local, and read + gated-write.
- [x] No new data store; Rust kernel ledger/evidence remains source of truth.

## Drift-Prevention Checklist

- [x] Agent-first default preserved; correction and feedback are opt-in and never required.
- [x] Every intervention routes through existing gates; never raw bypass.
- [x] Rust = data plane; Elixir/Phoenix LiveView = local control + presentation.
- [x] Hosted/multi-user collaborative Studio deferred; CLI fallback intact.
- [x] Fun/taste and release go/no-go remain human; #1 and #23 remain open.

## Language Boundary

Documentation only for this issue. The contract records decisions for later Rust and Elixir implementation: Rust owns validation, evidence, provenance, determinism, diagnosis semantics, and artifact semantics; Elixir/Phoenix may render/capture/route only.

## Critical Risk Review

- Raw-bypass risk: mitigated by requiring all correction and feedback writes to become validated and recorded proposals, constraints, or directives before diagnosis, scheduling, source, scene, or artifact effects.
- Autonomy regression: mitigated by requiring a zero-human CLI fallback and no mandatory wait on human correction.
- Presentation-plane leakage: mitigated by forbidding Elixir/Phoenix diagnosis semantics, artifact semantics, validation, or trusted writes.
- Scope creep: mitigated by keeping hosted and collaborative Studio deferred and limiting this contract to local single-user control.

## Definition of Done

- This contract exists in docs and is indexed.
- Downstream M79 issues can reference the gated path, intervention-as-evidence invariant, two-plane boundary, and local-first CLI fallback.
- Verification passes.
