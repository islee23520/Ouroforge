# Scenario Coverage v70 — Diagnosis Correction and Intervention Feedback

Coverage v70 locks Era M M79 diagnosis correction and intervention feedback with a regression suite across the Rust data plane and local Studio executor surface.

## Boundary

- Agent-first default is preserved: the loop has a no-correction path that completes with zero human input.
- Human correction is opt-in and represented only as a validated, recorded proposal, constraint, or directive.
- Every accepted diagnosis correction carries review/apply, scene/source-apply, evaluator, and evidence/provenance references before transparent attribution priors can change.
- A corrected attribution improves a subsequent run through deterministic heuristic priors; no opaque ML, no fun/taste automation, and no release go/no-go automation are introduced.
- Elixir/OTP and Phoenix LiveView remain local control and presentation planes: they capture, route, and render, but do not own diagnosis or artifact semantics.
- Rust remains the data plane for truth, validation, evidence, provenance, determinism, diagnosis semantics, scene/source-apply, and artifact writes.
- No raw bypass, no new write path, no new data store, no hosted Studio, no multi-user collaboration, and no mandatory human dependency are introduced.
- CLI fallback remains intact; Studio is read plus gated-write only.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `diagnosis-correction-recorded-through-existing-gates` | Captured corrections must include existing gate/provenance refs before re-attribution. |
| `corrected-attribution-improves-subsequent-run` | Transparent heuristic priors prefer the corrected diagnosis on later attribution. |
| `no-raw-bypass-from-elixir-diagnosis-correction-surface` | Elixir capture/render code has no artifact write, diagnosis semantics, or trusted write authority. |
| `loop-completes-without-human-input` | The autonomous fallback completes without waiting for human correction. |
| `mandatory-human-and-opaque-inference-regressions-fail-closed` | Mandatory-human, broken CLI fallback, trusted-write, opaque-ML, or fun/taste-inference drift is rejected. |
| `coverage-v70-boundaries` | The suite records two-plane, local-first, transparent-prior, no-new-store, no-hosted-collab, and #1/#23 boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```

The companion Rust and Elixir tests assert the no raw-bypass invariant, the autonomous zero-human path, the evidence-backed diagnosis correction gate path, rejected-correction visibility, and transparent heuristic re-attribution.
