# 3D Render Smoke v1

Issue #600 adds a bounded browser runtime smoke path for explicit
`sceneKind: "3d"` scenes. The path projects scoped 3D node translations through
the active 3D camera and draws simple primitive markers on the existing browser
canvas. It is render evidence for capability-gate scenes, not a production 3D
renderer.

## Evidence

Runtime world-state evidence may include `scene3dRender` with schema version
`ouroforge.scene3d-render-smoke.v1`. The summary records:

- scene id, frame id, active camera id, mesh count, and material count;
- attempted, visible, skipped, and failed object counts;
- bounded renderable rows with node, mesh, material, primitive, screen, and
  fallback fields;
- fallback reasons for missing cameras, meshes, materials, or unsupported smoke
  primitives;
- an optional screenshot artifact link when a later scoped flow produces one.

Dashboard exports use the snake_case `scene3d_render` read model and browser
surfaces also accept the runtime camelCase `scene3dRender` shape.

## Boundary

This smoke path stays on the existing browser runtime canvas and read-only
dashboard/cockpit evidence surfaces. It does not add WebGPU, a native renderer,
GLTF/GLB import, advanced lighting, PBR/material graphs, skeletal animation,
remote asset fetching, hosted services, plugin/runtime bridges, native export,
or a production renderer/Godot replacement claim.

Generated screenshots, runs, previews, dashboard data, temp projects, browser
profiles, and local tool state remain untracked unless a later issue explicitly
scopes a tiny deterministic source-like fixture.
