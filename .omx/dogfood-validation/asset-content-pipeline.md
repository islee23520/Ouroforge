# Dogfood B7 Asset / Content Pipeline Evidence

## Metadata

- Blocker: B7 — Asset/content pipeline evidence is not durable on origin/main
- Report version: `dogfood-asset-content-pipeline-v1`
- Demo identity: `collect-and-exit-local-rc-candidate`
- Branch: `dogfood/b7-asset-content-pipeline-20260610022409`
- Base: `origin/main` after B6 acceptance and merge commit `1931f141`
- Source basis: `examples/playable-demo-v2/collect-and-exit/`, `examples/asset-pipeline-v1-regression/`, and `examples/game-runtime/`
- Evidence classification: `bounded-local-asset-content-pipeline-evidence`
- Issue state evidence: #1 OPEN; #23 OPEN.

## Purpose

This handoff makes B7 durable by tracking asset/content pipeline evidence for the compact dogfood demo. It records manifest/provenance integrity, local asset pack coverage, atlas/tileset/tilemap cross-reference resolution, runtime asset loading and tilemap evidence, dashboard/Studio read-model compatibility, generated-state boundaries, and guardrails. It is coordination/evidence only: no product asset pipeline feature, remote fetch/upload, trusted browser write, hosted service, release flow, auto-port, foreign-runtime embedding, or Era Q full-3D work is added.

## Merged prerequisite evidence

| Blocker | PR | Origin-main artifact | Status for B7 |
| --- | --- | --- | --- |
| B1 claim coverage | #2334 MERGED | `.omx/dogfood-validation/claim-coverage-matrix.md` | Present; keeps #1/#23 and forbidden-scope guardrails visible. |
| B2 compact demo spec | #2335 MERGED | `.omx/dogfood-validation/demo-game-spec.md` | Present; defines required compact-demo assets, tilemap, HUD, audio-intent metadata, and local/manual boundaries. |
| B3 pipeline dry-run | #2336 MERGED | `.omx/dogfood-validation/pipeline-dry-run.md` | Present; records failed-classified pipeline evidence without overclaiming readiness. |
| B4 export readiness | #2337 MERGED | `.omx/dogfood-validation/export-release-readiness.md` | Present; records local/manual export-readiness guardrails. |
| B5 gameplay/runtime stress | #2339 MERGED | `.omx/dogfood-validation/gameplay-runtime-stress.md` | Present; records bounded local gameplay/runtime stress evidence. |
| B6 Studio UX validation | #2340 MERGED | `.omx/dogfood-validation/studio-ux-validation.md` | Present; records read-only/review-gated Studio UX evidence. |

## Commands executed

All commands were run from a fresh B7 worktree based on `origin/main`. They are local, read-only over source fixtures, and write generated evidence only to temp/ignored roots when a smoke needs it.

```bash
node examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs
node --test \
  examples/asset-pipeline-v1-regression/evidence-smoke.test.cjs \
  examples/asset-pipeline-v1-regression/dashboard-compatibility-smoke.test.cjs \
  examples/game-runtime/tilemap.test.cjs
cargo test -p ouroforge-core --test scenario_coverage_v34_asset_pipeline --jobs 2
```

Observed output:

- `asset pack smoke OK; 5 assets verified`
- `asset pipeline v1 regression evidence smoke passed`
- `asset pipeline v1 dashboard compatibility smoke passed`
- `examples/game-runtime/tilemap.test.cjs` passed under Node test runner
- `scenario_coverage_v34_asset_pipeline`: 6 Rust tests passed

## Asset/content evidence summary

| Requirement | Verdict | Evidence / path | Notes |
| --- | --- | --- | --- |
| Compact demo asset pack integrity | pass | `examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs`; `asset-manifest.json`; `asset-provenance.json` | Verifies 5 assets, file existence, FNV content hashes, duplicate guards, license/source/classification, provenance coverage, and copyright risk `none`. |
| Atlas/tileset/tilemap cross-references | pass | Collect-and-Exit `asset-manifest.json`; atlas/tileset/tilemap fixtures | Atlas image refs and tilemap tileset refs resolve. |
| Asset pipeline regression evidence | pass | `examples/asset-pipeline-v1-regression/evidence-smoke.test.cjs` | Verifies manifest validation, hash mismatch, missing asset, atlas frame validation, tile collision extraction, runtime asset load evidence, and Studio read-model compatibility scenarios. |
| Dashboard/Studio asset read models | pass | `examples/asset-pipeline-v1-regression/dashboard-compatibility-smoke.test.cjs` | Verifies dashboard and cockpit asset integrity/loading/preview/inspector surfaces render read-only evidence and no script injection. |
| Runtime tilemap/content evidence | pass | `examples/game-runtime/tilemap.test.cjs` | Verifies ordered layers, debug state, authoring cells, collision entities, draw calls, runtime tile triggers, frame stats, and solid tile collision behavior. |
| Rust scenario coverage v34 | pass | `cargo test -p ouroforge-core --test scenario_coverage_v34_asset_pipeline --jobs 2` | Six Rust tests preserve asset pipeline docs/fixtures, QA cases, proposal/import cases, and generated-state/governance wording. |

## Bounded evidence and generated-state boundary

- Source-like assets are tracked only as fixture/demo source: sprite sheet, atlas JSON, tileset JSON, tilemap JSON, and collect audio fixture.
- Runtime/generated evidence from asset pipeline smokes is written to temporary directories and removed, or to ignored roots; B7 commits no generated runs, previews, screenshots, dashboard exports, or package outputs.
- Browser/Studio surfaces remain read-only over exported evidence; they do not upload assets, fetch remote assets, write trusted state, edit manifests, or execute commands.
- Invalid fixtures remain explicit failure evidence for hash mismatch, missing assets, atlas frame out-of-bounds, unknown tile refs, and malformed manifests.

## Gaps and conservative wording

- B7 proves bounded local asset/content evidence for the compact demo and regression fixtures, not a production asset marketplace, remote importer, CDN, cloud upload, or broad content pipeline.
- No new asset generator, binary import pipeline, dependency install, remote fetch, browser upload, or trusted source write path is introduced.
- Asset/content evidence supports local/manual dogfood validation only; it does not imply production/store readiness, native/mobile/console export readiness, Godot replacement status, or full Godot parity.

## Verification commands for this PR

```bash
node --test examples/dogfood-asset-content-pipeline-v1/asset-content-pipeline-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/asset-pack-smoke.test.cjs
node --test \
  examples/asset-pipeline-v1-regression/evidence-smoke.test.cjs \
  examples/asset-pipeline-v1-regression/dashboard-compatibility-smoke.test.cjs \
  examples/game-runtime/tilemap.test.cjs
cargo test -p ouroforge-core --test scenario_coverage_v34_asset_pipeline --jobs 2
git diff --check origin/main...HEAD
```

## Non-goals and guardrails

- #1 and #23 remain open.
- Era Q M102-M106 remain deferred/non-goal; no full-3D implementation is added.
- No product asset pipeline feature, hosted/cloud/multi-user behavior, trusted browser/source writes, remote fetch/upload, auto-port, embedded foreign runtime, release automation, signing, publishing, credential flow, or Steam depot flow is added.
- No production-ready, store-ready, commercial release, native export, Godot replacement, full Godot parity, marketplace, CDN, or shipped-game maturity claim is made.
