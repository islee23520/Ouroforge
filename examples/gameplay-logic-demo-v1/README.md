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
- `read-model-compatibility.md` documents commands, expected evidence, known
  gaps, cleanup policy, and dashboard/Studio read-model compatibility.

Generated `runs/`, `dashboard-data/`, screenshots, browser profiles, temp
projects, and local tool output are local review evidence and must not be
committed. This fixture does not add script execution, `eval`, dynamic import,
plugin loading, command bridges, local server bridges, browser trusted writes,
source mutation, auto-apply, auto-merge, self-approval, hosted/cloud behavior,
native export, a production-stable scripting API, production engine maturity, or
a Godot replacement claim.

## Expected evidence

The demo's tracked expected evidence is fixture-scoped:

- `evidence/runtime-events.fixture.json` records key, door, patrol hazard, and
  victory events.
- `evidence/scenario-outcome.fixture.json` records the three passing scenario
  outcomes.
- `evidence/behavior-evidence-bundle.fixture.json` links behavior definitions,
  runtime events, scenario outcomes, draft/review/apply metadata, rollback
  metadata, rerun comparison evidence, and the next-step hypothesis.
- `journal/behavior-evidence-journal.fragment.md` is the journal fragment that
  dashboard/Studio read models can display without inferring hidden state.

## Read-model compatibility

`read-model-compatibility.fixture.json` records the dashboard sections and Studio
panels expected to inspect the demo read-only. The compatibility smoke test
checks linked refs, display-only fields, empty/malformed policy wording, and the
no-script/no-command/no-write boundary.

## Commands

```bash
node examples/gameplay-logic-demo-v1/schema-smoke.test.cjs
node examples/gameplay-logic-demo-v1/evidence-flow-smoke.test.cjs
node examples/gameplay-logic-demo-v1/read-model-compatibility-smoke.test.cjs
node examples/evidence-dashboard/dashboard.test.cjs
node examples/authoring-cockpit/cockpit.test.cjs
```

## Known gaps

- The evidence files are expected fixture data, not proof of a live browser run.
- The demo does not add browser commands, write APIs, arbitrary scripts, hosted
  services, native export, production editor maturity, or Godot replacement
  claims.
- Later issues may add richer dashboard/Studio rendering after preserving this
  read-only compatibility contract.

## Cleanup policy

Tracked files stay under `examples/gameplay-logic-demo-v1/`. Local generated
`runs/`, `dashboard-data/`, screenshots, browser profiles, and `tmp-evidence/`
stay ignored by this fixture's `.gitignore`; delete those ignored roots after
manual review if created, and do not commit them unless a later issue explicitly
scopes tiny fixtures.

## Smoke test

```bash
node examples/gameplay-logic-demo-v1/schema-smoke.test.cjs
node examples/gameplay-logic-demo-v1/evidence-flow-smoke.test.cjs
node examples/gameplay-logic-demo-v1/read-model-compatibility-smoke.test.cjs
```

The smoke tests validate fixture links, scenario coverage of the required demo
beats, evidence flow, dashboard/Studio read-model compatibility, generated-state
hygiene, and no-arbitrary-script wording.
