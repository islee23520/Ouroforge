# Runtime State Invariant Checker v1

Status: **QA14.5.1/QA14.5.2/QA14.5.3 invariant model, evaluator, evidence writer, and read-model links** for issue #686.

This document defines the bounded model, in-process evaluator, Rust-owned evidence writer, and dashboard/read-model links for runtime invariant checks used by QA/playtest evidence. It does not create background workers, run hidden agents, mutate trusted state, or authorize auto-fix/apply/merge behavior.

## Scope

The v1 model is structured data: `runtime-invariant-model-v1` lists supported invariant specs, and `runtime-invariant-evidence-v1` records pass, fail, unsupported, missing, malformed, or stale outcomes. Paths point to run-relative evidence such as `evidence/scenarios/<scenario-id>/world-state.json`, scenario results, or invariant artifacts under `invariants/`.

QA14.5.2 adds `evaluate_runtime_invariants`, a deterministic Rust evaluator that checks supported invariants against supplied world-state/scenario-result JSON and emits validated `runtime-invariant-evidence-v1` records. Unsupported evidence sources remain explicit `unsupported` outcomes rather than hidden execution.

QA14.5.3 adds `write_runtime_invariant_evidence`, which persists validated invariant evidence into `evidence/scenarios/<scenario-id>/runtime-invariant-evidence-<model-id>.json` and indexes it with `artifact: "runtime_invariant_evidence"`. The run dashboard/read model exposes `runtime_invariants` with status counts, evidence refs, parsed summaries, parsed checks, malformed artifact detection, and stale run-id detection.

Arbitrary expressions, scripts, dynamic code, browser command bridges, and trusted mutations are forbidden.

## Supported invariant types

| Type | Required fields |
| --- | --- |
| `player_in_bounds` | `targetPath`, `boundsPath` |
| `entity_in_bounds` | `targetPath`, `boundsPath` |
| `finite_transform` | `targetPath` |
| `health_non_negative` | `targetPath` |
| `objective_flags_consistent` | `targetPath` |
| `scene_transition_valid` | `targetPath`, `transitionTargetPath` |
| `no_impossible_state` | `targetPath` |
| `required_entity_present` | `targetPath`, `requiredEntityId` |
| `behavior_state_consistent` | `targetPath`, `behaviorStatePath`, `allowedStates` |

Non-passing statuses require a bounded `message`. Passing checks must not include a message so success does not hide caveats.

## Evidence/read-model behavior

- Missing invariant evidence yields `runtime_invariants.present = false` and `status = "missing"`.
- Malformed JSON or invalid invariant evidence yields `status = "malformed"` and increments `malformed_count`.
- Evidence whose `runId` does not match the current run is rejected by the trusted writer and reported as `status = "stale"` if already indexed as an external/malformed fixture.
- Parsed checks keep their `evidenceRefs` so dashboard/Studio surfaces can link invariant results to scenario, evaluator, world-state, and read-model artifacts without performing trusted writes.

## Fixtures

- `examples/runtime-invariant-checker-v1/invariant-model.sample.json`
- `examples/runtime-invariant-checker-v1/invariant-evidence.sample.json`
- `examples/runtime-invariant-checker-v1/invalid/unsafe-expression.runtime-invariant.json`

The invalid fixture demonstrates that arbitrary expression fields are rejected by schema validation instead of interpreted.

## Guardrails

- No hidden background workers, unbounded spawning, remote/cloud swarm, or unsupervised loop.
- No auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, or trusted mutation.
- No browser trusted write, command bridge, local server bridge, hidden command execution, credentialed command, network/install command, dependency mutation, dynamic code loading, plugin loading, or visual scripting behavior.
- QA/playtest outputs are evidence and backlog inputs only until reviewed.
- Generated runs, fuzz inputs, screenshots, videos, traces, dashboard exports, and local tool state remain ignored unless explicitly fixture-scoped.
- Public wording remains conservative: invariant checks do not prove fun, subjective quality, market readiness, production safety, accessibility compliance, Godot replacement status, or production-ready status.
- #1 and #23 remain open.

## QA14.5.2 evaluator boundary

The evaluator accepts already-captured JSON evidence and returns structured invariant evidence. It does not launch browsers, spawn workers, write run files, open network connections, execute scripts, apply mutations, or infer subjective quality. Missing target state is reported as `missing`; malformed target state is reported as `malformed`; unsupported evidence refs are reported as `unsupported`.

Focused tests cover pass, fail, unsupported, missing, and malformed outcomes for the supported invariant types.

## QA14.5.3 persistence/read-model boundary

The Rust writer owns trusted persistence and evidence index updates. Browser/dashboard/Studio surfaces display invariant evidence only; they must not write trusted state, execute commands, launch workers, auto-fix, auto-apply, auto-merge, or claim gameplay quality guarantees.

Focused tests cover evidence writer indexing, read-model links, missing evidence, malformed artifacts, stale run refs, and dashboard escaping/display of runtime invariant summaries and checks.
