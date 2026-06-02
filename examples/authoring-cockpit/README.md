# Authoring Cockpit v0

Minimal browser UI for inspecting the existing game-runtime scene and editing only the fields supported by the Rust scene edit model.

Run locally from the repo root:

```bash
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Open <http://127.0.0.1:8000/examples/authoring-cockpit/>.

The inspector updates scene data in memory and shows the validated `ouroforge scene edit` command for writing through Rust-side validation. Direct browser file writes are intentionally not supported.

## QA and evidence loop

The cockpit includes a Run QA panel with the exact local commands:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

After exporting dashboard data, refresh the cockpit to view the latest run evidence and journal pane. The browser still does not mutate files directly.

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
