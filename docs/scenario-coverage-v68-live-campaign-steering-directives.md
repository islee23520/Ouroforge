# Scenario Coverage v68 — Live Campaign Steering Directives

Coverage v68 locks Era M M77 live campaign steering directives as a regression suite over the existing local executor and Rust data-plane gates.

## Boundary

- Agent-first default is preserved: the campaign loop has a no-directive path that continues and completes scheduling with zero human input.
- Human intervention is opt-in and represented only as validated, recorded proposals, constraints, or directives.
- Every accepted directive is routed through the constrained Rust CLI validation and record phases before it can affect executor scheduling.
- Elixir/OTP and Phoenix LiveView remain local control and presentation planes: they capture, route, render, and broadcast state, but do not own artifact semantics.
- Rust remains the data plane for truth, validation, evidence, provenance, determinism, scene/source-apply, and artifact writes.
- No hosted, multi-user, collaborative, or remote Studio behavior is introduced.
- No new data store, ledger, write path, validation engine, or artifact authority is introduced.
- CLI fallback remains intact; Studio is read plus gated-write only.
- Fun/taste verdict and release go/no-go remain human Ring 2 decisions.
- Governance anchors #1 and #23 remain open.

## Regression Rows

| Row | Locks |
| --- | --- |
| `live-steering-directives-recorded-through-gates` | Accepted directives must pass Rust CLI validation and record phases and carry evidence references before affecting ready-task order. |
| `no-raw-bypass-from-elixir-control-plane` | Elixir has no trusted write authority and performs no artifact, ledger, evidence, scene, source, release, merge, or deploy writes. |
| `loop-completes-without-human-input` | A no-directive campaign continues autonomously and yields ready work without waiting for a human. |
| `mandatory-human-regression-fails-closed` | Any fixture or state that marks human input as required is rejected by coverage assertions. |
| `pause-is-control-state-not-artifact-mutation` | A validated pause changes only ephemeral executor scheduling state and never mutates artifacts. |
| `coverage-v68-boundaries` | The suite records the two-plane, local-first, gated-write, no-new-store, no-hosted-collab boundaries. |

## Verification

```bash
cargo test --workspace --jobs 2
STU=$(git ls-files '**/mix.exs' | head -1); [ -n "$STU" ] && (cd "$(dirname "$STU")" && mix test) || true
```

The companion Rust and Elixir tests assert the no raw-bypass invariant, autonomous zero-human path, and validated directive evidence path.
