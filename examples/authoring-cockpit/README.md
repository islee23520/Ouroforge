# Studio v3 Project Workspace Cockpit

Static local browser UI for composing Project Workspace Loop v1 surfaces: project manifest context, project-bound run evidence, expressive component/trigger/HUD inspection, collision/transition/runtime-event inspection, Asset Pipeline v1 inspector panels, tilemap draft preview read models, scene edit command generation, transaction provenance, journal viewing, project semantic comparison, project-scoped scene-only mutation lifecycle state, replay evidence, live preview controls, and Rust-validated command strings.

Run locally from the repo root:

```bash
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Open <http://127.0.0.1:8000/examples/authoring-cockpit/>.

The inspector exposes inline edit controls for a subset of scalar fields and shows the validated `ouroforge scene edit` command for writing through Rust-side validation. Direct browser file writes are intentionally not supported.

Cockpit inline-editable scalar fields:

- `sprite.color`
- `components.transform.x`
- `components.transform.y`
- `components.velocity.x`
- `components.velocity.y`
- `components.size.width`
- `components.size.height`
- `components.controllable`

Additional Rust-validated scalar scene edit paths — validated by the
`ouroforge scene edit` command and preflightable as Scene Visual Edit Draft v1
`sceneOperation` records, but not surfaced as cockpit inline edit controls:

- `components.status.hitPoints`
- `components.status.maxHitPoints`
- `components.input.moveSpeed`
- `components.input.jumpImpulse`
- `components.cameraTarget.weight`
- `components.uiText.text`

Scene Visual Edit Draft v1 may describe these supported scalar edits as inert
`sceneOperation` records and Rust can preflight them into transaction previews
without applying writes. All other scene fields remain read-only in the cockpit
and are rejected before trusted writes. The browser still does not execute
commands, write files, persist trusted draft state, or apply previews.

Tilemap Visual Edit Draft v1 may describe bounded `tilemapOperation` records and
Rust can preflight them into inert preview summaries with affected-cell counts,
before/after preview hashes, and collision/trigger metadata. When
`dashboard-data.json` includes `tilemap_draft_preview`, the cockpit displays that
read model as escaped diagnostics only. It does not write tilemaps, persist draft
state, execute preview/apply commands, or treat preview metadata as review
approval.

Visual Diff Preview v1 may display Rust-generated `visual-diff-summary-v1`
records from `dashboard-data.json` under `visual_diff_preview` or
`visualDiffPreview`. The panel renders before/after summary text, operation
rows, source refs, collision/trigger counts, asset/entity/tile references, and
scenario-impact notes as escaped read-only diagnostics. It intentionally has no
apply buttons, browser persistence, trusted writes, command execution, local
server bridge, or review-decision controls; any trusted write remains routed
through Rust CLI transactions and review-gated apply.

Studio Draft Authoring Surface v1 may display temporary in-memory draft
read-models from `dashboard-data.json` under `studio_draft_authoring` or
`studioDraftAuthoring`. The surface supports scene, tilemap, and
asset-reference draft rows as escaped display-only data with disabled/read-only
controls, blocked-state hints, copyable draft JSON text, and copyable Rust CLI
preview command text. The browser does not persist trusted draft state, write
project/scene/tilemap/asset files, upload or fetch assets, execute local
commands, bridge to a local server, apply edits, or treat preview metadata as a
review decision. Trusted validation, transaction creation, review-gated apply,
and persistence stay in the Rust CLI/manual terminal flow. Visual Authoring Demo
v1 documentation must describe this as a local static demo over Rust-exported
evidence, not a production editor, public launch, hosted Studio, plugin runtime,
visual scripting system, native export path, command bridge, or Godot
replacement. Generated preview/transaction/run/compare/dashboard artifacts stay
untracked unless a later issue explicitly scopes a tiny source-like fixture.

Review-gated visual edit application evidence may appear in exported lifecycle data such as `mutation/visual-edit-applications.json`, review cockpit cards, journal sections, loop status/read-model summaries, or handoff evidence refs. The cockpit may render draft/proposal/patch-draft/decision ids, transaction links, before/after hashes, rollback metadata, rerun/compare refs, and reproducible command context as escaped read-only diagnostics. It must not turn those fields into apply, rerun, promote, resume, repair, or command-execution controls.

## Studio v3 demo surfaces

The cockpit composes completed local surfaces only:

- project workspace manifest/scene/seed/scenario pack context from generated dashboard data;
- project-bound run summary and generated-state status;
- run/evidence browser from generated dashboard data;
- expressive scene inspection for Rust-exported component counts, entity components, triggers, flags, and HUD values;
- collision/transition/event inspection for Rust-exported collision rules/events, scene transitions, animation state, and audio/runtime events;
- journal viewer when journal data exists;
- mutation review lifecycle state and manual command hints when artifacts exist;
- Studio Review Cockpit v1 proposal/review/application cards from exported lifecycle state;
- project-scoped scene-only mutation proposal/application lifecycle state when artifacts exist;
- regression promotion records and display-only dry-run commands when artifacts exist;
- regression run matrix status from generated dashboard data when project-bound runs exist;
- replay evidence surface when replay artifacts exist;
- live preview controls through the existing runtime probe;
- visual diff preview summaries with escaped before/after, operation, source-ref,
  collision/trigger, and scenario-impact notes from Rust-exported read models;
- tilemap draft preview summaries, affected-cell counts, hashes, and
  collision/trigger metadata from Rust-exported read models;
- scene edit command generation for Rust-validated fields;
- transaction-bound QA command generation;
- semantic run comparison artifact surface, including Project Comparison v1 context, when comparison artifacts exist;
- Asset Inspector v1 manifest/status, asset rows, atlas frame, tilemap, and runtime load evidence from Rust-exported dashboard data.

Known gaps are intentionally visible: no production editor, hosted studio, native shell, collaboration, plugin/marketplace UI, visual scripting, direct browser file writes, browser-side comparison algorithms, command bridge, or mutation acceptance/application from the browser. Studio Review Cockpit v1 review/regression boundaries are documented in `../../docs/studio-review-cockpit-v1.md`. Studio v3 verification evidence is recorded in `../../docs/studio-v3-project-workspace-cockpit.md`; Studio v2 evidence remains in `../../docs/studio-v2-cockpit.md`; legacy Studio v1 evidence remains in `../../docs/studio-v1-demo.md`.

## Studio Asset Inspector v1

When exported dashboard data includes `asset_inspector`, the cockpit displays a
read-only Asset Inspector panel with manifest/status counts, asset ids/types/paths,
warnings, atlas frame rows, tilemap summaries, runtime load attempts, and evidence
refs. The browser escapes displayed values and provides empty states when asset
evidence is unavailable. It never uploads assets, writes manifests or generated
evidence, fetches remote assets, executes commands, or acts as a marketplace,
plugin host, native export path, production editor, or visual asset editor. See
`../../docs/studio-asset-inspector-v1.md`.

## Production 2D runtime inspection boundary

When `dashboard-data.json` includes Production 2D read models, the cockpit may
display renderer/layer/camera, physics/collision/contact, input/action/replay,
animation/VFX/audio, save/load/runtime-state digest, and profiler/frame-budget
evidence as escaped read-only state. The runtime save/state panel shows
snapshots, save created/loaded events, replay digest comparisons, digest values,
authority labels, and disallowed actions for inspection only.

These panels do not write source, scene, tilemap, asset, save, project,
dashboard, run, or evidence files; do not persist trusted browser state; do not
execute commands; do not bridge to a local server; do not mutate saves or replay
baselines; and do not claim production editor, hosted Studio, native export,
plugin runtime, visual scripting, public launch, production-ready engine, or
Godot replacement behavior. Rust/local workflows remain the trusted authority
for validation, persistence, generated evidence, source-like fixtures, save/load
state, replay digest comparison, and CLI behavior. Generated run/dashboard/
screenshot/temp/local tool outputs stay untracked unless a later issue
explicitly scopes a tiny deterministic source-like fixture. See
`../../docs/production-2d-studio-inspection-v1.md`.

## Expressive Scene Inspection v2

When `dashboard-data.json` includes the Engine Expansion read model, the cockpit
displays two read-only Studio Authoring Surface v2 panels:

- **Expressive scene inspection**: component counts, entity components, trigger
  bindings, required flags, and HUD values.
- **Collision/transition/event inspection**: collision rules and events,
  manifest-validated declared scene transitions, transition event rows, reload
  status, animation entities, and audio/runtime events.

Both panels consume Rust-exported evidence only. They escape missing, malformed,
or hostile data, and they must not execute commands, write files, persist browser
state, own source scene truth, or replace Rust validation. Refresh exported
`dashboard-data.json` after running `dashboard export` to inspect newer evidence.

## Studio 2D Engine Inspection Surface v1

When `dashboard-data.json` includes Production 2D read models, the cockpit also
displays read-only inspection panels for camera/layer state, render breakdowns,
input actions, runtime save/state and replay digests, frame-budget/profiler
evidence, collision/transition/runtime events, animation/VFX rows, and audio
intent evidence. These panels are escaped static diagnostics over Rust-exported
data. They show empty or malformed states without crashing, and they do not write
files, execute commands, persist browser state, control the runtime as trusted
authority, apply mutations, rerun tests, upload/fetch assets, or bridge to a
local server.

See `../../docs/studio-2d-engine-inspection-surface-v1.md` for the #593 boundary
audit and verification checklist.

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
static scene/entity/component view, Engine Expansion state panel, expressive scene inspection, collision/transition/event inspection, project
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

## Authoring Loop Evidence Bundle v1

When exported dashboard data includes `loop_evidence_bundles`, the cockpit displays bundle status, artifact counts, step states, and missing refs as escaped read-only evidence. The browser never writes bundle data, packages artifacts, executes commands, repairs references, applies mutations, or promotes regressions. See `../../docs/authoring-loop-evidence-bundle-v1.md`.


## Studio Loop Cockpit v1

When exported dashboard data includes `loop_cockpit`, the cockpit displays a static Loop Cockpit panel with the loop plan/status timeline, current step, blockers, required decisions, next safe action, inert allowed command text, forbidden actions, evidence refs, missing bundle refs, and trust-boundary notices. Missing or malformed cockpit data is shown as an empty/warning state. The browser never executes loop commands, writes files, resumes loops, applies mutations, promotes regressions, repairs references, or merges changes; all trusted actions remain Rust CLI/manual terminal actions.

## Agent Handoff Contract v1

When exported dashboard data includes `agent_handoffs`, the cockpit displays a read-only Handoff Studio surface with the next safe action, blockers, required decisions, inert allowed command text, forbidden actions, evidence refs, guardrails, and boundary. The browser never creates command buttons, executes handoff commands, grants authority, applies mutations, repairs references, or merges changes. See `../../docs/agent-handoff-contract-v1.md`.


## Visual Authoring Demo v1 Studio boundary audit

For #352 VA1.10.3, Studio/cockpit presentation of the collect-and-exit visual
authoring demo remains an inert local authoring aid:

- scene, tilemap, and asset-reference draft rows are temporary/read-only display
  state; the browser may show copyable JSON or CLI text but must not persist
  trusted draft state;
- visual diff, review, apply, rerun, compare, and generated smoke ids are
  displayed only after Rust/local evidence export; the browser does not create
  review decisions, run commands, apply edits, rerun QA, compare runs, or write
  dashboard data;
- tilemap and asset-reference controls remain preview/display surfaces only, not
  upload/fetch/import/apply workflows;
- public copy must keep the demo framed as local-first Safe Local Edit Cockpit
  evidence, with no production editor, public launch, native export, plugin
  runtime, hosted service, visual scripting, command bridge, or Godot replacement
  claim.

VA1.10.3 changes documentation only. Any future trusted write must stay routed
through Rust validation, explicit review gates, rollback/evidence records, and
source/generated-state audits.
