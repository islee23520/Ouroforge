# Signal Gate Relay browser win-state evidence (#2493)

Closure classification: product-observed complete

This note records the #2493 evidence pass for the existing Signal Gate Relay
workflow. It closes only the bounded screenshot/evidence gap: start, relay/key
progress, and win-state screenshots are tied to one live local browser entry
point and one replay transcript. It does not claim pixel-perfect rendering,
commercial readiness, secure sandboxing, Godot parity/replacement, public release
automation, browser trusted writes, command bridges, hidden command execution,
self-approval, auto-apply, or auto-merge.

## Product surface

Local product command:

```bash
python3 -m http.server 8879 --bind 127.0.0.1
```

Live URL recorded in the generated bundle:

```text
http://127.0.0.1:8879/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json
```

## Fresh live run

```bash
export CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2493
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8879/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json' \
  --run-id issue-2493-signal-gate-win-evidence \
  --out-root runs/issue-2493/live-observability \
  --replay signal-gate-relay \
  --wait-ms 3000
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- render-verdict runs/issue-2493/live-observability/issue-2493-signal-gate-win-evidence --generated-at 2026-06-11T03:45:00Z --write
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/issue-2493/live-observability/issue-2493-signal-gate-win-evidence
```

Validation result:

```text
valid observability bundle: run_id=issue-2493-signal-gate-win-evidence run_kind=runtime artifacts=12
```

## Screenshot and replay trace

All generated screenshots remain under ignored `runs/` storage:

| State | Tick | Goal flags | Screenshot |
| --- | ---: | --- | --- |
| `start` | 0 | `player_alive=true`, `relay_1_active=false`, `key_collected=false`, `exit_reached=false` | `runs/issue-2493/live-observability/issue-2493-signal-gate-win-evidence/screenshots/start.png` |
| `relay-1` | 28 | `player_alive=true`, `relay_1_active=true`, `key_collected=false`, `exit_reached=false` | `runs/issue-2493/live-observability/issue-2493-signal-gate-win-evidence/screenshots/progress-relay-1.png` |
| `key-gate` | 58 | `player_alive=true`, `relay_1_active=true`, `key_collected=true`, `gate_open=true`, `exit_reached=false` | `runs/issue-2493/live-observability/issue-2493-signal-gate-win-evidence/screenshots/progress-key-gate.png` |
| `win-exit` | 123 | `player_alive=true`, `relay_1_active=true`, `key_collected=true`, `gate_open=true`, `exit_reached=true` | `runs/issue-2493/live-observability/issue-2493-signal-gate-win-evidence/screenshots/final.png` |

The win-state capture is linked to `input-replay.json` and
`world-samples.jsonl`: the final world sample records `exit_reached=true`,
`gate_open=true`, `key_collected=true`, `relay_1_active=true`, and
`player_alive=true` at tick 123.

## Diagnostics and verdict

- `console.jsonl` was empty: no page-console warning/error lines were recorded.
- `world-samples.jsonl` recorded `runtime_diagnostics: []`.
- `frame-stats.jsonl` recorded `runtimeFrameBudgetStatus=within-budget`.
- Rendered `verdict.md` classification: `product-observed complete` for this
  scoped evidence bundle.

## Generated-state audit

`git status --short --ignored` reported generated live evidence only as ignored
`!! runs/`; screenshots, browser profile state, and live bundles were not added to
trusted source.

#1 and #23 remain open governance anchors.
