# Scenario Coverage v75 — Human-Grade Studio Regression Suite

Coverage v75 locks Era N Milestone 85 Studio behavior after the Human-Grade Studio demo. The suite covers the unified local Phoenix LiveView control/presentation surface, live read-model observation, integrated gated intervention panels, interactive authoring routing, and the autonomous no-human fallback.

## Boundary

- Studio is local-first and optional; the autonomous CLI loop completes without a human surface.
- Studio renders Rust-owned evidence, diagnosis, journal, verdict, intervention, and authoring read models.
- Studio surfaces are read + gated-write: every write-affecting action is intervention-as-evidence and routes through existing Rust-owned gates.
- Rust remains the data plane for artifact truth, validation, determinism, evidence, provenance, review/apply, scene/source-apply, evaluator verdicts, and artifact semantics.
- Elixir/OTP + Phoenix LiveView is control + presentation only: local PubSub refresh, form capture, routing envelopes, and display feedback.
- No trusted Elixir write authority, direct artifact writes, browser command bridge, new data store, hosted collaboration, no-code/product overclaim, fun/taste automation, or release go/no-go automation is introduced.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `live-shell-renders-rust-owned-read-models` | Evidence, diagnosis, journal, and verdict views are read-only Rust-owned read models. |
| `integrated-panels-route-through-existing-gates` | Steering, amendment, constraint, correction, takeover, handback, and authoring panels route to existing gate families. |
| `authoring-stays-review-apply-and-scene-source-apply` | Interactive authoring drafts never apply directly and always cite review/apply plus scene/source-apply. |
| `trusted-write-and-command-bridge-drift-fails-closed` | Trusted writes, direct artifact writes, command bridges, new stores, and hosted collaboration are rejected. |
| `mandatory-human-regression-fails-closed` | Studio use remains opt-in and the no-human path completes. |
| `demo-observe-intervene-fallback-remains-conservative` | The M85 demo shows live observation, one gated intervention, and autonomous fallback without overclaiming. |
| `coverage-v75-boundaries` | The suite records read + gated-write, intervention-as-evidence, two-plane, local-first, no-new-store, no-hosted-collab, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
