# Godot-Plus Demo Enemy / System Behavior v1

Issue: #784
Status: **GPD12.7 behavior contract.** This document records the deterministic
enemy/system behavior archetype for the Godot-Plus Demonstration Game v1 vertical
slice (Signal Gate / Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on the GDD (#780),
scaffold (#781), core loop (#782), and level set (#783). The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and #23
remain open.

## Hazard drone archetype

`behaviors/hazard-drone.json` (schema `demo-behavior-archetype-v1`) defines the
**Signal Gate Hazard Drone**, a deterministic gate-guard archetype expressed
entirely through the existing runtime trigger contract (sensor collider +
`requiredFlags` + `onEnter` `clearFlag`). It adds no engine surface and no
executable behavior code.

### Deterministic state machine

| State | Meaning |
| --- | --- |
| `dormant` | Inactive until the signal key is collected; contact is harmless. |
| `armed` | Activated once `key_collected` is true; contact ends the run (`player_alive → false`). |

Transition: `dormant → armed` when `key_collected == true` (the drone arms the
moment the player collects the key). The transition is driven by the trigger
`requiredFlags: ["key_collected"]`, so it is fully deterministic.

### Collision / interaction effect

The drone's `onEnter` action is `clearFlag player_alive`, so contact with an
**armed** drone sets `player_alive = false` — a deterministic lose condition that
realizes the `lost` game-state from the core-loop contract (#782).

## Scenarios

| Scenario | Drone placement | Outcome |
| --- | --- | --- |
| `hazard-contact-fail` | past the key (x=192) | Collecting the key arms the drone; contact ends the run before the exit (`player_alive=false`, `exit_reached` not reached) — **lose**. |
| `hazard-dormant-pass` | before the key (x=64) | The player passes the drone while it is dormant, collects the key, and exits alive (`player_alive=true`, `exit_reached=true`) — **win with hazard present**. |

The two scenarios share one archetype and one arena; only the drone placement (and
thus whether it is armed at contact) differs, which demonstrates that the behavior
state — not luck — determines the outcome.

## Runtime probe / determinism evidence

`behavior-smoke.test.cjs` injects the archetype over the base scene and asserts:

- the runtime probe exposes the `hazard_drone` entity and its declared `dormant`
  behavior state (`runtimeState` lists the hazard);
- the lose path ends the run before the exit, and the **death frame is identical
  across runs** (deterministic);
- the dormant-pass path wins with the hazard present;
- no generated root is committed (temp-dir only).

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/behavior-smoke.test.cjs
```

## Boundaries

The behavior reuses the existing runtime trigger contract and adds no engine
surface, no executable plugin/behavior code, no committed scene/generated output,
no trusted browser write, no production/native/store export, and no full Godot
parity / replacement / production-ready / commercial-release claim. #1 and #23
remain open.
