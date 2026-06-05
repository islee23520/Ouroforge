# Godot-Plus Demo GDD v1 (Signal Gate / Collect and Exit)

Issue: #780
Status: **GPD12.3.1 design contract only.** This Game Design Document (GDD) turns
the #779 design pillars and playable success criteria into a concrete,
implementable description of the Godot-Plus Demonstration Game v1 vertical slice.
It does **not** implement gameplay, add assets, run QA, export/package builds,
mutate source through Studio, create executable plugins, publish a release, or
change the #1/#23 governance anchors.

## Canonical implementation target

The demo is implemented on the existing canonical fixture
`examples/playable-demo-v2/collect-and-exit/` and the Signal Gate design notes in
`docs/playable-demo-v2-collect-and-exit.md`. The legacy
`examples/godot-plus-demo-v1/` tree is superseded and must not be recreated. When
this GDD or a follow-up issue body references an older path or name, the canonical
target remains the `playable-demo-v2/collect-and-exit` fixture.

Follow-up implementation issues (#782 and later) extend this single source
fixture in small reviewable PR units; they do not fork a parallel demo tree.

## Game summary

**Signal Gate** is a single-screen, top-down/side-readable 2D action-puzzle
escape vertical slice. The player enters a compact one-screen arena, collects a
signal key, the gate opens once the key is held, and the player exits through the
opened gate. The slice is intentionally tiny: one screen, one objective chain, no
campaign, no procedural generation, no inventory, no dialogue, no network, and no
account behavior.

The slice exists to demonstrate a scoped evidence-native agentic workflow — local
runtime, deterministic scenario evidence, Studio inspect/draft surfaces,
review-gated source mutation handoff, export/package verification, inert plugin
descriptors, and conservative governance. It is **not** a full Godot replacement,
full Godot parity, production-ready engine/editor, secure sandbox, hosted service,
or commercial release.

## Controls

Controls follow the scene `input` component (`scheme: keyboard`, `moveSpeed: 3`,
`allowedActions: ["move", "jump"]`).

| Input | Action | Notes |
| --- | --- | --- |
| Arrow keys / WASD (left/right) | Move the player along the bounded arena floor | Bounded by walls and screen bounds (320×180). |
| Up / jump key | Jump impulse (`jumpImpulse: 8`) | Optional traversal aid; the win path does not require precise platforming. |

Browser/runtime surfaces remain read-only over local evidence; controls drive the
local runtime only and never act as a command bridge or trusted write path.

## Objective

Collect the signal key, let the gate open, and reach the exit trigger.

- **Primary objective:** reach `exit_reached = true` after `key_collected = true`
  has opened the gate (`door_open = true`).
- **Secondary readability goal:** every objective state is visible to scenario
  evidence and read-only dashboard/Studio surfaces.

## Core mechanics

Mechanics reuse existing runtime contracts (movement, collision, triggers,
flags, HUD values, animation, audio intent) without adding new engine surfaces.

1. **Move and collide.** The player (`player`) moves on a bounded one-screen
   tilemap with a solid floor (`floor`) and a solid wall (`wall`). Collision uses
   AABB colliders already declared in the scene.
2. **Signal key pickup.** Overlapping the key (`key`, sensor trigger
   `collect_key`) sets `key_collected = true`, sets `door_open = true`, and hides
   the key entity. This is the "collect the signal key" beat.
3. **Gate opens / exit.** The door (`door`, sensor trigger `enter_exit`) requires
   `door_open` before it accepts the player; overlapping the opened gate sets
   `exit_reached = true`. This is the "open the gate and exit" beat.
4. **HUD feedback.** HUD values (`hud_goal`, `hud_key`, `hud_health`) bind to
   the objective flags so key/goal/health readability is inspectable from run
   evidence and read-only surfaces.
5. **Hazard pressure (planned, #784).** A deterministic patrolling hazard
   ("hazard drone") and a `player_alive` failure transition are reserved for the
   Enemy/NPC/System Behavior issue (#784). The hazard is layered onto this same
   `collect-and-exit` fixture; it adds the deterministic failure path and must not
   add arbitrary scripting or executable plugin behavior.
6. **Review-gated iteration.** Any later agentic adjustment (tile, gate position,
   hazard route, behavior parameter) flows only through the existing review-gated
   Safe Source Mutation Apply path, never a direct Studio/browser trusted write.

## Entities

Entities map directly to `scenes/collect-and-exit.scene.json`.

| Entity id | Role | Key components / flags |
| --- | --- | --- |
| `player` | Player avatar | `input` (keyboard move/jump), dynamic `collider`, `status` (HP 3/3, `player_alive`), `animation` (`idle` sprite_frame clip), `audio` (`player_spawn`), `cameraTarget`. |
| `key` | Signal key pickup | sensor `trigger` `collect_key` → sets `key_collected`, `door_open`; hides key. `goalFlag: key_collected`. |
| `door` | Gate / exit | sensor `trigger` `enter_exit` requiring `door_open` → sets `exit_reached`. `goalFlag: exit_reached`. |
| `wall` | Solid boundary | static solid `collider`. |
| `floor` | Ground plane | static solid `collider` spanning the screen. |
| `hud_goal` | Goal readout | `uiText` / `hudValue` bound to `exit_reached`. |
| `hud_key` | Key counter | `hudValue` `key_count` (`0/1`) bound to `key_collected`. |
| `hud_health` | Health readout | `hudValue` `health` (`3/3`) bound to `player_alive`. |
| hazard drone (planned, #784) | Deterministic hazard | adds a failure transition for `player_alive`; not yet present in the fixture. |

State flags (`gameplayRules.flags`): `key_collected`, `door_open`,
`exit_reached`, `player_alive`.

## Levels

The v1 vertical slice ships **one** screen-sized level.

- **Bounds:** 320×180 viewport, 16-px tiles, `collect_and_exit_level` tilemap
  (10×6 grid) with ground, a `key_marker`, and an `exit_marker`.
- **Layout intent:** player spawns left (`spawn: {x:32, y:80}`), the key sits
  mid-arena (`key` at x≈128), the gate/door is right (x≈256), and a wall caps the
  far right (x≈304). The win path is left → key → gate → exit.
- **Scope:** one tutorial-style level only. Additional fixture variants, if any,
  belong to later issues (#783 Demo Level Set) and remain small enough for
  reliable local smoke verification. No campaign, no procedural generation.

## UI / HUD / feedback

- **Goal line** (`hud_goal`): "Goal: collect key and exit", bound to
  `exit_reached`.
- **Key counter** (`hud_key`): `0/1`, bound to `key_collected`.
- **Health** (`hud_health`): `3/3`, bound to `player_alive`.
- **Animation feedback:** the player's `idle` clip provides `sprite_frame`
  animation evidence.
- **Audio intent:** `player_spawn` audio event fires on `scene_loaded`
  (deterministic intent metadata, not a shipped audio mix).

All feedback is designed so that scenario assertions and read-only
dashboard/Studio surfaces can confirm objective state without screenshots alone.

## Win / lose states

- **Win:** `exit_reached = true` (after `key_collected` opened `door_open`). The
  win path is deterministic and replayable from source inputs.
- **Lose / blocked (deterministic failure path):**
  - **Blocked gate:** reaching the door without `key_collected` leaves
    `door_open = false` and `exit_reached = false` — the objective is provably
    not met.
  - **Hazard contact (planned, #784):** hazard contact transitions
    `player_alive = false`, producing a visible, reproducible failure verdict.
- Failure evidence is informative only; it never auto-applies a fix, auto-reruns,
  or self-approves a source mutation.

## Expected feel

Short, readable, and fair: a player should understand the objective within
seconds from the HUD, complete the win path in well under a minute, and find any
failure state (blocked gate / hazard) obvious and reproducible. The target feel
is "tiny complete arcade loop," not depth, length, or production polish.

## Test scenarios (overview)

The GDD-to-scenario mapping is detailed in
`docs/godot-plus-demo-scenario-mapping-v1.md`. At a high level:

- **Win path scenario** — `collect-and-exit-source-smoke` /
  `collect-key-hud-contract`: key collected, gate opened, exit reached, HUD +
  animation + audio + frame-budget + save/load evidence present.
- **Failure path scenarios** — blocked-gate (no key) and (planned, #784) hazard
  contact, each producing a deterministic non-pass verdict with journal/dashboard
  context.
- **Regression coverage** — later issues (#787 scenario matrix, #797-adjacent
  regression suite) keep success/failure/key-interaction scenarios stable.

## Boundaries (restated)

This GDD does not authorize: gameplay implementation in this PR unit; broad Godot
parity / replacement / production-ready engine/editor / secure sandbox /
commercial release claims; public launch, store publishing, signing,
native/mobile/console export, hosted/cloud/account behavior, or credentialed
operation; direct Studio trusted source writes, browser/local command bridges,
arbitrary shell execution, dependency install, network install/update,
CI/workflow mutation, auto-apply, auto-merge, self-approval, or reviewer bypass;
executable plugins, plugin marketplace, or network plugin install/update; or
tracking generated demo outputs, exports, QA runs, screenshots, videos, package
bundles, temp servers, or local tool state unless a later issue explicitly scopes
a fixture artifact.

#1 and #23 remain open; this document does not modify or close them.
