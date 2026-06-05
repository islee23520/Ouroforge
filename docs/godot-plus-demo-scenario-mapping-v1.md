# Godot-Plus Demo Scenario Mapping v1

Issue: #780
Status: **GPD12.3.3 mapping contract only.** This document maps the GDD
requirements (`docs/godot-plus-demo-gdd-v1.md`) and acceptance criteria
(`docs/godot-plus-demo-acceptance-criteria-v1.md`) to concrete scenario ids and
expected evidence paths. It does not implement gameplay, scenarios, or assets,
and it does not modify or close #1/#23.

## Canonical scenario source

All mappings target the canonical fixture
`examples/playable-demo-v2/collect-and-exit/`:

- scenario pack: `scenarios/collect-and-exit.json`
  (group `source-contract`, scenario `collect-key-hud-contract`);
- Seed: `seeds/collect-and-exit.yaml`
  (scenario `collect-and-exit-source-smoke`);
- scene: `scenes/collect-and-exit.scene.json` (id `collect-and-exit-scene`).

The legacy `examples/godot-plus-demo-v1/` tree is superseded and is not mapped.

## Requirement → scenario → evidence map

| GDD requirement | Acceptance ref | Scenario id | Expected evidence path / assertion | Status |
| --- | --- | --- | --- | --- |
| Scene identity loads | A1 | `collect-key-hud-contract` | `world_state.sceneId == "collect-and-exit-scene"` | implemented |
| Collect the signal key | A1 | `collect-key-hud-contract` | `world_state.goalFlags.key_collected == true` | implemented |
| Gate opens after key | A1 | `collect-key-hud-contract` | gate `door_open` precondition on `enter_exit`; `goalFlags.exit_reached` requires it | implemented |
| Reach the exit | A1 | `collect-key-hud-contract` | `world_state.goalFlags.exit_reached == true` | implemented |
| HUD readability | A4 | `collect-key-hud-contract` | `world_state.componentModel.hudValues` exists; `collect-and-exit-source-smoke` asserts `componentModel.counts.hudValue > 0` | implemented |
| Demo identity metadata | A4 | `collect-key-hud-contract` | `world_state.metadata.title == "Collect and Exit Vertical Slice Demo"` | implemented |
| Start-state checkpoint | A4, A7 | `collect-key-hud-contract` | `world_state.metadata.startState.checkpointSlot == "demo-start"` | implemented |
| Animation evidence | A5 | `collect-key-hud-contract` | `animation_evidence.0.mode == "sprite_frame"` | implemented |
| Audio intent evidence | A5 | `collect-key-hud-contract` | `audio_evidence.0.name == "player_spawn"` | implemented |
| Frame / perf budget | A6 | `collect-key-hud-contract` | `frame_stats.runtimeFrameBudgetStatus == "within-budget"` | implemented |
| Save / load start-state | A7 | `collect-key-hud-contract` | `runtime_events.events` contains type `runtime.save.loaded` | implemented |
| Deterministic failure: blocked gate | A3 | `blocked-gate-no-key` (planned, #784/#787) | reaching `door` without `key_collected` keeps `door_open == false` and `exit_reached == false`; non-pass verdict | planned |
| Deterministic failure: hazard contact | A3 | `hazard-contact-fail` (planned, #784/#787) | hazard overlap sets `player_alive == false`; non-pass verdict with journal/dashboard context | planned |
| Asset-reference integrity | A13 | n/a (validation + smoke) | `asset validate` passes; `asset-evidence-smoke.test.cjs` renders asset evidence | implemented |
| Studio/dashboard inspect | A8 | n/a (read-model smoke) | `evidence-read-model-smoke.test.cjs` renders dashboard + Studio read models; fixture-local generated roots remain absent | implemented |

"implemented" = the assertion already exists in the canonical scenario pack /
smoke. "planned" = the assertion is owned by the named later issue and must be
added on the same `collect-and-exit` fixture (no parallel demo tree).

## Evidence expectations per scenario

### `collect-key-hud-contract` (win path, implemented)

Expected runtime evidence: `metadata.title`, `metadata.startState`,
`runtimeFrameBudgetStatus: within-budget`, trigger events for key/exit, a
`runtime.save.loaded` event after restoring `demo-start`, HUD values, the
`sprite_frame` animation mode, and the `player_spawn` audio intent. Generated run
ids and dashboard export paths are recorded in PR/issue evidence and are not
committed.

### `collect-and-exit-source-smoke` (Seed scenario, implemented)

Mirrors the win path with `componentModel.counts.hudValue > 0` and the
key/exit/animation/audio assertions. Used as the Seed-level acceptance contract.

### `blocked-gate-no-key` and `hazard-contact-fail` (failure paths, planned)

Owned by #784 (behavior) and #787 (scenario matrix). Each must be deterministic,
reproducible from source inputs, and produce an explicit non-pass verdict with
journal/dashboard context. Failure scenarios never auto-apply a fix, auto-rerun,
or self-approve a source mutation.

## Coverage handoff to later issues

- **#782 core gameplay loop** — keep the win-path assertions (A1, A5, A7) green
  on the canonical fixture.
- **#783 level set** — any added level variant maps to the same assertion shape.
- **#784 behavior** — add the `blocked-gate-no-key` and `hazard-contact-fail`
  failure scenarios and the `player_alive` failure transition.
- **#785 UI/HUD** — keep A4 HUD evidence assertions stable.
- **#786 assets** — keep A5/A13 asset and audio/animation evidence stable.
- **#787 scenario matrix** — consolidate success/failure/key-interaction
  scenarios into the demo scenario matrix.
- **#788–#794, #797** — reuse these scenario ids for QA swarm, agentic
  iteration, Studio walkthrough, export/package, plugin descriptor, comparison,
  performance budget, and governance evidence.

## Boundaries

This mapping does not authorize gameplay/scenario implementation in this PR unit,
new engine surfaces, trusted browser writes, auto-apply/auto-merge/self-approval,
executable plugins, native/store export, or any Godot replacement/parity/
production-ready/commercial-release claim. #1 and #23 remain open.
