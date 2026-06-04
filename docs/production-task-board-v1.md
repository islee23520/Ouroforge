# Production Task Board v1

Production Task Board v1 is the #666 local artifact contract for Multi-Agent Production Pipeline v1. It represents accountable work state: task ids, role assignments, owner agents, statuses, dependencies, target artifacts/files, acceptance criteria, required evidence, blockers, and scoped timestamps.

The board is not an executor. It does not spawn agents, run commands, apply changes, merge PRs, write trusted browser state, publish releases, or mutate source files. Agent outputs remain untrusted until Rust/local validation and review-gated apply or promotion accepts them under the relevant follow-up issue.

## Schema

The schema version is `production-task-board-v1`. A board contains:

- `boardId` — stable local board identifier;
- `milestone` — governing milestone or issue reference;
- `tasks[]` — task records;
- `generatedState` — generated roots and `trackedFixtureOnly` policy;
- `guardrails` and `forbiddenActions` — conservative no-hidden-agent/no-auto-apply/no-command-bridge boundaries;
- `boundary` — plain-language statement that the board is artifact/state only and does not execute work.

Each task contains:

- `id`;
- `kind`;
- `role`;
- `ownerAgent`;
- `status`;
- `dependsOn[]`;
- `targetArtifacts[]`;
- `acceptanceCriteria[]`;
- `requiredEvidence[]`;
- `blockedReasons[]` where scoped;
- `timestamps` (`createdAt`, `updatedAt`, `assignedAt`, `completedAt`) where scoped.

## Statuses

Allowed statuses are:

- `proposed`
- `assigned`
- `in-progress`
- `blocked`
- `ready-for-review`
- `accepted`
- `rejected`
- `deferred`
- `completed`

Status is metadata only. It cannot imply hidden execution, automatic retries, acceptance, promotion, or PR merge authority.

## Task kinds

Allowed v1 task kinds are:

- `design-contract`
- `scene-design`
- `gameplay-implementation`
- `asset-import`
- `qa-scenario`
- `performance-regression`
- `review-gate`
- `critic-gate`
- `release-candidate-gate`
- `summary`

Unsupported kinds must fail validation instead of becoming generic execution authority.

## Fixture set

Tracked fixtures live under `examples/multi-agent-pipeline-v1/`:

- `production-task-board.fixture.json` — valid board with completed, ready-for-review, and assigned tasks.
- `production-task-board.blocked.fixture.json` — blocked board state with explicit blocker evidence.
- `production-task-board.stale.fixture.json` — stale/deferred board state that cannot self-refresh or promote outputs.
- `production-task-board.invalid.fixture.json` — unsupported task kind fixture that must fail validation.
- `production-task-board.malformed.fixture.json` — missing required evidence fixture that must fail validation.

Generated boards remain untracked under generated roots such as `runs/multi-agent-pipeline/` unless a future issue explicitly scopes a deterministic fixture.

## Trust boundary

Rust/local validation owns schema acceptance and any future trusted persistence. Browser, dashboard, and Studio surfaces may display board state only when separately scoped as read-only/draft-only consumers. They must not add browser-side trusted writes, command bridges, local server command bridges, credentialed commands, network/install commands, dependency mutation, CI/workflow mutation, release automation, auto-apply, auto-merge, self-approval, hidden promotion, or hidden worker orchestration.
