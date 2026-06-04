# Entity Objective Encounter Placement Draft v1

Issue: #632 - Entity, Objective, and Encounter Placement Draft v1.

Entity Objective Encounter Placement Draft v1 represents agent-proposed
placements as reviewable draft evidence. It can describe spawns, goals,
pickups, hazards, enemies, NPCs, doors, keys, checkpoints, camera anchors, and
linked behavior refs. It does not write scene files, apply placements, execute
behavior scripts, or make browser or Studio output authoritative.

## Contract

The `entity-objective-encounter-placement-draft-v1` artifact includes:

- stable draft, intent, plan, solver, and tilemap draft ids;
- target scene refs and bounded placement grid size;
- placement rectangles with typed placement kind, optional entity refs, asset
  refs, behavior refs, objective refs, and encounter group ids;
- objective records with required placement refs;
- before hash for stale-target detection;
- expected after summary for preview display;
- expected evidence paths under
  `evidence/entity-objective-placement-drafts/<draft-id>/`;
- drafted, stale, unsupported, or blocked status with blocked reasons when
  stale or blocked.

## Validation Boundary

Rust validation owns trusted interpretation. Validation rejects unsafe target
refs, oversized grids, out-of-bounds rectangles, duplicate placement and
objective ids, overlapping non-camera placements, malformed asset and behavior
refs, missing objective placements, malformed before hashes, malformed evidence
paths, unsupported placements in drafted status, unsupported status without an
unsupported placement, and blocked or stale status without reasons.

The artifact remains advisory and untrusted. It is separate from intent,
generation plans, tilemap drafts, reachability evidence, visual or semantic
diffs, review decisions, and apply records. Later issues must add their own
validation before any apply behavior exists.

## Read Model

The read model summarizes draft id, status, target scene, placement count,
objective count, encounter group count, behavior link count, expected evidence
refs, blocked reasons, and a boundary string. It is read-only display data and
carries no scene write, trusted apply, browser command bridge, auto-apply,
auto-merge, behavior execution, or subjective quality guarantee authority.
