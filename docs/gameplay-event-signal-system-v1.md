# Gameplay Event and Signal System v1

Issue: #613 — Event and Signal System v1. This document defines deterministic
runtime event and signal primitives under
[`Gameplay Scripting / Logic System v1`](gameplay-scripting-logic-system-v1.md).

Gameplay Event and Signal System v1 is a structured data contract for events
that behavior models may reference later. It describes event ids, event types,
signal names, source/target refs, payload data, tick/timestamp fields,
ordering indexes, consumed/unconsumed state, blocked reasons, and evidence
links without executing arbitrary code. It is not a script runtime,
production-stable scripting API, plugin loader, command bridge, browser trusted
writer, local server bridge, hosted service, native export path, or current
Godot replacement claim.

## Artifact shape

An event/signal artifact is source-like JSON with this top-level shape:

```json
{
  "schemaVersion": "gameplay-event-signal-system.v1",
  "eventLogId": "demo-event-log",
  "scope": "local-fixture",
  "status": "ready",
  "events": [],
  "evidenceRefs": []
}
```

Each event entry is declarative data:

- `id`: stable event id unique within the log.
- `eventType`: bounded event kind.
- `signalName`: optional signal routing name for behavior references.
- `source`: producer metadata such as `entityId`, `componentId`, `systemId`, or
  `sceneRef`.
- `target`: consumer/affected metadata such as `entityId`, `componentId`,
  `behaviorId`, or `sceneRef`.
- `payload`: JSON data only; no executable expressions, script bodies, command
  text, dynamic import instructions, plugin loader instructions, or trusted
  browser write instructions.
- `tick`: deterministic runtime tick for later ordering/replay.
- `timestampUnixMs`: optional captured timestamp for evidence display.
- `orderingIndex`: deterministic index within the tick or capture batch.
- `consumed`: whether the event was consumed by a scoped behavior/system.
- `consumedBy`: bounded consumer ids, required when `consumed` is true and empty
  when `consumed` is false.
- `blockedReason`: optional visible reason for malformed, unsupported, missing
  target, or unrouteable events.
- `evidenceRefs`: relative evidence or doc refs.

## Supported event types for v1 fixtures

The initial fixture vocabulary covers common gameplay behavior triggers without
custom code:

| Event type | Example producer |
| --- | --- |
| `collision_contact` | collision/contact probe |
| `trigger_entered` / `trigger_exited` | trigger volume |
| `item_collected` | inventory pickup |
| `flag_changed` | runtime flag update |
| `timer_elapsed` | bounded timer |
| `input_action` | declared input action |
| `scene_loaded` | scene lifecycle |
| `state_changed` | state machine transition |
| `behavior_executed` | validated behavior runtime outcome |

GL10.3.1 defines the model and validation fixtures only. GL10.3.2 adds a
read-only deterministic queue summary over validated event artifacts. GL10.3.3
owns event evidence and read-model/export compatibility surfaces.

## Deterministic ordering and bounded queue rules

Validated event artifacts use deterministic display/replay ordering before any
later runtime consumes them:

1. Validate event artifacts before ordering.
2. Sort by `tick`, then `orderingIndex`, then stable `id` as the final tie
   breaker.
3. Keep consumed and unconsumed events visible; unconsumed events are not dropped
   by the queue summary.
4. Bound v1 fixture queues to at most 256 events.
5. Treat the queue summary as read-only compatibility data, not runtime
   execution or event dispatch authority.

The queue summary shape is display/test data only:

- `schemaVersion: "gameplay-event-signal-queue-summary.v1"`;
- `eventLogId`, `eventCount`, `consumedCount`, and `unconsumedCount`;
- `earliestTick` and `latestTick` for deterministic replay displays;
- `orderedEventIds` sorted by `tick`, `orderingIndex`, and `id`;
- `queueRules` and a boundary string that explicitly says no runtime execution,
  no script execution, no command bridge, no browser trusted writes, no source
  apply, and no production-stable scripting API claim.

## Evidence/read-model export compatibility

GL10.3.3 exposes a read-only event/signal read model for dashboard, Studio, and
local evidence export compatibility. The read model summarizes already-validated
event artifacts; it does not emit runtime events, dispatch signals, apply
behavior, mutate source files, or create browser write authority.

The read-model shape is display/export data only:

- `schemaVersion: "gameplay-event-signal-read-model.v1"`;
- `eventLogId`, aggregate `status`, `eventCount`, `consumedCount`, and
  `unconsumedCount`;
- `eventTypeCounts`, `orderedEventIds`, `signalNames`, `sourceRefs`, and
  `targetRefs` for dashboard/Studio indexing;
- `linkedEvidenceRefs` and `blockedReasons` for audit display;
- nested `queueSummary` for deterministic ordering compatibility;
- a boundary string that explicitly says read-only, no runtime execution, no
  script execution, no command bridge, no browser trusted writes, no source
  apply, and no production-stable scripting API claim.

## Validation expectations

Rust/local validation rejects duplicate event ids, unknown event types, missing
source or target metadata, malformed/unsafe payload keys, traversal-like refs,
consumed/unconsumed state drift, unbounded payloads/participants, unsupported
script/plugin/command/trusted-write vocabulary, and blocked/unsupported artifact
statuses without reasons.

## Compatibility and generated-state notes

Gameplay event artifacts should be additive to existing Seeds, scenes, project
manifests, runs, scenarios, dashboard exports, Studio read models, behavior
model fixtures, 2D/3D fixtures, and source-like fixtures. This PR does not emit
runtime event evidence, mutate runtime queues, create browser write authority, or
change existing runtime event compatibility. Generated event logs, probe output,
dashboard exports, and local tool state remain ignored unless explicitly
fixture-scoped.

## Non-goals

Gameplay Event and Signal System v1 does not authorize arbitrary
JS/Rust/Python/Lua/WASM execution, `eval`, dynamic import, production-stable
scripting APIs, secure-sandbox claims, native export, plugin runtime, hosted
cloud/server/auth behavior, source apply, auto-merge, auto-apply, self-approval,
public launch automation, shipped-game maturity, or current Godot replacement
claims.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open.
