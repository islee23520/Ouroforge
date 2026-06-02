# Project Run v1

Project Run v1 binds an Ouroforge run to a Rust-validated local project
workspace. It is the #249 contract for Project Workspace Loop v1 and remains
additive: existing non-project runs continue to work without a manifest.

```text
project validate -> run <project seed> --project <root-or-manifest> [--scenario-pack <id>] -> journal/dashboard project context
```

## Commands

Validate a project first when doing a project-bound run:

```bash
cargo run -p ouroforge-cli -- project validate examples/project-workspace-fixtures/valid
```

Run a project-declared Seed with project context. The scaffold command creates
a tiny workspace whose scene can be hashed for run metadata:

```bash
cargo run -p ouroforge-cli -- project init .omx/tmp/project-run-v1-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/project-run-v1-smoke/seeds/platformer.yaml \
  --project .omx/tmp/project-run-v1-smoke \
  --scenario-pack smoke \
  --workers 1
rm -rf .omx/tmp/project-run-v1-smoke
```

`--project` accepts either a project root directory containing
`ouroforge.project.json` or the manifest path itself. `--scenario-pack` is
optional, but when it is present `--project` is required.

Legacy runs remain valid and do not require project metadata:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
```

## Validation and preflight behavior

When `--project` is provided, the CLI resolves and validates the project
manifest before creating the run directory or starting browser workers. The
preflight checks that:

- the manifest is valid Project Manifest v1;
- the requested Seed path is declared by the manifest and remains inside the
  project root;
- declared scene references exist and can be hashed;
- the selected scenario pack exists, validates as Scenario Pack v1, and belongs
  to the requested Seed and manifest-declared scenes.

Invalid project references fail before a run is created. Legacy `run <seed>`
behavior is unchanged when `--project` is omitted.

## Run metadata fields

Project-bound runs add an optional `project` object to `run.json`:

| Field | Meaning |
| --- | --- |
| `id` | Project id from `project.id`. |
| `name` | Human-facing project name from `project.name`. |
| `projectRoot` | CLI-resolved project root or manifest parent path used for the run. |
| `manifestPath` | Manifest file path used for validation. |
| `manifestHash` | Hash object for the manifest file, currently `fnv1a64-file-v1`. |
| `seedPath` | Project-relative Seed path declared by the manifest. |
| `scenes[]` | Project scene ids, project-relative paths, and scene hashes. |
| `scenarioPack` | Optional scenario pack id/path/scenario ids when `--scenario-pack` is used. |
| `transactionId` | Optional linked scene edit transaction id when the run also binds a transaction. |

Scene hashes use the existing canonical scene hash algorithm
`fnv1a64-canonical-json-v1`. Hashes are evidence/provenance identifiers, not a
security boundary.

## Ledger, journal, and dashboard exposure

Project-bound runs append a `run.project_bound` ledger event. The event records
project id, manifest path/hash, seed path, scene paths, optional scenario pack
id, and optional transaction id.

`journal update` renders a **Project Context** section when `run.json.project`
is present. The section is deterministic and read-only; malformed project
metadata is reported as unavailable instead of inferred.

`dashboard export` exposes project context in each run read model and summary:

- `run.project` contains the parsed project metadata when present and valid;
- `run.summary.project` mirrors the same metadata for run-list consumers;
- malformed or missing project metadata is exported as absent rather than
  blocking the rest of the dashboard read model.

The browser dashboard displays the exported project context only. It does not
validate projects, write files, execute commands, create runs, or infer trusted
state from browser JavaScript.

## Generated-state policy

Do not commit generated or local runtime/tool state produced while validating
Project Run v1:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- `.openchrome/`
- `.omc/`
- `.omx/`
- `.claude/`

Temporary scaffold/run smoke directories under `.omx/tmp/` must be removed
after evidence is captured. Tracked fixtures under `examples/project-workspace-fixtures/`
are source-like test data, not generated run output.

## Compatibility audit

Project Run v1 is intentionally additive:

- `run <seed>` without `--project` keeps producing legacy run artifacts without
  a `project` field;
- dashboard and journal code tolerate missing project metadata;
- malformed project metadata in a run artifact does not break dashboard export;
- transaction provenance remains optional and can be linked from project
  metadata when both are present;
- project comparison, project mutation, and Studio v3 are separate follow-up
  issues.

## Non-goals

Project Run v1 does not authorize:

- project comparison algorithms;
- mutation application or source-code mutation;
- Studio v3 behavior beyond existing dashboard data exposure;
- browser direct file writes, a local command bridge, auto-apply, or auto-merge;
- native export, plugin runtime, hosted/cloud/server/database/auth behavior;
- distributed QA/Elixir implementation;
- production editor or Godot replacement claims;
- public launch automation.
