# Scenario Coverage v85 — 2.5D Import Verification and Fidelity Report Regression Suite

Scenario Coverage v85 locks the Era P M99 2.5D import verification behavior for #2202. It is a Rust data-plane regression suite over `crates/ouroforge-core/src/gltf_25d_import.rs`, the M99 verification report, and the scripted demo summary in `examples/2-5d-gltf-import-v1/`.

## Boundary

- One-way on-ramp only: source-project/open-text glTF becomes Ouroforge-native skeleton verification evidence, fidelity rows, gap attribution, and Era R re-derivation tasks.
- No live foreign engine bridge, embedded runtime, shipped-build ripping, decompiled-code copying, or finished-game auto-port claim is allowed; no auto-port claim is permitted without a passing oracle.
- Clean-room re-derivation, not translation: behavior-bearing logic, physics, shaders, and VFX remain Era R work until captured oracle evidence passes.
- `stateHashPrimary` is the authoritative deterministic 2.5D verification gate.
- Perceptual SSIM/pixel-diff render evidence is secondary-only corroboration and must satisfy fixture tolerance before the report passes.
- Lossy imports and unsupported facts remain Yellow/Red with explicit gap attribution; they cannot be graded clean.
- Rust owns artifact truth. Elixir/Phoenix Studio is not touched by v85, owns no artifact semantics, and has no trusted write authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v85.m99-verification-report-pass` | The report passes only with matching deterministic state hash and passing secondary render tolerance. |
| `v85.lossy-import-not-clean` | Non-green fidelity rows require explicit gap attribution and remain visible. |
| `v85.no-auto-port-without-oracle` | Forged port/auto-translation claims and non-empty `claimedPortedUnits` fail validation. |
| `v85.deterministic-state-hash-break-fails` | Stale observed hashes and tampered verification report hashes fail. |
| `v85.perceptual-render-secondary-fails-on-tolerance` | Render evidence fails outside SSIM/pixel-diff tolerance and remains secondary-only. |
| `v85.coverage-ledger-and-demo-script` | The v85 matrix, docs, scripted demo, and tests are recorded as the Scenario Coverage version ledger for #2202. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
