# Gameplay Ability Action v1

Issue: #614 — State Machine and Ability Action Model v1. This document defines
the GL10.4.2 ability/action model under
[`Gameplay Scripting / Logic System v1`](gameplay-scripting-logic-system-v1.md).

Gameplay Ability Action v1 is a structured data contract for player/NPC
abilities and local gameplay actions. It describes action ids, triggers,
cooldowns, costs, durations, effects, targets, runtime status, evidence links,
and blocked reasons without executing arbitrary code. It is not a script
runtime, production-stable scripting API, plugin loader, command bridge, browser
trusted writer, local server bridge, hosted service, native export path, or
current Godot replacement claim.

## Artifact shape

```json
{
  "schemaVersion": "gameplay-ability-action.v1",
  "abilityPackId": "demo-abilities",
  "scope": "local-fixture",
  "status": "ready",
  "abilities": [],
  "evidenceRefs": []
}
```

Each ability entry is declarative data:

- `id`: stable ability id unique within the pack.
- `actionId`: stable action binding id unique within the pack.
- `label`: display label for docs, dashboard, or Studio inspection.
- `status`: `ready`, `partial`, `blocked`, or `unsupported`.
- `runtimeStatus`: `available`, `active`, `on_cooldown`, `insufficient_cost`,
  `blocked`, or `unsupported`.
- `target`: source-like target metadata such as `entityId`, `componentId`, or
  `sceneRef`. Targets are references, not code.
- `trigger`: one bounded trigger object.
- `cooldown`: optional bounded cooldown with `durationMs` and optional
  `remainingMs`.
- `costs`: bounded resource costs such as stamina, mana, ammo, item, charge, or
  counter costs.
- `durationMs`: optional bounded active duration.
- `effect`: one bounded structured effect object.
- `evidenceRefs` and `blockedReasons`: audit display and blocked/unsupported
  explanation.

## Supported vocabulary for v1 fixtures

| Category | Supported examples |
| --- | --- |
| Triggers | `on_input`, `on_event`, `on_signal`, `on_state_entered`, `on_timer`, `on_contact`, `on_collect` |
| Effects | `movement_impulse`, `set_state`, `emit_signal`, `set_flag`, `damage`, `heal`, `open_door`, `complete_objective`, `spawn_local_effect`, `play_animation` |
| Costs | `stamina`, `mana`, `ammo`, `item`, `charge`, `counter` |
| Runtime statuses | `available`, `active`, `on_cooldown`, `insufficient_cost`, `blocked`, `unsupported` |

The fixture examples include player dash, enemy alert attack, locked/opened door
interaction, hazard pulse, and collect/win-state progression. GL10.4.2 defines
the ability/action model and validation fixtures only. GL10.4.3 owns state and
ability evidence/read-model compatibility.

## Validation expectations

Rust/local validation rejects duplicate ability/action ids, missing triggers or
effects, unsupported triggers/effects/costs, invalid cooldowns, invalid costs,
invalid durations, unsafe targets, unsafe payload keys, traversal-like refs, and
blocked/unsupported statuses without reasons.

## Read-model/export compatibility notes

GL10.4.3 adds `gameplay-ability-action-read-model.v1` as a read-only evidence summary for ability pack status, ability/action ids, runtime status counts, target refs, trigger/effect/cost kinds, linked evidence refs, blocked reasons, and the no-runtime-execution boundary. Scenario, dashboard, Studio, and probe consumers may display that read model without gaining trusted write or runtime dispatch authority.

## Compatibility and generated-state notes

Gameplay ability/action artifacts should be additive to existing Seeds, scenes,
project manifests, runs, scenarios, dashboard exports, Studio read models,
behavior/event/state-machine fixtures, 2D/3D fixtures, and source-like fixtures.
This PR does not add runtime execution, behavior dispatch, browser write
authority, trusted source mutation, or evidence/read-model compatibility beyond
fixture-local validation. Generated ability/action drafts, probe output,
dashboard exports, and local tool state remain ignored unless explicitly
fixture-scoped.

## Non-goals

Gameplay Ability Action v1 does not authorize arbitrary JS/Rust/Python/Lua/WASM
execution, `eval`, dynamic import, production-stable scripting APIs,
secure-sandbox claims, native export, plugin runtime, hosted cloud/server/auth
behavior, source apply, auto-merge, auto-apply, self-approval, public launch
automation, shipped-game maturity, or current Godot replacement claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
