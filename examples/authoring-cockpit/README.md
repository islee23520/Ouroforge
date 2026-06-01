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
