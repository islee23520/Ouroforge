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

## Scene Component Model v2 fixture

`scene-components-v2.json` is the canonical additive component v2 fixture. It keeps Runtime v1 scene shape while exercising optional `status`, `input`, `trigger`, `goalFlag`, `cameraTarget`, and `uiText` components. The Rust validator checks bounded values and deterministic hashing; the browser runtime preserves the component payloads in `window.__OUROFORGE__.getWorldState().componentModel`, applies input `moveSpeed`, records simple trigger goal-flag actions, and renders `uiText` as canvas text.

Run the focused checks with:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene-components-v2.json
node examples/game-runtime/components-v2.test.cjs
```

See `docs/scene-component-model-v2.md` for the supported fields, edit paths, runtime/probe evidence shape, and explicit non-goals.

## Gameplay Trigger and Flag System v1 fixture

`trigger-flags-v1.json` is the focused source-like fixture for declared gameplay flags and trigger outcomes. It declares `gameplayRules.flags`, validates trigger/goal/HUD flag references, initializes runtime `goalFlags`, and supports `world_flags` scenario assertions plus read-only dashboard, journal, and Studio summaries.

Run the focused checks with:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/trigger-flags-v1.json
node examples/game-runtime/components-v2.test.cjs
```

See `docs/gameplay-trigger-flags-v1.md` for the evidence summary fields, assertion targets, read-only surface boundary, and explicit non-goals.

## Collision and Physics Rules v2 fixture

`physics-rules-v2.json` is the focused fixture for collision layer catalogs plus deterministic gravity/jump behavior. It declares opt-in `collisionRules`, a dynamic player, and a static floor. Runtime evidence exposes `getWorldState().physics.grounded`, `getWorldState().collisions`, and captured `getEvents()` runtime events for bounded scenario assertions.

Run the focused checks with:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/physics-rules-v2.json
node examples/game-runtime/physics.test.cjs
```

See `docs/collision-physics-v2.md` for collision layer validation, runtime rules, assertion targets, and explicit non-goals.


## Engine Expansion v1 playable template

`scene.json` is the one-screen playable template used by `seeds/platformer.yaml`. It intentionally combines completed Engine Expansion v1 features without adding new engine behavior: renderer layers/camera, tilemap collision layers, manifest-backed sprite/audio assets, sprite-frame animation, headless-safe audio intents, bounded AABB physics/contact evidence, reload probe state, and scene composition (`player-badge` is parented to `player`).

Run and inspect it with:

```bash
cargo run -p ouroforge-cli -- scene validate examples/game-runtime/scene.json
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
LATEST_RUN=$(ls -td runs/run-* | head -1)
cargo run -p ouroforge-cli -- compare "$LATEST_RUN" "$LATEST_RUN"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

The template seed has two scenarios: `bootstrap-smoke` checks renderer/tilemap/assets/animation/audio/composition/default observability, and `objective-contact` checks goal trigger plus wall contact evidence. Dashboard and compare surfaces are inspect-only: they read generated artifacts and do not mutate runs, execute browser-side comparisons, or infer semantic game quality.

Known gaps are deliberate: one screen only, no enemy AI, no campaign, no native export, no plugin system, no marketplace assets, no multiplayer, no monetization/publishing workflow, and no production/public-launch claim. Generated `runs/` evidence remains local and untracked.
