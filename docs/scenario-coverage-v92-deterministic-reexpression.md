# Scenario Coverage v92 — Deterministic Re-Expression Engine

Scenario Coverage v92 locks Era R M110 deterministic re-expression after the
fixture-backed demo. The suite verifies that the Rust data-plane can turn
captured oracle evidence into a gated native `behavior_runtime` draft candidate
while rejecting silent partial ports, ungated auto-translation claims, and
state-hash drift.

## Boundary

- One-way on-ramp only: M110 consumes source-project/open-text skeleton context
  and captured oracle evidence, then emits native re-expression plans; it does
  not bridge, embed, or absorb Unity/Unreal/Godot runtimes.
- Re-derivation, not translation: logic is implemented clean-room from observed
  behavior and interrogated intent. Decompiled, ripped, shipped-build, or
  foreign-runtime references are blocked.
- No auto-port: a captured oracle can create a candidate draft and M111 handoff,
  never a finished port claim.
- Lossy imports, missing oracles, partial evidence, blocked provenance, and
  unsupported physics/render feel stay Yellow/Red with explicit gaps and tasks.
- Determinism is primary. 2D requires bit-exact state hashes; 2.5D/3D requires
  deterministic state-hash primary with SSIM/pixel-diff render evidence only as
  secondary corroboration.
- Rust remains the data plane for artifact truth, validation, fidelity reports,
  deterministic hashing, behavior drafts, and verification handoffs.
- Elixir/Phoenix Studio may render evidence and route gated actions later only;
  it has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v92.captured-oracle-reexpresses-gated-draft-not-port` | Captured oracle evidence yields a gated native draft candidate and M111 handoff, not a port claim. |
| `v92.lossy-import-not-graded-clean` | Lossy/oracle-less imported logic is not Green and emits follow-up tasks. |
| `v92.ungated-auto-translation-port-fails` | Auto-translation or port-claim input is Red and emits no draft. |
| `v92.deterministic-state-hash-break-fails` | State-hash drift changes the deterministic digest. |
| `v92.3d-state-hash-primary-render-secondary` | 3D uses state-hash primary and perceptual render only as secondary evidence. |
| `v92.clean-room-source-only-and-no-studio-trusted-write` | Decompiled/runtime refs are blocked; Studio has no trusted write authority. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
