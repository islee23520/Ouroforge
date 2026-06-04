# 3D Mesh and Material References v1

Issue #599 adds a bounded local mesh/material reference schema for small 3D
capability-gate scenes. This is source-like validation shape only: it does not
add a renderer, broad GLTF/import pipeline, remote asset fetching, marketplace
behavior, PBR/shader graph support, native export, hosted/cloud behavior, or a
Godot replacement claim.

## Schema

3D mesh/material catalogs live under explicit `sceneKind: "3d"` scenes in the
`scene3d` graph:

- `meshes[]`: bounded mesh refs (`id`, `kind`, optional `materialRef`, metadata).
- `materials[]`: bounded material refs (`id`, `kind`, optional `baseColor`,
  optional `textureRef`, metadata).
- `nodes[].meshRef`: optional node reference to a mesh id when a mesh catalog is
  present.
- `nodes[].materialRef`: optional node reference to a material id when a material
  catalog is present.
- `components[]` may include `mesh` and `material` references for read-only
  declaration/evidence consistency.

Mesh `kind` is intentionally small:

- `primitive`: requires `primitive` of `cube`, `plane`, or `triangle`; must not
  declare `sourcePath` or `expectedHash`.
- `local_asset`: requires an `assets/...` local `sourcePath` ending in `.obj` or
  `.mesh.json`; may include an `expectedHash` using the existing bounded hash
  syntax.

Material `kind` is intentionally small: `unlit` or `lit`. `baseColor`, when
present, must use `#RRGGBB`. `textureRef` is an id reference only in this PR
unit; project-local asset integration is a later #599 PR unit.

## Fixture policy

Tracked source-like fixtures under `examples/3d-capability-gate-v1/` cover the
schema and validation boundary only:

- `scene-3d-mesh-material-valid.scene.json`
- `scene-3d-mesh-material-invalid-missing-ref.scene.json`
- `scene-3d-mesh-material-invalid-unsafe-path.scene.json`
- `scene-3d-mesh-material-invalid-stale-hash.scene.json`

Generated 3D meshes, screenshots, previews, run output, dashboard exports, temp
projects, and local tool state remain untracked unless a follow-up issue
explicitly scopes a tiny deterministic source-like fixture.

## Boundary

This model only validates bounded local reference shape and local-path/hash
syntax. It does not load, parse, import, render, upload, download, package, or
publish mesh/material assets. Local asset integrity integration, warning/read
models, dashboard/cockpit compatibility, and generated evidence fields are later
#599 PR units.
