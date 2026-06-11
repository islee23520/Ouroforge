# Issue #2492 collect-and-exit missing asset resolution

Closure classification: product-observed complete

This note records the stable summary for the generated live browser evidence that
resolved #2492. Bulky browser bundles, screenshots, and profiles stayed in the
ignored `runs/` root; the JSON summary beside this note preserves the exact run
facts needed to reproduce the observation.

## Scope

#2492 targeted only the collect-and-exit live `missing_asset` diagnostic for the
nested Session K scene URL. It did not reopen #2379, #2381, #2382, #2383, #1, or
#23, and it did not introduce browser trusted writes, command bridges, deployment,
signing, upload, release automation, self-approval, auto-apply, or auto-merge.

## Local product-surface commands

```bash
python3 -m http.server 8873 --bind 127.0.0.1
export CARGO_TARGET_DIR=/private/tmp/ouroforge-target-2492
node examples/game-runtime/nested-scene-asset-resolve.test.cjs
node examples/game-runtime/playable-demo-v2.test.cjs
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8873/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/before-controlled-failure.scene.json' \
  --run-id issue-2492-before-controlled-failure \
  --out-root runs/issue-2492/live-observability \
  --replay collect-and-exit \
  --wait-ms 3000
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8873/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/after-reviewed-fix.scene.json' \
  --run-id issue-2492-after-reviewed-fix \
  --out-root runs/issue-2492/live-observability \
  --replay collect-and-exit \
  --wait-ms 3000
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- render-verdict runs/issue-2492/live-observability/issue-2492-before-controlled-failure --generated-at 2026-06-11T03:25:00Z --write
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- render-verdict runs/issue-2492/live-observability/issue-2492-after-reviewed-fix --generated-at 2026-06-11T03:25:00Z --write
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/issue-2492/live-observability/issue-2492-before-controlled-failure
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/issue-2492/live-observability/issue-2492-after-reviewed-fix
bash ci/fast-check.sh
cargo fmt --check
```

## Observed result

| Run | Final `exit_reached` | Runtime diagnostics | Console diagnostics | Verdict |
| --- | --- | --- | --- | --- |
| `issue-2492-before-controlled-failure` | `false` | `[]` | empty `console.jsonl` | `product-observed FAIL` |
| `issue-2492-after-reviewed-fix` | `true` | `[]` | empty `console.jsonl` | `product-observed complete` |

Both generated bundles validated as live-observability bundles with 10 artifacts.
Each bundle includes `screenshots/start.png`, `screenshots/final.png`,
`input-replay.json`, `world-samples.jsonl`, `events.json`, `frame-stats.jsonl`,
`console.jsonl`, `manifest.json`, and rendered `verdict.md` under ignored `runs/`.

The after run proves the original #2492 blocker is gone: the collect-and-exit
nested scene replay reaches `exit_reached=true` and the final world sample has no
`missing_asset` runtime diagnostic for `collect_and_exit_sheet`.

## Generated-state audit

`git status --short --ignored` reported the live bundles only as ignored
`!! runs/`; no generated screenshots, browser profiles, package outputs, or run
bundles were added to tracked source.
