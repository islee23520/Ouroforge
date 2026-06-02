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


Engine Expansion v1 playable-template inspection:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
LATEST_RUN=$(ls -td runs/run-* | head -1)
cargo run -p ouroforge-cli -- compare "$LATEST_RUN" "$LATEST_RUN"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Refresh the dashboard after export to inspect the platformer template run, its
2 scenario results, linked world-state/fixture evidence, verdict, journal, and
read-only comparison artifact. Generated `runs/` and `dashboard-data.json`
remain local inspection state and must not be committed.

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

Mutation Review UI v1 renders mutation lifecycle artifacts and manual review
command hints as escaped, copyable text. It does not apply patches, write
accepted/rejected decisions, run Git operations, or call GitHub from the
browser.

Replay Controls v1 renders deterministic replay evidence with local,
in-memory step/reset/jump controls. It displays the current frame/tick,
evidence links, and linked world-state snapshots from the generated export.
It does not edit replay inputs, record new inputs, persist browser-side replay
state, or mutate run artifacts.
