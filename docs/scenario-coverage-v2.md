# Scenario Coverage v2

Scenario Coverage v2 separates Engine Expansion v1 regression coverage into small feature-focused seeds. The purpose is to keep renderer, tilemap, asset, animation, audio, physics, hot-reload, and composition regressions attributable instead of hidden inside a single integration demo.

## Coverage map

| Feature area | Seed/scenario | Evidence focus | Notes |
| --- | --- | --- | --- |
| Renderer/layer/camera | `seeds/engine-feature-renderer-tilemap.yaml` / `renderer-layer-camera-regression` | `world_state.renderer.*`, render order, `frame_stats.fixedDeltaMs` | Deterministic data assertions only; no screenshot golden suite. |
| Tilemap | `seeds/engine-feature-renderer-tilemap.yaml` / `tilemap-grid-layer-regression` | `world_state.tilemaps.*`, layer order, collision layer metadata | Reuses completed Engine Expansion v1 runtime state. |
| Asset loading | `seeds/engine-feature-asset-animation-audio.yaml` / `asset-manifest-regression` | `world_state.assetManifest.*`, `world_state.assets.*` | Checks manifest and loaded state only; no network or server. |
| Animation | `seeds/engine-feature-asset-animation-audio.yaml` / `animation-frame-regression` | `world_state.entities.*.components.animation.*` | Fixed-frame assertions; no visual AI judgment. |
| Audio event | `seeds/engine-feature-asset-animation-audio.yaml` / `audio-intent-regression` | `world_state.audioEvents.*` | Intent evidence only; no real playback assertion. |
| Collision/physics | `seeds/engine-feature-physics-reload-composition.yaml` / `physics-contact-trigger-regression` | `world_state.collisions.*`, `world_state.collisionEvents.*` | Fixed replay input; no wall-clock assertions. |
| Hot reload boundary | `seeds/engine-feature-physics-reload-composition.yaml` / `reload-boundary-regression` | `world_state.reloads`, component defaults, scene id | Documents the boundary; does not add source-code HMR or watchers. |
| Scene composition | `seeds/engine-feature-physics-reload-composition.yaml` / `scene-composition-regression` | `world_state.composition.*` | Parent/child world transform evidence. |

Later PRs in #213 run/export the suite and record dashboard compatibility evidence without committing generated artifacts.

## Guardrails

- Coverage PRs add regression fixtures and tests, not new engine behavior.
- Assertions must be deterministic and evidence-linked.
- Generated run evidence remains untracked under `runs/`.
- Browser/runtime smoke is useful for PR evidence but tracked changes should remain seed/docs/tests only unless a later issue explicitly authorizes code changes.

## AL2.4.4 dashboard compatibility evidence

The feature regression suite is expected to run with the standard local MVP command and then export dashboard data without committing generated artifacts:

```sh
cargo run -p ouroforge-cli -- run seeds/engine-feature-renderer-tilemap.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-feature-asset-animation-audio.yaml --workers 4
cargo run -p ouroforge-cli -- run seeds/engine-feature-physics-reload-composition.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output .omx/tmp/scenario-coverage-v2/dashboard-data.json
node examples/evidence-dashboard/dashboard.test.cjs
node examples/authoring-cockpit/cockpit.test.cjs
```

AL2.4.4 used initial generated run evidence to correct scenario expectations to the current Engine Expansion v1 runtime state. The important compatibility contract is that all feature seeds can produce passing verdicts locally, dashboard export can read the generated runs, and the generated `runs/` plus `.omx/tmp/` evidence remains untracked.
