# Sprite Atlas Manifest v1 fixtures

These JSON files are tiny deterministic schema fixtures for issue #334 / Sprite
Atlas Manifest v1. They exercise the `asset-manifest-v1` `sprite_atlas` subtype
and its `atlas` payload: image asset id, named frame rectangles, and animation
frame references.

The fixtures are source-like schema contracts only. They do not implement runtime
packing, generated atlas builds, preview generation, Studio editing, browser
uploads/writes, remote fetches, plugins, marketplaces, or native export.

- `asset-manifest.valid.json` contains one image asset and one sprite atlas asset
  with two frames and one animation.
- `invalid/` contains schema-level rejection fixtures used by core tests.

Generated runs, dashboard data, local tool state, and generated atlas output must
remain untracked.
