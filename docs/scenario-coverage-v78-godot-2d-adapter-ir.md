# Scenario Coverage v78 — Godot 2D Adapter to IR Regression Suite

Scenario Coverage v78 locks the Godot 2D adapter to IR on-ramp for Era O M89. It is a Rust data-plane regression suite for the source-project/open-text Godot subset (`project.godot`, `.tscn`, and `.tres`), not a finished-game migration product claim.

## Boundary

- One-way on-ramp only: Godot source text becomes Ouroforge-native skeleton IR, fidelity evidence, or Era R re-derivation tasks; no live Godot bridge or embedded Godot runtime is introduced.
- Source-project/open-text only: `.tscn`, `.tres`, and `project.godot` are accepted; shipped builds and opaque runtime artifacts are out of scope.
- Re-derivation is clean-room, not translation: scripts, callbacks, signals, input reactions, and physics behavior are inventoried as touchpoints and must be re-derived later from observed behavior plus interrogated intent.
- No auto-port and no ported claim without a captured passing oracle.
- Lossy imports, unsupported nodes, script refs, signal connections, and clean-room hand-off tasks remain Yellow/Red with explicit gaps; they are never silently graded clean.
- Determinism is locked by a stable state-hash over canonical migration IR: identical source text yields identical hashes, and declarative source drift changes the hash.
- Rust remains the data plane for parsing, mapping, validation, fidelity reports, and deterministic hashing.
- Elixir/Phoenix Studio is not touched by v78 and has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v78.godot-source-text-skeleton-import` | Source-owned Godot text imports declarative nodes, resources, input actions, and presentation skeletons into neutral IR without runtime bridging. |
| `v78.lossy-import-not-clean` | Unsupported Godot nodes and logic touchpoints stay Yellow/Red and cannot be graded clean. |
| `v78.no-auto-port-without-oracle` | Demo fidelity reports reject any claimed ported unit until later Ouroforge-native oracle evidence passes. |
| `v78.deterministic-state-hash-break-fails` | Stable state-hash catches nondeterminism and declarative source drift. |
| `v78.coverage-ledger-and-boundaries` | The v78 ledger fixture records source-only, clean-room, Rust data-plane, and no Studio trusted-write guardrails. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
