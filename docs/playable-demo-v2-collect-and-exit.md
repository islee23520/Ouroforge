# Playable Demo v2: Collect and Exit

Playable Demo v2 is a small, source-controlled one-screen fixture that exercises
completed Engine Expressiveness v2 building blocks together without expanding
Ouroforge's trust boundary. The demo lives at
`examples/playable-demo-v2/collect-and-exit/` and remains local-first,
Rust-trusted, browser-observable, and asset-backed through deterministic local
fixtures.

## Source files

- `ouroforge.project.json` declares the project, scene, Seed, scenario pack,
  source asset root, and generated-state roots.
- `asset-manifest.json` declares deterministic source-like local image, sprite
  atlas, tileset, tilemap, and audio fixture metadata with Rust-validated hashes.
- `assets/` contains tiny deterministic fixture files only; generated previews
  and run/dashboard outputs stay untracked.
- `scenes/collect-and-exit.scene.json` contains the player, floor, key trigger,
  gated door/exit trigger, HUD values, animation metadata, audio intent, runtime
  asset refs, visual tilemap evidence, the demo title/start-state metadata,
  bounded frame-budget defaults, and the `demo-start` checkpoint slot.
- `seeds/collect-and-exit.yaml` records the bounded acceptance contract.
- `scenarios/collect-and-exit.json` asserts the completed key/exit evidence path
  plus HUD, animation, audio, frame-budget, start-state, and save/load event
  evidence.
- `e2e-smoke.test.cjs` drives the runtime in-process, creates/restores the
  `demo-start` checkpoint through the save/load API, and deletes temporary smoke
  evidence before exit.
- `asset-evidence-smoke.test.cjs` verifies the asset-backed demo evidence shape
  against dashboard and Studio asset inspector renderers while keeping generated
  dashboard data in a temporary directory.



## P2D8.11.2 runtime integration commands

The demo is wired through the browser runtime query-loader and the in-process
source fixture smoke. These commands exercise the same source fixture through two
paths without committing generated outputs:

```bash
node examples/game-runtime/playable-demo-v2.test.cjs
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
```

Expected runtime evidence includes `metadata.title`, `metadata.startState`,
`runtimeFrameBudgetStatus: within-budget`, trigger events for key/exit, and a
`runtime.save.loaded` event after restoring `demo-start`. The save artifact is
browser-observable evidence only; trusted persistence remains Rust/local and any
run/dashboard export ids belong in PR or issue evidence, not source control.

## Visual authoring demo drafts

Issue #352 adds source-like Visual Authoring Demo v1 draft examples for the
collect-and-exit fixture. They are checked-in inputs for draft validation and
workflow review only; generated transaction previews, review records, run ids,
dashboard exports, screenshots, and compare outputs remain local/generated and
untracked unless a later issue explicitly scopes a fixture.

| Draft fixture | Demonstrates | Trusted follow-up boundary |
| --- | --- | --- |
| `examples/visual-edit-draft-v1/valid/collect-and-exit-scene-demo.visual-edit-draft.json` | Key move and HUD text change intent for `scenes/collect-and-exit.scene.json`. | Rust validates the draft and creates reviewable scene transaction previews; Studio may only show/copy inert draft text. |
| `examples/visual-edit-draft-v1/valid/collect-and-exit-tilemap-demo.visual-edit-draft.json` | Tile obstacle add/remove intent for `assets/tilemaps/collect-and-exit.tilemap.json`. | Rust validates tilemap bounds, tileset refs, and generated preview summaries before any trusted write path is considered. |
| `examples/visual-edit-draft-v1/valid/collect-and-exit-asset-frame-demo.visual-edit-draft.json` | Asset frame replacement intent for the manifest-backed `collect_and_exit_atlas` / `player_idle_1` frame. | Rust validates manifest ids, asset type, content hash, and frame metadata; browser surfaces do not upload, fetch, or persist assets. |

The end-to-end demo sequence is intentionally review-gated and local-first:

1. Compose or inspect the source-like draft JSON.
2. Run trusted Rust validation/preview to create generated transaction evidence.
3. Review the preview and record an explicit accepted/rejected decision.
4. Apply only through the review-gated CLI path when authorized.
5. Rerun the collect-and-exit smoke, compare evidence, and inspect the read-only
   dashboard/Studio display.
6. Record generated run/transaction ids in PR or issue evidence without tracking
   generated outputs.

This demo does not authorize browser trusted writes, local command bridges,
source mutation, auto-apply, auto-rerun, public launch automation, production
editor claims, plugin/runtime work, or changes to #1/#23.


