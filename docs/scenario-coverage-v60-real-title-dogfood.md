# Scenario Coverage v60: Real-Title Dogfooding Regression Suite

Coverage v60 locks the Era L Milestone 68 dogfood harness and real-title demo
regressions. The coverage matrix is
`examples/real-title-dogfood-v1/scenario-coverage-v60/matrix.fixture.json`.

The suite covers:

- Real-title stage evidence: substrate, scoring, run/shop, balance, juice, UI,
  localization, and Steam-export.
- Harness stage attribution for detect→explain→trace→attribute→propose→re-verify→apply-or-queue.
- Friction logging through the existing harness taxonomy and `ledger.jsonl`.
- Resumability/idempotency when the harness is re-run against an existing run.
- Autonomy invariants: no human input on the autonomous path, no high-risk or
  source-affecting auto-apply, no new verifier, and no new data plane.

## Verification

```bash
cargo test --workspace --jobs 2
```

## Boundaries

Coverage v60 is test-only Rust coverage over existing fixtures and the existing
harness. It reuses openchrome, scenario verdicts, the four gates plus
design-integrity, journal.md, ledger.jsonl, loop-coverage attribution, evolve,
source-apply, and trust-gradient. It does not introduce a verification engine,
telemetry store, or data plane. Fun/taste and release go/no-go remain human Ring 2. #1 and #23 remain open.
