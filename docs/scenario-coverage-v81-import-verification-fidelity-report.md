# Scenario Coverage v81 — Import Verification and Fidelity Report Regression Suite

Scenario Coverage v81 locks the Import Verification and Fidelity Report behavior for Era O M92. It is a Rust data-plane regression suite over the M92 demo path and `ImportVerificationReport`; it verifies imported declarative skeleton evidence and records re-derivation work, but it is not a finished-game migration or auto-port claim.

## Boundary

- One-way on-ramp only: source text becomes Ouroforge-owned skeleton verification evidence, fidelity reporting, oracle requirements, and Era R re-derivation tasks; no live foreign engine bridge or embedded Godot/Unity/Unreal runtime is introduced.
- Source-project/open-text only: Godot `.tscn`, `.tres`, and `project.godot` text are accepted; shipped builds, binary ripping, decompiled source, and translated script bodies are out of scope.
- Re-derivation is clean-room, not translation: behavior-bearing scripts, callbacks, signals, inputs, physics events, rendering/VFX hooks, and unsupported engine APIs remain Era R work from observed behavior plus interrogated intent.
- No auto-port and no ported or equivalent claim without a captured passing oracle.
- `openchrome-local-skeleton-smoke` is skeleton-shape evidence only; it is not port equivalence and does not prove logic was migrated.
- Lossy import facts and behavior-bearing gaps remain Yellow/Red with explicit gap summaries; a lossy import cannot be silently graded clean.
- Determinism is locked by stable `sha256:` state hashes over canonical reports: identical source text yields identical verification hashes, source drift changes the hash, and stale/tampered reports fail validation.
- Rust remains the data plane for parsing, mapping, import verification, fidelity reports, oracle requirements, Era R task records, provenance, and deterministic state-hash checks.
- Elixir/Phoenix Studio is not touched by v81 and has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v81.import-verification-report-demo-evidence` | The M92 demo path yields a Rust-owned import verification report with openchrome skeleton smoke evidence, fidelity counts, provenance, Era R re-derivation tasks, and zero `claimed_ported_units`. |
| `v81.lossy-import-not-clean` | Lossy skeleton gaps and behavior-bearing re-derivation work remain Yellow/Red and cannot be laundered into a clean report. |
| `v81.no-auto-port-without-oracle` | The validator rejects auto-translated or ungated `claimed_ported_units` and oracle-missing records with port permission. |
| `v81.deterministic-state-hash-break-fails` | Stable canonical verification state-hashes catch nondeterminism, source drift, and tampered stale-hash reports. |
| `v81.coverage-ledger-and-boundaries` | The v81 ledger fixture records source-only, clean-room, Rust data-plane, no Studio trusted-write, and open-anchor guardrails. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
