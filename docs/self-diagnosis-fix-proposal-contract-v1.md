# Self-Diagnosis and Fix-Proposal Contract v1

Issue #2033 defines the Era L M70 contract for turning an attributed self-audit
failure into root-cause hypotheses and a scoped engine-fix proposal. The fixture
is `examples/real-title-dogfood-v1/self-diagnosis-fix-proposal-v1/contract.fixture.json`.

Inputs stay inside the existing dogfood evidence pipeline:

- `verdict.json` from the openchrome/scenario/evaluator run;
- `journal.md` and `ledger.jsonl` from the existing run artifacts;
- loop-coverage attribution and self-audit bottleneck attribution;
- source-apply patch-preview and trust-gradient risk evidence.

Outputs are bounded:

1. a diagnosis record with evidence-linked root-cause hypotheses;
2. a `patch-preview.v1` source-apply proposal with `sourceMutationApplyStatus:
   blocked`;
3. required re-verification commands for later review; and
4. a read-only status model that forbids apply, merge, and auto-apply.

M70 does not apply fixes. High-risk/source-affecting proposals are queued for the
thin human go/no-go and later source-apply review. The autonomous path can still
diagnose and propose without a human, but source-affecting tails are never
auto-applied here.

## Verification

```bash
grep -RIlqi "loop.coverage\|ledger\|journal\|verdict" docs/ || true
cargo build --workspace --jobs 2
cargo test -p ouroforge-core --test self_diagnosis_fix_proposal_contract --jobs 2
```

## Boundaries

This contract is read-only and reuses openchrome, scenario verdicts, the four
gates plus design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage
attribution, evolve, source-apply, and trust-gradient. It introduces no new verification engine and no new data plane. The Rust kernel/evaluator/source-apply
remain the data plane; the Elixir executor remains the control plane. Fun/taste and release go/no-go stay human Ring 2 decisions. #1 and #23 remain open.
