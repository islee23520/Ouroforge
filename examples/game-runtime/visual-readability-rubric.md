# Runtime Shell Visual Readability Rubric (M120.1)

Issue: #2359  
Closure classification target: contract-complete.  
Scope: dogfood runtime scenes and shell screenshots; not production art, AAA polish, marketplace assets, or public release quality.

All criteria are judged from named screenshots defined by `runtime-shell-ux.md`. A criterion that cannot be judged from a screenshot is out of scope.

## Canonical screenshot states

- `start` -> `screenshots/state-start.png`
- `key-collected` -> `screenshots/state-key-collected.png`
- `gate-open` -> `screenshots/state-gate-open.png`
- `win` -> `screenshots/state-win.png`
- `fail` -> `screenshots/state-fail.png`
- `paused` -> `screenshots/state-paused.png`
- `restarted` -> `screenshots/state-restarted.png`

## Pass/fail criteria

| ID | Criterion | Named screenshot states | Pass condition | Fail examples |
| --- | --- | --- | --- | --- |
| VR-01 viewport framing | `start`, `key-collected`, `gate-open`, `win`, `fail`, `paused`, `restarted` | The game canvas is visually framed, centered in the shell, and not crowded by raw debug text. | Bare canvas/debug page, raw JSON above the game, or clipped viewport. |
| VR-02 playable scale | `start`, `key-collected`, `gate-open`, `win`, `fail` | Player, key/objective, gate/exit, and hazards are large enough to identify at screenshot size. | Actor/objective are indistinguishable pixels or hidden by HUD/debug text. |
| VR-03 camera/objective framing | `start`, `key-collected`, `gate-open`, `win`, `fail` | The current objective area or player path is visible enough to understand next action. | Player or objective is offscreen without visible explanation. |
| VR-04 HUD contrast | `start`, `key-collected`, `gate-open`, `win`, `fail`, `paused`, `restarted` | HUD labels and values are readable against their background and use text in addition to color. | Low-contrast HUD, color-only success/fail cue, or unreadable tiny labels. |
| VR-05 objective readability | `start`, `key-collected`, `gate-open`, `win`, `fail`, `restarted` | Status/HUD communicates key/gate/exit/player state without opening debug JSON. | User must inspect raw JSON to know objective state. |
| VR-06 actor/object distinction | `start`, `key-collected`, `gate-open`, `win`, `fail` | Player, collectible, exit/gate, and hazard/blocking object are visually distinct by shape, placement, label, or color+contrast. | Collectible and exit share confusing visuals or merge with background. |
| VR-07 tile/sprite consistency | `start`, `key-collected`, `gate-open`, `win`, `fail` | Scene visuals look like one intentional small-game fixture, with coherent tile/sprite scale and no mixed placeholder clutter dominating the shot. | Random debug rectangles dominate, inconsistent scale makes state unreadable. |
| VR-08 feedback state clarity | `key-collected`, `gate-open`, `win`, `fail`, `paused`, `restarted` | The screenshot visibly communicates the state transition: collected key, open gate, win, fail/blocked, paused, or reset. | State transition is only in hidden logs or indistinguishable from `start`. |
| VR-09 debug secondary | `start`, `key-collected`, `gate-open`, `win`, `fail`, `paused`, `restarted` | Evidence/debug JSON is collapsed or visually secondary and cannot be mistaken for the game UI. | Raw JSON is the primary visible page content. |
| VR-10 restart recovery | `restarted` | Restarted screenshot shows initial objective values and a reset/restarted message. | Restart appears as stale win/fail state or silently keeps collected flags. |

## Rubric use

- #2360 reports pass/fail for `start`, `key-collected`, `win`, and `fail` at minimum.
- #2361 may capture generated screenshots under ignored `runs/` and compare against this rubric/report.
- The rubric is pass/fail and screenshot-observable only; it does not require commercial art quality or pixel-perfect baselines.
