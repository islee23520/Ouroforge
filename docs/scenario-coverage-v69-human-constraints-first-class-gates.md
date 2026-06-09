# Scenario Coverage v69 — Human Constraints as First-Class Gates

Coverage v69 locks Era M M78 human constraints as first-class gates with a regression suite across the Rust evaluator and the local Studio executor surface.

## Boundary

- Agent-first default is preserved: the loop has a no-constraint path that completes with zero human input.
- Human intervention is opt-in and represented only as validated, recorded proposals, constraints, or directives.
- Every human constraint carries review/apply, scene/source-apply, evaluator, and evidence/provenance references before it can affect gate readiness.
- A violating output is blocked by the Rust `humanConstraints` evaluator gate with evidence; a passing output composes with the existing declared-gate-and categories.
- Elixir/OTP and Phoenix LiveView remain local control and presentation planes: they capture, route, and render, but do not own artifact semantics.
- Rust remains the data plane for truth, validation, evidence, provenance, determinism, scene/source-apply, and artifact writes.
- No raw bypass, no new write path, no new data store, no hosted Studio, no multi-user collaboration, and no mandatory human dependency are introduced.
- CLI fallback remains intact; Studio is read plus gated-write only.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `constraints-recorded-through-existing-gates` | Captured constraints must include the existing gate refs before routing to Rust. |
| `violating-output-blocked-with-evidence` | Forbidden mechanic, required style, and budget cap violations fail closed with evidence. |
| `no-raw-bypass-from-elixir-human-constraint-surface` | Elixir capture/render code has no artifact write or trusted write authority. |
| `loop-completes-without-human-input` | The autonomous fallback completes without waiting for a human constraint. |
| `mandatory-human-regression-fails-closed` | Mandatory-human, broken CLI fallback, or trusted-write drift is rejected. |
| `coverage-v69-boundaries` | The suite records two-plane, local-first, no-new-store, no-hosted-collab, and #1/#23 boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```

The companion Rust and Elixir tests assert the no raw-bypass invariant, the autonomous zero-human path, and the evidence-backed human-constraint gate path.
