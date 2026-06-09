# Self-Audit and Attribution Demo v1

Issue #2031 demonstrates the Era L M69 self-audit demo by composing the existing
self-audit attribution contract, bottleneck attribution, and acceptance evaluator.
The demo plants a defect in the already-collected dogfood evidence stream and
emits both ranked bottlenecks and per-milestone acceptance verdicts.

It is intentionally a thin Rust-kernel read model over the existing pipeline:
openchrome/scenario verdicts, the four gates plus design-integrity, `journal.md`,
`ledger.jsonl`, loop-coverage attribution, evolve, source-apply, and
trust-gradient. It introduces no verification engine, no persistent store, and no
new data plane.

Verification:

```bash
cargo test -p ouroforge-core --test self_audit_demo --jobs 2
```

Autonomy and safety boundaries remain unchanged: the autonomous path requires
zero human input, while high-risk/source-affecting changes are never auto-applied
and remain queued for the thin human go/no-go. Fun/taste and release go/no-go
remain human Ring 2 decisions.
