### Era B: Godot-Class Engine Core

#### Milestone 8: Production-Grade 2D Engine Core

Goal: make Ouroforge capable of building small but real 2D games, not only fixtures.

Target deliverables:

- robust 2D renderer with layers, cameras, parallax, sprites, tilemaps, UI/HUD, and debug overlays;
- collision and physics beyond smoke fixtures;
- animation, particles/VFX v1, audio bus/effects v1, and input abstraction;
- save/load runtime state and deterministic replay compatibility;
- runtime debugging/profiling evidence;
- scenario coverage for gameplay, visual, performance, input, audio, and state regressions.

Success criteria:

- A small vertical-slice 2D game can be authored, run, tested, and iterated locally.
- Agents can generate or modify scenes, assets, gameplay parameters, and scenarios through trusted APIs.
- Runtime evidence covers gameplay state, visual output, performance, input, audio/event state, and regression outcomes.

#### Milestone 9: 3D Engine Capability Gate

Goal: establish a bounded 3D capability path without prematurely claiming broad production parity.

Target deliverables:

- 3D scene graph, transform hierarchy, camera, lights, materials, and mesh import;
- 3D collision/physics v1;
- 3D animation playback v1;
- 3D runtime probe and scenario assertions;
- 3D performance and screenshot evidence;
- a small 3D demo scene generated and verified through the evidence loop.

Success criteria:

- A small 3D scene can be created, modified, run, observed, and evaluated by agents.
- 3D remains a capability gate until explicit compatibility/performance milestones justify stronger claims.

#### Milestone 10: Script and Gameplay Logic System

Goal: move from data-only scene changes to safe gameplay behavior authoring.

Target deliverables:

- gameplay scripting or structured behavior system;
- event/signal system, state machines, ability/action model, and deterministic update hooks;
- generated tests and scenario assertions for gameplay logic;
- safe script/code generation boundary;
- review-gated script mutation and rollback evidence.

Success criteria:

- Agents can add behaviors such as dash abilities, enemy patrol AI, quest triggers, dialogue state, or win/loss rules.
- Generated behavior is connected to runtime probes, evaluator verdicts, and regression evidence.
- Unsafe or source-affecting changes remain gated by review, sandboxing, and rollback policy.

#### Milestone 11: Agentic Scene and Level Designer

Goal: make level design an evidence-backed artifact, not only a manual editor action.

Target deliverables:

- level intent schema and playable-space constraints;
- tile/terrain/object placement generation;
- reachability, pathing, collision, pacing, and difficulty checks;
- scenario generation from level goals;
- visual diff and before/after evidence;
- human-editable and agent-editable scene state with conflict tracking.

Success criteria:

- Agents can create multi-room or multi-scene levels from design intent.
- Reachability, objective completion, collision safety, and pacing constraints are automatically evaluated.
- Human edits can be reabsorbed into the same evidence-backed loop.
