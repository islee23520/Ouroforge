# Scenario Coverage v95 — Re-Derivation UX and Human Intent/Feel Escalation

Scenario Coverage v95 locks Era R M113 Studio behavior for re-derivation UX and
human intent/feel escalation. The surface is read + gated-write only: it renders
Rust-owned evidence and routes human notes through `ouroforge` CLI/review gates.
It does not own artifact semantics or perform trusted writes.

## Boundary

- One-way on-ramp: source-project/open-text skeletons feed clean-room
  re-derivation evidence; no engine absorption, live bridge, or embedded
  Unity/Unreal/Godot runtime.
- Oracle-gated: no auto-port without oracle, no finished-game claim, and no
  `ported`/`fully ported` copy unless captured acceptance evidence passes.
- Lossy imports are not clean: Yellow/Red rows keep gap summaries and
  re-derivation tasks rather than silently dropping behavior.
- Determinism is primary: 2D requires bit-exact state hashes; 2.5D/3D require
  state-hash primary and perceptual SSIM/pixel-diff render evidence secondary.
- Studio is Elixir/Phoenix control + presentation only; Rust remains the data plane and owns artifact truth, fidelity, deterministic evidence, and gates.
- Intent/feel/fun/release decisions remain Ring 2 human-owned.
- #1 and #23 remain open.

## Coverage ledger

| Row | Locked behavior |
| --- | --- |
| `v95.demo-renders-honest-fidelity-summary` | Demo renders Green/Yellow/Red and escalation counts honestly. |
| `v95.no-auto-port-without-oracle-fails` | Missing captured oracle cannot be Green or ported. |
| `v95.lossy-import-not-graded-clean` | Lossy/human-feel rows stay Yellow/Red with gaps/tasks. |
| `v95.ungated-auto-translated-port-fails` | Trusted writes, decompiled refs, shipped builds, and bridges fail closed. |
| `v95.deterministic-state-hash-break-fails` | Invalid/missing state-hash evidence fails; 3D render evidence is secondary. |
| `v95.human-intent-feel-routes-through-gated-cli` | Human notes route through allowed CLI preview/review gates. |
| `v95.clean-room-source-only-and-no-studio-trusted-write` | Docs preserve source-only clean-room and two-plane boundaries. |

## Verification

```bash
cd studio/executor
mix compile --warnings-as-errors
mix test test/ouroforge_executor/scenario_coverage_v95_test.exs
```
