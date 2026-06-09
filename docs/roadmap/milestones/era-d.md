### Era D: Real Game Shipping

#### Milestone 16: Build, Export, and Packaging

Goal: support reproducible game builds and export artifacts.

Target deliverables:

- web export and native desktop export paths;
- build profiles and platform compatibility matrix;
- asset packaging and build manifests;
- version metadata;
- smoke run evidence for built artifacts;
- signing/notarization design gate where relevant;
- release artifact evidence.

Example target commands:

```bash
ouroforge build --project games/tiny-rpg --target web
ouroforge build --project games/tiny-rpg --target macos
```

Success criteria:

- Builds produce artifacts, manifests, smoke evidence, and platform verdicts.
- Release candidates remain governed by explicit readiness and rollback criteria.

#### Milestone 17: Plugin and Extension System

Goal: enable an ecosystem without weakening the trust boundary.

Target deliverables:

- plugin/extension design gate;
- permission manifest;
- editor extension API;
- runtime extension API;
- host API separation;
- dependency review;
- plugin tests, evidence, and compatibility metadata.

Success criteria:

- Plugins can extend editor/dashboard/runtime capabilities within declared permissions.
- Marketplace or public distribution remains separate until explicitly authorized.
- Plugin behavior is testable, reviewable, and evidence-linked.

#### Milestone 18: Full Studio Editor

Goal: graduate from static cockpit/read-only surfaces to a full human-and-agent Studio.

Target deliverables:

- scene tree, inspector, asset browser, timeline/animation editor, tilemap editor, shader/material editor, and script integration;
- profiler/debugger and live runtime preview;
- evidence viewer, agent task panel, mutation review panel, and build/export panel;
- trusted write APIs owned by Rust/local validation, not browser command bridges;
- collaboration and conflict-resolution surfaces for human/agent edits.

Success criteria:

- A human can build a small game through Studio and CLI.
- Agents can manipulate the same project through structured APIs.
- All trusted writes remain validated, reviewable, rollbackable, and evidence-linked.

#### Milestone 19: Godot-Plus Demonstration Game

Goal: prove the long-term claim through a complete canonical game, not a roadmap assertion.

Target deliverables:

- a small but complete game with menu, gameplay loop, multiple scenes or levels, animation, audio, UI, save/load, and exported build;
- documented agent/human co-authoring journal;
- QA swarm evidence;
- mutation/review/regression history;
- fresh-clone reproduction path.

Example target commands:

```bash
ouroforge game build demos/agentic-platformer
ouroforge game verify demos/agentic-platformer --qa-swarm 8
ouroforge game export demos/agentic-platformer --target web
```

Success criteria:

- The demo game is playable from a fresh clone.
- The game can be built, verified, and exported with evidence.
- Agent-authored changes are traceable from intent to implementation, test, verdict, review, and promotion.
- The demo supports a defensible claim that Ouroforge enables cleaner agentic game production than editor-only workflows for the scoped game class.
