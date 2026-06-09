# Semantic-Port Coverage and Convergence v1

Era R M112 makes semantic re-derivation progress visible after M111 behavioral
A/B verification. The Rust data-plane owns the coverage ledger and convergence
verdict; Studio may later render it but has no artifact semantics or
trusted-write authority.

## Data shapes

The concrete M112 shapes live in `crates/ouroforge-core::semantic_port_coverage`:

- `SemanticPortCoverageRequest` binds the source project ref, target
  dimensionality, convergence policy, and all semantic-port unit rows.
- `SemanticPortCoverageUnit` references the behavioral-unit record, captured
  oracle, M111 evidence, deterministic primary state hash, optional 2.5D/3D
  render digest, residual gaps, re-derivation tasks, and optional loop-coverage
  attribution.
- `SemanticPortCoverageReport` emits the Rust-owned summary, residual backlog,
  convergence status, deterministic digest, and follow-up re-derivation tasks.

## Boundary

- One-way on-ramp only: coverage records native re-derivation progress, not a
  live bridge or embedded Unity/Unreal/Godot runtime.
- Re-derivation, not translation: refs must be source-project/open-text and
  clean-room; decompiled, shipped-build, ripped, foreign-runtime, and live-bridge
  refs fail closed.
- Oracle-gated: a unit is `verified` only with Green fidelity, captured oracle
  ref, M111 evidence, deterministic state hash, no residual gaps, and
  loop-produced/loop-verified attribution when attribution is supplied.
- No silent fully ported claim: reports may say semantic coverage is complete,
  pending, blocked, or Ring 2 human-feel escalated, but `portedClaimAllowed` and
  `fullyPortedClaimAllowed` remain false.
- Determinism is primary: 2D uses bit-exact state hashes; 2.5D/3D requires
  deterministic state-hash primary and perceptual SSIM/pixel-diff render evidence
  only as secondary corroboration.
- Human fun/feel and release go/no-go remain Ring 2 human-owned; automation may
  terminate into human escalation, not auto-score or auto-release.
- #1 and #23 remain open governance anchors.

## Convergence statuses

| Status | Meaning |
| --- | --- |
| `passed` | Every unit is verified and the policy allows stopping when all verified. |
| `continue` | Pending units remain and the iteration budget is not exhausted. |
| `human_escalated` | At least one unit needs Ring 2 human-feel review. |
| `blocked` | A blocked unit exists or the convergence budget is exhausted. |

## Verification

```bash
cargo fmt --all
cargo build --workspace --jobs 2
cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2
```
