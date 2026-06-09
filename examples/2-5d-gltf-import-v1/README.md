# 2.5D glTF Import v1 Fixture

Milestone 97 fixture for `docs/gltf-geometry-orthographic-camera-import-contract-v1.md`.

This example is a bounded one-way source-project glTF presentation import:

- Rust normalizes `source/ortho-demo.gltf` through `ouroforge_core::gltf_25d_import`.
- `fidelity-report.fixture.json` records the Rust-owned native presentation scene, fidelity grades, re-derivation tasks, and deterministic `stateHashPrimary`.
- `render-smoke.test.cjs` performs a tiny local perceptual-render surrogate over the normalized scene. The render result is secondary corroboration only; the state hash remains authoritative.
- M98 billboard/sprite-stack presentation metadata is normalized as `nativeScene.presentationLayers`; those layers are presentation-only and cannot mutate deterministic logic/evidence.
- `run-demo.sh` writes a generated fidelity report under ignored `runs/`, runs the render smoke, and runs the existing migration loop seed as verification evidence.

Verification:

```bash
cargo test -p ouroforge-core --test gltf_25d_import_contract --jobs 2
node examples/2-5d-gltf-import-v1/render-smoke.test.cjs
examples/2-5d-gltf-import-v1/run-demo.sh
```

Guardrails: no live bridge, no embedded engine runtime, no shipped-build ripping, no decompiled-code copying, no gameplay logic translation, no trusted Studio write path.
