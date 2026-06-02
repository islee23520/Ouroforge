# Studio Evidence Fidelity Surfaces

Issue #292 extends the static authoring cockpit so evidence fidelity state is
visible from exported dashboard data. Studio remains read-only: it displays
status cards, warnings, evidence refs, and copyable commands, but it does not
write files, execute local commands, rerun QA, apply mutations, merge branches,
or start a command bridge.

## Read-model source

`dashboard export` now includes `evidence_fidelity` on run detail and summary
read models. The current status groups are:

- `transaction` — scene edit transaction provenance presence or warning;
- `runtime_probe` — Runtime Probe Contract v2 status;
- `input_replay` — input replay sequence presence/completeness;
- `openchrome_cdp` — worker screenshots, console logs, performance metrics, and
  CDP trace summary completeness;
- `command_context` — reproducible run command context presence.

Each status contains a bounded `status`, `summary`, `observed_count`,
`missing_count`, `warnings`, and `evidence_refs` list. Missing data is an explicit
warning/empty state, not an inferred pass.

## Cockpit surface

The authoring cockpit renders an **Evidence fidelity** panel from the exported
read model. It uses escaped text only and bounds status CSS classes to known
values (`present`, `partial`, `missing`, `malformed`, `legacy`, `unknown`).
Malformed status entries are displayed as unavailable instead of being inferred.

The panel intentionally avoids:

- browser file writes;
- local command execution;
- browser/local command bridges;
- auto-rerun;
- auto-apply / auto-merge;
- production editor or visual scripting claims.

## Local smoke command

```bash
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
rm -f examples/evidence-dashboard/dashboard-data.json
```

`examples/evidence-dashboard/dashboard-data.json` is generated local state and
must not be committed.

## Fresh EF1.8.4 smoke evidence

Fresh local smoke evidence for #292 EF1.8.4:

- Dashboard export: `examples/evidence-dashboard/dashboard-data.json` generated
  locally and removed before commit.
- Exported run count: 319 local generated runs.
- Latest run inspected: `run-1780423522020-71709`.
- Evidence fidelity keys present in export:
  - `command_context`
  - `input_replay`
  - `openchrome_cdp`
  - `runtime_probe`
  - `transaction`
- Evidence dashboard Node syntax/smoke passed.
- Authoring cockpit Node syntax/smoke passed.

Generated `runs/`, `.omx/`, `.openchrome/`, `target/`, and dashboard export data
remain local/untracked.
