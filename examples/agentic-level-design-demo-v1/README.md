# Agentic Level Design Demo v1

This source fixture composes the Agentic Scene and Level Designer v1 artifact
chain for #640. It demonstrates a deterministic collect-and-exit level design
loop from intent to plan, draft, evidence, review/apply metadata, rerun commands,
and Studio inspection.

The demo is intentionally fixture-scoped. It does not generate a new level
autonomously, mutate project files, execute commands from the browser, or claim a
production editor.

## Demo Chain

`demo-chain.fixture.json` links:

- level intent;
- scene generation plan;
- spatial layout constraint evidence;
- tilemap terrain draft;
- entity/objective/encounter placement draft;
- reachability and pathing evidence;
- objective completion proof;
- difficulty and pacing heuristic evidence;
- level visual and semantic diff;
- agent-generated level draft;
- review-gated level apply metadata;
- Studio level design inspection read model inputs.

## Verification

Run the focused smoke:

```bash
node examples/agentic-level-design-demo-v1/demo-smoke.test.cjs
```

Useful supporting checks:

```bash
cargo test agent_generated_level_draft_v1
cargo test review_gated_level_apply_v1
node examples/playable-demo-v2/collect-and-exit/e2e-smoke.test.cjs
node examples/playable-demo-v2/collect-and-exit/evidence-read-model-smoke.test.cjs
node --check examples/authoring-cockpit/cockpit.js
node examples/authoring-cockpit/cockpit.test.cjs
```

## Generated State

Generated runs, dashboard exports, screenshots, previews, and temporary project
copies are intentionally not tracked. The smoke asserts that these fixture-local
generated roots are absent:

- `examples/agentic-level-design-demo-v1/runs`;
- `examples/agentic-level-design-demo-v1/dashboard-data`;
- `examples/agentic-level-design-demo-v1/screenshots`;
- `examples/agentic-level-design-demo-v1/tmp`.

## Boundary

This demo preserves the milestone boundary:

- no autonomous full game generation;
- no browser trusted writes;
- no command bridge or local server bridge;
- no hidden command execution;
- no auto-apply or auto-merge;
- no self-approval;
- no production editor or full visual level editor;
- no visual scripting;
- no native export, hosted/cloud service, plugin runtime, marketplace, account
  system, production-ready engine, or Godot replacement claim.

Difficulty and pacing evidence is advisory only; it does not prove subjective
game quality.
