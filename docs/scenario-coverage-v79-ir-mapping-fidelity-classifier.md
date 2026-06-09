# Scenario Coverage v79 — IR to Ouroforge Mapping and Fidelity Classifier Regression Suite

Scenario Coverage v79 locks the IR to Ouroforge mapping and fidelity classifier for Era O M90. It is a Rust data-plane regression suite for turning source-project/open-text adapter IR into Ouroforge-native candidate skeleton artifacts and fidelity evidence, not a finished-game migration product claim.

## Boundary

- One-way on-ramp only: adapter IR becomes Ouroforge-native candidate scenes, entities, asset mappings, input mappings, oracle records, fidelity evidence, or Era R re-derivation tasks; no live foreign engine bridge or embedded Godot/Unity/Unreal runtime is introduced.
- Source-project/open-text only: the upstream adapter supplies source-owned, text-derived IR; shipped builds, binary ripping, and decompiled code are out of scope.
- Re-derivation is clean-room, not translation: behavior-bearing scripts, callbacks, signals, and physics semantics remain Era R work from observed behavior plus interrogated intent.
- No auto-port and no ported claim without a captured passing oracle.
- Lossy import facts and behavior-bearing gaps remain Yellow/Red with explicit gap summaries; a lossy import cannot be silently graded clean.
- Determinism is locked by a stable state-hash over the canonical mapping artifact: identical IR yields identical hashes, source drift changes the hash, and stale/tampered hashes fail validation.
- Rust remains the data plane for parsing, mapping, validation, fidelity reports, oracle records, and deterministic state-hash checks.
- Elixir/Phoenix Studio is not touched by v79 and has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v79.ir-to-native-mapping-report` | Adapter IR maps into native candidate scenes, entities, assets, inputs, oracle records, and an honest fidelity report without port claims. |
| `v79.lossy-import-not-clean` | Unsupported features, script callbacks, partial physics/collider semantics, and behavioral gaps stay Yellow/Red and cannot be graded clean. |
| `v79.no-auto-port-without-oracle` | The mapping validator rejects auto-translated or ungated `claimed_ported_units` until later Ouroforge-native oracle evidence passes. |
| `v79.deterministic-state-hash-break-fails` | Stable canonical mapping state-hashes catch nondeterminism, source drift, and tampered stale-hash artifacts. |
| `v79.coverage-ledger-and-boundaries` | The v79 ledger fixture records source-only, clean-room, Rust data-plane, no Studio trusted-write, and open-anchor guardrails. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
