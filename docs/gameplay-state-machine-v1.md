# Gameplay State Machine v1

Issue: #614 — State Machine and Ability Action Model v1. This document defines
the GL10.4.1 state-machine model under
[`Gameplay Scripting / Logic System v1`](gameplay-scripting-logic-system-v1.md).

Gameplay State Machine v1 is a structured data contract for local gameplay state
changes. It describes state machine ids, target entities/components, declared
states, transitions, guards, entry/exit actions, evidence links, and blocked
reasons without executing arbitrary code. It is not a script runtime,
production-stable scripting API, plugin loader, command bridge, browser trusted
writer, local server bridge, hosted service, native export path, or current
Godot replacement claim.

## Artifact shape

A state-machine artifact is source-like JSON with this top-level shape:

```json
{
  "schemaVersion": "gameplay-state-machine.v1",
  "statePackId": "demo-state-machines",
  "scope": "local-fixture",
  "status": "ready",
  "stateMachines": [],
  "evidenceRefs": []
}
```

Each state machine entry is declarative data:

- `id`: stable state machine id unique within the pack.
- `label`: display label for docs, dashboard, or Studio inspection.
- `status`: `ready`, `partial`, `blocked`, or `unsupported`.
- `target`: source-like target metadata such as `entityId`, `componentId`, or
  `sceneRef`. Targets are references, not code.
- `initialStateId`: declared starting state id.
- `states`: declared states with bounded `entryActions`, `exitActions`, and
  optional blocked reasons.
- `transitions`: declared transitions with `from`, `to`, one bounded `trigger`,
  optional bounded `guards`, optional bounded `actions`, and optional blocked
  reason.
- `evidenceRefs` and `blockedReasons`: audit display and blocked/unsupported
  state explanation.

## Supported vocabulary for v1 fixtures

| Category | Supported examples |
| --- | --- |
| Triggers | `on_event`, `on_signal`, `on_input`, `on_timer`, `on_state_elapsed`, `on_flag_changed` |
| Guards | `flag_equals`, `state_is`, `has_item`, `cooldown_ready`, `entity_alive`, `counter_at_least` |
| Actions | `set_state`, `emit_signal`, `set_flag`, `start_cooldown`, `play_animation`, `spawn_local_effect`, `complete_objective` |
| Statuses | `ready`, `partial`, `blocked`, `unsupported` |

The fixture examples include player dash readiness, enemy patrol/alert, locked
and opened door state, hazard active/inactive, and collect/win progression.
GL10.4.1 defines the state-machine model and validation fixtures only. GL10.4.2
owns ability/action model work. GL10.4.3 owns state/ability evidence and
read-model compatibility.

## Validation expectations

Rust/local validation rejects duplicate machine/state/transition ids, missing
states, invalid initial states, transitions that reference undeclared states,
unsupported triggers/guards/actions, unsafe targets, unsafe payload keys,
traversal-like refs, unbounded state/transition lists, and blocked/unsupported
statuses without reasons.

## Compatibility and generated-state notes

Gameplay state-machine artifacts should be additive to existing Seeds, scenes,
project manifests, runs, scenarios, dashboard exports, Studio read models,
behavior/event fixtures, 2D/3D fixtures, and source-like fixtures. This PR does
not add ability/action schema, runtime execution, behavior dispatch, browser
write authority, or trusted source mutation. Generated state-machine drafts,
probe output, dashboard exports, and local tool state remain ignored unless
explicitly fixture-scoped.

## Non-goals

Gameplay State Machine v1 does not authorize arbitrary JS/Rust/Python/Lua/WASM
execution, `eval`, dynamic import, production-stable scripting APIs,
secure-sandbox claims, native export, plugin runtime, hosted cloud/server/auth
behavior, source apply, auto-merge, auto-apply, self-approval, public launch
automation, shipped-game maturity, or current Godot replacement claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
