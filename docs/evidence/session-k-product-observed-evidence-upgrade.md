# Session K product-observed evidence upgrade (#2490)

Status: `contract-complete with product-observed gap`.

This evidence upgrade adds a fresh live-browser before/after replay for the Session K loop-closure lane (#2379, #2381, #2382, #2383). It proves the reviewed fix changes the live runtime outcome from a controlled failure to a mechanical pass, but it does **not** reclassify the lane as product-observed complete because the after bundle still records a live missing-asset diagnostic.

## Source fixtures

The replay uses scoped scene fixtures under `examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/`:

- `before-controlled-failure.scene.json` — real sandbox scene with the exit trigger gated by `impossible_review_gate_flag`; this intentionally prevents `exit_reached` and models the controlled pre-fix failure.
- `after-reviewed-fix.scene.json` — reviewed-fix scene with the exit trigger requiring `door_open`, matching the collect-and-exit success path.

The only semantic fixture delta is the exit trigger gate:

```diff
- "requiredFlags": ["door_open"]
+ "requiredFlags": ["impossible_review_gate_flag"]
```

The before fixture also includes `"controlledFailure": "session-k-product-observed-before"` metadata on that trigger so the injected failure is explicit.

## Live browser bundle paths

Generated run artifacts are intentionally left under ignored `runs/` storage and are not committed:

- Before bundle: `runs/session-k-product-observed/live-observability/session-k-before-controlled-failure-examples`
- After bundle: `runs/session-k-product-observed/live-observability/session-k-after-reviewed-fix-examples`

Stable summary manifest committed with this note:

- `docs/evidence/session-k-product-observed-evidence-upgrade.json`

## Commands used

Local static server:

```bash
python3 -m http.server 8873 --bind 127.0.0.1
```

Before replay:

```bash
export CARGO_TARGET_DIR="$PWD/target-session-k-po"
node tools/live-observability-runner/runner.mjs \
  --url "http://127.0.0.1:8873/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/before-controlled-failure.scene.json" \
  --run-id session-k-before-controlled-failure-examples \
  --out-root runs/session-k-product-observed/live-observability \
  --replay collect-and-exit \
  --wait-ms 3000
```

After replay:

```bash
export CARGO_TARGET_DIR="$PWD/target-session-k-po"
node tools/live-observability-runner/runner.mjs \
  --url "http://127.0.0.1:8873/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/after-reviewed-fix.scene.json" \
  --run-id session-k-after-reviewed-fix-examples \
  --out-root runs/session-k-product-observed/live-observability \
  --replay collect-and-exit \
  --wait-ms 3000
```

Bundle validation and verdict rendering:

```bash
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/session-k-product-observed/live-observability/session-k-before-controlled-failure-examples
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/session-k-product-observed/live-observability/session-k-after-reviewed-fix-examples
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- render-verdict runs/session-k-product-observed/live-observability/session-k-before-controlled-failure-examples --generated-at 2026-06-11T00:20:00Z --write
cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- render-verdict runs/session-k-product-observed/live-observability/session-k-after-reviewed-fix-examples --generated-at 2026-06-11T00:20:00Z --write
```

The runner wrote and validated the bundles. It also emitted a post-run Chrome temp-profile cleanup `ENOTEMPTY` error on some runs; explicit validator results below confirm this did not invalidate the bundles.

## Replay evidence

Both bundles used the live observability API surface:

`whenReady`, `getWorldState`, `getFrameStats`, `getEvents`, `setInput`, `step`.

### Before controlled failure

Target:

`http://127.0.0.1:8873/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/before-controlled-failure.scene.json`

Verdict:

- Mechanical contract: `contract-fail`
- Product observation: `product-observed FAIL`

Replay checkpoints:

| Checkpoint | Tick | `player_alive` | `key_collected` | `door_open` | `exit_reached` |
| --- | ---: | --- | --- | --- | --- |
| `start` | 0 | true | false | n/a | false |
| `after-key` | 40 | true | true | true | false |
| `after-exit` | 85 | true | true | true | false |

### After reviewed fix

Target:

`http://127.0.0.1:8873/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/session-k-product-observed/after-reviewed-fix.scene.json`

Verdict:

- Mechanical contract: `contract-pass`
- Product observation: `product-observed FAIL`

Replay checkpoints:

| Checkpoint | Tick | `player_alive` | `key_collected` | `door_open` | `exit_reached` |
| --- | ---: | --- | --- | --- | --- |
| `start` | 0 | true | false | n/a | false |
| `after-key` | 40 | true | true | true | false |
| `after-exit` | 85 | true | true | true | true |

## Before/after comparison

The fresh live-browser replay demonstrates the expected mechanical improvement:

- Before: `exit_reached` remains `false` after the collect-and-exit replay.
- After: `exit_reached` becomes `true` after the same replay.

That is sufficient for the follow-up evidence upgrade's contract-complete portion.

## Review/apply gate evidence

This upgrade also ties the browser replay to the already-landed #2379 review/apply contract instead of introducing a browser-trusted write path. The relevant source contract is `crates/ouroforge-source-apply/src/source_apply_controlled_failure_flow.rs`; it composes the independent review gate, sandbox promotion evidence, and before/after comparison refs while forbidding trusted maintainer-worktree apply, self-approval, auto-merge, and untrusted command execution.

Scenario Coverage v107 is locked by `crates/ouroforge-core/tests/scenario_coverage_v107_controlled_failure_flow.rs`:

- `controlled_failure_flow_uses_real_sandbox_file_and_links_before_after_comparison` requires a real sandbox file and before/after comparison links.
- `self_approval_rejection_path_is_exercised_before_flow_passes` exercises and rejects the self-review/self-approval path.
- `missing_real_file_or_regression_blocks_product_observed_ready_claim` blocks product-observed-ready claims when the real sandbox file is absent or the comparison regresses.

Targeted verification for this evidence upgrade:

```bash
CARGO_TARGET_DIR=$PWD/target-session-k-po cargo test -p ouroforge-core --test scenario_coverage_v107_controlled_failure_flow
```

## Product-observed gap / backlog

The after bundle still contains this live runtime diagnostic:

```json
{
  "code": "missing_asset",
  "class": "missing-asset",
  "severity": "warning",
  "message": "Asset collect_and_exit_sheet did not load.",
  "details": {
    "assetId": "collect_and_exit_sheet",
    "assetType": "image",
    "path": "assets/sprites/collect-and-exit-sheet.png",
    "status": "failed",
    "failureReason": "Image load failed"
  }
}
```

Because this usability diagnostic remains in the after run, Session K must remain classified as `contract-complete` with a `product-observed` backlog/gap. This document deliberately does not claim product-observed completion for #2379, #2381, #2382, #2383, or #2490.

## Verification results

Fresh validation in the isolated worktree:

- `cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/session-k-product-observed/live-observability/session-k-before-controlled-failure-examples` → `valid observability bundle ... artifacts=10`
- `cargo run --manifest-path crates/ouroforge-observability/Cargo.toml -- validate runs/session-k-product-observed/live-observability/session-k-after-reviewed-fix-examples` → `valid observability bundle ... artifacts=10`

Closure rule: merge this as an evidence upgrade only. The product-observed gap remains open/backlogged until the missing sprite asset diagnostic is resolved and a fresh after bundle renders `product-observed PASS`.
