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
delete, upload, or index run artifacts outside the generated export. For Visual
Authoring Demo v1, dashboard wording must stay display-only and conservative:
it may show generated smoke ids, visual diff summaries, review/apply/rerun/compare
refs, and Studio-compatible read models, but it must not claim browser trusted
writes, command execution, hosted/public launch behavior, production editor
status, plugin runtime, visual scripting, native export, or Godot replacement.



Project-bound run inspection:

```bash
cargo run -p ouroforge-cli -- project init .omx/tmp/project-dashboard-smoke --template minimal-2d
cargo run -p ouroforge-cli -- run .omx/tmp/project-dashboard-smoke/seeds/platformer.yaml --project .omx/tmp/project-dashboard-smoke --scenario-pack smoke --workers 1
LATEST_RUN=$(ls -td runs/run-* | head -1)
cargo run -p ouroforge-cli -- journal update "$LATEST_RUN"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
rm -rf .omx/tmp/project-dashboard-smoke
```

Refresh the dashboard after export to inspect the read-only Project Context
panel. The browser displays Rust-exported project metadata only; it does not
validate project manifests, write files, execute commands, or infer trusted
project state.

Engine Expansion v1 playable-template inspection:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 4
LATEST_RUN=$(ls -td runs/run-* | head -1)
cargo run -p ouroforge-cli -- compare "$LATEST_RUN" "$LATEST_RUN" --output-dir "$LATEST_RUN/comparisons"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Refresh the dashboard after export to inspect the platformer template run, its
2 scenario results, linked world-state/fixture evidence, verdict, journal, and
read-only comparison artifact, including Project Comparison v1 fields when present. Generated `runs/` and `dashboard-data.json`
remain local inspection state and must not be committed.

Engine Expansion v1 integration-demo inspection:

```bash
cargo run -p ouroforge-cli -- seed validate seeds/engine-expansion-v1-demo.yaml
cargo run -p ouroforge-cli -- run seeds/engine-expansion-v1-demo.yaml --workers 4
BEFORE_RUN=$(ls -td runs/run-* | sed -n '2p')
AFTER_RUN=$(ls -td runs/run-* | head -1)
test -n "$BEFORE_RUN"
test -n "$AFTER_RUN"
cargo run -p ouroforge-cli -- compare "$BEFORE_RUN" "$AFTER_RUN" --output-dir "$AFTER_RUN/comparisons"
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Refresh the dashboard after export to inspect the integration seed, scenario
results, verdict, journal, comparison artifact, screenshots, world-state
artifacts, and Engine Expansion summary cards. The dashboard remains read-only
and does not execute comparisons, accept mutations, or infer semantic gameplay
quality in browser JavaScript.

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

Regression Promotions v1 renders run-local `regression-promotions/*.json`
records and copyable `scenario promote --dry-run` commands as escaped text.
It does not generate drafts, promote scenarios, mutate scenario packs, execute
commands, or bridge to a shell from browser JavaScript.

Regression Run Matrix v1 reads top-level `regression_matrix` from the generated
dashboard export and displays project/scenario-pack scenario history with current
status, last pass, last fail, and context counts. It is display-only: no browser
reruns, scheduling, auto-promotion, source writes, or shell bridge. See
`../../docs/regression-run-matrix-v1.md`.

Replay Controls v1 renders deterministic replay evidence with local,
in-memory step/reset/jump controls. It displays the current frame/tick,
evidence links, and linked world-state snapshots from the generated export.
It does not edit replay inputs, record new inputs, persist browser-side replay
state, or mutate run artifacts.


Openchrome/CDP Evidence Fidelity v2 inspection:

```bash
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 2
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
```

Refresh the dashboard after export to inspect escaped worker/session metadata for
worker screenshots, console logs, performance metrics, and CDP trace summaries.
The dashboard displays metadata produced by Rust/openchrome CDP workers, including
`worker_id`, `worker_session_id`, `run_id`, `execution_boundary`, and
`cdp_transport`; it does not compute browser-side comparisons, rerun workers,
write files, or execute commands. Agent handoffs exported as `agent_handoffs` and the normalized Studio loop cockpit read-model exported as `loop_cockpit` are rendered as escaped read-only evidence; command and next-action text remains inert and never becomes a browser command button, command bridge, write path, apply/promote/merge control, or authority grant. Remove generated `dashboard-data.json` before
committing. See `docs/openchrome-cdp-evidence-fidelity-v2.md`.


Reproducible Run Command Context v1 inspection:

```bash
cargo run -p ouroforge-cli -- run seeds/platformer.yaml --workers 1
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
```

Refresh the dashboard to inspect the escaped, display-only Reproducible Command
Context panel. The panel is copied from Rust-authored `run.json` metadata and
must not execute commands, rerun QA, write files, or start a command bridge. See
`docs/reproducible-run-command-context-v1.md`.

Evidence-Backed Journal v2 appears through the existing Journal Viewer when
`journal.md` contains `Authoring Governance Lifecycle` /
`journal-authoring-governance-v2`. The dashboard only displays escaped exported
journal snippets and links; it does not generate summaries, apply mutations, or
write journal files. See `../../docs/evidence-backed-journal-v2.md`.


## Visual Authoring Demo v1 dashboard boundary audit

For #352 VA1.10.3, dashboard display of Visual Authoring Demo v1 evidence is
limited to escaped, read-only data exported by trusted Rust/local commands:

- preview ids, review decisions, visual edit application ids, run ids, compare
  summaries, and dashboard export paths may be displayed as evidence links or
  inert text only;
- generated smoke artifacts remain local/ignored and are not bundled as public
  release assets;
- dashboard JavaScript must not execute copied commands, create review
  decisions, apply drafts, rerun scenarios, write files, upload/fetch assets, or
  bridge to a shell/local server;
- public wording must describe this as a conservative local demo workflow, not a
  production editor, public launch, hosted service, plugin runtime, visual
  scripting system, native export path, or Godot replacement.

Use Node syntax/tests plus a wording scan before claiming this audit is current.
