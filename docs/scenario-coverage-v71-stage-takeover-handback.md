# Scenario Coverage v71 — Stage Takeover and Handback

Coverage v71 locks Era M M80 stage takeover and handback behavior with regression assertions for gated manual work, no raw bypass, and autonomous-first fallback.

## Boundary

- Human takeover is opt-in at a bounded stage and never required for the autonomous loop.
- Manual work is captured as evidence and provenance metadata, not as a trusted Elixir artifact write.
- Handback can resume only after Rust CLI validation, record, and reverify phases accept the manual work.
- Elixir/OTP and Phoenix LiveView are local control and presentation only; Rust remains the data plane for validation, determinism, evidence, provenance, and artifact semantics.
- Studio surfaces are read + gated-write; no raw artifact, ledger, evidence, scene, source, release, merge, or deploy write authority exists in Elixir.
- No hosted, multi-user, collaborative, or real-time remote Studio behavior is introduced.
- CLI fallback remains sufficient, and a no-human run completes without waiting.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `stage-takeover-locks-local-session-only` | Stage lock/unlock is local executor control-plane state and grants no artifact authority. |
| `manual-work-captured-as-evidence` | Human work requires base, evidence, and provenance refs before handback. |
| `handback-reverifies-through-rust-gates` | Handback requires validation, record, and reverify evidence through the Rust CLI boundary. |
| `no-raw-bypass-from-elixir-control-plane` | Elixir does not perform direct trusted writes or contain raw bypass fixtures. |
| `loop-completes-without-human-input` | The autonomous default path completes with no human surface or wait. |
| `coverage-v71-boundaries` | The suite records local-first, two-plane, read + gated-write, no-new-store, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
