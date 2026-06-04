# QA Agent Work Queue v1

QA Agent Work Queue v1 records explicit QA/playtest work items for the
multi-agent production pipeline. It is an inert local artifact and validation
contract, not a worker runtime, browser command bridge, hidden agent loop,
trusted write path, release pipeline, production-readiness claim, or Godot
replacement claim.

## Artifact shape

A queue uses `schemaVersion: qa-agent-work-queue-v1` and includes:

- `queueId`, `milestone`, and `items`;
- per-item `queueItemId`, `scenarioTarget`, `riskArea`, `runCommandContext`,
  `expectedEvidence`, `priority`, assigned role/agent, `status`, optional
  `statusTransition`, optional `failureClassification`, task/work-package refs,
  review gate refs, run and evaluator evidence refs, blocked reasons, and stale
  run refs;
- generated-state roots, guardrails, forbidden actions, and boundary text.

Supported statuses are `pass`, `fail`, `deferred`, `blocked`, `flaky`, and
`needs-rerun`. Failed and flaky items keep failure classifications visible.
Deferred and blocked items keep blocked reasons visible. Needs-rerun items keep
stale run refs and/or rerun blockers visible instead of silently trusting stale
QA evidence.

## Trust boundary

The queue is inert local evidence:

- it does not execute commands;
- it does not spawn agents, hidden/background workers, remote worker pools, or
  unbounded QA loops;
- it does not mutate trusted files, apply changes, merge PRs, publish, sign,
  deploy, or change visibility;
- it does not write trusted browser state or provide a browser/local command
  bridge;
- it does not auto-apply, auto-merge, self-approve, bypass reviewers, prevent reviewer bypass, or hide QA
  failures;
- it does not use credentialed commands, network/install commands, dependency
  mutation, CI/workflow/build-script mutation, release automation, or dynamic
  code loading;
- it does not claim production readiness, commercial readiness, arbitrary game
  completion, or Godot replacement capability.

`runCommandContext` is reproducible inert command text for reviewers/operators.
Rust/local validation owns trusted queue validation. Browser, dashboard, Studio,
and cockpit surfaces may display escaped read-only queue summaries only. QA
outputs remain untrusted until Rust/local validation and accepted review decide
whether they feed fixes, reruns, or later promotion gates.

## MAP13.9.2 validation rules

Rust validation rejects queue drift before a QA queue can be trusted as local
evidence:

- `scenarioTarget.scenarioPackRef` must identify a scenario pack JSON fixture or
  source-like scenario pack, not generated run or dashboard state.
- `runCommandContext` is inert text only and rejects mutation, apply, merge,
  publish, deploy, install, dependency mutation, release, credentialed, network
  fetch, and file-mutating command words such as `gh pr merge`, `git push`,
  `npm install`, `cargo add`, `curl`, `ssh`, `rm`, or `chmod`.
- `expectedEvidence` must be unique, remain under generated QA roots, and include
  scenario plus evaluator expectations; observed run/evaluator evidence refs must
  also remain under generated QA roots.
- `staleRunRefs` are explicit generated-root paths, must not duplicate current
  run/evaluator evidence refs, and `needs-rerun` items require
  `failureClassification: stale-run-ref`.
- Optional `statusTransition` is state-machine metadata only. Its `to` status
  must match the item `status`, and invalid terminal or mutation-shaped jumps
  such as `pass -> fail` are rejected.

## MAP13.9.3 dashboard and Studio linkage

Dashboard export indexes `qa-agent-work-queue` / `qa_agent_work_queue` artifacts
as `qa_agent_work_queues`. The read model reports queue/item counts, status
counts, malformed artifact count, and linked refs. Linked refs include:

- scenario pack targets;
- expected scenario/evaluator evidence;
- observed run and evaluator evidence;
- task board, work-package, and review-gate refs;
- stale run refs that force visible `needs-rerun`/blocked states.

Studio multi-agent pipeline inspection reads `qaAgentWorkQueue` /
`qaAgentWorkQueues` inputs directly for the `qa-queue` section instead of only
inferring queue readiness from task-board text. The section reports present,
blocked, missing, empty, or malformed status from the validated queue artifact.
The evidence dashboard and authoring cockpit render escaped, read-only QA queue
panels. They show inert command text as copyable/display data only and do not
create command buttons, browser command bridges, trusted writes, auto-apply,
auto-merge, self-approval, hidden workers, or agent spawning controls.

## Generated-state policy

Generated QA queue output lives under local generated roots such as
`runs/multi-agent-pipeline`. Generated task boards, handoffs, work packages,
snapshots, review gates, QA queues, evidence bundles, runs, dashboard exports,
temporary projects, and local tool state remain untracked unless explicitly
fixture-scoped. The tracked fixtures under `examples/multi-agent-pipeline-v1/`
are schema and regression examples only.

## Fixture examples

- `qa-agent-work-queue.valid.fixture.json` — passed QA queue item with scenario,
  evaluator, run, work-package, task-board, and review gate links;
- `qa-agent-work-queue.failed.fixture.json` — failed item with visible failure
  classification;
- `qa-agent-work-queue.deferred.fixture.json` — deferred item with blocked
  reason;
- `qa-agent-work-queue.flaky.fixture.json` — flaky item with multiple run
  evidence refs;
- `qa-agent-work-queue.blocked.fixture.json` — blocked item with review-gate
  blocker;
- `qa-agent-work-queue.stale.fixture.json` — needs-rerun item with stale run
  refs;
- `qa-agent-work-queue.malformed.fixture.json` — intentionally malformed queue
  item fixture for validation tests.

Issue #672 MAP13.9.1 defines the queue artifact, statuses, docs, and fixtures.
MAP13.9.2 owns deeper validation for scenario targets, command contexts,
evidence expectations, stale refs, state transitions, and mutation attempts.
MAP13.9.3 owns scenario/evaluator/run/review linkage and dashboard/Studio
compatibility.

Issues #1 and #23 must remain open unless a separate explicit governance
decision says otherwise.
