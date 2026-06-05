# QA Error Classifier v1 fixtures

Issue: #690 — Console, Crash, and Runtime Error Classifier v1.

These fixtures exercise `ouroforge_core::qa_error_classifier`. They are evidence
inputs only: classifications are not trusted truth and the artifact never
performs trusted mutation.

## Valid fixtures

- `classifier.classified.fixture.json` — console warning/error, exception,
  crash, probe-unavailable, asset-load-failure, scenario-timeout, and unknown
  entries → `classified`.
- `classifier.inconclusive.fixture.json` — an inconclusive probe entry with a
  blocker → `blocked`.
- `classifier.stale.fixture.json` — stale run ref with a blocker → `stale`.

## Invalid fixtures (fail closed)

- `invalid/classifier.missing-console-evidence.fixture.json`
- `invalid/classifier.missing-probe-evidence.fixture.json`
- `invalid/classifier.malformed-payload.fixture.json`
- `invalid/classifier.unknown-severity.fixture.json`
- `invalid/classifier.unsupported-kind.fixture.json`
- `invalid/classifier.missing-classification.fixture.json`
- `invalid/classifier.kind-class-mismatch.fixture.json`
- `invalid/classifier.stale-no-blocker.fixture.json`
- `invalid/classifier.unsafe-ref.fixture.json`
- `invalid/classifier.unsafe-boundary.fixture.json`
