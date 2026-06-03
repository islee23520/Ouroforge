# Runtime Asset Loading Evidence v1

Runtime Asset Loading Evidence v1 records generated evidence for local project
asset load attempts observed by the browser runtime and persisted by the Rust
scenario runner. It is an evidence/read-model contract, not an asset hosting,
upload, editor, or packaging feature.

## Evidence shape

Scenario runs may emit `runtime_asset_load_evidence` JSON artifacts when runtime
world state contains asset load metadata. Each artifact records:

- `runId`, `workerId`, `workerSessionId`, and optional `scenarioId` correlation;
- optional `manifestId` from the runtime world state;
- one or more load records with `attemptId`, `assetId`, `assetType`, safe local
  `path` when accepted, `status`, timing, optional dimensions/duration, and
  explicit `failureReason` for failed/rejected/fallback statuses.

Statuses are intentionally explicit: `attempted`, `loaded`, `failed`,
`rejected`, and `fallback`. Failed, rejected, and fallback records require a
reason so missing assets do not disappear behind silent defaults.

## Runtime capture boundary

The browser runtime may observe load attempts and emit in-memory runtime events,
but it does not own trusted persistence. Rust reads exported probe/world-state
data, validates source-like local asset paths, rejects unsafe paths such as
absolute paths, remote URLs, `..` escapes, generated roots, or hidden local tool
roots, and writes generated evidence under the run directory.

The runtime/browser surface must not:

- fetch remote assets or add a remote asset pipeline;
- upload files or write trusted manifests/scenes/assets;
- execute commands, install dependencies, or bridge to a local server;
- package/export assets, implement native export, or act as a production editor;
- silently substitute fallback assets without an explicit rejected/fallback event.

## Dashboard and Studio read models

Dashboard exports aggregate parsed runtime loading artifacts under
`asset_loading`. The read model includes attempt/loaded/failed/rejected/fallback
counts, evidence refs, parsed load records, and a boundary string. The evidence
dashboard and authoring cockpit display this data as escaped read-only state.
They may link to generated local evidence files but do not load remote assets,
write files, upload assets, rerun scenarios, or execute commands.

Runs produced before this contract remain readable: missing loading evidence is
shown as an empty state rather than inferred from unrelated assets or world-state
fields.

## Limitations

- Browser timing is observational and intended for local smoke evidence, not
  production performance benchmarking.
- Dimensions/duration are recorded only when the runtime can observe them.
- Audio/font media duration may be absent until a later issue scopes richer media
  probes.
- Read-model rows are summaries; the generated JSON artifacts remain the source
  of truth for review.
- Runtime loading evidence does not replace manifest/reference integrity checks;
  it complements Rust-trusted asset validation by proving observed local runtime
  load behavior.

## Verification

Focused changes should run the relevant runtime evidence tests plus dashboard and
cockpit smoke checks:

```bash
cargo test runtime_asset_load_evidence --lib
node --check examples/game-runtime/assets.js
node examples/game-runtime/assets.test.cjs
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Issue-level closure still requires the broad gates listed in
`docs/asset-pipeline-v1.md` and confirmation that #1 and #23 remain open.
