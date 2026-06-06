# Scenario Coverage v21: Loop Coverage Metric

Issue context: #1464 and the Loop Coverage Metric v1 chain.

Scenario Coverage v21 covers the fixture-scoped loop coverage states needed for Milestone 20:

- `computed`: supported inputs compare against a baseline without exceeding the drop threshold.
- `regressed`: supported inputs compare against a baseline and the loop-covered fraction drops past the threshold.
- `insufficient-data`: supported inputs lack a usable baseline or have stale/missing attribution evidence.
- `unsupported`: the artifact kind is outside the loop coverage contract.

The coverage is deterministic and offline. It reads committed JSON fixtures and verifies that the Rust contract, demo smoke test, dashboard summary, and Studio cockpit inspection all preserve the same descriptive verdict states.

## Boundary

Loop coverage is descriptive authorship attribution only. It is not a quality guarantee, no production-ready claim is made, and no Godot replacement claim is made. A regressed verdict blocks a loop-coverage claim only; it is not merge authority, release authority, or a game-quality judgment.

Dashboard and Studio surfaces are read-only. They may render the metric but must not write trusted files, mutate source, apply changes, use no auto-apply path, use no auto-merge path, execute commands, run solvers, or provide mutation affordances. The metric is not a browser trusted-write path and not a Studio apply path.

#1 and #23 remain open. Coverage v21 is an additive regression suite and does not close or rewrite those anchors.
