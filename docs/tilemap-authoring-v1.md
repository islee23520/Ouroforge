# Tilemap / Level Editor v1 authoring spec

Issue: #2369. Scenario Coverage owner: #2371 / v104.

## Source format and path convention

Dogfood tilemaps use `ouroforge.tilemap-source.v1` JSON fixtures under
`examples/tilemap-authoring-v1/maps/<map-id>.tilemap.json`. M128 content and
M123 drafting share this exact convention.

A source tilemap records `mapId`, `sourcePath`, `tilesetRef`, dimensions,
`tileSize`, a palette, ordered layers, and spawn/trigger/objective markers.
Empty cells are represented as `"."`. Browser/Studio surfaces may display this
source and create drafts, but they do not write it directly.

## Draft/apply boundary

Tile painting uses `ouroforge.tilemap-draft.v1`. Drafts target a repo-relative
source path and the source `baseDigest` (`fnv64:<hex>`), then list bounded
paint/erase operations. Validation rejects stale base digests, unknown tiles,
unknown layers, and out-of-bounds cells before preview. The preview is generated
state only and is never treated as trusted source.

## Reachability and objective evidence

Reachability uses named Rust diagnostics from `TilemapReachabilityDiagnostic`.
Blocked maps fail closed with diagnostics such as `objectiveUnreachable`.
Valid maps require live replay evidence (`ouroforge.tilemap-live-replay.v1`) that
reaches the objective before a product-observed claim is made.

## Non-goals and guardrails

This is a bounded 2D tilemap authoring contract. It is not a production editor,
Godot replacement, arbitrary scripting system, browser trusted-write path,
command bridge, auto-apply path, auto-merge path, publish/deploy/sign/upload
path, or release-readiness claim. #1 and #23 remain open governance anchors.
