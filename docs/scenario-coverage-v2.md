# Scenario Coverage v2

Scenario Coverage v2 separates Engine Expansion v1 regression coverage into small feature-focused seeds. The purpose is to keep renderer, tilemap, asset, animation, audio, physics, hot-reload, and composition regressions attributable instead of hidden inside a single integration demo.

## Coverage map

| Feature area | Seed/scenario | Evidence focus | Notes |
| --- | --- | --- | --- |
| Renderer/layer/camera | `seeds/engine-feature-renderer-tilemap.yaml` / `renderer-layer-camera-regression` | `world_state.renderer.*`, render order, `frame_stats.fixedDeltaMs` | Deterministic data assertions only; no screenshot golden suite. |
| Tilemap | `seeds/engine-feature-renderer-tilemap.yaml` / `tilemap-grid-layer-regression` | `world_state.tilemaps.*`, layer order, collision layer metadata | Reuses completed Engine Expansion v1 runtime state. |

Later PRs in #213 extend this map for asset loading, animation, audio events, physics/collision, hot reload, and scene composition.

## Guardrails

- Coverage PRs add regression fixtures and tests, not new engine behavior.
- Assertions must be deterministic and evidence-linked.
- Generated run evidence remains untracked under `runs/`.
- Browser/runtime smoke is useful for PR evidence but tracked changes should remain seed/docs/tests only unless a later issue explicitly authorizes code changes.
