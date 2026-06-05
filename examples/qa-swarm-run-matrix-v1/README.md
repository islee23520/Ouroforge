# QA Swarm Run Matrix v1 fixtures

Issue: #693 — QA Swarm Run Matrix v1.

These fixtures exercise `ouroforge_core::qa_run_matrix`. They are evidence inputs
only: dashboard/Studio surfaces stay read-only or draft-only and the artifact
never performs trusted mutation.

## Valid fixtures (parse and roll up status)

- `matrix.complete.fixture.json` — all rows resolved → `complete`.
- `matrix.partial.fixture.json` — a skipped row → `partial`.
- `matrix.flaky.fixture.json` — a rerun group with flaky rows → `partial`.
- `matrix.inconclusive.fixture.json` — an inconclusive row → `partial`.
- `matrix.missing.fixture.json` — a missing_evidence row → `partial`.
- `matrix.stale.fixture.json` — stale evidence refs → `stale`.
- `matrix.unsupported.fixture.json` — an unsupported row → `partial`.

## Invalid fixtures (fail closed)

- `invalid/matrix.malformed-verdict.fixture.json`
- `invalid/matrix.duplicate-row.fixture.json`
- `invalid/matrix.invalid-worker-id.fixture.json`
- `invalid/matrix.missing-run-ref.fixture.json`
- `invalid/matrix.missing-budget.fixture.json`
- `invalid/matrix.inconsistent-rerun-group.fixture.json`
- `invalid/matrix.missing-evidence.fixture.json`
- `invalid/matrix.stale-no-blocker.fixture.json`
- `invalid/matrix.not-read-only.fixture.json`
- `invalid/matrix.unsafe-boundary.fixture.json`
