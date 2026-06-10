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

## Verdict generation

M116.4 verdicts are rendered by the Rust schema crate, not by the JS runner:

```sh
CARGO_TARGET_DIR=/Users/jh0927/.cache/ouroforge-targets/session-b-2346 \
  cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- \
  render-verdict /tmp/ouroforge-live-test/collect-and-exit-2348-det-a \
  --generated-at 2026-06-10T00:00:00Z \
  --write
```

Renderer guarantees:

- every M115.3 checklist id is present in `verdict.md`;
- every checklist claim references concrete bundle artifact paths;
- fatal failures are separated from usability gaps/diagnostics;
- same bundle input produces byte-identical output except the `Generated at:` line.

The canonical collect-and-exit golden fixture lives at:

- `crates/ouroforge-observability/fixtures/collect-and-exit-product-fail/`

Its verdict is intentionally `contract-pass` / `product-observed FAIL`: replay reaches `exit_reached=true`, but the sampled runtime diagnostic records `missing_asset` for `collect_and_exit_sheet`, so practical product usability must remain a gap/backlog item rather than a green claim.

## Signal Gate Relay replay

Use `--replay signal-gate-relay` for the M130 Signal Gate Relay first-playable path. The replay drives the local runtime through start, relay activation, key/gate, and terminal win-exit checkpoints, then captures `screenshots/final.png` at the win state. This replay is browser evidence only: it does not grant browser trusted writes, command bridges, self-approval, auto-apply, or auto-merge.

Example:

```sh
python3 -m http.server 8879 --bind 127.0.0.1
CARGO_TARGET_DIR=/tmp/ouroforge-target-2490 \
  node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8879/examples/game-runtime/?scene=/examples/playable-demo-v2/signal-gate-dogfood/scenes/signal-gate-relay.scene.json' \
  --run-id m130-2391-signal-gate-win-2499 \
  --out-root runs/m130/2391-first-playable \
  --replay signal-gate-relay
```
