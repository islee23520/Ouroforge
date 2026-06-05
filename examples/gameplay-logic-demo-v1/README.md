# Gameplay Logic Demo v1

Fixture-scoped source inputs for #623 GL10.13.1.

This demo proves that Ouroforge can describe a tiny deterministic gameplay loop
with structured behavior data, not arbitrary executable scripting. The fixture
links existing Milestone 10 behavior model, event/signal, state-machine,
ability/action, behavior draft, behavior apply, and behavior evidence contracts
into one local demo package.

Tracked source inputs:

- `ouroforge.project.json` wires the source scene, Seed, scenario pack, asset
  root, and generated-state roots.
- `scenes/gameplay-logic-demo.scene.json` contains the tiny deterministic scene:
  player, blue key, blue door, spike hazard, guard patrol marker, dash pickup,
  exit, and HUD row.
- `scenarios/gameplay-logic-demo.scenario-pack.json` describes focused scenario
  evidence for item collection, door/flag logic, hazard/enemy behavior,
  ability/action metadata, and win condition.
- `behaviors/gameplay-logic-demo.behavior.json` is the structured behavior
  artifact executed by Rust tests for key, door, dash, patrol, and win evidence.
- `seeds/gameplay-logic-demo.yaml` mirrors the scenario intent for CLI/runtime
  workflows.
- `behavior-inspection-demo.json` links the structured behavior/event/state/
  ability/draft/apply/evidence fixtures that Studio and dashboard surfaces can
  inspect read-only.

Generated `runs/`, `dashboard-data/`, screenshots, browser profiles, temp
projects, and local tool output are local review evidence and must not be
committed. This fixture does not add script execution, `eval`, dynamic import,
plugin loading, command bridges, local server bridges, browser trusted writes,
source mutation, auto-apply, auto-merge, self-approval, hosted/cloud behavior,
native export, a production-stable scripting API, production engine maturity, or
a Godot replacement claim.

## Smoke test

```bash
node examples/gameplay-logic-demo-v1/schema-smoke.test.cjs
```

The smoke test validates fixture links, scenario coverage of the required demo
beats, generated-state hygiene, and no-arbitrary-script wording.
