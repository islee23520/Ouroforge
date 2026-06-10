# Runtime Shell UX Contract (M118.1)

Issue: #2352  
Closure classification target: contract-complete.  
Scope: `examples/game-runtime/**` only; this is a browser runtime shell contract, not an editor, engine rewrite, release, marketplace, or production-art claim.

## Shell layout contract

The runtime shell is a playable game page first and an evidence page second.

Required regions, in visual order:

1. **Header**: game title, loaded scene label, and run-state badge.
2. **Main play area**: framed pixel-art canvas with stable viewport sizing.
3. **HUD strip**: objective flags, player/alive state, key/gate/exit status, tick/play state, and recent objective event summary.
4. **Controls row**: keyboard affordances plus pause/resume and restart controls once implemented.
5. **Status message**: concise start, progress, win, fail/blocked, paused, or restarted message.
6. **Evidence/debug panel**: collapsed by default; contains raw JSON/debug evidence only when expanded.

The raw JSON/debug payload must not be the primary page experience. `window.__OUROFORGE__` remains available and may only be additively extended.

## Canonical state and screenshot contract

Live bundles use `screenshots/state-<name>.png` for canonical shell states. File names are lowercase ASCII and use hyphen separators. The state name is the semantic verification target; implementation labels may use friendlier copy as long as the screenshot state is unchanged.

| Canonical state | Screenshot path | Required visible UI elements | Pass/fail purpose |
| --- | --- | --- | --- |
| `start` | `screenshots/state-start.png` | Game title; loaded scene; framed canvas; controls help; HUD with initial key/gate/exit values; status copy explaining the objective; debug panel collapsed. | Fails a debug/probe page where raw JSON dominates or core play instructions are missing. |
| `key-collected` | `screenshots/state-key-collected.png` | All `start` elements plus HUD showing key collected/inventory updated; recent event mentioning key collection. | Verifies objective progress is understandable without reading raw JSON. |
| `gate-open` | `screenshots/state-gate-open.png` | All `key-collected` elements plus HUD/status showing the exit/gate is available/open. | Verifies the player can see that the objective advanced to the exit phase. |
| `win` | `screenshots/state-win.png` | All shell chrome plus terminal success message; HUD showing completed objective; restart affordance visible. | Verifies successful completion is visible as a game state, not only as probe data. |
| `fail` | `screenshots/state-fail.png` | All shell chrome plus blocked/failure message; HUD showing player not alive or objective blocked; restart affordance visible. | Verifies loss/blocked states are explicit and recoverable. |
| `paused` | `screenshots/state-paused.png` | All shell chrome plus paused badge/message; pause control reads as resume-capable; canvas remains framed. | Verifies pause is visible and does not look like a frozen or crashed page. |
| `restarted` | `screenshots/state-restarted.png` | Same visible initial objective values as `start`; status/event copy indicates a restart occurred; debug panel still collapsed. | Verifies restart returns objective flags to initial values and reports the reset. |

### Alias handling

Issue text may refer to `win/exit` and `fail/blocked`; the runtime bundle filenames are canonicalized to `state-win.png` and `state-fail.png`. `gate-open` is the canonical state for the exit/gate becoming available before the terminal win state.

## Accessibility and readability basics

- Canvas has a useful accessible label.
- Interactive controls are real buttons/selects with visible focus styles.
- HUD/status text has sufficient contrast against the shell background.
- Color is not the only cue for key/gate/win/fail state.
- Debug details use a native collapsible control or equivalent semantics.

## Trace to implementation issues

- #2353 implements the shell layout, collapsed debug panel, and live `start` screenshot readiness.
- #2354 binds documented runtime state keys to the HUD and records checkpoint screenshots for `start`, `key-collected`, `gate-open`, and `win`.
- #2355 adds pause/restart and terminal win/fail UX, including `paused`, `restarted`, `win`, and `fail` evidence.
- #2359 consumes these named screenshot states for visual rubric criteria.
- #2360 reports rubric pass/fail over `start`, `key-collected`, `win`, and `fail`.
- #2361 captures generated screenshots under ignored `runs/` roots and documents any committed fixture baseline rendering settings.

## Non-goals and closure note

This contract is intentionally minimum-playable and evidence-native. It does not claim commercial polish, a production release, full Godot parity, native export, marketplace assets, remote plugin execution, trusted writes, command bridges, dependency installation, publishing, deployment, signing, uploading, CI/workflow mutation, or broad engine/editor scope.

Completion of #2352 is **contract-complete** unless a later issue attaches product-observed browser evidence.
