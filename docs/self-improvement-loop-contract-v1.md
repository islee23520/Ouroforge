# Self-Improvement Loop Contract v1

Issue: #2037 — Era L Milestone 71.

This contract defines how a generated fix proposal is re-verified and routed
without adding a new verification engine or data plane. It reuses the existing
pipeline only:

- openchrome real-title run via
  `cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2`
- scenario verdicts and the four gates plus design-integrity
- `verdict.json`, `journal.md`, and `ledger.jsonl`
- loop-coverage attribution
- source-apply patch preview / transaction / rollback artifacts
- trust-gradient risk tier, auto-apply decision, audit, and kill-switch artifacts

## Re-verify contract

A proposal can be routed only after the real Era I engine-builder deckbuilder
subject is re-run through openchrome and evidence is recorded in the existing
run artifacts. Re-verification requires mechanical, runtime, visual, semantic,
and design-integrity gates to pass. Missing, stale, flaky, or failed evidence
blocks the route rather than silently passing.

## Routing contract

- **Auto-apply eligible**: low-risk, non-source-affecting, reversible proposals
  with fresh evidence, passing gates, trust-gradient auto-apply eligibility, an
  available rollback handle, and no kill-switch.
- **Human go/no-go**: high-risk or source-affecting proposals, including Rust
  kernel/evaluator/source-apply changes, remain queued for a thin human
  go/no-go. They are never self-applied.
- **Blocked**: failed/missing re-verification evidence, stale source-apply
  evidence, missing rollback, or an engaged kill-switch blocks any apply route.

## Boundaries

The Rust kernel/evaluator/source-apply remains the data plane. The Elixir
executor remains the control plane and is unchanged. The contract does not execute openchrome, apply patches, merge branches, create a verifier, create a persistent store, or introduce a new data plane. Fun/taste and release go/no-go
remain human Ring 2. #1 and #23 remain open.

## Verification

```bash
grep -RIlqi "loop.coverage\|ledger\|journal\|verdict" docs/ || true
cargo build --workspace --jobs 2
```

## Re-verify-then-apply loop

The M71 apply loop is deterministic over recorded evidence. It reads the
proposal, the existing source-apply preview/transaction refs, the openchrome
re-run verdict, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
trust-gradient routing, rollback metadata, and kill-switch state.

- A reversible low-risk proposal that passes re-verify and improves the
  attributed milestone evidence can be auto-applied with zero human input via
  the existing source-apply/trust-gradient path.
- A regression after re-verify is rejected and rolled back through the existing
  rollback handle; the failed evidence remains visible instead of being treated
  as success.
- A high-risk or source-affecting proposal never auto-applies and remains queued
  for the thin human go/no-go.

The loop records a report only; it does not create a new store, telemetry schema,
verification engine, browser executor, or data plane.
