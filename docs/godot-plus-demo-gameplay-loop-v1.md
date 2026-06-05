# Godot-Plus Demo Core Gameplay Loop v1

Issue: #782
Status: **GPD12.5.1–3 core loop contract.** This document records the core
gameplay loop for the Godot-Plus Demonstration Game v1 vertical slice
(Signal Gate / Collect and Exit), proven on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on the GDD (#780) and the
scaffold (#781). The legacy `examples/godot-plus-demo-v1/` tree is superseded and
is not used. #1 and #23 remain open.

## Loop definition

The minimum complete loop, all observable through the existing browser runtime
probe contract:

1. **Spawn** — the player starts at the scene spawn (`metadata.startState.spawn`)
   with initial flags `key_collected=false`, `exit_reached=false`,
   `player_alive=true` and the gate closed.
2. **Move / control** — keyboard input drives the player along the bounded arena
   (`input` component, `moveSpeed`).
3. **Primary interaction** — overlapping the key collects it
   (`key_collected=true`).
4. **Objective tracking** — collecting the key opens the gate
   (`door_open=true`).
5. **Win** — overlapping the opened gate reaches the exit
   (`exit_reached=true`). Loop game-state becomes `won`.
6. **Restart / reset** — restoring the `demo-start` checkpoint via the runtime
   save/load API resets the flags to the start state and records a
   `runtime.save.loaded` event.

## Loop game-state

The loop game-state is derived from objective flags:

| Condition | State |
| --- | --- |
| `player_alive == false` | `lost` |
| `exit_reached == true` | `won` |
| otherwise | `in-progress` |

The **objective-blocked** path (player never collects the key) keeps the win
gated: `door_open` stays unset and `exit_reached` stays false, so the loop stays
`in-progress` and is provably not won. This is the deterministic loop-level
non-win condition.

Hazard-contact death (`player_alive -> false`, the `lost` state) is introduced by
the Enemy/NPC/System Behavior issue (**#784**) on this same fixture; the loop
already exposes `player_alive` so #784 only adds the failure transition.

## Runtime probe contract

The runtime probe exposes loop state through:

- `getWorldState().componentModel.goalFlags` — objective flags.
- `getWorldState().object` — the player object (id/transform/size).
- `runtimeState(slot)` (`runtime-state-v1`) — flags, per-entity transform/
  velocity/status (player hit points), input, camera, and a deterministic
  `digest`.
- `getFrameStats().runtimeFrameBudgetStatus` — `within-budget`.

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/gameplay-loop-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
```

`gameplay-loop-smoke.test.cjs` drives spawn -> move -> interact -> objective ->
win -> restart, proves the objective-blocked non-win path, and asserts the
runtime probe exposes objective/player/game-state data. It writes only to a temp
directory outside the repository and removes it before exit; it fails closed if
any generated root appears inside the fixture.

## Boundaries

The loop uses only the existing runtime/scene/scenario contracts. It adds no new
engine surface, no scene mutation, no generated-output tracking, no trusted
browser write, no production/native/store export, no executable plugin runtime,
and no full Godot replacement / full Godot parity / production-ready /
commercial-release claim. #1 and #23 remain open.
