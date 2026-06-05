# Godot-Plus Demo Level Set v1

Issue: #783
Status: **GPD12.6 level-set contract.** This document records the deterministic
level set for the Godot-Plus Demonstration Game v1 vertical slice (Signal Gate /
Collect and Exit), on the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`. It builds on the GDD (#780),
scaffold (#781), and core loop (#782). The legacy
`examples/godot-plus-demo-v1/` tree is superseded and is not used. #1 and #23
remain open.

## Level set

The level set (`levels/level-set.json`, schema `demo-level-set-v1`) defines four
bounded one-screen rooms as deterministic configurations over the base scene
`scenes/collect-and-exit.scene.json`. Each level keeps the collect-the-key,
open-the-gate, reach-the-exit objective; difficulty ramps by approach distance and
(at the final tier) a tighter frame budget. No new scene files are committed —
each level is a deterministic spawn/key/gate configuration applied at load time,
so generated run evidence stays untracked.

| Level | Tier | Objective | Spawn → key → gate | Scenario |
| --- | --- | --- | --- | --- |
| First Contact (`L1-first-contact`) | 1 | Tutorial spacing | 32 → 96 → 224 | `level-l1-win` |
| Longer Approach (`L2-longer-approach`) | 2 | Longer approach | 32 → 144 → 248 | `level-l2-win` |
| Tight Window (`L3-tight-window`) | 3 | Far key/gate | 24 → 176 → 272 | `level-l3-win` |
| Final Gate (`L4-final-gate`) | 4 | Full traverse, tighter budget | 24 → 192 → 288 | `level-l4-win` |

## Difficulty ramp

Tier 1 → 4 increases the spawn-to-key-to-gate travel distance across the arena.
Tier 4 additionally tightens the runtime frame budget
(`updateMs 8→6`, `renderMs 16→14`, `evidenceMs 4→3`, `totalMs 20→18`) while
remaining within budget. Every level is winnable from deterministic source
inputs; the ramp adds pressure, not unfair states.

## Scenario coverage per level

Each level carries its own scenario (`scenario.id` + `assertions`) requiring
`key_collected`, `exit_reached`, and `player_alive`. `level-set-smoke.test.cjs`
applies each level's configuration, plays it to a win, and evaluates the level's
assertions, so every level has deterministic per-level scenario coverage.

## Runtime / Studio level-state identification

Each level injects `metadata.levelId` and `metadata.levelTitle`, which the runtime
world state exposes so the runtime probe and read-only Studio/dashboard surfaces
can identify the current level. The smoke asserts `metadata.levelId` matches the
loaded level.

## Verification

```bash
node examples/playable-demo-v2/collect-and-exit/level-set-smoke.test.cjs
```

The smoke proves the level set is bounded (3–5 levels), the difficulty ramp is
monotonic, every level is winnable, per-level scenario assertions pass, the frame
budget holds, and the current level is identifiable. It writes only to a temp dir
outside the repository and fails closed on any committed generated root.

## Boundaries

The level set reuses the existing runtime/scene contract and adds no engine
surface, no committed scene/generated output, no trusted browser write, no
production/native/store export, no executable plugin runtime, and no full Godot
parity / replacement / production-ready / commercial-release claim. #1 and #23
remain open.
