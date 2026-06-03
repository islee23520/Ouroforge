# Scenario Coverage v4: Asset Pipeline Regression Suite

Scenario Coverage v4 separates Asset Pipeline v1 regression coverage from the
playable demo so manifest validation, integrity failures, atlas bounds, tilemap
authoring extraction, runtime loading evidence, and Studio/dashboard read-model
compatibility remain attributable.

The source fixture is `examples/asset-pipeline-v1-regression/`. It is
intentionally local-first and source-like. Generated runs, dashboard exports,
previews, screenshots, and local tool state remain untracked.

## Coverage matrix

| Coverage area | Fixture/scenario | Evidence path | Boundary |
| --- | --- | --- | --- |
| Manifest validation | `asset-manifest.json`, scenario `manifest-validation` | Rust asset manifest validation; `world_state.assetManifest.*` | Rust-trusted local path/hash/schema validation only. |
| Hash mismatch | `invalid/hash-mismatch.asset-manifest.json`, scenario `hash-mismatch-regression` | Focused Rust test failure plus future scenario verdict refs | No remote fallback, no browser trust escalation. |
| Missing asset | `invalid/missing-asset.asset-manifest.json`, scenario `missing-asset-regression` | Focused Rust test failure plus future scenario verdict refs | Missing local files stay explicit failures/warnings. |
| Atlas frame validation | `invalid/atlas-frame-out-of-bounds.asset-manifest.json`, scenario `atlas-frame-validation` | Rust atlas frame bounds validation; animation evidence smoke | No sprite editor or atlas generation. |
| Tile collision extraction | `asset_regression_tilemap`, scenario `tile-collision-extraction` | `tilemaps.tilemaps[0].authoring.*Cells` | Read-only tile authoring evidence; no visual editor. |
| Runtime asset loading | scenario `runtime-asset-load-evidence` | `world_state.assets`, temporary evidence smoke records | Browser observes local manifest refs; it does not upload/write/fetch remote assets. |
| Studio/dashboard compatibility | scenario `studio-read-model-compatibility` | dashboard/cockpit compatibility smoke | Existing read-only surfaces render escaped exported data only. |

## Focused verification

```bash
cargo test asset_pipeline_v1_regression --lib
cargo run -p ouroforge-cli -- project validate examples/asset-pipeline-v1-regression/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/asset-pipeline-v1-regression/seeds/asset-pipeline-regression.yaml
cargo run -p ouroforge-cli -- asset validate examples/asset-pipeline-v1-regression/asset-manifest.json
node examples/asset-pipeline-v1-regression/evidence-smoke.test.cjs
node examples/asset-pipeline-v1-regression/dashboard-compatibility-smoke.test.cjs
```

## Known gaps

- The fixture does not implement a production asset editor, marketplace, remote
  asset fetch pipeline, native export, packaging, plugin loading, or public launch
  automation.
- Invalid manifests under `invalid/` are deliberate failing fixtures, not inputs
  to commit as generated output.
- Dashboard/Studio compatibility is source/test evidence over exported read-model
  shapes; the browser remains read-only and does not perform trusted writes or
  execute commands.
- Generated evidence is intentionally temporary or ignored. Closure evidence
  should record command output and PR/issue links rather than committing `runs/`
  or `dashboard-data/`.
