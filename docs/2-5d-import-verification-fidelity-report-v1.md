# 2.5D Import Verification and Fidelity Report v1

Era P M99 verifies 2.5D imports after the M97/M98 glTF presentation on-ramp. The report is a Rust-owned data-plane artifact: it verifies the imported native presentation skeleton with a deterministic state-hash primary gate, records perceptual SSIM/pixel-diff render evidence as secondary corroboration only, and keeps all fidelity gaps visible.

## Boundary

- One-way source-project/open-text import only; no live bridge and no embedded Unity/Unreal/Godot runtime.
- Clean-room re-derivation, not translation: behavior-bearing logic, physics, shaders, VFX, and engine-specific hooks become Era R tasks.
- No unit is claimed ported, auto-ported, or auto-translated without captured acceptance evidence and a passing oracle.
- Rust owns artifact truth, fidelity grading, state-hash validation, and gap attribution.
- Elixir/Phoenix Studio is not part of this report, owns no artifact semantics, and has no trusted-write authority.
- #1 and #23 remain open governance anchors.

## Rust-owned data shapes

| Shape | Code location | Purpose |
| --- | --- | --- |
| IR nodes | `crates/ouroforge-core/src/gltf_25d_import.rs::Gltf25dNativeNode` | Normalized glTF node skeleton facts with deterministic transforms and presentation roles. |
| Mapping records | `crates/ouroforge-core/src/gltf_25d_import.rs::Gltf25dNativeScene` | Ouroforge-native 2.5D presentation scene, meshes, materials, cameras, and M98 presentation layers. |
| Behavioral-unit records | `crates/ouroforge-core/src/gltf_25d_import.rs::Gltf25dReDerivationTask` | Logic/physics/shader/VFX/source-engine gaps routed to Era R for clean-room re-derivation. |
| Oracle records | `crates/ouroforge-core/src/gltf_25d_import.rs::Gltf25dFidelityRow::oracle_required` | Per-unit oracle requirement that blocks any port claim until captured acceptance evidence passes. |
| Verification report | `crates/ouroforge-core/src/gltf_25d_import.rs::Gltf25dVerificationReport` | M99 gate result, fidelity rows, gap attribution, data-shape ledger, and deterministic report hash. |

## Verification gates

1. `deterministic-state-hash-primary` must pass. The observed native scene state hash must match the import report's `stateHashPrimary`.
2. `perceptual-render-secondary` must pass fixture tolerances for SSIM and pixel-diff, but it remains corroboration only and never replaces the state-hash gate.
3. Non-green fidelity rows must have explicit gap attribution; unsupported or behavior-bearing facts are never silently dropped or laundered into clean/green.
4. `claimedPortedUnits` remains empty for this on-ramp report.

## Verification

```bash
cargo fmt --all
cargo build --workspace --jobs 2
cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2
cargo run -p ouroforge-cli -- run seeds/migration-demo.yaml --workers 2 || true
```
