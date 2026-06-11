# Issue #2497 Dogfood Game 2 product-observed loop

Closure classification: product-observed complete

Dogfood Game 2 is **Crate Garden Puzzle**, a turn-based grid logic puzzle. It was
selected because Signal Gate Relay is a real-time platform/relay action slice,
while Crate Garden uses discrete Sokoban-style grid pushes, target coverage, and
puzzle-state reasoning. This is a bounded generalization check, not a broad genre
support, full production, commercial readiness, or engine-parity claim.

## Loop artifacts

| Stage | Artifact/evidence |
| --- | --- |
| Seed/GDD intent | `examples/playable-demo-v2/crate-garden-puzzle/seeds/crate-garden-puzzle.yaml` |
| Scaffold/project manifest | `examples/playable-demo-v2/crate-garden-puzzle/ouroforge.project.json` |
| Before playable | `examples/playable-demo-v2/crate-garden-puzzle/scenes/before-review.scene.json` |
| After playable | `examples/playable-demo-v2/crate-garden-puzzle/scenes/crate-garden-puzzle.scene.json` |
| Scenario | `examples/playable-demo-v2/crate-garden-puzzle/scenarios/crate-garden-puzzle-core.json` |
| Review/apply decision | `examples/playable-demo-v2/crate-garden-puzzle/review/review-apply-decision.json` |
| Playtest/backlog verdict | `examples/playable-demo-v2/crate-garden-puzzle/playtest/playtest-backlog.json` |
| Stable run summary | `docs/evidence/issue-2497-dogfood-game-2-crate-garden.json` |

## Product-surface evidence

Local server:

```bash
python3 -m http.server 8897 --bind 127.0.0.1
```

Before review replay:

```bash
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8897/examples/game-runtime/?scene=/examples/playable-demo-v2/crate-garden-puzzle/scenes/before-review.scene.json' \
  --run-id issue-2497-crate-garden-before \
  --out-root runs/issue-2497/live-observability \
  --replay grid-puzzle \
  --wait-ms 3000
```

After review/apply replay:

```bash
node tools/live-observability-runner/runner.mjs \
  --url 'http://127.0.0.1:8897/examples/game-runtime/?scene=/examples/playable-demo-v2/crate-garden-puzzle/scenes/crate-garden-puzzle.scene.json' \
  --run-id issue-2497-crate-garden-after \
  --out-root runs/issue-2497/live-observability \
  --replay grid-puzzle \
  --wait-ms 3000
```

Both bundles validated with `crates/ouroforge-observability` and include browser
screenshots, replay transcript, world/event samples, frame stats, console logs,
manifest, and rendered verdict.

## Before/after result

| Run | Final objective flags | Runtime diagnostics | Console diagnostics | Verdict |
| --- | --- | --- | --- | --- |
| `issue-2497-crate-garden-before` | `grid_status=playing`, `grid_won=false`, `move_count=1` | `[]` | empty | `product-observed FAIL` |
| `issue-2497-crate-garden-after` | `grid_status=won`, `grid_won=true`, `move_count=4` | `[]` | empty | `product-observed complete` |

The after run proves the second-game playable loop works in the browser product
surface: the replay reaches the grid puzzle win state, screenshots show start,
first move, and final states, and live diagnostics are clean.

## Bugs found and fixed during live verification

1. `examples/game-runtime/index.html` did not load `grid-puzzle.js`, causing live
   grid-puzzle scenes to fail into fallback scene with `scene_load_failed`.
2. The observability verdict renderer only recognized `exit_reached` and
   misclassified a grid-puzzle win as `contract-fail`; it now accepts scoped
   `grid_won=true` objective flags as product-observed success.

Both fixes were followed by fresh browser reruns and regenerated verdicts.

## Raw JSON / usability gap ledger

The happy path did **not** avoid raw JSON/YAML as the primary workflow. Manual
raw-file authoring was used for seed YAML, project manifest JSON, grid-puzzle
scene JSON, review/apply decision JSON, and playtest backlog JSON. This is not
hidden: follow-up backlog issue #2512 tracks replacing the second-game raw
JSON/YAML authoring path with a guided local Studio/CLI workflow.

## Generated-state audit

Generated browser profiles, screenshots, live bundles, and run summaries stayed
under ignored `runs/issue-2497/`. Tracked source contains only stable fixtures,
source fixes, tests, and this evidence summary.

#1 and #23 remain open governance anchors; #1 body was not edited.
