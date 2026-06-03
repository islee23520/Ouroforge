# Asset Reference Integrity v1

Asset Reference Integrity v1 completes the Asset Pipeline v1 content-authoring
contract for scene references that point at project asset manifest ids. It is a
local-first validation and evidence surface: Rust owns trusted validation,
warning generation, and evidence read models; browser/dashboard surfaces only
render exported data.

## Scope

The integrity contract covers scene references collected from:

- entity sprite `asset` fields;
- animation frame and clip frame `asset` fields;
- audio event `asset` fields;
- tilemap tile `asset` fields.

Project asset manifests resolve these references by stable asset id. Image-like
scene references must resolve to `image` or `sprite_atlas` assets. Audio event
references must resolve to `audio` assets. Reference strings are id components,
not URLs or filesystem paths.

## Hard validation vs warning evidence

Ouroforge exposes two related Rust-owned checks:

1. `ProjectAssetManifest::validate_scene_references(scene)` is the hard resolver.
   It rejects unknown ids, invalid id components such as remote URL-like strings,
   and type mismatches.
2. `ProjectAssetManifest::check_scene_reference_integrity(scene, base_dir)`
   produces an `asset-reference-integrity-v1` report for evidence/read-model
   surfaces. It records warnings for:
   - `missing_asset_ref` — the scene points at an unknown manifest id;
   - `invalid_asset_id` — the scene reference is not a bounded asset id, including
     `http://` or `https://` URL-like values;
   - `invalid_asset_type` — the id exists but is the wrong asset type for the
     scene field;
   - `missing_asset_file` — the referenced manifest entry points at a missing
     project-local file;
   - `stale_asset_hash` — the referenced file exists but its observed
     `fnv1a64-file-v1` hash does not match the manifest `contentHash`;
   - `asset_path_unresolved` / `asset_path_outside_project` /
     `asset_hash_unreadable` — filesystem integrity failures encountered while
     checking a referenced local asset.

The warning path does not fetch, download, upload, or trust remote assets. A
remote URL-like scene reference remains an invalid id warning/error, not a source
to probe.

## Evidence and dashboard contract

Asset reference integrity reports may be indexed in run evidence with metadata:

```json
{
  "artifact": "asset_reference_integrity",
  "boundary": "local Rust validation only; no remote asset fetches"
}
```

The dashboard read model aggregates indexed `asset_reference_integrity` artifacts
under `asset_integrity` with warning counts, evidence refs, parsed warning rows,
and the original artifacts. The static evidence dashboard renders this as a
read-only panel and escapes all untrusted warning text.

`update_journal` adds an `Asset Reference Integrity` section when such evidence
is indexed. Legacy runs without this artifact render an explicit empty state.

## Compatibility audit

- Existing scene JSON remains backward-compatible. Scenes without project asset
  manifest evidence or without indexed integrity reports still load through the
  existing dashboard/journal paths.
- Existing run directories remain readable. Missing `asset_integrity` data is an
  empty read-model state, not a failure.
- Existing dashboard exports remain compatible with the browser UI. The UI checks
  both snake_case and camelCase field names for the new read model and keeps
  legacy data display-only.
- Existing runtime asset manifests embedded in scenes are not replaced by this
  project asset manifest integrity contract. Runtime fetch behavior for scene
  loading remains separate and must not be expanded into trusted remote asset
  fetching.
- Generated `runs/`, dashboard exports, temporary project output, local tool
  state, and target output remain generated local state and must stay untracked.

## Non-goals and guardrails

This feature does not implement:

- remote asset fetching, remote hosting, CDN behavior, cloud storage, accounts,
  or a marketplace;
- browser trusted writes, uploads, command execution, local-server bridges, or
  editor persistence;
- native export, packaging, asset bundle export, compression/transcoding, or a
  production visual asset editor;
- source-code mutation, arbitrary patch application, dependency mutation, or a
  plugin loader.

#1 and #23 remain long-running open issues and must not be closed by this work.

## Verification checklist

For changes to this contract, run at minimum:

```bash
gh issue view 336 --json state
gh issue view 1 --json state
gh issue view 23 --json state
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
node --check examples/evidence-dashboard/dashboard.js
node examples/evidence-dashboard/dashboard.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
git diff --check
```
