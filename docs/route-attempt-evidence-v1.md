# Route Attempt Evidence v1

Status: **QA14.6.1 route attempt artifact** for issue #687.

`route-attempt-evidence-v1` records bounded attempts to complete an objective through explicit local playtest strategies. It is evidence, not proof of global solvability or game quality.

The artifact records objective id, scenario id, start state, strategy id/kind, bounded action sequence, route nodes, outcome, blockers, evidence refs, budget used, unsupported reason, and guardrails.

Supported outcomes are `passed`, `failed`, `blocked`, `inconclusive`, and `unsupported`. Supported strategy kinds are `simple_heuristic`, `graph_search`, and `manual_trace`.

Fixtures:

- `examples/route-attempt-evidence-v1/route-attempt-success.sample.json`
- `examples/route-attempt-evidence-v1/invalid/blocked-route-attempt.json`
- `examples/route-attempt-evidence-v1/invalid/unsupported-route-attempt.json`
- `examples/route-attempt-evidence-v1/invalid/malformed-route-attempt.json`

Guardrails:

- Route attempts are bounded evidence/backlog inputs until reviewed.
- The contract does not run solvers, spawn workers, execute browser commands, mutate trusted state, or authorize auto-fix/apply/merge behavior.
- Unsupported and inconclusive outcomes are explicit; no complete-solver or quality guarantee is implied.
