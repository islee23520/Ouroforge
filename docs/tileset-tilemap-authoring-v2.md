# Tileset and Tilemap Authoring v2

Tileset and Tilemap Authoring v2 is a bounded, local-first contract for small 2D
level data. It connects manifest-declared tilesets/tilemaps to runtime evidence
and dashboard read models without adding a visual editor, browser write authority,
asset marketplace, native export, or source mutation path.

## Source contract

Authoring data lives in `asset-manifest.json` entries:

- `type: "tileset"` entries carry a `tileset` payload with `tileWidth`,
  `tileHeight`, and stable tile ids.
- tiles may set `solid`, `trigger`, `hazard`, and `goal` metadata.
- `type: "tilemap"` entries carry a `tilemap` payload with `tilesetAssetId`,
  bounded `width`/`height`, and ordered layers.
- tilemap layers are `visual`, `collision`, or `trigger`; every populated cell
  must reference an existing tile id in the referenced tileset.

Rust validation rejects duplicate tile ids/indexes, duplicate layer ids, unknown
tileset refs, non-tileset refs, bad layer lengths, and unknown tile ids before
runtime/read-model code consumes the data.

## Runtime evidence

The browser runtime remains evidence-first and local-only:

- `getWorldState().tilemaps.tilemaps[*].authoring` exposes deterministic
  `collisionCells`, `triggerCells`, `hazardCells`, and `goalCells` derived from
  normalized tilemaps.
- solid tiles participate in AABB collision as synthetic read-only tilemap
  bodies; trigger tiles participate as synthetic trigger bodies.
- trigger tiles with a declared `trigger` flag initialize that flag to `false`
  and set it to `true` when overlapped, making `world_flags` and
  `runtime_events` scenario assertions deterministic.
- collision/trigger outcomes remain ordinary runtime evidence under
  `collisions`, `collisionEvents`, and `getEvents()`.

Synthetic tilemap bodies are runtime evidence participants only. They are not
scene entities to edit, persist, export, upload, or mutate from the browser.

## Scenario assertion targets

Scenarios can assert tilemap authoring and outcomes through existing bounded
assertion targets:

- `world_state` for `tilemaps.tilemaps.*.authoring.*` counts and cell fields;
- `collision_evidence` for tilemap collision event fields such as
  `staticEntityId`;
- `world_flags` for trigger-tile flag outcomes;
- `runtime_events` for recorded trigger/collision event types.

The checked-in `seeds/engine-feature-renderer-tilemap.yaml` seed asserts the
read-only authoring cell evidence exposed by the runtime read model.

## Dashboard/read-model integration

`dashboard export` summarizes tilemap authoring evidence under
`engine_summaries.tilemaps`:

- tilemap count and deterministic layer-order count;
- aggregate collision/trigger/hazard/goal cell counts;
- per-tilemap grid, layer count, and authoring cell counts.

The static dashboard renders this section as escaped, read-only evidence. It
links no write action, command bridge, upload flow, or editor operation.

## Explicit non-goals

This contract does not add:

- a map editor, visual tile painter, or browser trusted write path;
- asset marketplace/plugins, remote hosting, CDN, or account flows;
- native export/packaging or asset bundle export;
- arbitrary scene/source mutation, patch application, or dependency mutation;
- production game-editor claims or Godot-replacement scope.
