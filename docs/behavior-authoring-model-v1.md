# Behavior Authoring Model v1

Behavior Authoring Model v1 is the #2372/M124 contract for structured behavior
authoring without arbitrary scripting. It is a data-only state-machine model for
hazards, NPCs, and system behaviors.

## Model shape

A behavior spec contains:

- `states`: declared state ids and labels.
- `transitions`: `from`/`to` state edges with one trigger and one or more
  actions.
- `triggers`: enumerated trigger kinds plus bounded typed parameters.
- `actions`: enumerated allowed action vocabulary only.
- `parameterSchema`: editable typed parameters (`bool`, integer, decimal,
  text/entity/prefab ids, and `vector2`).
- `preview`: read-only deterministic preview or scenario assertion generation.
- `scenarioAssertions`: deterministic world/runtime assertions linked to the
  authored behavior.
- `draftBoundary`: must be `safe-source-apply-review-required`.
- `evidenceRefs`: runtime evidence references by run id, local artifact path,
  and digest.

## Allowed action vocabulary

The only accepted actions are:

- `set-state`
- `set-flag`
- `emit-event`
- `move-by-vector`
- `move-toward-entity`
- `apply-damage`
- `spawn-entity-from-prefab`
- `despawn-self`
- `start-timer`
- `stop-timer`
- `play-animation`
- `play-sound-cue`
- `set-velocity`
- `clamp-to-patrol-route`
- `open-gate`
- `complete-objective`

Unknown actions fail closed during JSON parsing. Text fields also reject script,
plugin, eval, command bridge, browser trusted write, auto-apply, self-approval,
remote URL, production-ready, and Godot-replacement authority wording.

## Examples supported

- Hazard: `on-player-contact` triggers `apply-damage` and `emit-event`, then
  transitions from `armed` to `cooldown`.
- NPC: `on-distance-less-than` triggers `move-toward-entity` and `set-state`
  from `patrol` to `chase`.
- System behavior: a flag/timer transition can `open-gate` or
  `complete-objective` while remaining data-only.

## Draft/apply boundary

The browser/Studio/editor surface may preview data and generate draft operations
only. Trusted mutation requires Safe Source Apply review; this model does not
execute scripts, load plugins, run commands, mutate trusted files, self-approve,
auto-apply, merge, publish, deploy, or install dependencies.

## Runtime evidence mapping

Every practical behavior claim must map to deterministic evidence:

- expected runtime event ids;
- scenario assertions (`world-state`, `runtime-event`, `world-flag`, or
  `frame-stats`);
- evidence refs with `runId`, local artifact path, and digest.

This issue is contract-complete only. It does not claim live browser/editor
product evidence unless a later issue attaches such evidence under #1 semantics.
