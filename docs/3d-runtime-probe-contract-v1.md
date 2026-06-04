# 3D Runtime Probe Contract v1

Issue #603 defines the additive 3D slice of `window.__OUROFORGE__` for bounded
local 3D scenes. The probe lets agents and scenario runners inspect 3D runtime
state without changing the existing Runtime Probe Contract v2 method set or the
legacy 2D response shapes.

This is an evidence contract for browser-local observation. It is not trusted
persistence, a browser write API, a command bridge, production 3D parity, broad
3D compatibility, a secure sandbox guarantee, native export support, plugin
runtime support, hosted/cloud behavior, or a Godot replacement claim.

## Contract placement

The 3D probe contract is intentionally additive:

- `window.__OUROFORGE__.getWorldState()` remains the primary state snapshot
  method and still returns the existing 2D fields.
- `window.__OUROFORGE__.getFrameStats()` remains the primary frame counter method
  and still returns existing 2D/runtime counters.
- `window.__OUROFORGE__.getEvents()` remains the runtime event stream source.
- 3D state appears as versioned fields inside those existing method responses;
  no 2D consumer is required to call a new method.
- Future helpers may be added only as optional aliases if they return the same
  read-only data and never become trusted writers.

## `getWorldState()` 3D fields

When a page has no 3D scene state, the fields may be absent or present with
`present: false`. When 3D state is present, the snapshot should expose these
fields using bounded JSON objects/arrays:

| Field | Purpose | Missing/unavailable behavior |
| --- | --- | --- |
| `sceneKind` or `scene3d.present` | Distinguishes 2D-only, mixed, and 3D scene observations | absent means legacy/2D-only, not failure by itself |
| `scene3d` | Source-like bounded scene graph summary: version, nodes, active camera id, mesh/material refs, colliders, animation clip/state declarations when available | `present: false` plus `unavailableReason` when runtime knows 3D is unsupported |
| `scene3dTransforms` | Deterministic transform hierarchy/world-transform read model keyed by node id | empty array/object with reason when graph is absent or malformed |
| `scene3dCamera` or `camera.camera3d` | Active 3D camera state, projection, viewport, and camera count | `present: false` plus reason when no 3D camera exists |
| `scene3dRender` | Render smoke/read-model state: attempted, visible, skipped, failed object counts and warnings | explicit skipped/failed counts; no inferred success from absence |
| `scene3dCollision` / `scene3dCollisions` | Bounded 3D collision and trigger events | empty events with explicit `present`/reason when unavailable |
| `scene3dAnimation` | Bounded transform animation playback state: clip/state counts, active/stopped state counts, warnings, per-state rows | empty states with explicit `present`/reason when unavailable |
| `scene3dAnimationEvents` | Read-only runtime animation events such as `runtime.scene3d.animation.state` | empty array when there are no animation events |

All 3D fields should include or be mappable to a schema version, for example
`ouroforge.scene3d-render-smoke.v1`, `ouroforge.scene3d-camera-state.v1`,
`ouroforge.scene3d-collision-evidence.v1`, or
`ouroforge.scene3d-animation-evidence.v1`.

## `getFrameStats()` 3D counters

`getFrameStats()` should expose 3D counters as additive numeric fields when the
runtime can compute them:

- `scene3dRenderFrameId`;
- `scene3dRenderAttemptedObjectCount`;
- `scene3dRenderVisibleObjectCount`;
- `scene3dRenderSkippedObjectCount`;
- `scene3dRenderFailedObjectCount`;
- `scene3dAnimationFrameId`;
- `scene3dAnimationStateCount`;
- `scene3dAnimationActiveStateCount`;
- `scene3dAnimationWarningCount`.

Absent counters mean legacy or unavailable 3D probe state. Malformed counters
must be reported as malformed evidence by capture/evaluator code rather than
silently rewritten into passing data.

## `getEvents()` 3D event expectations

3D events remain ordinary bounded runtime events. Event types should use explicit
names so scenario/evaluator code can filter without guessing, for example:

- `runtime.scene3d.render.object`;
- `runtime.scene3d.collision.contact`;
- `runtime.scene3d.collision.trigger`;
- `runtime.scene3d.animation.state`.

Events are read-only evidence. They do not authorize browser-side persistence,
source mutation, local server calls, shell execution, auto-apply, auto-merge, or
self-approval.

## Malformed and missing state

Scenario runners and evidence capture should distinguish:

- `legacy`: no 3D fields and no 3D assertion/evidence expectation;
- `missing`: 3D state expected but unavailable;
- `malformed`: field exists but is not the expected object/array/number shape;
- `unavailable`: runtime explicitly reports no 3D scene or unsupported 3D slice;
- `present`: bounded 3D state is available and shape-valid.

Missing 3D state is allowed for 2D-only scenarios. It becomes a failure only when
the scenario, assertion, dashboard export, or issue-specific evidence contract
requires 3D state.

## 2D compatibility requirements

The 3D probe contract must not break existing 2D consumers:

- `getWorldState()` still exposes legacy 2D fields such as input, camera,
  collisions, tilemaps, runtime state, assets, and object summaries where the
  current runtime provides them.
- `getFrameStats()` still exposes existing tick/frame/entity/event/render budget
  counters.
- `getEvents()` still returns the existing event stream shape.
- Existing 2D Seeds, scenes, project manifests, runs, dashboard exports, Studio
  read models, and source-like fixtures remain backward-compatible unless a
  later issue explicitly scopes a migration.

## Trust boundary

Browser probe state is evidence input only. Rust/local code owns trusted scene validation, source-like fixture validation,
generated evidence artifact writing,
scenario verdicts, project/run binding, and CLI contracts. Browser/dashboard and
Studio surfaces may display 3D probe evidence or prepare explicitly scoped drafts,
but they remain read-only or draft-only for trusted state.

The 3D probe contract forbids:

- trusted browser file writes;
- filesystem reads outside explicitly loaded local demo assets;
- shell commands or command bridges;
- local server bridges;
- hidden command execution;
- unrestricted source mutation;
- auto-apply, auto-merge, or self-approval;
- dependency, CI, workflow, or build-script mutation;
- native export, packaging, signing, publishing, hosted/cloud behavior, plugin
  runtime, marketplace behavior, or third-party code loading.

## Runtime implementation status

PR 3D9.8.2 implements the browser-runtime probe side for the local demo runtime:

- `getWorldState()` exposes top-level `scene3dCamera`, `scene3dTransforms`, and
  `scene3dProbe` fields while preserving existing 2D fields and existing
  `scene3dRender`, `scene3dCollision`, and `scene3dAnimation` evidence.
- `camera.scene3dCamera` and `camera.camera3d` provide compatibility aliases for
  read-only 3D camera inspection.
- `scene3dTransforms` resolves local/world transform rows for the bounded 3D
  hierarchy and reports unavailable state explicitly for 2D-only scenes.
- `getFrameStats()` adds 3D camera, transform, collision, render, and animation
  counters without changing existing 2D counters.

## Implementation handoff

PR 3D9.8.3 may capture/export the fields and add malformed/missing read-model
tests. Follow-up PRs must keep the same conservative wording, generated-state
audit, 2D compatibility checks, and #1/#23 protection checks.
