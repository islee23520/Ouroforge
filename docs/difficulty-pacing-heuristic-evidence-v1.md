# Difficulty Pacing Heuristic Evidence v1

Issue: #635 - Difficulty, Pacing, and Balance Heuristic Evidence v1.

Difficulty Pacing Heuristic Evidence v1 records transparent local metrics for
level difficulty, pacing, and balance review. It compares metrics such as route
length, hazard density, pickup spacing, enemy spacing, checkpoint spacing, route
complexity, timing budget, objective density, and safe-zone coverage against
declared target ranges.

These heuristics are evidence inputs, not game-quality truth. They do not prove
fun, quality, market readiness, production balance, or subjective player
experience. They add no scene write capability and do not apply drafts.

## Artifact Shape

The `difficulty-pacing-heuristic-evidence-v1` artifact includes:

- stable evidence, intent, plan, placement, scene, reachability, and objective
  proof refs;
- bounded metric rows with target min/max and optional computed value;
- warnings for below target, above target, missing input, malformed input,
  unsupported metrics, and contradictory targets;
- expected evidence under `evidence/difficulty-pacing/<evidence-id>/`;
- status, blocked reasons, and guardrails.

Validation recomputes warnings from metric targets and values, then rejects
warning drift. Unsupported metrics must include a reason. Missing values remain
visible as missing evidence, and negative values remain visible as malformed
input warnings.

## Read Model

The read model reports metric counts, warning counts, missing/unsupported/
malformed counts, linked evidence refs, and blocked reasons.

The boundary is read-only difficulty, pacing, and balance heuristic evidence:
transparent local metrics only, not a fun score, not a quality guarantee, no
scene write, no trusted apply, no browser command bridge, no auto-apply, and no
auto-merge.

## Non-Goals

- No autonomous full game generation.
- No ML or AI quality scorer.
- No subjective fun, quality, balance, market-readiness, or production-readiness
  guarantee.
- No browser trusted writes, command bridge, local server bridge, auto-apply, or
  auto-merge.
- No production editor, native export, hosted/cloud behavior, plugin runtime, or
  Godot replacement claim.
