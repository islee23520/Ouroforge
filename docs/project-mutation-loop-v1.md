# Project Mutation Loop v1: Workspace-Safe Scene Mutation

Project Mutation Loop v1 makes the existing scene-only mutation application path
project-aware. A mutation may update a scene file only when Rust can prove that
file is declared by the selected `ouroforge.project.json` manifest and that the
manifest/scene hashes match the operation being applied.

This is still a manual local authoring loop. The browser can display mutation
state and copyable CLI commands, but Rust remains the trusted validator and
writer.

## Contract

Project-scoped scene mutation application must:

- apply scene data only;
- require an existing mutation proposal id;
- require an allowed `SceneEdit` path from `SUPPORTED_SCENE_EDIT_PATHS`;
- require `validationRequired: true`;
- require the target scene to be declared in the selected project manifest;
- reject target scenes that are outside the manifest-authorized scene path;
- reject stale manifest hashes before writing;
- reject stale scene hashes before writing;
- emit a scene edit transaction artifact;
- record successful applications in `mutation/scene-applications.json`;
- record rollback metadata that points back to the pre-application scene hash;
- append a `mutation.scene_applied` ledger event;
- leave QA reruns, comparison, and review as manual next steps.

## Non-goals

Project Mutation Loop v1 does **not** implement:

- arbitrary patch or source-code mutation;
- browser-side mutation application;
- browser-side trusted file writes;
- command bridge or local server command execution;
- auto-accept, auto-merge, or hidden review decisions;
- plugin runtime, hosted/cloud/server/database/auth infrastructure;
- production editor behavior.

## Project-scoped operation shape

The operation may include a `project` context. When the CLI is called with
`--project` and the operation has no project context, the CLI derives it from the
selected manifest and target scene. When the operation already has project
context, `--project` must match it exactly or application fails before writing.

```json
{
  "schemaVersion": "scene-only-mutation-v1",
  "proposalId": "mutation-...",
  "targetScenePath": "/absolute/or/workdir/path/project/scenes/main.scene.json",
  "project": {
    "projectId": "minimal_2d",
    "manifestPath": "/absolute/or/workdir/path/project/ouroforge.project.json",
    "manifestHash": {
      "algorithm": "fnv1a64-file-v1",
      "value": "..."
    },
    "scenePath": "scenes/main.scene.json",
    "sceneHash": {
      "algorithm": "fnv1a64-canonical-json-v1",
      "value": "..."
    }
  },
  "edit": {
    "entityId": "player",
    "path": "components.transform.x",
    "value": 48
  },
  "expectedBeforeSceneHash": {
    "algorithm": "fnv1a64-canonical-json-v1",
    "value": "..."
  },
  "validationRequired": true
}
```

Legacy non-project scene-only mutation operations remain supported, but they do
not receive manifest authorization or project provenance.

## CLI flow

```bash
cargo run -p ouroforge-cli -- mutation apply-scene <run-dir> \
  --project <project-root-or-ouroforge.project.json> \
  --operation <operation.json> \
  --transaction-output <transaction.json>
```

Successful output prints:

- transaction id;
- transaction artifact path;
- before scene hash;
- after scene hash;
- a manual next QA command.

The command does not run QA, compare runs, accept mutations, merge patches, or
write review decisions.

## Application record and rollback metadata

Successful project-scoped applications are recorded in
`mutation/scene-applications.json`:

```json
{
  "id": "scene-application-...",
  "proposalId": "mutation-...",
  "transactionId": "scene-edit-...",
  "targetScenePath": "/path/project/scenes/main.scene.json",
  "project": {
    "projectId": "minimal_2d",
    "manifestPath": "/path/project/ouroforge.project.json",
    "manifestHash": { "algorithm": "fnv1a64-file-v1", "value": "..." },
    "scenePath": "scenes/main.scene.json",
    "sceneHash": { "algorithm": "fnv1a64-canonical-json-v1", "value": "..." }
  },
  "transactionArtifactPath": "/tmp/.../transaction.json",
  "beforeSceneHash": { "algorithm": "fnv1a64-canonical-json-v1", "value": "..." },
  "afterSceneHash": { "algorithm": "fnv1a64-canonical-json-v1", "value": "..." },
  "rollback": {
    "scenePath": "/path/project/scenes/main.scene.json",
    "restoreHash": { "algorithm": "fnv1a64-canonical-json-v1", "value": "..." },
    "strategy": "...beforeSceneHash..."
  },
  "status": "applied"
}
```

