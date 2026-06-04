# Agent Decision Ledger v1

Agent Decision Ledger v1 records why multi-agent pipeline decisions were made.
It is append-only local audit evidence for future agents and reviewers. It is
not an action log, command queue, mutation authority, merge approval, or release
automation surface.

## Artifact shape

A ledger uses `schemaVersion: agent-decision-ledger-v1` and includes:

- `ledgerId`, `milestone`, and `status` (`active`, `stale`, `malformed`, or
  `append-only-violation`);
- `decisions[]` with `decisionId`, `taskId`, `role`, `actorId`, `rationale`,
  `evidenceRefs`, `alternativesRejected`, `confidence`, `scopeRisk`, `outcome`,
  `timestamp`, and optional `blockedReasons`;
- `pipelineRefs` linking the ledger to task board, work packages, handoffs,
  review gates, QA results, performance/regression results, and production
  evidence bundles;
- `appendOnly` metadata (`sequence`, optional `previousLedgerHash`,
  `entryOrderHash`, and an append-only note);
- `generatedState`, `guardrails`, `forbiddenActions`, and `boundary` text.

Decision outcomes are conservative audit statuses: `accepted`, `rejected`,
`deferred`, `blocked`, and `superseded`. They do not apply or promote changes.
Confidence is `low`, `medium`, or `high`. Scope risk is `narrow`, `moderate`, or
`broad`.

## Generated-state policy

Generated decision ledgers live under local generated roots such as
`runs/multi-agent-pipeline`. Generated task boards, handoffs, work packages,
snapshots, evidence bundles, runs, dashboard exports, temporary projects, and
local tool state remain untracked unless explicitly fixture-scoped.

Fixture examples under `examples/multi-agent-pipeline-v1/` are tracked only as
schema and regression examples.

## Trust boundary

The ledger is inert audit evidence:

- it does not execute commands;
- it does not spawn agents;
- it does not apply changes;
- it does not merge changes;
- it does not write trusted browser state;
- it does not provide browser command bridges;
- it does not auto-apply, auto-merge, self-approve, or bypass reviewers;
- it does not create hidden background agents or unbounded spawning;
- it does not provide cloud orchestration, production CI/CD, release automation,
  signing, publishing, or public visibility changes;
- it does not claim production readiness;
- it does not claim Godot replacement capability.

Browser/dashboard/Studio surfaces may display escaped read-only ledger summaries
only. The Rust read model `agent-decision-ledger-read-model-v1` exposes ledger
status, decision/outcome counts, append-only sequence and hash metadata, stale or
malformed blockers, and links to work packages, handoffs, review gates, QA,
performance/regression, and production evidence bundles without repairing drift
or granting write authority. Rust/local validation owns trusted artifact
validation and any generated evidence writing. Agent outputs remain untrusted
until Rust/local validation and review-gated apply or promotion.

## Status fixtures

- `agent-decision-ledger.valid.fixture.json` — active append-only audit ledger;
- `agent-decision-ledger.stale.fixture.json` — stale linked evidence is visible;
- `agent-decision-ledger.malformed.fixture.json` — malformed state is explicit;
- `agent-decision-ledger.append-only-violation.fixture.json` — append-only drift
  is represented as evidence, not repaired by the browser;
- `agent-decision-ledger.invalid.fixture.json` — unsupported outcomes and unsafe
  boundary drift are rejected.

Issues #1 and #23 must remain open unless a separate explicit governance
decision says otherwise.
