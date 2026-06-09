# Scenario Coverage v72 — Non-Developer Generative Front-Door UX

Coverage v72 locks Era N M82 non-developer generative front-door UX behavior with regression assertions for guided intake, deterministic proposal preview, gated Rust routing, no raw bypass, and autonomous-first fallback.

## Boundary

- Human brief/conversation input is opt-in and never required for the autonomous loop.
- Guided Studio intake captures non-developer intent as intervention-as-evidence, not as a trusted artifact write.
- Proposal preview is deterministic, proposal-only, and cannot apply or promote generated output.
- Every human write-affecting action routes through the Rust generative-front-door validation path and later review/apply, evaluator, evidence, and provenance gates.
- Elixir/OTP and Phoenix LiveView are local control and presentation only; Rust remains the data plane for validation, determinism, evidence, provenance, review/apply, and artifact semantics.
- Studio surfaces are read + gated-write; no raw artifact, ledger, evidence, scene, source, release, merge, deploy, auto-apply, or reviewer-bypass authority exists in Elixir.
- No hosted, multi-user, collaborative, real-time remote Studio, new data store, or browser command bridge is introduced.
- CLI fallback remains sufficient, and a no-human run completes without waiting.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `guided-intake-captures-intervention-evidence` | Non-developer brief/conversation intake is inert intervention-as-evidence with provenance. |
| `proposal-preview-is-deterministic-and-proposal-only` | Preview exposes a proposal candidate and required gates, not trusted write authority. |
| `human-write-routes-through-rust-gates` | Guided submission routes through `generative-front-door validate` and requires later review/apply. |
| `no-raw-bypass-from-elixir-guided-surface` | Elixir Studio files contain no raw file write or trusted-write bypass authority. |
| `loop-completes-without-human-input` | The autonomous default path completes with no human surface or wait. |
| `coverage-v72-boundaries` | The suite records local-first, two-plane, read + gated-write, no-new-store, no-hosted-collab, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
