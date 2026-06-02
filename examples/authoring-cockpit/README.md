# Studio v2 Authoring Cockpit

Static local browser UI for composing Authoring Loop v2 surfaces: scene edit command generation, transaction provenance, QA evidence inspection, journal viewing, semantic run comparison, scene-only mutation lifecycle state, replay evidence, live preview controls, and Rust-validated command strings.

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


## Studio v2 demo surfaces

The cockpit composes completed local surfaces only:

- run/evidence browser from generated dashboard data;
- journal viewer when journal data exists;
- mutation review lifecycle state and manual command hints when artifacts exist;
- scene-only mutation proposal/application lifecycle state when artifacts exist;
- replay evidence surface when replay artifacts exist;
- live preview controls through the existing runtime probe;
- scene edit command generation for Rust-validated fields;
- transaction-bound QA command generation;
- semantic run comparison artifact surface, including Project Comparison v1 context, when comparison artifacts exist.

Known gaps are intentionally visible: no production editor, hosted studio, native shell, collaboration, plugin/marketplace UI, visual scripting, direct browser file writes, browser-side comparison algorithms, or mutation acceptance from the browser. Studio v2 verification evidence is recorded in `../../docs/studio-v2-cockpit.md`; legacy Studio v1 evidence remains in `../../docs/studio-v1-demo.md`.

## QA and evidence loop

The cockpit includes a Run QA panel with local display-only commands:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

After exporting dashboard data, refresh the cockpit to view the latest run evidence, authoring provenance, semantic comparison, scene-only mutation lifecycle, and journal panes. The browser still does not mutate files directly.

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


## Studio v2 command boundary

Engine Expansion v1 integration-demo inspection:

```bash
cargo run -p ouroforge-cli -- seed validate seeds/engine-expansion-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open <http://127.0.0.1:8000/examples/authoring-cockpit/> to inspect the
static scene/entity/component view, Engine Expansion state panel, and copyable
Rust validation/reload/dashboard commands. The cockpit inspects the same
`examples/game-runtime/scene.json` used by the integration seed; it does not run
the seed, compare runs, persist edits, or write files from browser JavaScript.

The authoring cockpit is a static, local preview surface. It may display copyable Rust CLI commands such as `cargo run -p ouroforge-cli -- scene validate`, `scene edit`, `scene reload-validate`, and dashboard export commands, but it must not execute them from browser JavaScript. Persistent scene changes remain routed through Rust validation in the CLI. Browser-owned persistence APIs such as localStorage, indexedDB, showSaveFilePicker, direct file writes, native shell calls, hosted backends, auth, database, plugin UI, and visual scripting are outside this demo boundary.
