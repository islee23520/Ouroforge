# QA Flake and Rerun Policy v1 fixtures

Issue: #691 — Flaky Evidence and Rerun Policy v1.

These fixtures exercise `ouroforge_core::qa_flake_rerun_policy`. They are evidence
inputs only: reruns are bounded, results are review-gated, and the artifact never
performs trusted mutation.

## Valid fixtures (parse and classify)

- `policy.stable-pass.fixture.json` — consistent passing reruns → `stable-pass`.
- `policy.stable-fail.fixture.json` — consistent failing reruns → `stable-fail`.
- `policy.flaky.fixture.json` — pass-then-fail with divergent fields → `flaky`.
- `policy.inconclusive.fixture.json` — inconclusive outcome within budget → `inconclusive`.
- `policy.exhausted.fixture.json` — rerun budget used up without consistency → `exhausted`.
- `policy.unsupported.fixture.json` — reruns not supported → `unsupported`.
- `policy.stale.fixture.json` — stale run ref → `stale`.

## Invalid fixtures (fail closed)

- `invalid/policy.unbounded-reruns.fixture.json`
- `invalid/policy.missing-threshold.fixture.json`
- `invalid/policy.overlapping-outputs.fixture.json`
- `invalid/policy.missing-cleanup.fixture.json`
- `invalid/policy.stale-no-blocker.fixture.json`
- `invalid/policy.malformed-comparison.fixture.json`
- `invalid/policy.missing-original-evidence.fixture.json`
- `invalid/policy.classification-mismatch.fixture.json`
- `invalid/policy.unsafe-boundary.fixture.json`
