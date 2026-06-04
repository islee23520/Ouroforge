# Gameplay Behavior Model v1

Issue: #612 — Gameplay Behavior Model v1. This document defines the first
structured behavior artifact under
[`Gameplay Scripting / Logic System v1`](gameplay-scripting-logic-system-v1.md).

Gameplay Behavior Model v1 is a data-first contract for common local gameplay
logic. It describes behavior ids, target entities/components, triggers,
conditions, actions, variables, cooldowns/timers, evidence links, and blocked
reasons without executing arbitrary code. It is not a script runtime,
production-stable scripting API, plugin loader, command bridge, browser trusted
writer, hosted service, native export path, or current Godot replacement claim.

## Artifact shape

A behavior model artifact is source-like JSON with this top-level shape:

```json
{
  "schemaVersion": "gameplay-behavior-model.v1",
  "behaviorPackId": "demo-behaviors",
  "scope": "local-fixture",
  "status": "ready",
  "behaviors": [],
  "evidenceRefs": []
}
```

Each behavior entry is declarative data:

- `id`: stable behavior id unique within the pack.
- `label`: display label for docs, dashboard, or Studio inspection.
- `status`: `ready`, `partial`, `blocked`, or `unsupported`.
- `target`: source-like target metadata such as `entityId`, `componentId`,
  `sceneRef`, or `assetRef`. Targets are references, not code.
- `triggers`: deterministic start conditions such as contact, timer, input,
  flag change, state entry, or collect events.
- `conditions`: bounded predicates such as flag state, inventory/objective state,
  timer elapsed, state match, cooldown readiness, or entity alive/dead checks.
- `actions`: bounded effects such as setting a flag, incrementing a counter,
  damaging an entity, opening a door, completing an objective, moving along a
  named path, starting a cooldown, emitting a signal, setting a state, playing a
  declared animation, or spawning a local visual/audio effect.
- `variables`: typed local values used by the behavior model. Values remain JSON
  data and do not contain executable expressions.
- `cooldowns` / `timers`: bounded millisecond timers with explicit ids,
  durations, repeat behavior, and evidence expectations.
- `evidenceRefs`: relative evidence or doc refs that explain why the behavior is
  ready, partial, blocked, or unsupported.
- `blockedReasons`: required when status is `blocked` or `unsupported`.

## Supported vocabulary for v1 fixtures

The initial fixture vocabulary is intentionally small and covers common 2D demo
logic without custom code:

| Category | Supported examples |
| --- | --- |
| Triggers | `on_start`, `on_contact`, `on_collect`, `on_flag_changed`, `on_timer`, `on_input`, `on_state_entered`, `on_signal` |
| Conditions | `flag_equals`, `counter_at_least`, `has_item`, `objective_incomplete`, `timer_elapsed`, `state_is`, `cooldown_ready`, `entity_alive` |
| Actions | `set_flag`, `increment_counter`, `damage`, `heal`, `open_door`, `complete_objective`, `move_along_path`, `start_cooldown`, `emit_signal`, `set_state`, `play_animation`, `spawn_local_effect` |
| Statuses | `ready`, `partial`, `blocked`, `unsupported` |

The fixture examples include patrol, collect item, damage on contact, door opens
on flag, win condition, timed hazard, and simple ability trigger behavior. Later
issues may validate and interpret the vocabulary, but this schema PR does not
add runtime execution.

## Explicitly unsupported data

Behavior artifacts must not contain executable script bodies, `eval`, dynamic
imports, plugin loader instructions, command strings, local server bridge
instructions, browser trusted-write instructions, credential/network/install
commands, unrestricted source-apply instructions, or auto-merge/auto-apply
instructions. Unsupported script-like examples belong only in invalid fixtures as
inert negative test data.

## Compatibility and read-model/export compatibility notes

Gameplay behavior artifacts should be additive to existing Seeds, scenes,
project manifests, runs, scenarios, dashboard exports, Studio read models, 2D/3D
fixtures, and source-like fixtures. Read models expose behavior ids, target
summaries, trigger/condition/action counts, status, evidence refs, and blocked
reasons without interpreting arbitrary code or creating write authority.

The read-model shape is display/export data only:

- `schemaVersion: "gameplay-behavior-model-read-model.v1"`;
- `behaviorPackId` and aggregate `status`;
- `behaviorCount`, `readyCount`, `partialCount`, `blockedCount`, and
  `unsupportedCount`;
- `behaviorIds`, `targetRefs`, `triggerKinds`, `conditionKinds`, and
  `actionKinds` for dashboard/Studio indexing;
- `linkedEvidenceRefs` and `blockedReasons` for audit display;
- a boundary string that explicitly says read-only, no runtime execution, no
  script execution, no command bridge, no browser trusted writes, no source
  apply, and no production-stable scripting API claim.

Rust/local validation owns trusted persistence, behavior draft/apply validation,
generated evidence writing, source-like fixture validation, and CLI contracts.
Browser/dashboard/Studio/cockpit surfaces may render escaped read-only or draft-
only behavior summaries when explicitly scoped; they must not become trusted
writers or command bridges.

## Follow-up validation expectations

GL10.2.2 should validate duplicate ids, unsafe targets, unsupported actions,
invalid conditions, missing referenced flags/entities/assets, invalid cooldown or
timer values, and blocked/unsupported statuses without reasons. GL10.2.3 should
add read-model/export compatibility notes and backward-compatibility tests.

## Non-goals

Gameplay Behavior Model v1 does not authorize arbitrary JS/Rust/Python/Lua/WASM
execution, production-stable scripting APIs, secure-sandbox claims, native
export, plugin runtime, hosted/cloud/server/auth behavior, source apply,
auto-merge, auto-apply, public launch automation, shipped-game maturity, or
current Godot replacement claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
