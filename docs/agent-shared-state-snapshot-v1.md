# Agent Shared State Snapshot v1

Agent Shared State Snapshot v1 gives multi-agent work a consistent local view of
project state before work begins and makes stale or conflicting state visible.
It is read-only context evidence. It is not trusted write authority, a command
queue, a browser command bridge, a hidden worker runtime, release automation, or
a source mutation mechanism.

## Artifact shape

A snapshot uses `schemaVersion: agent-shared-state-snapshot-v1` and includes:

- `snapshotId`, `milestone`, `status`, and UTC `observedAt`;
- project manifest, asset manifest, scenario pack, scene, tilemap, and behavior
  version hashes using `sha256:<64-hex>` text;
- recent run ids, open task ids, ownership map entries, and pending reviews;
- optional `beforeContext` and `afterContext` artifact refs for review and
  handoff comparisons;
- visible `staleRefs`, `missingRefs`, `malformedReasons`, and `conflictingRefs`
  where scoped;
- generated-state roots, guardrails, forbidden actions, and boundary text.

Supported status values are `fresh`, `stale`, `partial`, `missing`,
`malformed`, and `conflicting`. Fresh snapshots must not hide stale, missing,
malformed, or conflicting refs. Stale, partial/missing, malformed, and
conflicting snapshots must carry the matching visible evidence fields.

## Generated-state policy

Generated snapshots live under local generated roots such as
`runs/multi-agent-pipeline`. Generated task boards, handoffs, work packages,
snapshots, evidence bundles, runs, dashboard exports, temporary projects, and
local tool state remain untracked unless explicitly fixture-scoped. The tracked
fixtures under `examples/multi-agent-pipeline-v1/` are schema and regression
examples only.

## Trust boundary

The snapshot is inert context evidence:

- it does not execute commands;
- it does not spawn agents;
- it does not mutate task boards, source files, dashboard exports, or local tool
  state;
- it does not write trusted browser state;
- it does not repair stale state;
- it does not auto-apply, auto-merge, self-approve, bypass reviewers, or promote
  agent outputs;
- it does not create hidden background agents, unbounded spawning, remote worker
  pools, hosted/cloud orchestration, or account systems;
- it does not use credentialed commands, network/install commands, dependency
  mutation, CI/CD mutation, workflow mutation, build-script mutation, release
  automation, signing, publishing, native export, or public visibility changes;
- it does not claim production readiness or Godot replacement capability.

Browser/dashboard/Studio surfaces may display escaped read-only snapshot
summaries only. Rust/local validation owns trusted artifact validation and any
generated evidence writing. Agent outputs remain untrusted until Rust/local
validation and review-gated apply or promotion.

## Fixture examples

- `agent-shared-state-snapshot.fresh.fixture.json` — complete current context;
- `agent-shared-state-snapshot.stale.fixture.json` — stale refs are visible;
- `agent-shared-state-snapshot.partial.fixture.json` — partial evidence is
  visible;
- `agent-shared-state-snapshot.missing.fixture.json` — missing evidence is
  visible;
- `agent-shared-state-snapshot.malformed.fixture.json` — malformed state is
  explicit;
- `agent-shared-state-snapshot.conflicting.fixture.json` — ownership/state
  conflict evidence is explicit;
- `agent-shared-state-snapshot.invalid.fixture.json` — unsupported hash format
  is rejected.

Issue #670 MAP13.7.1 only defines schema, docs, and fixtures. Staleness
comparison against current project/task artifacts belongs to MAP13.7.2, and
read-model/display compatibility belongs to MAP13.7.3.

Issues #1 and #23 must remain open unless a separate explicit governance
decision says otherwise.
