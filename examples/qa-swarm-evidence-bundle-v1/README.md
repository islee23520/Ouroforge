# QA Swarm Evidence Bundle v1 fixtures

Issue: #694 — QA Swarm Evidence Bundle v1.

These fixtures exercise `ouroforge_core::qa_evidence_bundle`. They are evidence
inputs only: the bundle never performs trusted mutation and the dashboard export
stays read-only or draft-only.

## Valid fixtures (parse and roll up status)

- `bundle.complete.fixture.json` — all components present and resolved → `complete`.
- `bundle.partial.fixture.json` — an unresolved component → `partial`.
- `bundle.blocked.fixture.json` — top-level blockers, no component gaps → `blocked`.
- `bundle.flaky.fixture.json` — unresolved flaky states → `partial`.
- `bundle.stale.fixture.json` — a stale component → `stale`.

## Invalid fixtures (fail closed)

- `invalid/bundle.malformed.fixture.json`
- `invalid/bundle.unresolved-output.fixture.json`
- `invalid/bundle.missing-evidence.fixture.json`
- `invalid/bundle.missing-cleanup.fixture.json`
- `invalid/bundle.missing-budget.fixture.json`
- `invalid/bundle.inconsistent-matrix.fixture.json`
- `invalid/bundle.not-read-only.fixture.json`
- `invalid/bundle.status-mismatch.fixture.json`
- `invalid/bundle.unsafe-boundary.fixture.json`
