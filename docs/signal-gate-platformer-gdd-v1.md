# Signal Gate Platformer GDD v1

`signal-gate-platformer` is a one-screen platformer fixture for Milestone 21. The player crosses a short obstacle lane, waits for a signal gate to open, and reaches an exit marker.

## Objective

Reach the exit after the signal gate opens while preserving deterministic evidence for movement, hazard avoidance, signal timing, and exit completion.

## Constraints

- One screen only.
- One controllable player.
- One jumpable hazard lane.
- One timed signal gate.
- One exit trigger.
- No procedural content.
- No external network or browser dependency for committed evidence.
- No per-game evaluator bypass.

## Acceptance

- Mechanical: replay input reaches the exit only after the signal opens.
- Runtime: fixture frame budget status is within budget and deterministic event order is preserved.
- Visual: scene evidence includes player, hazard lane, signal gate, and exit marker.
- Semantic: flags show `signal_open`, `hazard_cleared`, and `exit_reached` in the accepted path.

## Scenarios

`signal-gate-source-smoke` validates the source fixture and deterministic acceptance path. The scenario pack and Seed keep the same assertion DSL used by collect-and-exit so Rust/local validation can parse both classes through the same contracts.

## Evidence Boundary

The demo evidence is fixture-scoped and deterministic. It is suitable for regression validation of authoring-loop contract shape, not for a production-readiness claim.
