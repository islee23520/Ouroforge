# Spatial Layout Constraint Solver v1

Issue: #630 - Spatial Layout and Placement Constraint Solver v1.

Spatial Layout Constraint Solver v1 is deterministic local validation evidence
for bounded 2D-first level layouts. It checks declared placements against a
small grid and typed constraints before later draft artifacts exist. It does
not generate scenes, does not write trusted files, does not apply drafts, and
does not use hidden AI judgment to decide whether a layout is good. In short:
it does not generate or write trusted layout changes.

## Contract

The `spatial-layout-constraint-solver-v1` artifact includes:

- stable solver and generation-plan ids;
- target scene and optional tilemap refs;
- bounded grid dimensions and tile size;
- placement rectangles for spawn, goal, hazard, platform, door, key,
  checkpoint, solid, and reward objects;
- typed constraints for bounds, no overlap, spawn safe zone, goal reachable
  zone, hazard spacing, platform spacing, door/key relation, checkpoint
  spacing, and unsupported cases;
- deterministic constraint results for satisfied, violated, unsupported, and
  skipped checks;
- expected evidence paths under
  `evidence/spatial-layout-constraints/<solver-id>/`;
- validated, violated, unsupported, or blocked status with blocked reasons when
  blocked.

## Validation Boundary

Rust validation owns trusted interpretation. Validation rejects unsafe target
refs, unbounded grids, out-of-bounds rectangles, duplicate placements,
unsupported constraints without reasons, malformed evidence paths, result/status
drift, missing referenced placements, and blocked artifacts without reasons.

The artifact remains advisory and untrusted. It is separate from intent,
generation plans, drafts, reachability evidence, visual or semantic diffs,
review decisions, and apply records. Later issues must add their own validation
before any draft or apply behavior exists.

## Read Model

The read model summarizes solver id, plan id, status, grid size, placement
count, constraint count, satisfied/violated/unsupported/skipped counts, expected
evidence refs, blocked reasons, and a boundary string. It is read-only display
data and carries no scene generation, trusted write, browser command bridge,
auto-apply, auto-merge, or subjective quality guarantee authority.
