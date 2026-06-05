# QA Failure Classification and Mutation Backlog v1 fixtures

Issue: #692 — Failure Classification and Mutation Backlog v1.

These fixtures exercise `ouroforge_core::qa_failure_backlog`. They turn failures
into evidence-linked backlog items, not automatic fixes; items stay review-gated
and the artifact never performs trusted mutation.

## Valid fixtures

- `backlog.classified.fixture.json` — gameplay-logic, performance, and visual items → `classified`.
- `backlog.unknown.fixture.json` — an unknown-class item awaiting triage → `classified`.
- `backlog.flaky.fixture.json` — a flaky-class item → `classified`.
- `backlog.stale.fixture.json` — stale run ref with a blocker → `stale`.
- `backlog.blocked.fixture.json` — an item with a blocker → `blocked`.

## Invalid fixtures (fail closed)

- `invalid/backlog.missing-evidence.fixture.json`
- `invalid/backlog.invalid-owner-lane.fixture.json`
- `invalid/backlog.unsupported-class.fixture.json`
- `invalid/backlog.missing-reproduction.fixture.json`
- `invalid/backlog.stale-no-blocker.fixture.json`
- `invalid/backlog.duplicate-id.fixture.json`
- `invalid/backlog.auto-fix-attempt.fixture.json`
- `invalid/backlog.unsafe-ref.fixture.json`
- `invalid/backlog.status-mismatch.fixture.json`
- `invalid/backlog.unsafe-boundary.fixture.json`
