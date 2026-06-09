# Scenario Coverage v90 — Legacy Logic Ingestion and Behavioral-Unit Extraction

Scenario Coverage v90 locks Era R M108 legacy logic ingestion and behavioral-unit
extraction against the semantic re-derivation contract. The suite is a
regression suite, not a migration product claim: it verifies that source-owned
C# and degraded IL2CPP-signature metadata are analyzed read-only into native
Ouroforge evidence shapes while unsupported, lossy, or legally unsafe inputs stay
visible as gaps.

## Boundary

- One-way on-ramp only: imported facts become Ouroforge-native IR/evidence or
  re-derivation tasks; no live Unity/Unreal/Godot bridge is introduced.
- Source-project/open-text only; decompiled or ripped source is rejected.
- Re-derivation is not translation: source logic is catalogued to identify
  behavior and coupling, not copied or translated.
- No auto-port and no ported claim without a captured passing oracle.
- Lossy imports, degraded IL2CPP signatures, unknown engine APIs, missing
  attestations, and unsupported features are Yellow/Red with explicit gaps, not
  clean/Green.
- Determinism is locked by a stable analysis digest/state-hash: identical inputs
  produce identical digests regardless of input order, and behavior text changes
  change the digest.
- Rust remains the data plane for parsing, mapping, validation, evidence,
  fidelity reports, and deterministic hashing.
- Elixir/Phoenix Studio is not touched by v90 and has no trusted-write or
  artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v90.accepted-source-catalogs-units-and-couplings` | Eligible C# yields IR nodes, engine touchpoints, units, and re-derivation tasks without translation. |
| `v90.lossy-import-not-green` | Lossy/degraded imports stay Yellow/Red and preserve gaps. |
| `v90.no-auto-port-without-oracle` | Missing oracle means every unit remains not ported. |
| `v90.deterministic-state-hash-break-fails` | Stable digest catches nondeterminism and source behavior drift. |
| `v90.decompiled-source-rejected` | Decompiled/ripped or unattested sources are rejected before extraction. |
| `v90.coverage-ledger-and-boundaries` | The v90 ledger fixture records clean-room/source-only/two-plane guardrails. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
