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
Local asset loading v1 is intentionally static and manifest-gated.
`scene.json` declares `assetManifest` entries with local `assets/...` paths, and
scene sprite/tile/audio asset fields reference those entries by ID. The Rust
scene validator rejects unknown manifest IDs, remote URLs, absolute paths,
invalid characters, directory escapes, duplicate manifest entries, and
unsupported asset extensions. Use
`cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene.json`
to surface manifest validation errors before runtime use.

The browser runtime loads only manifest-owned image paths from the same local
static server and exposes bounded manifest plus asset metadata through
`window.__OUROFORGE__.getWorldState().assetManifest` and `.assets`. Generated
run state stays under `runs/` and must remain untracked. This is not a bundler,
import pipeline, cache, marketplace, hot-reload system, or editor asset browser.
Animation v1 chooses a bounded `sprite_frame` mode: each animated entity owns
named clips, fixed per-clip `frameDuration`, loop policy, and optional
manifest-backed frame asset IDs. The browser runtime advances the current frame
strictly by fixed simulation ticks and exposes `currentClip`, `elapsedFrames`,
and `frameIndex` through `window.__OUROFORGE__.getWorldState()`. This keeps
animation replay-deterministic and probe-observable without timelines, skeletal
rigs, blend trees, graphs, editor tooling, wall-clock playback, or asset import
complexity.
Audio v1 is evidence-first and headless-safe: scene entities may declare named
`scene_loaded` audio intent events with manifest-backed asset IDs and `play` or
`stop` actions. The browser runtime records bounded intent entries in
`getWorldState().audioEvents` with `playback: "intent"` and `muted: true` by
default. Automated QA verifies those event records only; speaker output,
browser audio device access, and audible playback are not required for
acceptance. No mixer, DSP, spatial audio, timeline, streaming, native backend,
or audio editor subsystem is introduced.


## Engine Expansion v1 playable template

`scene.json` is the one-screen playable template used by `seeds/platformer.yaml`. It intentionally combines completed Engine Expansion v1 features without adding new engine behavior: renderer layers/camera, tilemap collision layers, manifest-backed sprite/audio assets, sprite-frame animation, headless-safe audio intents, bounded AABB physics/contact evidence, reload probe state, and scene composition (`player-badge` is parented to `player`).

Run and inspect it with:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene.json
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Known gaps are deliberate: one screen only, no enemy AI, no campaign, no native export, no plugin system, no marketplace assets, no multiplayer, no monetization/publishing workflow, and no production/public-launch claim. Generated `runs/` evidence remains local and untracked.
