# Project Manifest v1

Project Manifest v1 is the Rust-trusted local workspace contract for Project
Workspace Loop v1. It lets Ouroforge identify a small game project and validate
which scene, Seed, scenario pack, asset, run, and generated-state paths belong
to that project before project-scoped run, compare, mutation, or Studio behavior
is added. Project Run v1 now consumes this manifest for validated run
metadata binding; see `docs/project-run-v1.md`.

The manifest file is named exactly `ouroforge.project.json`.

## Example

```json
{
  "schemaVersion": "project-manifest-v1",
  "project": {
    "id": "project_workspace_fixture",
    "name": "Project Workspace Fixture"
  },
  "scenes": [
    { "id": "main", "path": "scenes/main.scene.json" }
  ],
  "seeds": [
    { "id": "smoke", "path": "seeds/smoke.yaml" }
  ],
  "scenarioPacks": [
    { "id": "regression", "path": "scenarios/regression.json" }
  ],
  "assetRoots": ["assets"],
  "runsRoot": "runs",
  "generated": {
    "roots": ["runs", "target", "dashboard-data"]
  }
}
```

Tracked fixtures live under `examples/project-workspace-fixtures/`.

## Fields

| Field | Required | Meaning |
| --- | --- | --- |
| `schemaVersion` | yes | Must be `project-manifest-v1`. Future schema values are rejected. |
| `project.id` | yes | Stable local project id. It uses the same bounded id character set as other Ouroforge ids: ASCII letters, numbers, `_`, and `-`. |
| `project.name` | yes | Human-facing project name. |
| `scenes[]` | yes, non-empty | Scene references with unique `id` and project-relative `path`. |
| `seeds[]` | yes, non-empty | Seed references with unique `id` and project-relative `path`. |
| `scenarioPacks[]` | optional | Scenario pack references with unique `id` and project-relative `path`. The manifest validates existence only; scenario pack execution is scoped to #248. |
| `assetRoots[]` | yes, non-empty | Project-relative source asset directories. |
| `runsRoot` | yes | Project-relative generated run root. It must also appear in `generated.roots`. |
| `generated.roots[]` | yes, non-empty | Project-relative generated/local roots that source paths must not overlap. |

## Validation command

Validate either a manifest file or a project root directory:

```bash
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid/ouroforge.project.json
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
```

A successful validation prints a deterministic summary:

```text
Project manifest valid: project_workspace_fixture
Manifest: examples/project-workspace-fixtures/valid/ouroforge.project.json
Source refs: 3
Asset roots: 1
Runs root: runs
Generated roots: runs,target,dashboard-data
```

Invalid manifests exit non-zero and include the failing field/path, for example a
missing scene reference:

```text
project manifest scenes ref main missing file: scenes/missing.scene.json
```

## Path safety contract

Manifest paths are project-local source declarations unless explicitly listed as
generated roots.

Validation rejects:

- absolute paths;
- `..` traversal or other paths that escape the project root;
- duplicate ids or duplicate paths in source reference lists;
- hidden path components and local tool/runtime roots such as `.git`, `.omx`,
  `.omc`, `.openchrome`, and `.claude`;
- source paths that equal, contain, or are contained by a generated root;
- missing referenced files for scenes, Seeds, and scenario packs;
- missing referenced directories for asset roots;
- a `runsRoot` that is not included in `generated.roots`.

`generated.roots` may name directories that do not yet exist. They are policy
entries, not source inputs.

## Relation to Seed, Run, Evidence, and Mutation

The manifest does not replace existing Seed, run, evidence, comparison, journal,
or mutation artifacts. It gives later project-scoped commands a Rust-validated
workspace boundary:

- **Seed**: existing `seed validate` and `run <seed>` remain valid without any
  project manifest. Project-scoped commands may later use manifest `seeds[]` to
  select authorized Seed files.
- **Run/Evidence**: current runs still work under the repository-level `runs/`
  default. Project Run v1 can bind `run <seed> --project <root-or-manifest>`
  to manifest provenance, scene hashes, scenario pack context, journal output,
  and dashboard export. See `docs/project-run-v1.md`.
- **Scenario packs**: the manifest can reference pack files and `project validate`
  resolves them through Scenario Pack v1 validation. The pack schema and current
  execution boundary are documented in `docs/scenario-pack-v1.md`.
- **Mutation**: scene-only mutation can be limited to manifest-authorized
  scene paths through `mutation apply-scene --project`. The project mutation
  contract is documented in `docs/project-mutation-loop-v1.md`.
- **Studio**: browser-facing Studio surfaces may display manifest-derived state
  later, but browser JavaScript must not become a trusted writer or command
  bridge.

## Backward compatibility

Non-project workflows remain compatible:

```bash
cargo run -p ouroforge-cli -- seed validate seeds/platformer.yaml
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

No manifest is required for these existing commands.

## Generated-state policy

Do not commit generated or local runtime/tool state:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- `.openchrome/`
- `.omc/`
- `.omx/`
- `.claude/`

Small deterministic manifest fixtures under `examples/project-workspace-fixtures/`
are tracked source-like test data, not generated run output.

## Non-goals

Project Manifest v1 does not authorize:

- `project init` or project scaffold behavior;
- scenario pack execution;
- additional project run metadata binding beyond Project Run v1;
- project comparison changes;
- Studio v3 UI changes;
- native export;
- plugin runtime;
- hosted/cloud/server/database/auth behavior;
- distributed QA/Elixir implementation;
- browser-side trusted file writes or a command bridge;
- arbitrary source-code mutation;
- public launch automation or Godot replacement claims.
