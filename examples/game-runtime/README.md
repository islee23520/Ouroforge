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
Local asset loading v1 is intentionally static and browser-only. Scene sprites may
reference committed demo images under `assets/...`; the Rust scene validator
rejects remote URLs, absolute paths, invalid characters, and directory escapes.
The browser runtime loads those image paths directly from the same local static
server and exposes deterministic asset metadata through
`window.__OUROFORGE__.getWorldState().assets`. This is not a bundler, import
pipeline, cache, marketplace, or editor asset browser.
Animation v1 chooses a single minimal `sprite_frame` mode: each animated entity
owns local color frames plus a fixed `frameDuration`, and the browser runtime
advances the current frame strictly by fixed simulation ticks. This keeps the
state probe-observable without timelines, skeletal rigs, graphs, editor tooling,
or asset import complexity.
Audio v1 is evidence-first: scene entities may declare named `scene_loaded`
audio events, and automated QA verifies the emitted event log in
`getWorldState().audioEvents`. Speaker output and browser playback are not
required for acceptance, and no mixer, DSP, timeline, streaming, or audio engine
subsystem is introduced.
