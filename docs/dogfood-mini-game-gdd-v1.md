# Dogfood Mini-Game GDD v1 — Signal Gate Relay

Issue: #2384
Milestone: M128.1 First Real Dogfood Game Vertical Slice v1
Status: GDD and acceptance-matrix contract only. This document defines the first real Ouroforge dogfood mini-game production target; it does not implement runtime, Studio, editor, art, behavior, scenario, or release changes.

## Governance and closure classification

- Target closure classification for #2384: `contract-complete`.
- #1 and #23 remain open governance anchors.
- This GDD is bounded local dogfood scope, not a commercial-release, production-ready, secure-sandbox, broad Godot parity, or Godot replacement claim.
- Generated run bundles, screenshots, browser profiles, package outputs, and observability artifacts stay in ignored/generated roots unless a later issue explicitly fixture-scopes a tiny source artifact.
- Fun, feel, clarity, pacing, and release/go-no-go judgment are **human-owned**. Ouroforge may record deterministic evidence and human verdict fields, but it must not automate the final fun/quality verdict.

## Negative baseline from #2351

The #2351 live audit gap ledger is a blocking input, not a soft-pass precedent. This GDD must not assume the current collect-and-exit fixture already proves practical dogfood quality.

Explicit #2351 gaps carried into this target:

| Gap id | Category | Severity | GDD response |
| --- | --- | --- | --- |
| `ce-live-2351-001` | `runtime_ux` | blocking | Require product-facing launch, controls, objective, status, and debug-panel separation before live playability can pass. |
| `ce-live-2351-002` | `renderer_quality` | major | Treat missing/placeholder visual evidence as a dependency on M120, not as acceptable dogfood art. |
| `ce-live-2351-003` | `input_control` | major | Require discoverable human controls and replay parity, not only harness API driving. |
| `ce-live-2351-004` | `qa_evaluator_depth` | major | Preserve `contract-complete` vs `product-observed complete/FAIL` classification in later evidence. |
| `ce-live-2351-005` | `dogfood_game_quality` | blocking | Define a coherent 5-10 minute mini-game with a separate human-owned fun/feel verdict. |

## Game summary

**Signal Gate Relay** is a compact 2D top-down action-puzzle mini-game intended to take a first-time player **5-10 minutes** across four short rooms. The player is a courier restoring signal power to a locked relay gate. Each room teaches one mechanic, then combines it with light timing pressure:

1. learn movement and objective reading;
2. collect a key shard and open a signal gate;
3. route through hazards with pause/restart recovery;
4. finish a relay sequence that proves win, fail, restart, replay, HUD, visual, authoring, and evidence loops can be observed.

The target is larger than the one-screen collect-and-exit fixture but intentionally small enough for local dogfood, repeatable live capture, and review-gated authoring iteration.

## Player objective and loop

Primary objective: activate three relay nodes, collect the final signal key, open the exit gate, and reach the relay exit.

Core loop per room:

1. read the HUD objective and room status;
2. move through a readable tile layout;
3. collect or activate the room objective;
4. avoid or recover from a bounded hazard/fail state;
5. observe a visible state transition on the gate/HUD;
6. continue to the next room or restart from the room checkpoint.

Win condition: `exit_reached = true` after `relay_nodes_active = 3`, `key_collected = true`, and `gate_open = true`.

Fail/blocked conditions:

- contact with an active hazard transitions to a visible loss/checkpoint state;
- attempting the exit before the key or relays are complete remains blocked and visible;
- missing assets, diagnostics, stale replay, or absent screenshots block product-observed completion even when the mechanical win path succeeds.

## Scope guard

Included:

- four short rooms/encounters in one local project;
- keyboard movement, pause/resume, restart/checkpoint, win/loss status;
- HUD objective/status/key/relay indicators;
- authored scene, tilemap, and behavior drafts through the planned Studio surfaces;
- deterministic replay and live observability evidence;
- visual readability baseline and screenshot targets;
- human-owned playtest notes for fun/feel.

Excluded:

- commercial release, store export, signing, marketplace, hosted/cloud, accounts, network multiplayer, public release automation, native/mobile/console export;
- executable plugin runtime, arbitrary scripting expansion, browser trusted writes, hidden command execution, command bridge, dependency install, auto-apply, auto-merge, self-approval, reviewer bypass;
- broad engine rewrite or unbounded editor parity;
- automated fun/quality/release verdicts.

## Mechanics

