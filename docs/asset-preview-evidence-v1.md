# Asset Preview Evidence v1

Asset Preview Evidence v1 is a Rust-trusted, read-only evidence contract for
local project asset preview metadata. It lets reviewers inspect asset status,
image dimensions, sprite atlas frames, tilemap metadata, audio/font metadata,
and missing/stale warnings without granting browser/dashboard/Studio authority
to upload, write, fetch remote assets, or execute commands.

## Evidence model

`asset-preview-evidence-v1` artifacts contain:

- `manifestId`, optional `runId`, and `generatedAtUnixMs` provenance;
- `previews[]` rows keyed by `assetId`, `assetType`, `sourcePath`, and
  `previewKind`;
- optional image dimensions, atlas frame rectangles, tilemap dimensions/layer and
  tile counts, audio duration/channel metadata, and font family/style metadata;
- optional generated or source-like `previewPath` references when a later issue
  scopes concrete preview artifacts;
- top-level and per-record warnings such as missing source files or stale hashes;
- a boundary string that repeats the read-only/no-browser-authority contract.

The schema rejects unsafe source paths, hidden local tool roots, `..` escapes,
absolute paths, remote URLs, invalid ids, duplicate asset rows, empty evidence,
zero dimensions/durations, and generated-root paths marked as source-like.

## Generation/export boundary

The Rust helper `generate_asset_preview_evidence` exports metadata from a
validated `ProjectAssetManifest` and reuses source asset integrity checks to
surface warnings. It does not create image thumbnails, transcode media, write
preview files, upload assets, or publish packages. Generated preview outputs, if
introduced by a later issue, must remain ignored local state unless explicitly
scoped as tiny deterministic source-like fixtures.

## Dashboard and Studio compatibility

Dashboard exports can surface parsed preview artifacts as `asset_preview` with
counts, warning rows, evidence refs, parsed records, and a display boundary. The
evidence dashboard and authoring cockpit render this data as escaped read-only
HTML. Older runs without preview evidence remain compatible and show an empty
state instead of inferring preview status.

Browser surfaces must not:

- fetch remote asset URLs or host previews;
- upload or write project assets, manifests, scenes, generated evidence, or run
  state;
- execute commands, install dependencies, or bridge to a local server;
- edit assets or present a production visual asset editor;
- claim native export, marketplace, plugin, or cloud-storage support.

## Verification

Focused checks for this contract include:

```bash
cargo test asset_preview_evidence --lib
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

Issue-level closure still requires the broad gates in `docs/asset-pipeline-v1.md`,
latest-main verification, generated-state audit, and confirmation that #1 and
#23 remain open.
