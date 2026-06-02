# Input Replay and Deterministic Scenario Evidence v2

Input Replay Evidence v2 makes scenario-driven input replay a first-class run
artifact. It preserves the local-first, Rust-trusted boundary: replay artifacts
are JSON evidence files under a generated run directory, not commands, browser
writes, or automatic rerun instructions.

## Scope

This contract covers evidence produced by scenario execution:

- legacy `input_replay` artifacts for inline `replay` and `replayRef` scenario
  steps;
- new `scenario_input_replay` artifacts for supported scenario input surfaces;
- scenario-result links to replay evidence;
- replay-aware compare, journal, and dashboard summaries.

The contract does not add a replay editor, browser replay controls, Playwright,
a command bridge, video capture, hosted QA, or deterministic networking.

## Artifact schema

Each `scenario_input_replay` artifact is a JSON file indexed in
`evidence/index.json` with metadata similar to:

```json
{
  "artifact": "scenario_input_replay",
  "scenario_id": "bootstrap-smoke"
}
```

The artifact body uses schema version
`ouroforge.scenario-input-replay.v1`:

```json
{
  "schemaVersion": "ouroforge.scenario-input-replay.v1",
  "scenarioId": "bootstrap-smoke",
  "stepIndex": 1,
  "action": { "kind": "scenario_input_step" },
  "frame": 2,
  "tick": 2,
  "input": { "right": true },
  "probe": {
    "contractVersion": "runtime-probe-v2",
    "worldStatePath": "evidence/scenarios/bootstrap-smoke/world-state-1.json",
    "frameStatsPath": "evidence/scenarios/bootstrap-smoke/frame-stats-1.json"
  },
  "result": {
    "scenarioResultPath": "evidence/scenarios/bootstrap-smoke/scenario-result-1.json"
  }
}
```

Validation is owned by Rust. The model rejects unknown fields, unsafe scenario or
worker ids, unsupported schema versions, unbounded frames, empty input patches,
invalid runtime-probe contract versions, and artifact paths that escape their
allowed roots.

## Emission model

Scenario execution emits `scenario_input_replay` artifacts for supported input
surfaces:

- direct `input` scenario steps produce one artifact at the current scenario
  frame;
- inline replay events produce one artifact per replay frame group;
- `replayRef` events produce one artifact per referenced replay frame group.

The existing `input_replay` artifact remains for compatibility. Scenario result
JSON now includes both:

- `evidence.input_replays` — compatibility list containing legacy replay
  artifacts plus generated scenario input replay artifacts;
- `evidence.scenario_input_replays` — deterministic scenario input replay
  artifacts only.

Runtime probe correlation is recorded after `getWorldState` and `getFrameStats`
are captured. Result correlation points back to the scenario result. Replay
artifacts are evidence provenance, not replay execution authority.

## Compare, journal, and dashboard summaries

`compare` includes replay-specific deltas:

- `before.inputReplayArtifacts` / `after.inputReplayArtifacts`;
- `deltas.input_replay_artifacts`;
- `semantic.inputReplay.beforeCount` / `afterCount`;
- `semantic.inputReplay.added` / `removed`;
- semantic reason kind `input_replay` when replay evidence changes.

The run journal observation section reports legacy replay and scenario input
replay counts. The dashboard replay read-model treats `scenario_input_replay`
artifacts as replay sequences and surfaces their scenario id, frame/tick,
action kind, result links, and probe checkpoints when available.

## Determinism and limitations

Replay evidence is deterministic only within the current local browser runtime
contract:

- frame values are derived from scenario wait/replay frame semantics, not wall
  clock time;
- runtime state is trusted only through Rust-captured evidence artifacts;
- no guarantee is made for network, multiplayer, browser scheduler, or external
  service determinism;
- replay evidence may explain what input was applied, but it does not prove that
  a future run will produce identical world state without rerunning QA.

## Generated-state policy

Replay artifacts are generated state under `runs/<run-id>/evidence/...` and must
remain untracked. They may be used as local verification evidence in issue/PR
comments, but should not be committed unless a future issue explicitly scopes a
small fixture under a tracked test fixture directory.

Before closing replay-related work, verify:

```bash
git status --short --ignored
cargo test input_replay
cargo test scenario
cargo test compare
node examples/evidence-dashboard/dashboard.test.cjs
```

Expected ignored local roots include `.omx/`, `runs/`, and `target/`. A clean PR
should not add generated run directories, dashboard export data, browser cache,
or local tool state.
