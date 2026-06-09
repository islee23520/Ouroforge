# Scenario Coverage v73 — Onboarding, Templates, In-Product Docs, and First-Run

Coverage v73 locks Era N M83 onboarding behavior with regression assertions for the local template gallery, sample seed references, in-product first-run docs, Rust-owned gate evidence, no raw bypass, and autonomous-first fallback.

## Boundary

- Template gallery selection is opt-in intervention-as-evidence and never required for the autonomous loop.
- Templates and first-run docs expose real local project, seed, and docs references plus copyable CLI commands only.
- Studio onboarding state is read + gated-write; it cannot write artifacts, append ledgers/evidence, apply proposals, execute a browser command bridge, or certify evaluator truth.
- Every human write-affecting action routes through existing Rust data-plane gates: project validation, run evidence, evaluator evidence, and later review/apply or scene/source-apply when changes exist.
- Elixir/OTP and Phoenix LiveView are local control and presentation only; Rust remains the data plane for validation, determinism, evidence, provenance, review/apply, and artifact semantics.
- No hosted, multi-user, collaborative, real-time remote Studio, new data store, or raw artifact write path is introduced.
- CLI fallback remains sufficient, and a no-human run completes without waiting for onboarding UI.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `template-gallery-exposes-real-local-seeds` | Gallery entries point at repository-local project, seed, and docs references. |
| `first-run-commands-are-copy-only-cli` | First-run steps expose copyable commands but no command bridge or Studio execution authority. |
| `human-write-routes-through-existing-rust-gates` | Project validation, run evidence, evaluator evidence, and review/apply remain required gates. |
| `no-raw-bypass-from-elixir-onboarding-surface` | Raw write/apply bypass strings and trusted Studio write authority fail closed. |
| `loop-completes-without-human-input` | The autonomous default path completes with no human surface or wait. |
| `coverage-v73-boundaries` | The suite records local-first, two-plane, read + gated-write, no-new-store, no-hosted-collab, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
