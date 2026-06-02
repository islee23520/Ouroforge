# Ouroforge Minimal 2D Runtime Foundation

Issue #14.1 foundation only: fixed timestep, simple entity/component-like world state, keyboard input state, and the existing `window.__OUROFORGE__` probe API.

Serve locally:

```bash
python3 -m http.server 8771 --bind 127.0.0.1 --directory examples/game-runtime
```

Rendering draws simple colored rectangle sprites to canvas and loads `scene.json`.
The scene now uses Runtime v1 schema shape (`schemaVersion`, stable entity IDs,
sprite declarations, transform/velocity/size/controllable components, collider
declarations, tags, and metadata). `window.__OUROFORGE__.getWorldState()`
exposes the normalized schema v1 scene for evidence capture.
Runtime v1 collision detection is intentionally minimal: axis-aligned collider
declarations marked `dynamic` or `static` produce deterministic collision events
in `getEvents()` and the current `collisions` list in `getWorldState()`.
Snapshot/restore is in-memory only: `snapshot()` returns a local snapshot ID and
`restore(snapshotId)` restores cloned deterministic world, input, and event state
for QA branching without browser storage or save-game semantics.
