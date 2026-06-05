# QA Performance Budget v1 fixtures

Issue: #689 — Performance Budget Swarm Evaluation v1.

These fixtures exercise `ouroforge_core::qa_performance_budget`. They are
evidence inputs only: performance metrics are not trusted truth, browser-sourced
metrics stay advisory, and the artifact never performs trusted mutation.

## Valid fixtures (parse and classify)

- `budget.pass.fixture.json` — all trusted metrics within budget → `pass`.
- `budget.fail.fixture.json` — a trusted timing metric exceeds its limit → `fail`.
- `budget.inconclusive.fixture.json` — only browser-probe metrics → `inconclusive`.
- `budget.missing.fixture.json` — a threshold's metric was not captured → `missing`.
- `budget.stale.fixture.json` — the run matrix entry is stale → `stale`.
- `budget.unsupported.fixture.json` — an unsupported comparator → `unsupported`.
- `budget.baseline-change.fixture.json` — baseline present, frame time regressed → `fail`.

## Invalid fixtures (fail closed)

- `invalid/budget.malformed-metric.fixture.json` — negative metric value.
- `invalid/budget.missing-baseline.fixture.json` — baseline required but absent.
- `invalid/budget.unsupported-no-blocker.fixture.json` — unsupported threshold without blockers.
- `invalid/budget.stale-no-blocker.fixture.json` — stale run without blockers.
- `invalid/budget.browser-no-warning.fixture.json` — browser metric without a trust warning.
- `invalid/budget.status-mismatch.fixture.json` — declared status disagrees with classification.
- `invalid/budget.unsafe-ref.fixture.json` — run matrix ref escapes local roots.
- `invalid/budget.unsafe-boundary.fixture.json` — boundary omits required conservative wording.
