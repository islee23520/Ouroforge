# Scenario Coverage v81 — Unity 2D Adapter to IR Regression Suite

Scenario Coverage v81 locks the Unity 2D adapter-to-IR behavior for Era O M93.
This is a Rust data-plane regression suite over `unity_2d_adapter_ir` and the
Unity demo path. It proves source-text skeleton import and fidelity reporting;
it is not engine absorption, a live Unity bridge, or a finished-game auto-port.

## Boundary

- One-way on-ramp only: Unity Force-Text source becomes Ouroforge-owned skeleton
  IR, fidelity records, oracle requirements, and Era R re-derivation tasks.
- Source-project/open-text only: `.unity`, `.prefab`, `.asset`, and `.meta` text
  are accepted. Shipped builds, binary ripping, decompiled source, translated
  script bodies, and embedded Unity runtimes are out of scope.
- `.meta` GUID references are resolved as provenance and asset links; unresolved
  or partial references remain visible fidelity caveats instead of silent drops.
- Re-derivation is clean-room, not translation: MonoBehaviour data, callbacks,
  input reactions, animation events, and physics behavior remain Era R work from
  observed behavior plus interrogated intent.
- No auto-port and no ported or equivalent claim without a captured passing oracle. `claimed_ported_units` must stay empty in the M93 artifacts.
- Lossy import facts and behavior-bearing gaps remain Yellow/Red; a lossy import
  cannot be silently graded clean.
- Determinism is locked by stable `sha256:` state hashes over canonical Unity IR:
  identical Force-Text inputs yield identical hashes, source drift changes the
  hash, and stale/tampered IR fails validation.
- Rust remains the data plane for parsing, mapping records, fidelity reports,
  oracle records, provenance, and deterministic state-hash checks.
- Elixir/Phoenix Studio is not touched by this coverage suite and has no trusted
  write or artifact semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v81.unity-force-text-skeleton-import` | The M93 Unity demo fixture imports Force-Text `.unity`/`.prefab`/`.asset` plus `.meta` references into Rust-owned IR and records zero `claimed_ported_units`. |
| `v81.unity-lossy-import-not-clean` | MonoBehaviour logic, prefab override caveats, and physics re-simulation remain Yellow/Red and cannot be laundered into a clean report. |
| `v81.unity-no-auto-port-without-oracle` | Validators reject auto-translated or ungated `claimed_ported_units` and oracle-missing records with port permission. |
| `v81.unity-deterministic-state-hash-break-fails` | Stable canonical state hashes catch nondeterminism, source drift, and tampered stale-hash reports. |
| `v81.unity-coverage-ledger-and-boundaries` | The v81 ledger fixture records source-only, clean-room, Rust data-plane, no Studio trusted-write, and open-anchor guardrails. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