Rollback remains metadata, not an automatic undo command. A human or future
Rust-owned rollback command must inspect the transaction, restore the scene to a
known-good source state, and rerun validation/QA. The browser must not perform
rollback or mutation writes.

## Manual review loop

After applying a project-scoped scene mutation:

1. Run project QA with the relevant Seed/scenario pack.
2. Export dashboard data.
3. Compare before/after runs.
4. Inspect semantic/project evidence and mutation lifecycle state.
5. Decide manually whether to keep, revert, or reject the mutation.
6. Record review decisions only through trusted Rust CLI commands.

A typical command sequence is:

```bash
cargo run -p ouroforge-cli -- project validate <project>
cargo run -p ouroforge-cli -- mutation apply-scene <run-dir> \
  --project <project>/ouroforge.project.json \
  --operation <operation.json> \
  --transaction-output <transaction.json>
cargo run -p ouroforge-cli -- run <project>/seeds/platformer.yaml \
  --project <project>/ouroforge.project.json \
  --transaction <transaction.json>
cargo run -p ouroforge-cli -- compare <before-run> <after-run> --output-dir <after-run>/comparisons
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

## Dashboard and cockpit behavior

The evidence dashboard and authoring cockpit display project mutation context
when `mutation/scene-applications.json` contains it:

- project id;
- manifest path and hash;
- manifest-relative scene path;
- scene hash;
- rollback restore hash;
- display-only `mutation apply-scene --project ...` command strings.

They remain read-only. They do not apply, accept, reject, rollback, merge, write
trusted files, or execute local commands.

## Generated-state policy

Generated run directories, transaction artifacts, dashboard exports, smoke test
projects, and temporary operation JSON files must remain untracked unless a
specific issue scopes a tiny deterministic fixture as source-like test data.

For PW1.7.4, local smoke evidence was generated under `/tmp` and removed after
checking the output. No generated transaction/run/dashboard artifact is intended
to be committed.

## PW1.7.4 evidence

Representative generated local smoke command:

```bash
target/debug/ouroforge mutation apply-scene <tmp-run-dir> \
  --project <tmp-project>/ouroforge.project.json \
  --operation <tmp>/operations/apply.json \
  --transaction-output <tmp>/transactions/apply.json
```

Observed smoke result:

```text
Scene-only mutation applied: scene-edit-18166528079396206128
project mutation smoke ok scene-edit-18166528079396206128
```

The smoke confirmed that `mutation/scene-applications.json` contained
`project.projectId = minimal_2d`, `project.scenePath = scenes/main.scene.json`,
and a `rollback` object. The temporary project/run/operation/transaction files
were deleted after validation.

## Closure checks for #251

Before closing #251, verify on latest `main`:

```bash
gh issue view 251 --repo shaun0927/Ouroforge
cargo fmt --check
cargo test
cargo run -p ouroforge-cli -- project validate <project-fixture>
cargo run -p ouroforge-cli -- mutation apply-scene <run-dir> \
  --project <project-fixture> \
  --operation <operation-json> \
  --transaction-output <transaction-json>
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
cargo clippy --all-targets --all-features -- -D warnings
```

Then audit:

- scene-only data mutation only;
- project manifest authorization required;
- stale manifest and scene hashes rejected before writes;
- rollback metadata recorded;
- browser/UI remains read-only;
- no source-code mutation;
- no generated artifacts committed;
- #1 and #23 remain open unless a later issue explicitly changes governance.
