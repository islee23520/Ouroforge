# Playable Demo v2: Collect and Exit

This source fixture now covers the #319 playable loop and the #340 asset-backed
refresh. It defines a deterministic one-screen project with a player, key
trigger, door/exit trigger, minimal HUD values, animation metadata, audio intent
metadata, local asset manifest fixtures, sprite atlas/tilemap metadata, a Seed,
and a scenario pack.

The fixture is intentionally source-like only. Browser/runtime surfaces are
read-only over local evidence; do not commit generated `runs/`, `dashboard-data/`,
screenshots, previews, or temporary smoke outputs for this demo.

Suggested validation from a full Rust toolchain:

```bash
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml
cargo run -p ouroforge-cli -- asset validate examples/playable-demo-v2/collect-and-exit/asset-manifest.json
```



## Production 2D vertical-slice fixture refresh (#591 P2D8.11.1)

This fixture is the canonical tiny Production 2D vertical-slice source fixture for
Milestone 8 follow-up evidence. The scene metadata now records the demo title,
scenario id, bounded frame-budget defaults, and a `demo-start` checkpoint start
state. The e2e smoke creates and restores that checkpoint through the runtime
save/load API while proving movement, camera/render evidence, tilemap cells,
collision/trigger flags, HUD values, animation state, VFX/audio intent evidence,
and scenario assertions.

Fresh-clone source validation/smoke commands:

```bash
cargo run -p ouroforge-cli -- project validate examples/playable-demo-v2/collect-and-exit/ouroforge.project.json
cargo run -p ouroforge-cli -- seed validate examples/playable-demo-v2/collect-and-exit/seeds/collect-and-exit.yaml
cargo run -p ouroforge-cli -- asset validate examples/playable-demo-v2/collect-and-exit/asset-manifest.json
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
```

Generated run ids and dashboard export paths must be recorded in PR/closure
evidence, not committed. This fixture does not claim shipped-game maturity,
production profiler accuracy, native export, hosted services, plugin runtime, or
Godot replacement status.

## Visual authoring demo draft fixtures

Visual Authoring Demo v1 uses source-like draft JSON fixtures under
`examples/visual-edit-draft-v1/valid/` to document the safe local edit cockpit
workflow for this demo without committing generated transaction or run output:

- `collect-and-exit-scene-demo.visual-edit-draft.json` previews a key-position
  move and a HUD goal text update against `scenes/collect-and-exit.scene.json`.
- `collect-and-exit-tilemap-demo.visual-edit-draft.json` previews adding and
  removing one collision obstacle cell against the demo tilemap fixture.
- `collect-and-exit-asset-frame-demo.visual-edit-draft.json` previews replacing
  the player sprite frame reference with the manifest-backed `player_idle_1`
  atlas frame.

These files are deterministic source fixtures for validation and review. The
trusted follow-up flow remains: copy or pass a draft to the Rust CLI, generate a
transaction/preview, record a review decision, apply only through the
review-gated CLI path, rerun/compare manually, and inspect exported evidence in
read-only dashboard/Studio surfaces. Browser/Studio surfaces must not write
project files, execute local commands, persist trusted state, import/fetch
assets, or apply drafts.


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

## Evidence smoke

PR EE2.8.3 adds a source-controlled smoke harness that drives the runtime through
key collection and exit completion, evaluates the scenario pack against bounded
evidence paths, writes temporary evidence outside the repository, and removes it
before exit:

```bash
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
```

AP1.9.3 adds an asset evidence compatibility smoke that runs the asset-backed
demo in-process, writes temporary dashboard data outside the repository, renders
dashboard and Studio asset panels against that generated payload, and deletes the
temporary output before exit:

```bash
node examples/playable-demo-v2/collect-and-exit/asset-evidence-smoke.test.cjs
```

The smoke intentionally leaves `runs/`, `dashboard-data/`, screenshots, and other
generated artifacts untracked.

Canonical documentation: `docs/playable-demo-v2-collect-and-exit.md`.
