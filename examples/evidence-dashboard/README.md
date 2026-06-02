# Evidence Dashboard v0

Read-only local UI for inspecting Ouroforge run artifacts.

Generate data from the repo root:

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open <http://127.0.0.1:8000/examples/evidence-dashboard/>. The UI only reads `dashboard-data.json` and links to artifact paths; it does not edit run artifacts.

Runtime v1 demo inspection:

```bash
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Refresh the dashboard after export to inspect the generated run, verdict, journal, screenshots, and Runtime v1 scenario evidence.
