# Current collect-and-exit live audit (#2351)

Status: `contract-pass` / `product-observed FAIL`.

This is the first formal product-observed audit for the current collect-and-exit runtime page after the M116 live observability harness and M117.1 taxonomy landed. It preserves the corrected #1 semantics: a replay can satisfy the mechanical contract while the product observation remains failed for practical game-engine usability.

## Live evidence bundle

Generated bundle layout: `runs/live-observability/collect-and-exit-2351-live-audit/`.

Command shape:

```bash
python3 -m http.server 8871 --bind 127.0.0.1
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-e-2351 \
  node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8871/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json' \
  --run-id collect-and-exit-2351-live-audit \
  --replay collect-and-exit \
  --wait-ms 1000 \
  --retries 1
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-e-2351 \
  cargo run --quiet --manifest-path crates/ouroforge-observability/Cargo.toml -- \
  render-verdict runs/live-observability/collect-and-exit-2351-live-audit \
  --generated-at 2026-06-10T07:30:00Z \
  --write
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-e-2351 \
  cargo run --quiet --manifest-path crates/ouroforge-observability/Cargo.toml -- \
  validate runs/live-observability/collect-and-exit-2351-live-audit
```

Validation output:

```text
valid observability bundle: run_id=collect-and-exit-2351-live-audit run_kind=runtime artifacts=10
```

## Observed facts

- Target URL: `http://127.0.0.1:8871/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json`.
- Screenshots: `screenshots/start.png`, `screenshots/final.png` in the generated bundle.
- Runtime replay: `input-replay.json` records three objective checkpoints: `start`, `after-key`, and `after-exit`.
- Final objective flags: `key_collected=true`, `door_open=true`, `exit_reached=true`.
- Console warning/error lines: `0`.
- Runtime diagnostics: `missing_asset` for `collect_and_exit_sheet` / `assets/sprites/collect-and-exit-sheet.png`.
- Frame stats: `frame-stats.jsonl` present with one sampled line.
- Event sample: `events.json` present with eleven event entries.
- Visible page state: the page title is `Ouroforge Minimal 2D Runtime Foundation`, it explains `window.__OUROFORGE__`, shows a small canvas-like probe, and exposes raw world-state JSON beneath the viewport.

## Verdict

The current page is `product-observed FAIL` for practical game-engine usability. The replay proves the current harness can drive the collect-and-exit contract to completion, but the live product surface still presents as a runtime/debug probe with raw JSON, no discoverable play shell, and a missing sprite asset diagnostic. These are gap/backlog inputs, not soft-pass conditions.

## Gap ledger

Machine-readable ledger: `docs/product-observed/collect-and-exit-live-audit-gap-ledger.json`.

The ledger uses category/severity ids from `docs/product-gap-taxonomy.json` and maps every finding to one owning milestone from M118-M130.

## Generated-state policy

The live bundle, screenshots, browser profile, and local server output are generated evidence and remain outside tracked source under ignored `runs/live-observability/`. This document records the reproducible command and observed facts only.

#1 and #23 remain open governance anchors.
