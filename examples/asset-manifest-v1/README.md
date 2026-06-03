# Asset Manifest v1 fixtures

These JSON files are tiny deterministic schema fixtures for Asset Pipeline v1.
They are source-like contract fixtures only; the asset paths are not runtime
loading evidence. They also anchor the documented Asset Reference Integrity v1
contract for local ids, stale hashes, invalid types, and missing files without
implementing preview generation, Studio behavior, browser uploads/writes, remote
fetches, or asset packaging.

- `asset-manifest.valid.json` exercises the v1 manifest shape for local source-like
  assets and generated preview records.
- `invalid/` contains schema-level rejection fixtures used by core serde tests.

Generated asset previews, runs, dashboard data, and local tool state remain
untracked unless a later issue explicitly scopes a deterministic fixture.
