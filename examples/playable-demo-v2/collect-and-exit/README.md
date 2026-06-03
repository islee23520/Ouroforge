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
