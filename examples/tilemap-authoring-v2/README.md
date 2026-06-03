# Tileset and Tilemap Authoring v2 fixtures

These JSON files are tiny deterministic schema fixtures for issue #335 / Tileset
and Tilemap Authoring v2. They exercise `asset-manifest-v1` `tileset` and
`tilemap` payloads for tile properties and tilemap layers.

Current support is documented in `docs/tileset-tilemap-authoring-v2.md`: Rust
validates the manifest schema and references, the runtime exposes read-only
collision/trigger/hazard/goal cell evidence, and dashboard export summarizes the
same evidence for inspection.

The fixtures remain source-like local contracts. They do not authorize visual
editor behavior, browser writes/uploads, generated previews as source of truth,
remote fetches, plugins, marketplaces, native export, or arbitrary source
mutation.
