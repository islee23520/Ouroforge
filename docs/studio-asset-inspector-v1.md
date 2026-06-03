# Studio Asset Inspector v1

Studio Asset Inspector v1 is a read-only Authoring Cockpit surface for Asset
Pipeline v1 evidence. It lets authors inspect local project asset state exported
by Rust dashboard data without moving trusted asset authority into the browser.

## Inputs

The cockpit consumes generated dashboard JSON fields only:

- `asset_inspector`: normalized manifest/status read model assembled by Rust from
  asset integrity, runtime loading, and preview evidence.
- `asset_loading`: runtime load attempts, statuses, paths, durations, and failure
  reasons.
- `asset_preview`: preview records, atlas frame metadata, tilemap summaries, and
  preview warnings.

All of those inputs are generated evidence. Refresh them with the Rust CLI and
keep generated run/dashboard outputs untracked unless a separate issue explicitly
scopes a tiny deterministic fixture.

## Displayed evidence

The `examples/authoring-cockpit/` Studio panel shows:

- manifest/status counts, asset count, warning count, preview count, atlas frame
  count, tilemap count, runtime attempt count, and loaded/failed totals;
- asset id, type, path, hash when present in the exported read model, warning
  labels, runtime statuses, atlas frame counts, and tilemap dimensions;
- atlas frame rows with frame id and rectangle metadata from preview evidence;
- tilemap rows with dimensions, layer count, tile count, and tileset asset id;
- runtime load rows with attempt id, local path, status, duration, and failure
  reason; and
- evidence refs linking the display back to generated asset evidence.

The browser escapes all displayed fields before inserting markup and shows empty
states when data is missing.

## Trust boundary

The inspector is display-only. It must not:

- upload assets;
- write manifests, scenes, dashboard data, previews, or generated evidence;
- fetch remote assets or CDN resources;
- execute commands, start a local server bridge, or call native shell APIs;
- install dependencies, load plugins, export native packages, or act as an asset
  marketplace; or
- claim production-editor or visual asset-editor behavior.

Rust/local CLI code remains the trusted boundary for manifest parsing, path/hash
validation, integrity checks, runtime loading evidence, preview evidence, and any
future persistence.

## Reproducible smoke

From the repository root, the source-only smoke for this surface is:

```bash
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Broad issue closure for #339 also runs the Rust/dashboard gates listed in
`docs/asset-pipeline-v1.md`. These checks do not require committing generated
`runs/`, dashboard exports, temporary preview output, or local tool state.

## Closure evidence for #339

The #339 AP1.8.4 closure pass refreshed this document and verified that:

- #339 is the active Studio Asset Inspector v1 issue;
- #1 and #23 remain open;
- cockpit tests cover populated, empty, and hostile/XSS asset inspector data;
- browser code still has no direct persistence, upload, fetch, command execution,
  or native shell bridge; and
- generated/local state remains outside tracked source changes.
