# Scenario Coverage v61: Self-Audit and Attribution Regression Suite

Coverage v61 locks the Era L Milestone 69 self-audit attribution and acceptance
audit behavior. The coverage matrix is
`examples/real-title-dogfood-v1/scenario-coverage-v61/matrix.fixture.json`.

The suite covers:

- The self-audit attribution contract links real dogfood evidence to milestones,
  gates, success criteria, and trends while extending loop-coverage attribution.
- Planted-defect bottleneck attribution ranks the intended milestone and ignores
  passing signals.
- Acceptance auditing emits per-milestone declared-gate-style verdicts.
- Regression and missing-evidence cases fail closed rather than silently passing.
- The self-audit demo composes ranked bottlenecks and per-milestone acceptance
  verdicts without adding a verifier or data plane.

## Verification

```bash
cargo test --workspace --jobs 2
```

## Boundaries

Coverage v61 is test-only Rust coverage over existing fixtures and the existing
openchrome/evidence pipeline. It reuses scenario verdicts, the four gates plus
design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve, source-apply, and trust-gradient. It does not introduce a verification engine, telemetry store, or data plane. The autonomous path requires zero human input; high-risk/source-affecting fixes are never auto-applied and remain queued
for the thin human go/no-go. Fun/taste and release go/no-go remain human Ring 2.
#1 and #23 remain open.
