# Scenario Coverage v91 — Tacit-Knowledge Interrogation and Oracle Capture

Scenario Coverage v91 locks Era R M109 tacit-knowledge interrogation and oracle
capture. The suite verifies that clean-room intent answers and observed behavior
traces become source-independent, Rust-owned oracle evidence only when the data is
eligible, deterministic, and provenance-backed.

## Boundary

- One-way on-ramp only: M109 records intent/oracle evidence for later native
  re-expression; it does not translate source code or absorb a foreign engine.
- Source-project/open-text and human-provenance evidence only; decompiled,
  shipped-build, ripped, or live-bridge evidence is blocked.
- No auto-port: captured oracle evidence is re-expression-ready, not a finished
  port claim.
- Oracle-less, partial, lossy, blocked, or nondeterministic records stay
  Yellow/Red with explicit gaps.
- Determinism is locked by state-hash primary evidence and the report digest.
- Rust remains the data plane for artifact truth, validation, fidelity reports,
  deterministic hashing, and oracle specs.
- Elixir/Phoenix Studio may render and route gated human input only; it has no trusted-write or artifact-semantics authority.
- #1 and #23 remain open governance anchors.

## Regression Rows

| Row | Locks |
| --- | --- |
| `v91.captured-oracle-is-reexpress-ready-not-ported` | Captured oracle evidence enables re-expression only, never a port claim. |
| `v91.no-auto-port-without-oracle` | Missing/incomplete oracle evidence stays not Green and not ported. |
| `v91.blocked-provenance-not-green` | Decompiled/ripped/foreign-runtime provenance is Red/blocked. |
| `v91.deterministic-state-hash-break-fails` | State-hash changes alter the deterministic report digest. |
| `v91.rust-owned-no-studio-trusted-write` | Rust owns artifact truth; Studio cannot perform trusted writes. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
