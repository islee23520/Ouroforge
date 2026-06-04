# Reachability Pathing Evidence v1

Issue: #633 - Reachability and Pathing Evidence v1.

Reachability Pathing Evidence v1 proves or visibly fails bounded local path
queries for generated level drafts. It links the intent, plan, tilemap draft,
and placement draft, then evaluates explicit grid queries. It does not infer unsupported movement,
does not prove subjective level quality, does not write scene files, and does
not apply drafts.

## Contract

The `reachability-pathing-evidence-v1` artifact includes:

- stable evidence, intent, plan, tilemap draft, and placement draft ids;
- target scene ref and bounded grid dimensions;
- explicit cell states for walkable, blocked, hazard, door, key, checkpoint,
  and objective cells;
- path queries for start-to-goal, required pickup, key-to-door, and
  checkpoint-to-objective checks;
- scoped movement kind, with unsupported movement requiring a reason;
- deterministic query results for reachable, unreachable, unsupported, and
  blocked outcomes;
- blockers for unreachable paths, including missing edges, blocked cells,
  hazardous cells, missing objectives, and unknown state;
- expected evidence paths under `evidence/reachability-pathing/<evidence-id>/`;
- validated, failed, stale, unsupported, or blocked status with blocked reasons
  when stale or blocked.

## Validation Boundary

Rust validation owns trusted interpretation. Validation rejects unsafe target
refs, oversized grids, duplicate or out-of-bounds cells, malformed movement
records, malformed evidence paths, result drift from deterministic local graph
analysis, validated status with failed or unsupported results, unsupported
status without unsupported results, and stale or blocked status without reasons.

Walking movement is evaluated with a bounded four-way graph search. Jump,
action, and custom movement are not guessed; they must be marked unsupported
until a later issue scopes those mechanics explicitly.

The artifact remains advisory and untrusted. It is separate from intent,
generation plans, tilemap drafts, placement drafts, objective proof, visual or
semantic diffs, review decisions, and apply records.

## Read Model

The read model summarizes evidence id, status, query counts, reachable,
unreachable, unsupported, and blocked result counts, expected evidence refs,
blocked reasons, and a boundary string. It is read-only display data and carries
no scene write, trusted apply, browser command bridge, auto-apply, auto-merge,
or gameplay quality guarantee authority.
