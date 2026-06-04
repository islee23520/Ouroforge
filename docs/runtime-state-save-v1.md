# Runtime State and Save Artifact v1

This is the approved v1 contract for issue #587 P2D8.7.1. It defines the schema boundary only; runtime save/load behavior is intentionally deferred to later PR units.

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

## Deferred work

- P2D8.7.2 implements runtime snapshot/save/load behavior and hardens snapshot semantics.
- P2D8.7.3 implements replay state digests, divergence evidence, scenario assertions, and read-model compatibility.
