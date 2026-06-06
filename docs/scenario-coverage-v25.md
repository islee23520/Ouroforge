# Scenario Coverage v25

Scenario Coverage v25 records fixture-only regression coverage for
Game Complexity Ladder v1. It is tied to #1497 and covers declared state shapes
for rung gates, engine-growth justification, backward compatibility, generated
state boundaries, and governance checks.

This page documents source-controlled fixtures and a local Node smoke runner. It
does not add runtime behavior, browser authority, network access, engine
capability, source mutation, or roadmap changes.

## Matrix

The matrix lives at
`examples/game-complexity-ladder-v1/scenario-coverage-v25/matrix.json`.

It declares:

- schema `scenario-coverage-v25-complexity-ladder-v1`;
- issue `1497`;
- fixture-scoped coverage only;
- `generatedState: false`;
- the ladder contract reference `docs/game-complexity-ladder-v1.md`;
- allowed rung gate states: `satisfied`, `unsatisfied`,
  `insufficient-evidence`, and `out-of-order`;
- allowed engine-growth justification states: `justified`, `unjustified`, and
  `missing-prerequisite`;
- guardrails that keep network, browser execution, browser trusted writes,
  command bridge, auto-fix, auto-apply, auto-merge, broad engine claims, and
  #1/#23 modification out of scope.

The runner
`examples/game-complexity-ladder-v1/scenario-coverage-v25-complexity-ladder.test.cjs`
reads the matrix and every JSON fixture under
`examples/game-complexity-ladder-v1/scenario-coverage-v25/`.

## Fixtures

The rung gate fixtures record the four gate states required by the ladder
contract:

- `rung-gate-states/collect-and-exit-satisfied.json` records the first rung as
  satisfied by existing source-controlled collect-and-exit evidence.
- `rung-gate-states/platformer-unsatisfied.json` records a later rung as
  unsatisfied when required platformer observations are missing.
- `rung-gate-states/top-down-insufficient-evidence.json` records a missing
  evidence state for the top-down objective rung.
- `rung-gate-states/multi-scene-out-of-order.json` records dependency ordering
  when earlier rung gates are not all satisfied.

The engine-growth fixtures record whether a requested capability is tied to a
specific rung gate and evidence gap:

- `engine-growth/jump-collision-justified.json` records an allowed demand shape
  for bounded jump-relevant collision evidence tied to the platformer rung.
- `engine-growth/broad-renderer-unjustified.json` records an unjustified request
  without a specific rung gate or evidence gap.
- `engine-growth/multi-scene-transition-missing-prerequisite.json` records a
  request blocked by prior rung state.

Each fixture is source-controlled fixture data. The fixtures do not authorize
implementation work beyond their declared regression shape.

## Backward Compatibility

The backward compatibility fixture is
`backward-compatibility/signal-gate-collect-and-exit-source-compatibility.json`.
It asserts that existing source refs remain readable, including:

- `docs/game-complexity-ladder-v1.md`;
- `docs/playable-demo-v2-collect-and-exit.md`;
- `examples/playable-demo-v2/collect-and-exit/ouroforge.project.json`;
- `examples/playable-demo-v2/collect-and-exit/scenes/collect-and-exit.scene.json`;
- `examples/playable-demo-v2/collect-and-exit/scenarios/collect-and-exit.json`;
- `examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs`;
- `examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs`.

The compatibility check is a source-reference check. It does not rerun remote
state, fetch issue branches, or modify the existing demo.

## Generated-State Boundary

Scenario Coverage v25 treats all listed JSON files as fixtures. The matrix and
fixtures use `fixtureScoped: true` and `generatedState: false`.

The runner verifies those fields and scans the matrix, fixtures, and this
document for wording that would enable auto-apply, auto-merge, browser trusted
writes, network execution, or live browser execution. Generated run outputs,
dashboard exports, tool state, and build outputs remain outside the trusted
source state unless a future scoped issue adds explicit source fixture coverage.

## Governance Audit

Scenario Coverage v25 preserves the Game Complexity Ladder v1 ordering rule:
later rungs are not claimable until prior rung gates have sufficient evidence.
Engine-growth requests must cite a specific rung gate and a bounded evidence
gap.

#1 and #23 remain unchanged. This coverage does not close, replace, narrow, or
modify either anchor, and it does not implement #1498.
