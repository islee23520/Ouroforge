# Game Complexity Ladder v1 Demo

Issue: #1496

This fixture-scoped demo records one rung climbed for Game Complexity Ladder v1:
the **Collect-and-exit** rung is satisfied by existing loop-produced
Signal Gate / Collect-and-Exit evidence under
`examples/playable-demo-v2/collect-and-exit/`. It is a deterministic source
fixture and documentation package, not a new engine surface.

## Claim

- Rung: `game-complexity-ladder-v1.collect-and-exit`
- Rung gate status: `satisfied`
- Demo fixture: `examples/game-complexity-ladder-v1/demo/rung-demo.fixture.json`
- Canonical Signal Gate root: `examples/playable-demo-v2/collect-and-exit/`

The claim is intentionally narrow: one rung is satisfied. This does not claim
later ladder rungs, full engine parity, replacement status for Godot, production
readiness, native export, hosted operation, executable plugins, or browser-owned
trusted writes.

## Evidence Package

The demo fixture links to source-controlled evidence for the required gates:

| Gate | Status | Evidence |
| --- | --- | --- |
| Validation | `pass` | Project, Seed, and scenario fixture refs under `examples/playable-demo-v2/collect-and-exit/`. |
| Runtime / scenario behavior | `pass` | `gameplay-loop-smoke.test.cjs`, `e2e-smoke.test.cjs`, and the gameplay loop contract. |
| Review / governance | `pass` | The existing agentic-iteration failure seed, proposal, independent review decision, journal, and smoke test. |
| Generated-state / trust boundary | `pass` | Existing fixture README and contracts that keep generated runs/dashboard output out of source and browser surfaces read-only. |

Loop coverage is recorded as `satisfied` / `pass` for the Signal Gate /
collect-and-exit local authoring loop. The fixture cites the existing smoke tests
and docs that cover spawn, movement, collection, gate opening, exit completion,
restart, controlled failure, review-gated apply, rerun comparison, and generated
state audit.

## Engine Growth

There is no new engine growth for this demo. The fixture records
`engineGrowthJustification.status = "none"`, an empty capability list, and no
unjustified capabilities. It requests no renderer, physics, audio, animation,
runtime, editor, Studio, plugin runtime, network, or browser-authority expansion.

## Guardrails

The fixture is deterministic, fixture-scoped, source-controlled, and offline. It
requires no network access and no live browser to inspect the rung gate state.
It does not recreate superseded trees and does not implement or claim #1497 or #1498.

#1 and #23 remain open governance anchors. This demo does not modify, close,
replace, or narrow either issue.

## Verification

Run:

```bash
node examples/game-complexity-ladder-v1/demo/demo-smoke.test.cjs
```

The smoke test reads the fixture and this document, asserts the rung gate state,
checks all evidence refs exist, verifies loop coverage and engine-growth
linkage, and audits the deterministic/no-network/no-live-browser guardrails.
