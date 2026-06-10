# Live observability runner

This directory contains the M116.2 local browser harness runner. It is JS/Node CDP glue only; bundle truth and validation remain owned by the Rust `crates/ouroforge-observability` validator.

## Local-only policy

The runner fails closed unless `--url` is an explicit local HTTP URL:

- `http://127.0.0.1:<port>/...`
- `http://localhost:<port>/...`

Remote hosts, HTTPS, file/data URLs, and URLs without an explicit port are rejected before Chrome launch.

## Usage

Start a local static server from the repository root, then run the collect-and-exit runtime URL:

```sh
python3 -m http.server 8871 --bind 127.0.0.1
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-b-2346 \
  node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8871/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json' \
  --run-id collect-and-exit-2347
```

By default the runner writes `runs/live-observability/<run-id>/` and invokes:

```sh
cargo run --quiet --manifest-path crates/ouroforge-observability/Cargo.toml -- validate <bundle-root>
```

Use `--out-root` for temporary evidence paths and `--validator-manifest` when testing a different checkout. `--skip-validation` is for debugging only; issue evidence must leave validation enabled.

## Generated artifacts

A successful run writes the M116.1 required bundle files:

- `manifest.json` with `schema_version`, `run_id`, `target_url`, `run_kind`, tool/browser metadata, `retry_attempts`, and artifact inventory digests.
- `console.jsonl`
- `frame-stats.jsonl`
- `world-samples.jsonl`
- `events.json`
- `input-replay.json`
- `screenshots/start.png` and, when replay is enabled, `screenshots/final.png`
- `verdict.md` stub

Generated bundles remain ignored under `runs/` unless a later issue explicitly promotes a minimal fixture.

## M116.2 verification evidence

On 2026-06-10, PR 3 verified the runner against collect-and-exit with generated output outside the repo:

```text
valid observability bundle: run_id=collect-and-exit-2347 run_kind=runtime artifacts=9
/tmp/ouroforge-live-test/collect-and-exit-2347
```

Observed artifact facts:

- `screenshots/start.png` and, when replay is enabled, `screenshots/final.png`: PNG, 756x469.
- `frame-stats.jsonl`: captured a CDP performance metric line with `Frames`.
- `manifest.json`: recorded local target URL, Node version, Chrome/CDP metadata, retry attempts, and digests.
- `console.jsonl`: present and valid; empty for this run.

This is product-observed harness evidence for capture/validation, not a claim that the game runtime itself is product-observed complete.


## Collect-and-exit replay and determinism check

Use `--replay collect-and-exit` to drive the current collect-and-exit scene through existing runtime API keys only. The runner records replay steps and objective flag checkpoints in `input-replay.json`.

For M116.3 PR 3, two consecutive generated bundles were compared by their `input-replay.json.objective_flag_sequence` values:

```text
collect-and-exit-2348-det-a == collect-and-exit-2348-det-b
start: key_collected=false, exit_reached=false
after-key: key_collected=true, door_open=true, exit_reached=false
after-exit: key_collected=true, door_open=true, exit_reached=true
```

The same run recorded `observability_api_keys_used` as:

```text
whenReady,getWorldState,getFrameStats,getEvents,setInput,step
```

Known live diagnostic preserved in the bundle: `missing_asset` for `collect_and_exit_sheet`; this is runtime/product evidence, not a harness failure.
