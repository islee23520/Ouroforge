# Project Comparison v1

Project Comparison v1 extends Ouroforge run comparison with project-level
semantic evidence for Project Workspace Loop v1. It compares Rust-authored run
artifacts only; it is not a browser algorithm, gameplay-quality judge, mutation
review authority, or auto-apply system.

```text
project-bound run A -> project-bound run B -> compare -> semantic.project -> CLI/dashboard/cockpit read-only display
```

## Commands

Create two project-bound runs and compare them:

```bash
cargo run -p ouroforge-cli -- project init .omx/tmp/project-compare-v1-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/project-compare-v1-smoke/seeds/platformer.yaml \
  --project .omx/tmp/project-compare-v1-smoke \
  --scenario-pack smoke \
  --workers 1
BEFORE_RUN=$(ls -td runs/run-* | head -1)
# Make a Rust-validated project source change, then run again.
cargo run -p ouroforge-cli -- run .omx/tmp/project-compare-v1-smoke/seeds/platformer.yaml \
  --project .omx/tmp/project-compare-v1-smoke \
  --scenario-pack smoke \
  --workers 1
AFTER_RUN=$(ls -td runs/run-* | head -1)
cargo run -p ouroforge-cli -- compare "$BEFORE_RUN" "$AFTER_RUN" --output-dir "$AFTER_RUN/comparisons"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
rm -rf .omx/tmp/project-compare-v1-smoke
```

The CLI prints a read-only `Project comparison:` section when the comparison
artifact contains `semantic.project`. The full structured evidence remains in
the generated comparison JSON.

## Semantic JSON fields

Project Comparison v1 adds `semantic.project` to run comparison artifacts:

| Field | Meaning |
| --- | --- |
| `before` | Parsed `run.json.project` metadata from the before run, or `null`. |
| `after` | Parsed `run.json.project` metadata from the after run, or `null`. |
| `relation` | `legacy`, `project_added`, `project_removed`, `same_project`, or `different_project`. |
| `changed` | `true` when project presence, project id, manifest hash, seed path, scenario pack, transaction id, or scene hashes changed. |
| `changes[]` | Bounded deterministic changes with `kind`, `summary`, `before`, and `after`. |
| `warnings[]` | Safe warnings for mixed project/legacy comparisons. |

Supported `changes[].kind` values currently include:

- `project_presence`
- `project_id`
- `manifest_hash`
- `seed_path`
- `scenario_pack`
- `transaction_id`
- `scene_hash`

Malformed project metadata is not trusted. It becomes a semantic warning and is
not coerced into project state.

## CLI/read-model/UI behavior

- Rust comparison generation owns the trusted comparison artifact.
- `ouroforge compare` prints existing semantic reasons plus a dedicated
  `Project comparison:` summary derived from `semantic.project`.
- `dashboard export` carries the existing comparison artifact semantic JSON in
  the dashboard read model.
- The evidence dashboard and authoring cockpit render project comparison fields
  read-only from exported JSON.
- Browser JavaScript does not compute project sameness, hash scenes, validate
  manifests, run comparisons, apply mutations, persist files, or execute local
  commands.

## Compatibility behavior

- Legacy-vs-legacy comparisons produce `relation: legacy` and no project
  changes.
- Project-vs-legacy comparisons produce `project_added` or `project_removed`
  with warnings.
- Same-project comparisons can still report project changes when manifest hash,
  scene hash, seed path, scenario pack, or linked transaction id changes.
- Existing comparison classifications (`improved`, `regressed`, `changed`,
  `no_change`) remain based on existing verdict/scenario/evidence counters;
  project context is semantic evidence, not a gameplay-quality score.

## Verification evidence from implementation

Implementation PR gates for #250 verified:

- semantic model tests for same project, changed scene/scenario context,
  project-bound vs legacy, malformed metadata, and legacy compatibility;
- CLI integration test for two project-bound scaffold runs with a scene hash
  change and generated comparison artifact assertions;
- dashboard/cockpit Node tests for present, missing, and malformed project
  comparison fields;
- dashboard export smoke proving generated comparison artifacts carry
  `semantic.project` through the read model;
- `cargo fmt --check`, `cargo test --workspace`, Node syntax/tests, and
  `cargo clippy --all-targets --all-features -- -D warnings`.

## Generated-state policy

Do not commit generated/local comparison state:

- `runs/` and run-local `comparisons/` outputs;
- `examples/evidence-dashboard/dashboard-data.json`;
- `target/`;
- `.openchrome/`, `.omc/`, `.omx/`, and `.claude/`.

Temporary project compare smoke directories under `.omx/tmp/` must be removed
after evidence is captured.

## Non-goals

Project Comparison v1 does not authorize:

- AI gameplay-quality judging;
- browser-side trusted comparison computation;
- mutation acceptance/application or source-code mutation;
- project run metadata implementation beyond existing Project Run v1;
- Studio v3 project workspace cockpit behavior beyond read-only comparison
  rendering already scoped here;
- native export, plugin runtime, hosted/cloud/server/database/auth behavior;
- distributed QA/Elixir implementation;
- public launch automation or Godot replacement claims.
