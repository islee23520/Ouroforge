# Scenario Coverage v62: Self-Diagnosis and Fix-Proposal Regression Suite

Coverage v62 locks the Era L Milestone 70 diagnosis and fix-proposal behavior.
The coverage matrix is
`examples/real-title-dogfood-v1/scenario-coverage-v62/matrix.fixture.json`.

The suite covers:

- Planted-defect diagnosis correctness: the real-title four-gates failure remains
  attributed to `m68-real-title-run` / `#2025` with evidence-linked causal
  stages.
- Existing-pipeline-only diagnosis evidence: verdict.json, `journal.md`,
  `ledger.jsonl`, loop-coverage attribution, source-apply, and trust-gradient.
- Source-apply proposal scoping: proposals are `patch-preview.v1` artifacts
  scoped to Rust engine/source targets under `crates/`.
- High-risk classification: source-affecting proposals stay blocked/unapplied,
  forbid `auto_apply` and `merge`, and require the thin human go/no-go tail.
- Fail-closed regressions for hidden human input, high-risk auto-apply drift,
  non-source targets, and missing high-risk review language.

## Verification

```bash
cargo test --workspace --jobs 2
```

## Boundaries

Coverage v62 is test-only Rust coverage over existing fixtures and the existing
openchrome/evidence pipeline. It reuses scenario verdicts, the four gates plus
design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve, source-apply, and trust-gradient. It does not introduce a verification engine, telemetry store, persistent store, or data plane. The autonomous path
requires zero human input. High-risk/source-affecting fixes are never
auto-applied and remain queued for the thin human go/no-go. Fun/taste and
release go/no-go remain human Ring 2. #1 and #23 remain open.
