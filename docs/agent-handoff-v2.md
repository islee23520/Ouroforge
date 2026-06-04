# Agent Handoff Artifact v2

Agent Handoff Artifact v2 is the #667 context-transfer contract for Multi-Agent Production Pipeline v1. It lets the next role act from explicit evidence instead of hidden chat context.

The schema version is `agent-handoff-v2`. A handoff contains:

- `handoffId`;
- `fromRole` and `toRole`;
- `taskId`;
- `status` (`ready`, `blocked`, `stale`, `failed`, or `completed`);
- `artifactRefs`;
- `assumptions`;
- `decisions`;
- `evidenceLinks`;
- `openRisks`;
- `acceptanceChecklist`;
- `staleStateIndicators`;
- `nextRecommendedAction`;
- `forbiddenActions`;
- `generatedState`;
- optional `v1Compatibility` metadata;
- `boundary`.

## Fixture set

Tracked fixtures live under `examples/multi-agent-pipeline-v1/`:

- `agent-handoff-v2.valid.fixture.json` — ready handoff with complete fixture-scoped evidence.
- `agent-handoff-v2.stale.fixture.json` — stale-state indicator that stops downstream action until evidence is refreshed.
- `agent-handoff-v2.blocked.fixture.json` — blocked handoff with explicit risk and unchecked acceptance item.
- `agent-handoff-v2.v1-compatible.fixture.json` — v2 artifact documenting compatibility with `agent-handoff-contract-v1`.
- `agent-handoff-v2.invalid.fixture.json` — intentionally malformed handoff missing evidence links.

## Trust boundary

Handoff v2 is advisory evidence only. It does not execute commands, lock files, spawn agents, write trusted browser state, apply changes, merge branches, self-approve, release artifacts, or grant production authority. Rust/local validation owns trusted acceptance; dashboard and Studio surfaces may only display handoff state when separately scoped.
