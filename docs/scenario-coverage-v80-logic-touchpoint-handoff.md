# Scenario Coverage v80 — Logic Touchpoint Detection and Re-Derivation Hand-off Regression Suite

Scenario Coverage v80 locks the Logic Touchpoint Detection and Re-Derivation Hand-off for Era O M91. It is a Rust data-plane regression suite for inventorying source-project/open-text Godot logic touchpoints and emitting Era R clean-room re-derivation tasks, not a finished-game migration product claim.

## Boundary

- One-way on-ramp only: source text becomes an Ouroforge-owned logic inventory, oracle requirements, fidelity report, and Era R re-derivation tasks; no live foreign engine bridge or embedded Godot/Unity/Unreal runtime is introduced.
- Source-project/open-text only: Godot `.tscn`, `.tres`, and `project.godot` text are accepted; shipped builds, binary ripping, decompiled source, and translated script bodies are out of scope.
- Re-derivation is clean-room, not translation: behavior-bearing scripts, callbacks, signals, inputs, physics events, rendering/VFX hooks, and unsupported engine APIs remain Era R work from observed behavior plus interrogated intent.
- No auto-port and no ported or equivalent claim without a captured passing oracle.
- Lossy import facts and behavior-bearing gaps remain Red with explicit gap summaries; a lossy import cannot be silently graded clean.
- Determinism is locked by a stable state-hash over the canonical hand-off artifact: identical source IR yields identical hashes, source drift changes the hash, and stale/tampered hashes fail validation.
- Rust remains the data plane for parsing, detection, validation, fidelity reports, oracle requirements, Era R task records, and deterministic state-hash checks.
- Elixir/Phoenix Studio is not touched by v80 and has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v80.logic-touchpoint-handoff-report` | Godot adapter IR becomes a deterministic logic touchpoint hand-off artifact with coupling classifications, oracle requirements, Era R tasks, and an honest fidelity report without port claims. |
| `v80.lossy-import-not-clean` | Unsupported features, script refs, input signals, physics/collision signals, and rendering/VFX gaps remain Red and cannot be laundered into a clean Green report. |
| `v80.no-auto-port-without-oracle` | The hand-off validator rejects auto-translated or ungated `claimed_ported_units` and oracle-missing records with port permission. |
| `v80.deterministic-state-hash-break-fails` | Stable canonical hand-off state-hashes catch nondeterminism, source drift, and tampered stale-hash artifacts. |
| `v80.coverage-ledger-and-boundaries` | The v80 ledger fixture records source-only, clean-room, Rust data-plane, no Studio trusted-write, and open-anchor guardrails. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
