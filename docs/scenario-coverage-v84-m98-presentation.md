# Scenario Coverage v84 — M98 Billboard, Sprite-Stack, and 2D-in-3D Presentation Regression Suite

Scenario Coverage v84 locks Era P M98 presentation behavior for #2199. It is a Rust data-plane regression suite over `crates/ouroforge-core/src/gltf_25d_import.rs`, the M98 demo fixture in `examples/2-5d-gltf-import-v1/`, and the scripted demo updated for #2198.

## Boundary

- one-way on-ramp only: source-project glTF presentation extras become Ouroforge-native skeleton evidence, while behavior is clean-room re-derived in Era R. There is no live engine bridge and no embedded Unity, Unreal, Godot, or glTF runtime.
- Source-project/open-text only: the suite covers committed glTF 2.0 JSON source fixtures, not shipped-build ripping, binary extraction, decompiled source, runtime payloads, or copied engine code.
- M98 coverage is bounded: billboard, sprite-stack, and 2D-in-3D plane layers are presentation-only and cannot mutate deterministic logic/evidence.
- Fidelity remains honest: unsupported presentation kinds, logic-coupled presentation extras, custom shaders, logic, physics, shader, and VFX markers remain Yellow/Red gaps or Era R re-derivation tasks. A lossy import cannot be graded clean.
- Oracle-gated: no auto-port claim is allowed; no row can be claimed `ported`, auto-ported, auto-translated, behavior-equivalent, or complete without captured passing oracle evidence.
- Determinism is primary: the report validator recomputes `stateHashPrimary` from the canonical Rust-owned `nativeScene` and rejects stale or tampered hashes. Render smoke is perceptual secondary corroboration only, never a cross-machine bit hash.
- Rust owns artifact truth. Elixir/Phoenix Studio is not touched by v84, owns no artifact semantics, and has no trusted write authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v84.m98-presentation-primitives` | Billboard, sprite-stack, and 2D-in-3D plane extras normalize into `nativeScene.presentationLayers`. |
| `v84.presentation-cannot-mutate-logic` | M98 layers are presentation-only and cannot mutate deterministic logic/evidence. |
| `v84.lossy-presentation-not-clean` | Unsupported or logic-coupled presentation cannot be silently graded clean/Green. |
| `v84.no-auto-port-without-oracle` | Forged port/auto-translation claims fail report validation without oracle evidence. |
| `v84.deterministic-state-hash-break-fails` | A tampered native scene with stale state hash fails validation; changed M98 source changes the state hash. |
| `v84.coverage-ledger-and-demo-script` | The v84 matrix, docs, scripted demo, and tests are recorded as the Scenario Coverage version ledger for #2199. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```

The lane implementation also runs `cargo fmt --all`, `cargo build --workspace --jobs 2`, the focused v84 Rust test, the glTF demo script, and #1/#23 open-state checks before merge.
