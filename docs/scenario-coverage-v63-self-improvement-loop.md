# Scenario Coverage v63: Self-Improvement Loop Regression Suite

Coverage v63 locks the Era L Milestone 71 self-improvement behavior. The
coverage matrix is
`examples/real-title-dogfood-v1/scenario-coverage-v63/matrix.fixture.json`.

The suite covers:

- re-verify gate integrity: openchrome, scenario verdicts, the four gates plus
  design-integrity, rollback, kill-switch, source-apply, and trust-gradient;
- low-risk auto-apply only when the proposal is reversible, trust-gradient
  approved, no hidden human input exists, and before/after evidence improves;
- rollback when a post-apply re-verify regresses;
- high-risk/source-affecting queueing as thin one-click human go/no-go
  provenance, not auto-apply;
- fail-closed regressions for unverified, irreversible, hidden-human, and
  new-data-plane drift;
- the full demo chain: detect, explain, trace, attribute, propose, re-verify,
  apply, and queue on the existing Era I engine-builder deckbuilder.

## Verification

```bash
cargo test --workspace --jobs 2
```

## Boundaries

Coverage v63 is test-only Rust coverage over existing fixtures and the existing
openchrome/evidence pipeline. It reuses scenario verdicts, the four gates plus
design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve, source-apply, trust-gradient, rollback, and kill-switch. It does not introduce a verification engine, telemetry store, persistent store, or data plane. The autonomous low-risk path requires zero human input. Unverified,
irreversible, regressing, high-risk, or source-affecting fixes are never auto-applied; the high-risk tail remains queued for thin one-click human
go/no-go provenance while unrelated autonomous work continues. Fun/taste and
release go/no-go remain human Ring 2. #1 and #23 remain open.
