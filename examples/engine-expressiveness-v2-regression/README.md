# Engine Expressiveness v2 Regression Suite

This fixture is the Scenario Coverage v3 source suite for issue #320. It keeps
feature-specific contracts separate from the integrated playable demo so failures
are attributable by feature area.

Tracked source inputs:

- `ouroforge.project.json` wires the source scene, Seed, scenario pack, asset
  root, and generated-state roots.
- `scenes/expressiveness-v2.scene.json` is a deterministic local scene fixture.
- `seeds/engine-expressiveness-v2-regression.yaml` contains the eight focused
  Scenario DSL checks.
- `scenarios/expressiveness-v2-regression.json` groups the same checks as a
  project-level scenario pack.

Covered areas: component schema, trigger/flags, multi-scene transition evidence
contract, HUD values, collision layers, animation event evidence, audio event
intent, and the playable collect-and-exit loop.

Generated `runs/`, `dashboard-data/`, `target/`, screenshots, and temporary
smoke output are local review evidence and must not be committed.
