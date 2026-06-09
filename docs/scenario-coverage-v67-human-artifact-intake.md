# Scenario Coverage v67 — Human-Authored Artifact Intake

Coverage v67 locks Era M M76 human-authored artifact intake behavior as a regression suite for read + gated-write intake, no raw bypass, and autonomous-first fallback.

## Boundary

- Human-authored artifacts enter only as `intervention-as-evidence` with `author=human` provenance (author=human provenance).
- Every human write is normalized and routed through the existing review/apply, scene/source-apply, evaluator, and evidence/provenance gates before review/apply readiness.
- Studio/Phoenix captures and renders local control-plane requests only; it never performs direct artifact writes, owns validation, or becomes trusted write authority.
- Rust remains the data plane for validation, determinism, evidence, provenance, and artifact semantics.
- No raw bypass token, trusted Studio write, direct artifact write, or mandatory-human dependency can become review/apply ready.
- The autonomous default loop completes with zero human input; human artifact intake is opt-in.
- CLI fallback remains sufficient, hosted/multi-user Studio remains deferred, and #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `human-artifact-intake-routes-through-existing-gates` | Human-authored content is ready only after review/apply, scene/source-apply, evaluator, and evidence/provenance gates pass. |
| `no-raw-bypass-from-human-or-studio` | Raw bypass flags, direct writes, trusted Studio writes, and bypass refs are rejected. |
| `loop-completes-without-human-input` | The autonomous default path does not require the human intake surface. |
| `presentation-plane-cannot-own-truth` | Elixir emits a Rust CLI submission and contains no artifact-write authority. |
| `coverage-v67-boundaries` | The suite records local-first, two-plane, read + gated-write, no-new-store, and #1/#23 governance boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```
