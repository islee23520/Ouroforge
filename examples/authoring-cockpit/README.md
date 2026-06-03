# Studio v3 Project Workspace Cockpit

Static local browser UI for composing Project Workspace Loop v1 surfaces: project manifest context, project-bound run evidence, scene edit command generation, transaction provenance, journal viewing, project semantic comparison, project-scoped scene-only mutation lifecycle state, replay evidence, live preview controls, and Rust-validated command strings.

Run locally from the repo root:

```bash
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Open <http://127.0.0.1:8000/examples/authoring-cockpit/>.

The inspector updates scene data in memory and shows the validated `ouroforge scene edit` command for writing through Rust-side validation. Direct browser file writes are intentionally not supported.

Supported Rust-validated scene edit fields:

- `sprite.color`
- `components.transform.x`
- `components.transform.y`
- `components.velocity.x`
- `components.velocity.y`
- `components.size.width`
- `components.size.height`
- `components.controllable`

All other scene fields remain read-only in the cockpit and are rejected by the
Rust `ouroforge scene edit` command.


## Studio v3 demo surfaces

The cockpit composes completed local surfaces only:

- project workspace manifest/scene/seed/scenario pack context from generated dashboard data;
- project-bound run summary and generated-state status;
- run/evidence browser from generated dashboard data;
- journal viewer when journal data exists;
- mutation review lifecycle state and manual command hints when artifacts exist;
- Studio Review Cockpit v1 proposal/review/application cards from exported lifecycle state;
- project-scoped scene-only mutation proposal/application lifecycle state when artifacts exist;
- regression promotion records and display-only dry-run commands when artifacts exist;
- regression run matrix status from generated dashboard data when project-bound runs exist;
- replay evidence surface when replay artifacts exist;
- live preview controls through the existing runtime probe;
- scene edit command generation for Rust-validated fields;
- transaction-bound QA command generation;
- semantic run comparison artifact surface, including Project Comparison v1 context, when comparison artifacts exist.

Known gaps are intentionally visible: no production editor, hosted studio, native shell, collaboration, plugin/marketplace UI, visual scripting, direct browser file writes, browser-side comparison algorithms, command bridge, or mutation acceptance/application from the browser. Studio Review Cockpit v1 review/regression boundaries are documented in `../../docs/studio-review-cockpit-v1.md`. Studio v3 verification evidence is recorded in `../../docs/studio-v3-project-workspace-cockpit.md`; Studio v2 evidence remains in `../../docs/studio-v2-cockpit.md`; legacy Studio v1 evidence remains in `../../docs/studio-v1-demo.md`.

## QA and evidence loop

The cockpit includes Run QA and project workspace panels with local display-only commands:

```bash
cargo run -p ouroforge-cli -- project validate <project>/ouroforge.project.json
cargo run -p ouroforge-cli -- run <project>/seeds/platformer.yaml \
  --project <project>/ouroforge.project.json \
  --scenario-pack <pack-id> \
  --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs \
  --output examples/evidence-dashboard/dashboard-data.json
```

After exporting dashboard data, refresh the cockpit to view project workspace context, latest project-bound run evidence, authoring provenance, semantic project comparison, Studio review cockpit lifecycle cards, project-scoped scene-only mutation lifecycle, regression promotion records, regression run matrix status, and journal panes. The browser still does not execute commands or mutate files directly.

## Live preview controls

The cockpit embeds the existing browser runtime and uses its
`window.__OUROFORGE__` probe API for local pause, resume, step, and
reset/reload controls. The controls show frame stats and world-state excerpts
from the probe. Preview state is ephemeral browser memory only: it is not saved,
does not write scene files, and does not replace the runtime probe API.

## Runtime v1 demo compatibility

The cockpit remains compatible with the Runtime v1 demo scene for inspection and
for the existing Rust-validated scalar scene edits. It does not add animation,
audio, asset-browser, or timeline editing in #67; those fields remain visible in
the scene data but are not directly editable from the static cockpit. Run the
playable demo with:

```bash
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4
```


## Studio v3 command boundary

Engine Expansion v1 integration-demo inspection:

```bash
cargo run -p ouroforge-cli -- seed validate seeds/engine-expansion-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open <http://127.0.0.1:8000/examples/authoring-cockpit/> to inspect the
static scene/entity/component view, Engine Expansion state panel, project
workspace panels, and copyable Rust validation/run/compare/mutation/dashboard
commands. The cockpit inspects exported data; it does not run seeds, compare
runs, persist edits, apply mutations, or write files from browser JavaScript.

The authoring cockpit is a static, local preview surface. It may display copyable Rust CLI commands such as `cargo run -p ouroforge-cli -- project validate`, `run --project`, `compare`, `scene validate`, `scene edit`, `scene reload-validate`, `mutation apply-scene --project`, and dashboard export commands, but it must not execute them from browser JavaScript. Persistent scene changes remain routed through Rust validation in the CLI. Browser-owned persistence APIs such as localStorage, indexedDB, showSaveFilePicker, direct file writes, native shell calls, hosted backends, auth, database, command bridges, auto-apply/auto-merge, plugin UI, and visual scripting are outside this demo boundary.


Reproducible run command context is shown as escaped display-only evidence when
`dashboard-data.json` includes `run_command_context`. The cockpit may show the
copyable command, seed path, workers, scenario pack, and Openchrome/CDP boundary,
but it must not execute, rerun, bridge, or persist that command. See
`docs/reproducible-run-command-context-v1.md`.


Studio Review Cockpit v1 shows proposal/review/application/promotion/matrix
status as escaped display-only cards from exported `review_cockpit`,
`mutation_lifecycle`, `regression_promotions`, and `regression_matrix` data. It
must not accept, apply, promote, rerun, write files, or execute displayed CLI
commands. See `../../docs/studio-review-cockpit-v1.md`.

Regression Run Matrix v1 is shown as escaped display-only project/scenario-pack
status from exported `regression_matrix` data. The cockpit may show current
status, last pass/fail labels, and context counts, but it must not schedule CI,
rerun scenarios, promote scenarios, execute commands, write scenario packs, or
store remote analytics. See `../../docs/regression-run-matrix-v1.md`.

Studio evidence fidelity surfaces show transaction provenance, Runtime Probe
Contract status, input replay presence, Openchrome/CDP evidence completeness,
and reproducible command context from exported dashboard data. Missing or
malformed evidence is displayed as warning/empty state. The cockpit remains
read-only and must not write files, execute commands, rerun QA, or apply
mutations. See `docs/studio-evidence-fidelity-surfaces.md`.


## Authoring Loop Dry-Run v1

When exported dashboard data includes `loop_dry_run`, the cockpit displays it as escaped read-only state with inert command text. The browser never executes dry-run command text, writes plan files, creates reports, mutates scenes, records decisions, or promotes regressions. See `../../docs/authoring-loop-dry-run-v1.md`.

## Authoring Loop Execution v1

When exported dashboard data includes `loop_execution`, the cockpit displays the Rust CLI step summary as escaped read-only evidence. The browser never runs loop steps, applies mutations, records decisions, promotes regressions, or writes trusted plan state. See `../../docs/authoring-loop-execution-v1.md`.

## Authoring Loop Recovery v1

When exported dashboard data includes `loop_recovery` or `loop_status`, the cockpit displays recovery state as escaped read-only evidence. The browser never resumes, retries, repairs artifacts, applies mutations, promotes regressions, or writes trusted plan state. See `../../docs/authoring-loop-recovery-v1.md`.
