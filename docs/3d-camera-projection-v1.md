# 3D Camera and Projection v1

Issue #598 adds a bounded 3D camera/projection model for source-like validation fixtures. This is validation and configuration shape only; it does not add runtime rendering, camera editor tooling, viewport persistence, cinematic timelines, export/package behavior, hosted/cloud behavior, plugin runtime, or a Godot replacement claim.

## Schema

3D cameras live under explicit `sceneKind: "3d"` scenes in the `scene3d` graph:

- `activeCameraId`: required when `scene3d.cameras` is non-empty.
- `cameras[]`: bounded camera configs.
- `id`: path-safe camera id.
- `nodeId`: optional reference to a 3D node.
- `active`: optional boolean marker; if set, it must match `activeCameraId` and only one camera may be active.
- `transform`: camera transform using the bounded 3D transform schema.
- `projection`: perspective or orthographic projection config.
- `viewport`: non-negative origin plus positive width/height.

Perspective projection requires `fovDegrees` from 1 through 179, positive `near`, and `far > near`. Orthographic projection requires positive `orthographicHeight`, positive `near`, and `far > near`. `aspectMode` defaults to `viewport`; `square` is the only other accepted bounded value.

## Fixtures

Tracked source-like fixtures under `examples/3d-capability-gate-v1/` cover:

- `scene-3d-camera-valid.scene.json`;
- `scene-3d-camera-invalid-missing-active.scene.json`;
- `scene-3d-camera-invalid-projection.scene.json`;
- `scene-3d-camera-invalid-viewport.scene.json`.

Generated screenshots, previews, run output, dashboard exports, and temp project state remain untracked unless a later issue explicitly scopes a tiny deterministic fixture.

## Boundary

This model only lets Rust/local validation identify an active 3D camera and reject invalid camera/projection/viewport shape. Camera evidence/read-model summaries and scenario checks are separate #598 PR units. Existing 2D camera config remains unchanged.
