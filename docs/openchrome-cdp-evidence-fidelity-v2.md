# Openchrome/CDP Evidence Fidelity v2

Issue #290 hardens the local browser-worker evidence chain without changing the
execution boundary. Ouroforge still runs local Rust-owned workers against a local
Chrome DevTools Protocol target through openchrome-compatible CDP access. The
browser remains an observation surface: it may produce screenshots, console
logs, performance metrics, CDP trace summaries, and failure artifacts, but it
does not write trusted source-like project state or execute repository commands.

## Evidence contract

Each browser-worker evidence artifact is expected to be traceable back to the
run and worker context that produced it. Worker-owned browser artifacts include
metadata such as:

- `artifact` — normalized artifact category, for example `screenshot`,
  `console_log`, `performance_metrics`, `cdp_trace_summary`, or
  `browser_worker_failure`;
- `worker_id` — stable worker label within the run, such as `worker-1`;
- `worker_session_id` — run/worker join key, such as
  `<run-id>:<worker-id>`;
- `run_id` — generated run id;
- `evidence_dir` — run-relative evidence directory for the worker;
- `execution_boundary` — `openchrome_cdp`;
- `cdp_transport` — `chrome_devtools_protocol`;
- target binding fields such as `target_selection`, `target_ws_url_bound`, and
  the local runtime URL when available;
- artifact-specific bounds such as `bounded`, `limit`, and `optional`.

Scenario-level console, performance, and CDP summary artifacts remain indexed as
scenario evidence. Browser-worker artifacts add worker/session metadata so local
multi-worker smoke evidence can be audited without guessing which CDP target or
worker generated a file.

## Failure evidence

Worker failures are explicit evidence, not silent fallbacks. When target setup or
a worker run fails, Ouroforge writes an indexed `browser_worker_failure` JSON
artifact under the worker evidence directory and records a `browser.worker.failed`
ledger event with the generated failure path. This keeps pass/fail/failure cases
visible to later dashboard, journal, compare, and manual issue-review surfaces.

Failure evidence does not make the browser authoritative. Rust-owned verdict and
scenario logic still decide run status; browser failure artifacts only explain
what happened at the observation boundary.

## Dashboard/read-model behavior

`dashboard export` preserves console/performance/CDP summary artifacts and their
metadata in the generated dashboard JSON. The static dashboard renders selected
artifact metadata as escaped text and keeps malformed/missing artifact warnings
visible. It must not:

- compute browser-side comparisons;
- execute commands;
- start a command bridge;
- write files;
- retry or rerun browser workers;
- accept, apply, or merge proposals.

The dashboard is a read-only inspection surface over Rust-generated export data.

## Local smoke command

Use a local Chrome-capable environment for the runtime smoke:

```bash
cargo run -p ouroforge-cli -- run seeds/runtime-v1-demo.yaml --workers 2
cargo run -p ouroforge-cli -- dashboard export --runs-root runs --output examples/evidence-dashboard/dashboard-data.json
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
rm -f examples/evidence-dashboard/dashboard-data.json
```

`runs/` and `examples/evidence-dashboard/dashboard-data.json` are generated local
state. Do not commit them.

## Fresh EF1.6.4 smoke evidence

Fresh local smoke evidence for #290 EF1.6.4:

- Run: `runs/run-1780422766075-64093`
- Browser workers: 2/2 passed
- Scenario suite: 1/1 passed
- Verdict: `passed`
- Verdict summary: `1 scenario result(s) passed with consistent evidence.`
- Dashboard export: `examples/evidence-dashboard/dashboard-data.json` generated
  locally for inspection and removed before commit.
- Worker screenshot artifacts: 2 indexed worker screenshots.
- Worker console artifacts: 2 worker console logs plus scenario console evidence.
- Worker performance artifacts: 2 worker performance metric files plus scenario
  performance evidence.
- Worker CDP artifacts: 2 worker CDP trace summaries plus scenario CDP summary
  evidence.
- Worker failure artifacts: 0 for this passing smoke.
- Representative worker metadata:
  - `execution_boundary: openchrome_cdp`
  - `cdp_transport: chrome_devtools_protocol`
  - `worker_session_id: run-1780422766075-64093:worker-1`
  - `worker_session_id: run-1780422766075-64093:worker-2`

Verification run alongside the smoke:

```bash
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
```

The Node dashboard smoke passed with generated dashboard data and after the
export was removed.

## Limitations and non-goals

Openchrome/CDP Evidence Fidelity v2 intentionally does not add:

- Playwright;
- hosted browser farms;
- distributed QA or Elixir orchestration;
- browser-side trusted writes;
- command bridges;
- video capture pipelines;
- production telemetry or analytics infrastructure;
- browser-side verdict authority.

If Chrome is unavailable in a future CI or local environment, issue/PR evidence
should record the smoke gap and use the focused Rust and Node tests as the
next-best validation. That gap must not be hidden as a passing browser smoke.
