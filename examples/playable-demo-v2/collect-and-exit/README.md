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
