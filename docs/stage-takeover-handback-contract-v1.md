# Stage Takeover and Handback Scope & Contract v1

Era M, Milestone 80 defines how a local Studio operator may take over a bounded stage and hand it back without breaking Ouroforge's agent-first default.

## Goal

Fix the contract for stage takeover and handback before implementation. The capability is an intervention-as-evidence path over existing gates, not a new authoring authority.

## Final Implementation Scope

- Stage takeover requests are captured as validated, recorded proposals, constraints, or directives.
- Handback requires evidence that any human-suggested write has completed the reused gated path.
- The gated path is the existing review/apply, scene/source-apply, evaluator, evidence, and provenance chain.
- Studio surfaces may read state and submit gated-write requests only.
- CLI fallback remains the canonical local-first path for the autonomous loop and stage execution.

## Success Criteria

- Every takeover and handback write path is specified as read + gated-write.
- The intervention-as-evidence invariant is explicit and machine-testable downstream.
- The two-plane boundary is preserved: Rust owns data-plane validation and artifact semantics; Elixir/OTP and Phoenix LiveView own only local control and presentation.
- The autonomous loop can still complete with zero human input.
- Governance anchors #1 and #23 remain open.

## Verification Method

```bash
set -euo pipefail
grep -RIlqi "read \+ gated-write\|intervention-as-evidence\|two-plane\|local-first" docs/ || true
cargo build --workspace --jobs 2
```

## Guardrails

- Agent-first default preserved: human takeover is opt-in at defined stage boundaries and never required for the loop to progress.
- Every human intervention is a validated, recorded proposal, constraint, or directive routed through review/apply, scene/source-apply, evaluator, evidence, and provenance gates.
- No raw write bypasses gates, determinism, or audit.
- Studio surfaces are read + gated-write only.
- Two-plane: Rust is the data plane for artifact truth, validation, determinism, and semantics; Elixir/OTP plus Phoenix LiveView are the local control and presentation plane.
- Local-first: a fresh checkout can run the full loop through the CLI without Studio.
- Hosted, multi-user, collaborative, or real-time remote Studio remains Layer-3 DEFER.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.

## Explicit Non-Goals

- Raw human writes to artifacts, ledgers, evidence, scenes, sources, releases, merges, or deploy surfaces.
- Making human takeover mandatory or blocking autonomous progress on human input.
- A hosted, multi-user, collaborative, or real-time remote Studio.
- Elixir/Phoenix ownership of artifact semantics or validation.
- A new data store, write path, validator, ledger, or provenance plane.
- Automating fun/taste verdicts or release go/no-go.

## Implementation Approach

1. Render current stage state in the local Studio from existing Rust-backed evidence and executor state.
2. Capture takeover intent as a proposal, constraint, or directive with actor, reason, base references, and target stage.
3. Validate and record that intervention through existing Rust gates before it can affect scheduling or source/scene state.
4. During takeover, any write-affecting suggestion remains pending until review/apply, scene/source-apply, evaluator, and provenance evidence pass.
5. Handback records a directive that returns the stage to autonomous execution with evidence refs for accepted or rejected human changes.
6. CLI execution remains available and sufficient with no Studio process.

## PR Decomposition

- PR 1: this contract document.
- Downstream PRs: Elixir executor surface, demo, scenario coverage, and governance handoff must reference this contract.

## Over-Engineering Checklist

- [x] No capability beyond M80 stage takeover/handback scope.
- [x] No speculative cluster, hosted, collaborative, or multi-user behavior.
- [x] Existing gates are reused; no parallel write path is introduced.
- [x] Phoenix LiveView scope remains minimal, local, and read + gated-write.
- [x] No new data store; Rust kernel ledger/evidence remains source of truth.

## Drift-Prevention Checklist

- [x] Agent-first default preserved; takeover is opt-in and never required.
- [x] Every intervention routes through existing gates; never raw bypass.
- [x] Rust = data plane; Elixir/Phoenix LiveView = local control + presentation.
- [x] Hosted/multi-user collaborative Studio deferred; CLI fallback intact.
- [x] Fun/taste and release go/no-go remain human; #1 and #23 remain open.

## Language Boundary

Documentation only for this issue. The contract records decisions for later Rust and Elixir implementation: Rust owns validation, evidence, provenance, determinism, and artifact semantics; Elixir/Phoenix may render/capture/route only.

## Critical Risk Review

- Raw-bypass risk: mitigated by requiring all takeover writes to become validated and recorded proposals, constraints, or directives before scheduling or artifact effects.
- Autonomy regression: mitigated by requiring a zero-human CLI fallback and no mandatory wait on human takeover.
- Presentation-plane leakage: mitigated by forbidding Elixir/Phoenix artifact semantics, validation, or trusted writes.
- Scope creep: mitigated by keeping hosted and collaborative Studio deferred and limiting this contract to local single-user control.

## Definition of Done

- This contract exists in docs and is indexed.
- Downstream M80 issues can reference the gated path, intervention-as-evidence invariant, two-plane boundary, and local-first CLI fallback.
- Verification passes.
