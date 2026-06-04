# Tilemap Terrain Generation Draft v1

Issue: #631 - Tilemap and Terrain Generation Draft v1.

Tilemap Terrain Generation Draft v1 represents proposed tilemap and terrain
edits as reviewable draft evidence. It links level intent, scene generation
plan, and spatial layout constraint solver ids. It does not write tilemap files,
apply changes, or make browser or Studio output authoritative.

## Contract

The `tilemap-terrain-generation-draft-v1` artifact includes:

- stable draft, intent, plan, and solver ids;
- target scene and tilemap refs;
- bounded grid dimensions and declared layer ids;
- tile placements with tile ids, layer ids, tileset asset refs, and grid
  coordinates;
- terrain, collision, and trigger regions;
- before hash for stale-target detection;
- expected after summary for preview display;
- linked constraint statuses from the spatial layout solver;
- expected evidence paths under `evidence/tilemap-terrain-drafts/<draft-id>/`;
- drafted, stale, unsupported, or blocked status with blocked reasons when
  stale or blocked.

## Validation Boundary

Rust validation owns trusted interpretation. Validation rejects unsafe target
refs, oversized grids, unknown layer refs, out-of-bounds placements and
regions, duplicate ids, malformed before hashes, malformed evidence paths,
drafted status with violated or unsupported linked constraints, unsupported
status without an unsupported linked constraint, and blocked or stale status
without reasons.

The artifact remains advisory and untrusted. It is separate from intent,
generation plans, spatial layout constraints, reachability evidence, visual or
semantic diffs, review decisions, and apply records. Later issues must add
their own validation before any apply behavior exists.

## Read Model

The read model summarizes draft id, linked intent/plan/solver ids, status,
target tilemap, grid size, layer and edit counts, linked constraint status
counts, expected evidence refs, blocked reasons, and a boundary string. It is
read-only display data and carries no tilemap file write, trusted apply,
browser command bridge, auto-apply, auto-merge, or subjective quality guarantee
authority.
