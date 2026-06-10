# Live observability verdict

Generated at: 2026-06-10T00:00:00Z

## Classification

- Mechanical contract: `contract-pass`
- Product observation: `product-observed FAIL`
- Bundle: `collect-and-exit-product-fail`
- Run id: `collect-and-exit-product-fail`
- Run kind: `runtime`
- Target: `http://127.0.0.1:8871/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json`

## Checklist trace

| M115.3 id | Result | Artifact path(s) | Rationale |
| --- | --- | --- | --- |
| `po-check-live-url` | PASS | `manifest.json` | Local-only target URL is recorded in manifest. |
| `po-check-console` | PASS | `console.jsonl; world-samples.jsonl` | console warnings/errors: 0; runtime diagnostics: 1. |
| `po-check-screenshot` | PASS | `screenshots/final.png,screenshots/start.png` | Screenshot inventory is listed by concrete artifact path. |
| `po-check-replay` | PASS | `input-replay.json` | objective checkpoints: 3; final exit_reached: true. |
| `po-check-world-sample` | PASS | `world-samples.jsonl` | world sample lines: 1. |
| `po-check-event-sample` | PASS | `events.json` | event entries: 2. |
| `po-check-frame-stats` | PASS | `frame-stats.jsonl` | frame stat lines: 1. |
| `po-check-before-after` | PASS | `input-replay.json` | Replay objective sequence provides start/after checkpoints for this run; no source mutation is claimed. |
| `po-check-verdict` | FAIL | `verdict.md` | This verdict separates mechanical contract status from product-observed usability. |
| `po-check-generated-state` | PASS | `manifest.json` | No generated observability artifacts are tracked by this renderer; run `git status --short --ignored` for closure evidence. |

## Artifact summaries

- Console lines: `0`; warning/error lines: `0`.
- Frame stat lines: `1`.
- World sample lines: `1`.
- Event entries: `2`.
- Replay: `collect-and-exit`; objective checkpoints: `3`.
- Screenshots: `screenshots/final.png,screenshots/start.png`.
- Observability API keys used: `whenReady,getWorldState,getFrameStats,getEvents,setInput,step`.

## State progression

| Checkpoint | Tick | Flags |
| --- | ---: | --- |
| `start` | 0 | `{"exit_reached":false,"key_collected":false,"player_alive":true}` |
| `after-key` | 40 | `{"door_open":true,"exit_reached":false,"key_collected":true,"player_alive":true}` |
| `after-exit` | 85 | `{"door_open":true,"exit_reached":true,"key_collected":true,"player_alive":true}` |

## Fatal failures vs usability gaps

- Fatal failures: `0`.
- Usability gaps/diagnostics: `1`.
- `{"class":"missing-asset","code":"missing_asset","evidenceRefs":["load-collect_and_exit_sheet"],"message":"Asset collect_and_exit_sheet did not load.","sceneId":"collect-and-exit-scene","schemaVersion":"ouroforge.runtime-diagnostic.v1","severity":"warning","tick":0}`

## Usability note

The bundle can be a mechanical `contract-pass` while still being `product-observed FAIL`. Current diagnostics or missing checklist evidence must become gap/backlog input rather than being hidden behind green tests.
