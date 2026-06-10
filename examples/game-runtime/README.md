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
and `frameIndex` through `window.__OUROFORGE__.getWorldState()`. Each fixed-step
state advance also emits a bounded `runtime.animation.state` entry from
`getEvents()` with `sceneId`, `entityId`, and before/after animation state. This
keeps animation replay-deterministic and probe-observable without timelines,
skeletal rigs, blend trees, graphs, editor tooling, wall-clock playback, or asset
import complexity.
Audio v1 is evidence-first and headless-safe: scene entities may declare named
`scene_loaded` audio intent events with manifest-backed asset IDs and `play` or
`stop` actions. The browser runtime records bounded request entries in
`getWorldState().audioEvents` with `kind: "audio_request"`, deterministic
`requestId`, `sceneId`, `playback: "intent"`, and `muted: true` by default.
Automated QA verifies those event records only; speaker output, browser audio
device access, and audible playback are not required for acceptance. No mixer,
DSP, spatial audio, timeline, streaming, native backend, or audio editor
subsystem is introduced.

Scene Transitions v1 is manifest-bounded: scenes may declare `sceneTransitions`
with safe `toScene` targets, Rust project validation ensures those targets are
listed in the project manifest, and the browser runtime exposes
`sceneTransitions`, `transitionEvents`, and current `sceneId` through
`getWorldState()`. Scenario DSL can drive `transition` steps and assert
`transition_evidence`, while Dashboard/Studio read models remain read-only. See
`docs/scene-transitions-v1.md` for the supported fields and non-goals.

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

`scene.json` is the one-screen playable template used by `seeds/platformer.yaml`. Tilemap Authoring v2 evidence is documented in `docs/tileset-tilemap-authoring-v2.md`; runtime world state exposes read-only authoring cells and synthetic collision/trigger outcomes for scenario assertions without browser-side writes. It intentionally combines completed Engine Expansion v1 features without adding new engine behavior: renderer layers/camera, tilemap collision layers, manifest-backed sprite/audio assets, sprite-frame animation, headless-safe audio intents, bounded AABB physics/contact evidence, reload probe state, and scene composition (`player-badge` is parented to `player`).

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

## Playable Demo v2 collect-and-exit fixture

The runtime can load the source-only collect-and-exit fixture from issue #319 by
serving the repository root and passing a bounded local JSON scene path:

```bash
python3 -m http.server 8771 --bind 127.0.0.1
# open http://127.0.0.1:8771/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json
node examples/game-runtime/playable-demo-v2.test.cjs
```

The query parameter accepts only local JSON scene paths and falls back to
`scene.json` for protocols, traversal, or unsupported paths. The collect-and-exit
query smoke also verifies the fixture title/start-state metadata, bounded
frame-budget evidence, trigger path, and `demo-start` save/load checkpoint. This
keeps the demo observable through the existing runtime probe without adding
browser-side writes, command execution, generated evidence commits, or broader
engine systems.

## Runtime Shell UX evidence (M118)

`runtime-shell-ux.md` defines the canonical shell states and screenshot names
used by the M118/M120 work: `start`, `key-collected`, `gate-open`, `win`,
`fail`, `paused`, and `restarted`, with generated bundle paths of the form
`screenshots/state-<name>.png`.

For #2353, the runtime page is a playable shell first: the canvas is framed,
scene/controls/HUD/status regions are visible, and raw world-state JSON is kept
inside the collapsed `Evidence / debug JSON` panel. `window.__OUROFORGE__` is not
removed or narrowed; shell rendering reads the same runtime world-state evidence
model and updates page DOM additively.

Measured baseline for `components-v2.test.cjs` on the clean `origin/main` used by
#2353 was exit code 0. The focused shell DOM smoke is:

```bash
node examples/game-runtime/game-ui.test.cjs
node examples/game-runtime/components-v2.test.cjs
```

Live start-state screenshot capture is generated under ignored `runs/` roots,
for example `runs/session-f-2353/screenshots/state-start.png`. The local Chrome
headless run produced the start screenshot in this path; Chrome process stderr
contained browser infrastructure shutdown/network messages after termination, so
page-console fatal-error proof is not upgraded beyond the DOM smoke in #2353.

### HUD checkpoint report (M118.3)

`hud-binding.test.cjs` cross-checks collect-and-exit HUD DOM values against
`getWorldState()` samples at `start`, `key-collected`, `gate-open`, and `win`.
`hud-checkpoint-report.test.cjs` wraps that deterministic replay and records the
corresponding screenshot filenames expected in live bundles:

- `screenshots/state-start.png`
- `screenshots/state-key-collected.png`
- `screenshots/state-gate-open.png`
- `screenshots/state-win.png`

Run the report without writing generated artifacts:

```bash
node examples/game-runtime/hud-checkpoint-report.test.cjs
```

To materialize ignored local evidence for review, set:

```bash
OUROFORGE_WRITE_RUNS=1 node examples/game-runtime/hud-checkpoint-report.test.cjs
```

That writes `runs/session-f-2354/world-samples.jsonl` and
`runs/session-f-2354/hud-checkpoint-report.json`. These generated files remain
untracked; committed source contains only the reproducible replay/report logic.

### Scenario Coverage v99 (M118)

`scenario-coverage-v99.test.cjs` is the M118 runtime-shell scenario coverage
suite landed with #2355. It composes the deterministic shell/HUD/control checks:

- HUD checkpoint/world-state parity (`hud-binding.test.cjs`)
- pause, restart, win, and fail shell states (`pause-restart.test.cjs`)
- raw JSON secondary/collapsed shell DOM smoke (`game-ui.test.cjs`)
- collect-and-exit runtime fixture compatibility (`playable-demo-v2.test.cjs`)

Run it with:

```bash
node examples/game-runtime/scenario-coverage-v99.test.cjs
```

The suite is contract-complete deterministic runtime evidence. Browser pixel
screenshots remain generated evidence under ignored `runs/` bundles and are not
committed as trusted source.

### Visual Rubric Report (M120.2)

`visual-rubric-report.test.cjs` applies the M120.1 rubric to the four #2360
canonical report states: `start`, `key-collected`, `win`, and `fail`. The report
references live-bundle screenshot names (`screenshots/state-<name>.png`) and
cross-checks deterministic HUD/world evidence before marking criteria pass.

Run it with:

```bash
node examples/game-runtime/visual-rubric-report.test.cjs
```

This is a rubric report over deterministic source/runtime evidence. Browser
pixel screenshots remain generated evidence under ignored `runs/` bundles.
