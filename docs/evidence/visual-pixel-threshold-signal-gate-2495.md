# Signal Gate Relay pixel-threshold visual comparison evidence (#2495)

Issue: #2495
M130 phase: Signal Gate Relay dogfood / first-playable visual regression
Stable fixture: `examples/playable-demo-v2/signal-gate-dogfood/visual-comparison-signal-gate-relay.fixture.json`
Run summary: `docs/evidence/visual-pixel-threshold-signal-gate-2495-run.json`

## Closure classification

Closure classification: product-observed complete

This evidence records a fixture-scoped pixel-threshold comparison for the Signal
Gate Relay replay checkpoints `start`, `key-gate`, and `win-exit`. It is scoped to
local Chrome/headless raster output for the captured fixture settings and does
not claim pixel-perfect engine rendering, aesthetic quality, commercial
readiness, Godot parity/replacement, or a shipped-game visual guarantee.

## Fixture settings

| Item | Value |
| --- | --- |
| Live entry | `http://127.0.0.1:8895/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json` |
| Replay | `signal-gate-relay` (see `tools/live-observability-runner/runner.mjs`) |
| Screenshot states | `start`, `key-gate`, `win-exit` |
| Viewport | 756×469 |
| Device scale factor | 1 |
| Color scheme | light |
| Font policy | browser-default sans-serif; no webfont injection; Chrome headless default rasterization recorded by fixture |
| Threshold | `pixel-threshold` with `maxChangedPixels: 256` at 354,564 total pixels |
| Generated visual comparison root (ignored) | `runs/issue-2495/visual-comparison/` |
| Generated live bundle root (ignored) | `runs/issue-2495/live-observability/issue-2495-signal-gate-visual/` |

The threshold rationale in the fixture tolerates minor rasterization and timing
drift while still failing visible layout, sprite, or objective-state regressions
across the bounded checkpoints.

## Fresh browser evidence

Commands used:

```bash
python3 -m http.server 8895 --bind 127.0.0.1
export CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2495
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8895/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json' \
  --run-id issue-2495-signal-gate-visual \
  --out-root runs/issue-2495/live-observability \
  --replay signal-gate-relay \
  --wait-ms 3000
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- render-verdict runs/issue-2495/live-observability/issue-2495-signal-gate-visual --generated-at 2026-06-11T04:05:00Z --write
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/issue-2495/live-observability/issue-2495-signal-gate-visual
```

Validation result:

```text
valid observability bundle: run_id=issue-2495-signal-gate-visual run_kind=runtime artifacts=12
```

## Pixel-threshold comparison result

The generated baselines and fresh captures stayed under ignored `runs/` storage.
The comparison result was an explicit PASS for every scoped checkpoint:

| State | Baseline | Fresh capture | Changed pixels | Threshold | Result |
| --- | --- | --- | ---: | ---: | --- |
| `start` | `runs/issue-2495/visual-comparison/screenshots/state-start-baseline.png` | `runs/issue-2495/visual-comparison/screenshots/state-start.png` | 0 | 256 | PASS |
| `key-gate` | `runs/issue-2495/visual-comparison/screenshots/state-key-gate-baseline.png` | `runs/issue-2495/visual-comparison/screenshots/state-key-gate.png` | 0 | 256 | PASS |
| `win-exit` | `runs/issue-2495/visual-comparison/screenshots/state-win-exit-baseline.png` | `runs/issue-2495/visual-comparison/screenshots/state-win-exit.png` | 0 | 256 | PASS |

## Diagnostics and replay linkage

- `console.jsonl` was empty.
- `world-samples.jsonl` recorded `runtime_diagnostics: []`.
- Final flags: `exit_reached=true`, `gate_open=true`, `key_collected=true`,
  `relay_1_active=true`, `player_alive=true`.
- Rendered live-observability verdict: `product-observed complete` for the live
  bundle.
- The screenshot states are tied to the `input-replay.json` checkpoints and the
  final world sample in the generated live bundle.

## Generated-state audit

- Committed source: fixture JSON, smoke test, this evidence note, and the stable
  run summary JSON only.
- PNG baselines, browser captures, and live bundles stay under ignored `runs/`.
- The fixture references ignored `runs/` paths for reproduction; those paths are
  not tracked artifacts.

#1 and #23 remain open governance anchors.
