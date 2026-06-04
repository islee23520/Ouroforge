# QA Scenario Candidate v1

Status: **QA14.2.1 scenario candidate artifact** for issue #683.

`qa-scenario-candidate-v1` models QA scenario generation as reviewable, untrusted data before any run occurs. A candidate records source risk, target objective, source Seed/scenario-pack refs, bounded input strategy, assertions, expected evidence, budget, priority, status, blocked reasons, and guardrails.

The artifact does not spawn workers, run browsers, execute commands, mutate trusted state, or authorize auto-fix/apply/merge behavior. Candidates remain evidence/backlog inputs until reviewed or selected by an explicitly bounded run policy.

Fixtures:

- `examples/qa-scenario-candidate-v1/scenario-candidate.sample.json`
- `examples/qa-scenario-candidate-v1/invalid/overbroad-candidate.json`
- `examples/qa-scenario-candidate-v1/invalid/unsupported-candidate.json`
- `examples/qa-scenario-candidate-v1/invalid/blocked-candidate.json`

Guardrails:

- Candidate generation is bounded planning, not autonomous execution.
- Unsupported assertions, unsafe refs, missing targets/objectives/evidence, duplicate ids, overbroad budgets, and blocked candidates without reasons fail closed.
- Generated runs, fuzz inputs, screenshots, videos, traces, dashboard exports, and local tool state remain ignored unless explicitly fixture-scoped.
- Public wording remains conservative: no fun, subjective quality, market readiness, production safety, accessibility compliance, Godot replacement, or production-ready claim.

## QA14.2.2 validation hardening

QA14.2.2 adds fail-closed validation for duplicate expected output paths, assertion/evidence coverage, unsupported assertion/operator combinations, high-priority manual-review overbreadth, stale Seed/scenario-pack refs, stale source scenario ids, and indexed source-risk evidence refs. Candidate refs remain local validation evidence only; they do not execute scenarios or trust candidate outputs.

## QA14.2.3 linkage/read model

QA14.2.3 exposes scenario candidates through the run dashboard/read model as `qa_scenario_candidates`. The read model reports missing/malformed/proposed/blocked/deferred status, priority counts, evidence refs, parsed candidates, and a conservative read-only boundary. Dashboard/Studio display is inspect-only and must not run candidates, spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.
