# Self-Audit and Bottleneck-Attribution Contract v1

Era L Milestone 69 defines how the autonomous loop attributes real-build failures
to milestones and re-checks each milestone's #1 success criteria. The contract is
machine-checkable in Rust as `self-audit-attribution-contract-v1` and the fixture
is `examples/real-title-dogfood-v1/self-audit-attribution-v1/contract.fixture.json`.

## Contract

The contract extends `loop-coverage-attribution-v1`; it does not replace it. Each
mapping links evidence and loop-coverage attribution refs to a milestone, gate,
and loop stage. Acceptance audits then point at #1 success criteria and evaluate
existing evidence predicates. Trend definitions declare when a milestone has
improved, remained unchanged, regressed, or lacks enough evidence.

## Evidence pipeline

The contract reuses openchrome, scenario verdicts, the four gates plus
design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve, source-apply, and trust-gradient. It introduces no new verification
engine and no new data plane. In exact contract terms: no new verification engine and no new data plane.

## Boundaries

- Autonomous detect→explain→trace→attribute→propose→re-verify→apply-or-queue
  remains no-human-input by default.
- HIGH-RISK/source-affecting fixes are never auto-applied; they queue for thin
  human go/no-go through source-apply and trust-gradient.
- Fun/taste and release go/no-go remain human Ring 2.
- Rust kernel/evaluator/source-apply remain the data plane; the Elixir executor
  remains unchanged as control plane.
- #1 and #23 remain open.
