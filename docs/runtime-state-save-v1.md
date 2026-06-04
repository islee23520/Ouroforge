# Runtime State and Save Artifact v1

This is the approved v1 contract for issue #587. It defines the runtime state, save artifact, and deterministic replay evidence boundaries for the P2D8.7 PR units.

## Boundary

- Rust/local code owns trusted validation, evidence artifact writing, generated-state policy checks, and durable save artifact acceptance.
- Browser runtime may expose scoped world state and draft/local observations, but it does not become a trusted file writer or command bridge.
- Save artifacts are generated/local evidence, not source fixtures, unless a later issue explicitly scopes a tiny deterministic fixture.
- No cloud save, account, auth, remote sync, production persistence guarantee, native export, plugin runtime, or broad production/Godot-replacement claim is introduced by this contract.

## Runtime state

`runtime-state-v1` captures a bounded vertical-slice state:

- `runId`, optional `scenarioId`, `sceneId`, `tick`, and version metadata.
- Entity state rows keyed by `entityId` with optional transform, velocity, and status payloads.
- `flags`, `inventory`, and numeric `progress` maps for simple gameplay progress.
- Optional camera and input/action state payloads.
- A deterministic digest using `fnv1a64-canonical-json-v1`.

## Save artifact

`runtime-save-artifact-v1` wraps a runtime state with a save slot and generated-state policy:

- Generated evidence path: `evidence/runtime-state/saves/<slot>.save.json`.
- Local generated path: under `runs/` or `.omx/`, ending in `.save.json`.
- Trusted writer: `rust-local-runtime-save-v1`.
- Browser write access: `none`.

## Replay digest and divergence evidence

Replay evidence compares deterministic runtime state digests at key frames without granting browser write authority:

- `runtime-replay-digest-v1` captures `frameId`, `sceneId`, `tick`, `stateId`, and a `fnv1a64-canonical-json-v1` runtime state digest.
- `runtime-replay-divergence-v1` records `matched` or `diverged` comparisons between expected and actual digests.
- Diverged evidence must include `firstDivergence` with the frame/tick and reason.
- Generated evidence paths:
  - `evidence/runtime-state/replay/<frame>.digest.json`
  - `evidence/runtime-state/replay/<frame>.divergence.json`
- Trusted writer: `rust-local-scenario-runner-v1`.
- Browser write access: `none`.

Runtime helpers may expose digest/comparison payloads for scenario probes, but Rust/local validation owns trusted replay evidence acceptance and generated artifact writing.
