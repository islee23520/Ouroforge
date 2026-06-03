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
  asset refs, and visual tilemap evidence.
- `seeds/collect-and-exit.yaml` records the bounded acceptance contract.
- `scenarios/collect-and-exit.json` asserts the completed key/exit evidence path
  plus HUD, animation, and audio evidence.
- `e2e-smoke.test.cjs` drives the runtime in-process and deletes temporary smoke
  evidence before exit.
- `asset-evidence-smoke.test.cjs` verifies the asset-backed demo evidence shape
  against dashboard and Studio asset inspector renderers while keeping generated
  dashboard data in a temporary directory.

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
