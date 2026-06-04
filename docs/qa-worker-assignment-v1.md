# QA Worker Assignment and Budget Policy v1

Status: **QA14.4.1 worker assignment artifact, statuses, fixtures, and docs** for issue #685.

This document defines a local, bounded QA/playtest worker assignment artifact. It is a planning/evidence contract only: it does not spawn workers, run browsers, execute commands, mutate trusted state, or authorize auto-fix/apply/merge behavior.

## Scope

`qa-worker-assignment-v1` records explicit local worker assignments with:

- `workerId`
- scenario/fuzz target identity and evidence ref
- assigned lane
- budget (`maxRuns`, `maxDurationMs`, `maxArtifacts`, `maxOutputBytes`)
- per-assignment `timeoutMs`
- observed `runCount`
- generated `outputRoot`
- explicit `cleanupPolicy`
- status and blocked reasons

Supported target types are `scenario_candidate` and `fuzz_target`. Supported statuses are `assigned`, `passed`, `failed`, `deferred`, `blocked`, and `exhausted`.

## Boundaries

- Local-first only: no remote/cloud worker pool, hosted orchestration, or hidden background worker.
- Assignment artifacts are untrusted planning/evidence inputs until reviewed.
- Budgets, timeouts, output roots, and cleanup policies must be explicit and bounded.
- Output roots are generated evidence roots under `evidence/qa-workers/<worker-id>/`.
- Browser/dashboard/Studio surfaces may display this artifact but must not execute commands or write trusted state.

## Fixtures

- `examples/qa-worker-assignment-v1/worker-assignment.sample.json`
- `examples/qa-worker-assignment-v1/invalid/unbounded-worker-assignment.json`

The invalid fixture demonstrates that missing/unbounded budget and timeout values are rejected instead of treated as open-ended execution authority.

## Guardrails

- No hidden background workers, unbounded spawning, remote/cloud swarm, hosted orchestration, or unsupervised long-running loop.
- No auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, or trusted mutation.
- No browser trusted writes, command bridge, local server bridge, hidden command execution, credentialed command, network/install command, dependency mutation, dynamic code loading, plugin loading, or visual scripting behavior.
- QA/playtest outputs are evidence and backlog inputs only until reviewed.
- Generated runs, fuzz inputs, screenshots, videos, traces, dashboard exports, temp projects, and local tool state remain ignored unless explicitly fixture-scoped.
- Public wording remains conservative: worker assignment does not prove fun, subjective quality, market readiness, production safety, accessibility compliance, Godot replacement status, or production-ready status.
- #1 and #23 remain open.

## QA14.4.1 boundary

QA14.4.1 adds the artifact model, validation, statuses, fixtures, and documentation. It intentionally does not add worker execution, browser trusted writes, remote orchestration, read-model export, or dashboard/Studio display. Budget/output-root/stale-ref hardening and read-model compatibility remain separate fixed PR units under #685.
