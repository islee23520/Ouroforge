# Adversarial Input Fuzzing Plan v1

Status: **QA14.3.1 fuzzing plan artifact** for issue #684.

This contract represents adversarial input fuzzing as a bounded, deterministic plan before any execution. It does not run random fuzzers, spawn workers, execute browser commands, mutate trusted state, or authorize auto-fix/apply/merge behavior.

`adversarial-input-fuzzing-plan-v1` records:

- `inputDomain` with scenario candidate and/or replay evidence references;
- `deterministicSeed` for reproducible input generation;
- explicit `budget` fields: `maxSteps`, `maxRuns`, `maxDurationMs`, `maxArtifacts`, and `maxOutputBytes`;
- a finite `actionSet` limited to supported input/probe planning actions;
- explicit constraints, stop condition, expected evidence, output root, cleanup policy, status, blocked reasons, and guardrails.

Supported actions are `press_key`, `release_key`, `wait_frames`, `replay_step`, and `snapshot_probe`. Supported statuses are `planned`, `blocked`, and `exhausted`.

Fixtures:

- `examples/adversarial-input-fuzzing-v1/fuzzing-plan.sample.json`
- `examples/adversarial-input-fuzzing-v1/invalid/unbounded-fuzzing-plan.json`
- `examples/adversarial-input-fuzzing-v1/invalid/blocked-fuzzing-plan.json`
- `examples/adversarial-input-fuzzing-v1/invalid/unsupported-action-fuzzing-plan.json`
- `examples/adversarial-input-fuzzing-v1/invalid/stale-reference-fuzzing-plan.json` (schema-valid fixture reserved for indexed stale-reference validation)

Guardrails:

- Fuzzing plans are untrusted planning/evidence inputs until reviewed.
- Generated fuzz inputs and run artifacts stay under `evidence/fuzz/<planId>/` or linked `evidence/scenarios/...` locations.
- Missing budgets, missing cleanup policy, unsafe paths, unsupported actions, or missing blocked reasons are blockers.
- The contract makes no claim that fuzzing proves fun, subjective quality, market readiness, production safety, accessibility compliance, Godot replacement status, or production-ready status.
- Browser/dashboard/Studio surfaces may display this evidence but must not spawn workers, execute commands, write trusted state, auto-fix, auto-apply, or auto-merge.

Later QA14.3 PRs harden indexed replay/candidate references and add deterministic fuzz evidence read models. This initial artifact only defines and validates the bounded plan shape.
