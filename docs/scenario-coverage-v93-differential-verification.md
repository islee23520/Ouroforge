# Scenario Coverage v93 — Differential Verification Behavioral A/B

Scenario Coverage v93 locks Era R M111 differential verification after the first
Rust data-plane implementation. The suite verifies that a re-derived
Ouroforge-native candidate is compared against captured oracle evidence at the
outcome level, with mismatches reported as fidelity gaps, rollback work, and
re-derivation tasks instead of silent ports.

## Boundary

- Source-project/open-text inputs only: source skeleton and evidence refs stay in
  legal clean-room formats, never shipped-build or decompiled-code material.
- One-way on-ramp only: M111 consumes captured oracle evidence, deterministic
  M110 handoffs, and native observations; it does not bridge, embed, or absorb
  Unity/Unreal/Godot runtimes.
- Re-derivation, not translation: logic is verified clean-room from observed
  behavior and interrogated intent. Decompiled, ripped, shipped-build, or
  foreign-runtime references are blocked.
- No auto-port without oracle: missing, incomplete, or port-claiming oracles
  fail preflight before any semantic-port handoff can be emitted.
- Lossy imports, event mismatches, missing 3D render corroboration, blocked
  provenance, and state-hash breaks stay Yellow/Red with explicit gaps, rollback
  evidence, and re-derivation tasks.
- Determinism is primary. 2D requires bit-exact primary state hashes; 2.5D/3D
  requires deterministic state-hash primary with SSIM/pixel-diff render evidence
  only as secondary corroboration.
- Rust remains the data plane for artifact truth, validation, fidelity reports,
  deterministic hashing, behavioral A/B results, and M112 handoffs.
- Elixir/Phoenix Studio may render evidence and route gated actions later only;
  it has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v93.oracle-and-native-observation-pass-without-port-claim` | Captured oracle plus matching native observation yields Green and an M112 handoff, not a port claim. |
| `v93.lossy-outcome-mismatch-not-graded-clean` | Outcome mismatch is Yellow repair work with rollback evidence, never clean. |
| `v93.no-auto-port-without-oracle-fails-preflight` | Missing or port-claiming oracle evidence cannot pass. |
| `v93.ungated-auto-translated-port-fails` | Trusted writes, missing gates, decompiled refs, or runtime bridge refs fail preflight. |
| `v93.deterministic-state-hash-break-fails` | State-hash drift is Red, changes the digest, and blocks handoff. |
| `v93.3d-state-hash-primary-render-secondary` | 3D uses state-hash primary; perceptual render is secondary evidence. |
| `v93.clean-room-source-only-and-no-studio-trusted-write` | Source-only clean-room and two-plane boundaries remain documented. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
