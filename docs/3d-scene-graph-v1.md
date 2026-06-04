# 3D Scene Graph v1

Issue #597 introduces a bounded 3D scene graph contract for source-like validation fixtures. This is a schema and validation gate only; it is not renderer evidence, broad import support, production 3D parity, a plugin runtime, native export, hosted/cloud behavior, or a Godot replacement claim.

## Scene kind

Existing 2D scenes remain schema version `1` and default to `sceneKind: "2d"` when the field is absent. A 3D scene must opt in explicitly:

```json
{
  "schemaVersion": "1",
  "sceneKind": "3d",
  "id": "capability-gate-3d-valid",
  "bounds": { "width": 640, "height": 360 },
  "entities": [],
  "scene3d": { "version": "1", "nodes": [] }
}
```

The `entities` array is still the legacy 2D entity list. It may be present in an explicit 3D scene only as compatibility/read-model data. Trusted 3D persistence is owned by Rust/local validation.

## 3D graph shape

`scene3d` version `1` contains deterministic source-like nodes:

- `id`: path-safe node identifier.
- `parent`: optional parent node identifier. Hierarchy validation is completed by later #597 PR units.
- `localTransform`: required integer translation, rotation, and scale vectors with `x`, `y`, and `z` axes.
- `worldTransform`: optional resolved transform slot for later deterministic world-transform evidence.
- `meshRef` / `colliderRef`: optional local identifiers, not a broad asset import pipeline.
- `metadata`: bounded JSON metadata for fixture notes.

Scale axes must be non-zero. Numeric components are deliberately bounded for small deterministic local scenes.

## Source-like fixture policy

Tracked 3D fixtures live under `examples/3d-capability-gate-v1/` and are small, deterministic source-like inputs used by Rust validation tests. Generated 3D screenshots, run directories, previews, dashboard exports, temp projects, and local tool state remain untracked unless a later issue explicitly scopes a tiny deterministic fixture as source-like data.

The initial fixtures cover:

- valid explicit 3D schema: `scene-3d-valid.scene.json`;
- invalid explicit 3D schema with missing graph: `scene-3d-invalid-missing-graph.scene.json`;
- malformed 3D transform shape: `scene-3d-malformed-transform.scene.json`;
- mixed explicit 3D scene with legacy 2D compatibility data: `scene-3d-mixed-2d-compatibility.scene.json`.

## Boundaries

This contract intentionally excludes runtime rendering, physics, animation, Studio mutation, export/package behavior, plugin execution, hosted/cloud services, broad GLTF/import compatibility, and source-apply behavior. Browser/dashboard/Studio surfaces must remain read-only or draft-only unless a separately scoped Rust/local trusted API owns persistence.
