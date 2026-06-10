# Runtime Shell HUD Binding Contract (M118.3)

Issue: #2354  
Consumer: runtime shell only.  
API boundary: documented M119.1 `window.__OUROFORGE__` stable APIs; no raw debug-panel scraping.

## Stable API keys consumed

The shell reads these exported runtime API keys only:

- `getWorldState()` - primary read model for scene id, tick, goal flags, runtime events, and pause state.
- `getEvents()` - optional observability cross-check source for recent objective events in tests/harnesses.
- `setInput()` - replay driver for deterministic checkpoint tests.
- `step()` - replay driver for deterministic checkpoint tests.
- `loadScene()` / `whenReady` - bounded scene load paths.
- `pause()` / `resume()` - play-state controls and paused HUD state.
- `createSave()` / `loadSave()` - checkpoint/restart evidence paths for later #2355 work.

The shell does **not** read `#debug.textContent`, parse raw JSON from the page, execute commands, write trusted source, or depend on undocumented globals.

## World-state fields displayed

All fields are read from the object returned by `getWorldState()`:

| HUD element | World-state path | Meaning |
| --- | --- | --- |
| Scene label | `sceneId` plus active scene source | Current bounded scene identity. |
| Tick | `tick` | Fixed-step runtime tick. |
| Key | `componentModel.goalFlags.key_collected` / `coin_collected` / `has_key` | Objective key/inventory collected state. |
| Gate | `componentModel.goalFlags.door_open` / `gate_open` / `exit_open` | Gate/exit availability. |
| Exit | `componentModel.goalFlags.exit_reached` / `level_complete` / `won` | Terminal success state. |
| Player | `componentModel.goalFlags.player_alive` / `alive` | Player alive/fail state. Missing flag defaults to alive for older scenes. |
| Event | `runtimeEvents[]` | Latest scene/trigger/pause/resume event summary. |

## Collect-and-exit checkpoint expectations

The deterministic replay used by `hud-binding.test.cjs` validates these checkpoints against both DOM HUD text and `getWorldState()` samples:

| Checkpoint | Screenshot filename | Replay position | Expected world sample | Expected HUD |
| --- | --- | --- | --- | --- |
| `start` | `screenshots/state-start.png` | tick 0 | `key_collected=false`, `door_open` absent/false, `exit_reached=false`, `player_alive=true` | Key Missing, Gate Closed, Exit Locked, Player Alive |
| `key-collected` | `screenshots/state-key-collected.png` | after right replay to key | `key_collected=true`, `door_open=true` | Key Collected, Gate Open, Exit Ready |
| `gate-open` | `screenshots/state-gate-open.png` | same checkpoint as key for this fixture | `door_open=true`, `exit_reached=false` | Gate Open, Exit Ready |
| `win` | `screenshots/state-win.png` | after right replay to exit | `exit_reached=true`, `player_alive=true` | Run state Win, Exit Reached |

Generated `world-samples.jsonl` and screenshots stay under ignored `runs/` bundles. Committed source records only the deterministic assertions and binding contract.
