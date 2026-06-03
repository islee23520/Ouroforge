# Regression Run Matrix v1

Regression Run Matrix v1 is a local read model over existing Ouroforge run
evidence. It summarizes project-bound scenario status across recent generated
runs so maintainers can see whether promoted or existing scenarios are passing,
failing, or still pending.

It is not a scheduler, CI orchestrator, hosted analytics service, remote run
store, or production QA dashboard.

## Data flow

```text
runs/<run-id>/run.json project metadata
  + evidence index scenario_result artifacts
  + verdict.json
  + mutation/review/promotion records when present
  -> dashboard export regression_matrix
  -> read-only evidence dashboard and authoring cockpit panels
```

The matrix is grouped by:

1. project id;
2. scenario pack id;
3. scenario id;
4. run id.

For each scenario row, the read model records:

- current status from the latest project-bound run observation;
- last passing observation, when available;
- last failing observation, when available;
- per-run observations with run id, run directory, verdict status, scenario
  result path, and evidence refs;
- available context ids for mutation proposals, review decisions, and regression
  promotion records.

Project runs with missing or malformed project context, or without scenario-pack
context, are skipped with an explicit `skippedRuns` reason. The matrix does not
infer project or scenario-pack identity from paths.

## Status semantics

- `passed` means the run contains a readable scenario result for that scenario
  whose status is `passed`.
- `failed` means the run contains a readable scenario result whose status is
  `failed`, or a malformed/unknown scenario result status that must not be
  displayed as a pass.
- `pending` means the scenario is declared by the run's project scenario-pack
  context but no scenario result was recorded in that run.

`currentStatus` is the latest observation by run creation time. `lastPass` and
`lastFail` are the latest observations of those states, not independent proof
that the project is currently good or bad.

## Dashboard export

`dashboard export` writes the matrix as top-level `regression_matrix` in the
generated dashboard JSON:

```bash
cargo run -p ouroforge-cli -- dashboard export \
  --runs-root runs \
  --output examples/evidence-dashboard/dashboard-data.json
```

The existing `runs` array remains unchanged. Browser surfaces read the exported
matrix only; they do not recompute trusted status or write it back.

## Browser surface boundary

The evidence dashboard and authoring cockpit render the matrix as escaped,
read-only HTML. They may display status, links to generated evidence paths, and
counts of linked mutation/review/promotion context.

They must not:

- schedule CI or background QA;
- rerun scenarios;
- generate or promote regression drafts;
- mutate scenario packs or source files;
- execute shell commands;
- use a browser command bridge, hosted service, database, or remote run store.

Malformed or missing matrix data must render as empty/warning states rather than
being inferred as success.

## Generated-state policy

The matrix is derived from generated local state. Do not commit generated run or
dashboard export artifacts:

- `runs/`
- `target/`
- `examples/evidence-dashboard/dashboard-data.json`
- project-local `dashboard-data/`

Use temporary project directories or ignored generated roots for smoke evidence.
Before committing docs/code changes, verify generated output remains ignored or
removed with `git status --short --ignored=matching`.

## Verification

Focused regression matrix checks:

```bash
cargo test regression_matrix
cargo test dashboard_export_includes_regression_run_matrix_read_model
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Issue-level closure should additionally run the broad gates required by the issue
body on latest `main`, including `cargo fmt --check`, `cargo test`, and
`cargo clippy --all-targets --all-features -- -D warnings`.
