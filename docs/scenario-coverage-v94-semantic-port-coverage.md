# Scenario Coverage v94 — Semantic-Port Coverage and Convergence

Scenario Coverage v94 locks Era R M112 semantic-port coverage after the first
coverage/convergence implementation. The suite treats coverage as a Rust-owned
ledger over clean-room re-derivation evidence, not as an auto-port or fully
ported product claim.

## Boundary

- One-way on-ramp: source skeleton and observed behavior may feed native
  re-derivation evidence; no live bridge or embedded Unity/Unreal/Godot runtime.
- Source-project/open-text and clean-room only: no decompiled source,
  shipped-build ripping, source translation, or foreign-runtime refs.
- Oracle-gated: verified units require captured oracle refs, M111 evidence,
  Green fidelity, deterministic primary state hash, and no residual gaps.
- Lossy imports are Yellow/Red work: gaps remain residual backlog and
  re-derivation tasks rather than being graded clean.
- Determinism is primary: 2D uses bit-exact state hashes; 2.5D/3D use
  state-hash primary with SSIM/pixel-diff render evidence only as secondary.
- Rust remains the data plane and owns artifact truth; Studio can render the
  evidence later but has no trusted-write authority and no artifact semantics.
- No auto-port without oracle; `portedClaimAllowed` and
  `fullyPortedClaimAllowed` stay false even when semantic coverage is complete.
- #1 and #23 remain open.

## Coverage ledger

| Row | Locked behavior |
| --- | --- |
| `v94.coverage-complete-without-ported-claim` | Complete coverage may terminate convergence but cannot emit ported or fully-ported claims. |
| `v94.lossy-import-not-graded-clean` | Pending Yellow units keep residual backlog and re-derivation tasks. |
| `v94.no-auto-port-without-oracle-fails` | Missing oracle/evidence cannot pass as a verified semantic-port unit. |
| `v94.ungated-auto-translated-port-fails` | Decompiled, shipped-build, foreign-runtime, live-bridge, and trusted-write drift fails closed. |
| `v94.deterministic-state-hash-break-fails` | Invalid state hashes fail; changed valid hashes alter deterministic coverage digest. |
| `v94.3d-state-hash-primary-render-secondary` | 3D verified rows require render corroboration as secondary evidence. |
| `v94.clean-room-source-only-and-no-studio-trusted-write` | Docs preserve Rust/Studio two-plane and source-only boundaries. |

## Verification

```bash
cargo fmt --all
cargo test -p ouroforge-core --test scenario_coverage_v94_semantic_port_coverage -- --nocapture
gh issue view 1 --json state --jq .state
gh issue view 23 --json state --jq .state
```