| Mechanic | Description | Capability mapping |
| --- | --- | --- |
| Movement and collision | Player moves inside bounded 2D rooms with solid walls and readable collision. | M119 runtime APIs/diagnostics for deterministic state; M123 tilemap collision is a dependency for authored room layout. |
| Key and relay collection | Pickups/activations update flags, HUD, gate state, and replay checkpoints. | M118 HUD/status, M119 replay/diagnostics; M122 component drafts for entities; M124 behavior assertion generation for state transitions is a dependency. |
| Signal gates | Gates visibly open after required flags; premature gate contact is a blocked/fail-visible state. | M118 canonical runtime states; M119 diagnostics; M123 reachability/objective validation is a dependency. |
| Hazards | Patrol or timed hazards create deterministic fail/restart pressure without arbitrary scripting. | M124 behavior model/parameter draft/scenario assertions are dependencies because #2372-#2374 are open. |
| Pause/restart/checkpoint | Player can pause, resume, restart, and recover without page reload ambiguity. | Dependency on M118.4 (#2355 open). |
| Visual readability | Actor, objective, gates, hazards, HUD, and room boundaries must be readable in screenshots. | Dependency on M120.1-M120.3 (#2359-#2361 open). |
| Authoring loop | Designer/agent can inspect, draft, review, and hand off scene/tile/behavior edits. | M121 workspace and M122 selection/inspector are available; M122 apply handoff, M123 tilemap, and M124 behavior authoring remain dependencies. |

## Rooms and encounters

| Room | Target duration | Purpose | Required states/evidence | Dependencies |
| --- | ---: | --- | --- | --- |
| 1. Relay Yard | 60-90s | Teach movement, HUD objective, relay activation, and visible status updates. | `start`, first relay active, HUD objective update, screenshot `state-start.png`. | M118.2/M118.3 closed; M120 visual rubric open. |
| 2. Key Switch Hall | 90-120s | Collect a key shard, open a local gate, and show blocked-gate behavior if skipped. | `key-collected`, `gate-open`, blocked-before-key replay. | M118/M119 closed pieces; M123 reachability validation open. |
| 3. Hazard Timing Room | 120-180s | Avoid a simple deterministic hazard and demonstrate loss/restart clarity. | `fail/blocked`, `paused`, `restarted`, hazard contact replay, checkpoint restart. | M118.4 open; M124 behavior authoring open. |
| 4. Final Signal Gate | 90-150s | Combine relays/key/hazard into final exit and win evidence. | `win/exit`, final digest, generated live bundle, human verdict form. | M120 visual capture open; M123/M124 dependencies if authored in Studio. |

Total target playtime: 5-10 minutes including one failure/restart. A perfect replay may complete faster, but the product-observed playtest script should allow normal reading and recovery time.

## Assets, audio, and HUD target

Asset direction: simple but intentional local source-like 2D art, with clear silhouettes and consistent scale. Placeholder shapes may exist during draft work but cannot satisfy product-observed visual completion unless explicitly labeled as intentional and accepted by the M120 rubric.

Required visual/audio/HUD elements:

- player courier avatar;
- relay nodes in inactive/active states;
- signal key and final gate in locked/open states;
- hazard with idle/active danger states;
- room boundaries and collision-readable tiles;
- HUD: objective, relay count `0/3..3/3`, key status, gate status, pause/restart/win/loss message;
- audio intent events for relay activation, key pickup, gate open, hazard fail, restart, and win, if supported by existing deterministic event contracts.

Dependencies:

- M120.1 visual rubric must define pass/fail readability.
- M120.2 must provide the source-like visual refresh before product-observed visual pass.
- M120.3 must capture target-state screenshots outside trusted source unless fixture-scoped.

## Save, progression, and recovery

Minimum progression model:

- single local project;
- room checkpoint after each relay activation;
- restart returns to current room checkpoint;
- full reset returns to room 1;
- replay labels capture `start`, `relay-1`, `key-collected`, `gate-open`, `fail/blocked`, `paused`, `restarted`, and `win/exit`.

The GDD does not require a long-term save system or account storage. Recovery evidence is local and deterministic.

Dependency: pause/restart/checkpoint UX relies on M118.4 (#2355 open); deterministic replay labels rely on M119.2 (#2357 closed).

## Authoring and edit goals

The dogfood target should exercise Studio authoring without requiring broad editor parity:

- M121 workspace opens the local project, shows scene/asset lists, runs or provides safe copyable run instructions, and links evidence bundles.
- M122 scene editor selects entities, edits transform/status/trigger/HUD fields as drafts, and hands off review/apply evidence.
- M123 tilemap editor paints room tiles, markers, collision, and reachability/objective diagnostics.
- M124 behavior authoring edits allowlisted hazard timing/route parameters and emits expected scenario assertions.

Current dependency status:

- Available for planning: M121.1-M121.3 (#2362-#2364 closed), M122.1-M122.3 (#2365-#2367 closed).
- Explicit dependencies: M122.4 (#2368 open), M123.1-M123.3 (#2369-#2371 open), M124.1-M124.3 (#2372-#2374 open).

No browser or Studio surface may gain direct trusted write authority from this GDD.

## Live evidence and acceptance overview

A later product-observed claim for the implemented dogfood slice must include:

- runnable local URL/command identity;
- live screenshots for canonical states: `start`, `key-collected`, `gate-open`, `fail/blocked`, `paused`, `restarted`, `win/exit`;
- deterministic replay with labels and final state digest;
- runtime diagnostics model showing no hidden missing assets or render/input failures for a green claim;
- Studio workspace/scene/tile/behavior authoring evidence or explicit dependency/backlog lines;
- generated-state audit proving run/screenshots/browser/package outputs are not tracked source;
- human-owned fun/feel notes that state whether the slice is understandable, fair, and worth iterating.

For #2384, the evidence is this GDD plus the acceptance matrix in `docs/dogfood-mini-game-acceptance-matrix-v1.md`; no live playability claim is made.

## Human-owned fun/feel rubric

The target feel is "small but complete relay adventure": readable objective, light timing pressure, clear recovery, and a satisfying final gate opening. These are design targets only until a human playtest verdict is recorded.

Humans own final answers to:

- Is the first objective understandable without developer explanation?
- Is failure fair and recoverable?
- Does the 5-10 minute pacing feel like a mini-game rather than a fixture?
- Are visuals clear enough to invite iteration?
- Is the slice worth using as the next dogfood production target?

Ouroforge may collect observations, replay data, screenshots, diagnostics, and structured verdict forms. It must not convert deterministic success into an automated fun/feel pass.

## Trace to follow-up issues

- #2385 should turn this GDD into a project/scene/asset plan and preserve dependency flags for open M118-M124 capabilities.
- #2386 should implement/live-capture the vertical slice only after required runtime/visual/authoring dependencies are available or explicitly recorded as gaps.
- #2387 should record Scenario Coverage for the M128 slice as the milestone-final issue, including any deferred dependencies.
- #2391-#2394 should consume this target when defining the final production usability gate.
