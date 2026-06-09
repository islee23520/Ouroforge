# Scenario Coverage v83 — glTF Geometry and Orthographic-Camera Import Regression Suite

Scenario Coverage v83 locks the Era P M97 glTF geometry and orthographic-camera import behavior for #2195. It is a Rust data-plane regression suite over `crates/ouroforge-core/src/gltf_25d_import.rs`, the fixture in `examples/2-5d-gltf-import-v1/`, and the scripted demo added for #2194.

## Boundary

- one-way on-ramp only: source-project glTF presentation facts become Ouroforge-native skeleton evidence, while behavior is clean-room re-derived in Era R. There is no live engine bridge and no embedded Unity, Unreal, Godot, or glTF runtime.
- Source-project/open-text only: the suite covers committed glTF 2.0 JSON source fixtures, not shipped-build ripping, binary extraction, decompiled source, runtime payloads, or copied engine code.
- Fidelity remains honest: supported presentation geometry, PBR/unlit material references, and orthographic cameras can be Green; unsupported extensions, custom shaders, logic, physics, shader, and VFX markers remain Yellow/Red gaps or Era R re-derivation tasks. A lossy import or behavior-bearing unit cannot be graded clean.
- Oracle-gated: no auto-port claim is allowed; no row can be claimed `ported`, auto-ported, auto-translated, behavior-equivalent, or complete without captured passing oracle evidence. M97 keeps content imports best-effort and presentation-only.
- Determinism is primary: the report validator recomputes `stateHashPrimary` from the canonical Rust-owned `nativeScene` and rejects stale or tampered hashes. Render smoke is perceptual secondary corroboration only, never a cross-machine bit hash.
- Rust owns artifact truth. Elixir/Phoenix Studio is not touched by v83, owns no artifact semantics, and has no trusted write authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v83.gltf-geometry-ortho-import` | The fixture imports 2.5D presentation geometry, material refs, and an orthographic camera into Rust-owned native scene evidence. |
| `v83.lossy-import-not-clean` | Unsupported extensions/custom shader notes and behavior-bearing markers stay Yellow/Red or Era R tasks, not clean/Green. |
| `v83.no-auto-port-without-oracle` | Forged port/auto-translation claims fail report validation without oracle evidence. |
| `v83.deterministic-state-hash-break-fails` | A tampered native scene with a stale state hash fails validation; changed source changes the state hash. |
| `v83.coverage-ledger-and-demo-script` | The v83 matrix, docs, scripted demo, and tests are recorded as the Scenario Coverage version ledger for #2195. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```

The lane implementation also runs `cargo fmt --all`, `cargo build --workspace --jobs 2`, the focused v83 Rust test, the glTF demo script, and #1/#23 open-state checks before merge.