## VA1.10.2 generated smoke evidence ids

The VA1.10.2 smoke was run in an ignored `.omx/context/` copy of the
collect-and-exit project so review-gated apply could mutate only generated local
state. The checked-in project files were not modified by the smoke.

Source fixture preview evidence:

- scene draft `draft-collect-and-exit-scene-demo` generated transaction ids
  `scene-edit-1660076339805072354` and `scene-edit-6389978599957755934`;
- asset-reference draft `draft-collect-and-exit-asset-frame-demo` generated operation id
  `op-player-frame-replacement-preview`;
- tilemap source-like draft remained a validation fixture and the CLI guardrail
  rejected preview/apply with: `CLI rejected tilemap preview: reserved for later #348 PR units`.

Generated review/apply/rerun/compare evidence ids from the ignored smoke copy:

- before run id: `run-1780541604951-77018`;
- proposal id: `mutation-1780541604958-1`;
- patch draft id: `patch-draft-issue-352-va1-10-2-key-move`;
- review decision id: `review-decision-1`;
- generated apply draft id: `draft-issue-352-va1-10-2-key-move-apply-smoke`;
- apply transaction id: `scene-edit-14746733436601450764`;
- after run id: `run-1780541604976-77023`;
- generated dashboard export path: `.omx/context/issue-352-va1.10.2-smoke-workspace/collect-and-exit/dashboard-data/issue-352-va1.10.2-dashboard-data.json`
  (ignored local evidence; not committed).

The compare output recorded a same-project scene hash change from
`5b620d469b2bebae` to `7f1d82b4f1278834`. The apply response also wrote
`mutation/visual-edit-applications.json` in the ignored before-run directory,
which dashboard/Studio read models can inspect as escaped read-only state. No
browser trusted writes, command bridge, auto-rerun, auto-apply, source mutation,
or generated output tracking was introduced.


## VA1.10.3 Studio/dashboard wording audit

The Visual Authoring Demo v1 Studio/dashboard audit keeps the collect-and-exit
workflow conservative and local-first:

| Surface | Allowed display | Forbidden claim/action |
| --- | --- | --- |
| Dashboard | Rust-exported run ids, preview ids, review decision ids, visual application ids, compare summaries, and generated dashboard export paths as escaped read-only evidence. | No browser writes, command execution, review creation, apply/rerun/compare automation, public release automation, hosted service, or production dashboard claim. |
| Studio/cockpit | Temporary draft rows, inert copyable JSON/CLI text, visual diff summaries, tilemap/asset-reference preview summaries, and generated smoke ids from exported evidence. | No trusted browser persistence, local server bridge, asset upload/fetch/import, apply controls, review-decision controls, auto-rerun, auto-apply, or production editor claim. |
| Public roadmap/docs | Bounded local demo language: source-like fixtures plus ignored generated smoke evidence prove a safe workflow slice. | No Godot replacement, production-ready editor, native export, plugin runtime, marketplace, public launch, or secure-sandbox claim. |

VA1.10.3 does not add automation. It documents that all trusted writes remain
Rust CLI/manual terminal actions with explicit review gates, rollback/evidence
records, generated-state audit, and #1/#23 preservation.

## Runtime smoke

Run the focused Node checks:

```bash
node examples/game-runtime/playable-demo-v2.test.cjs
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs
```

The runtime can also be inspected manually by serving the repository root:

```bash
python3 -m http.server 8771 --bind 127.0.0.1
```

Then open:

<http://127.0.0.1:8771/examples/game-runtime/?scene=/examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json>

The `scene` query accepts only local JSON paths and falls back to the default
runtime scene for protocols, traversal, unsupported extensions, or unsupported
absolute paths.

## Full-toolchain validation

From an environment with Cargo available:

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml
cargo run -p ouroforge-cli -- asset validate examples/playable-demo-v2/collect-and-exit/asset-manifest.json
```

Generated run/dashboard/preview evidence remains local and must not be committed.
If a manual run is used for review, record the generated run id in the PR or issue
comment and keep `runs/`, `dashboard-data/`, screenshots, and temporary smoke
outputs untracked.

## Public wording boundaries

This demo is a conservative local prototype fixture. It proves that existing 2D
runtime, evidence, scenario, dashboard, and Studio read-model paths can observe a
small collect-and-exit loop. It does not claim public compatibility, native
export, plugin runtime, hosted services, production-grade authoring tool
behavior, marketplace features, visual scripting, public launch automation, or
replacement of an existing general-purpose game engine.
