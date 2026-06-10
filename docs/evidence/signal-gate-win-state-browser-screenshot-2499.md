# Signal Gate Relay win-state browser screenshot evidence (#2499)

Issue: #2499
M130 phase: #2391 first playable / Signal Gate Relay
Generated run root: `runs/m130/2391-first-playable/m130-2391-signal-gate-win-2499/`

## Closure classification

Closure classification: product-observed complete for the bounded win-state screenshot gap only.

This evidence resolves the M130 ledger gap `m130-2391-win-state-browser-screenshot` by adding a fresh live browser screenshot at the terminal Signal Gate Relay win state. It does not expand the M130 non-goals and does not claim commercial readiness, native export, hosted collaboration, secure sandboxing, Godot parity, public release automation, browser trusted writes, command bridges, self-approval, auto-apply, or auto-merge.

## Product-observed checklist trace

| Checklist item | Result | Evidence |
| --- | --- | --- |
| `po-check-live-url` | PASS | `manifest.json` records local URL `http://127.0.0.1:8879/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json`. |
| `po-check-console` | PASS | `console.jsonl` has zero warning/error lines; `world-samples.jsonl` has zero runtime diagnostics. |
| `po-check-screenshot` | PASS | `screenshots/final.png` captures the terminal win state; `screenshots/start.png` captures the starting state. |
| `po-check-replay` | PASS | `input-replay.json` uses replay `signal-gate-relay` and reaches `win-exit`. |
| `po-check-world-sample` | PASS | `world-samples.jsonl` records `exit_reached=true`, `gate_open=true`, `key_collected=true`, and `player_alive=true` at tick 123. |
| `po-check-event-sample` | PASS | `events.json` records page-load plus runtime event samples from the exercised browser surface. |
| `po-check-frame-stats` | PASS | `frame-stats.jsonl` records runtime frame stats with `runtimeFrameBudgetStatus=within-budget`. |
| `po-check-before-after` | PASS | The replay sequence records start, relay, key/gate, and win-exit checkpoints in one bundle. |
| `po-check-verdict` | PASS | Rendered `verdict.md` reports `product-observed complete` for this scoped screenshot evidence bundle. |
| `po-check-generated-state` | PASS | Generated artifacts remain under ignored `runs/`; tracked changes are limited to stable fixtures/docs/tool support. |

## Evidence bundle facts

- Bundle: `runs/m130/2391-first-playable/m130-2391-signal-gate-win-2499/`
- Screenshot resolving the gap: `runs/m130/2391-first-playable/m130-2391-signal-gate-win-2499/screenshots/final.png`
- Validation: `valid observability bundle: run_id=m130-2391-signal-gate-win-2499 run_kind=runtime artifacts=10`
- Rendered verdict: `Product observation: product-observed complete`
- Final flags: `exit_reached=true`, `gate_open=true`, `key_collected=true`, `relay_1_active=true`, `player_alive=true`
- Runtime diagnostics: none recorded in `world-samples.jsonl`

## Generated-state audit

Generated browser bundle files are intentionally not committed. The stable M130 fixture now references the generated path so reviewers can reproduce or inspect a local run without moving screenshots into trusted source.

#1 and #23 remain open governance anchors.
