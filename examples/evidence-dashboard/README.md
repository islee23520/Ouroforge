# Evidence Dashboard v1

Read-only local UI for browsing Ouroforge runs and evidence artifacts from the
generated dashboard export.

Generate data from the repo root:

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
python3 -m http.server 8000 --bind 127.0.0.1 --directory .
```

Then open <http://127.0.0.1:8000/examples/evidence-dashboard/>. The UI only
reads `dashboard-data.json` and links to artifact paths; it does not edit,
delete, upload, or index run artifacts outside the generated export.

Runtime v1 demo inspection:

```bash
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 4
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Refresh the dashboard after export to inspect run status, seed, verdict,
scenario status, worker count, evidence categories, artifact links, journals,
screenshots, Runtime v1 scenario evidence, and mutation artifacts when present.

Journal Viewer v1 renders journal sections as escaped text and links referenced
evidence, verdict, and mutation ids from the generated dashboard data. It does
not edit journals, execute markdown, or generate AI summaries.
