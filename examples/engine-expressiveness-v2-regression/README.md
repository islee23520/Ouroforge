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


## Evidence smoke

Run the source/evidence compatibility smoke without committing generated state:

```bash
node examples/engine-expressiveness-v2-regression/evidence-smoke.test.cjs
```

The smoke drives the local runtime fixture to key collection and exit, evaluates
each scenario-pack assertion against in-memory evidence, writes a temporary
verdict file under the OS temp directory, then deletes it. Evidence references in
that verdict are relative `evidence/...` paths so dashboard/read-model exporters
can link them without browser-side trusted writes or command execution.


## Coverage matrix and dashboard audit

Canonical coverage matrix: `docs/scenario-coverage-v3.md`.

The scenario pack and smoke verdict shape intentionally use bounded evidence
keys (`world_state`, `frame_stats`, `runtime_events`, `transition_evidence`,
`collision_evidence`, `animation_evidence`, `audio_evidence`, and `comparison`)
so dashboard and Studio read-model surfaces can link exported evidence without
computing browser-side comparisons, running commands, writing trusted files, or
mutating source. See `docs/scene-transitions-v1.md` for the transition schema,
runtime probe fields, assertion target, and explicit non-goals.
