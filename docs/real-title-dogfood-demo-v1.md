# Real-Title Dogfooding Demo v1

Era L Milestone 68 demonstrates the existing Era I engine-builder deckbuilder
through the live dogfood path without adding a verifier or data plane. The demo
script is fixture-scoped:

```bash
examples/real-title-dogfood-v1/demo/run-demo-v1.sh
```

The script runs the real title through the existing command:

```bash
cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2
```

It then resumes the existing dogfood harness for the generated run directory and
prints the existing `ledger.jsonl`. Generated browser/openchrome evidence remains
under ignored `runs/` directories; committed demo evidence stays under
`examples/real-title-dogfood-v1/demo/`.

## Friction summary

`examples/real-title-dogfood-v1/demo/friction-summary.fixture.json` records the
friction summary. The current demo has no hidden friction and links that claim to
existing `ledger.jsonl`, `journal.md`, and `verdict.json` evidence. Future
friction must be logged through the existing harness taxonomy, not a telemetry
store.

## Audit trail

`examples/real-title-dogfood-v1/demo/audit-trail.fixture.json` links each stage
of detect→explain→trace→attribute→propose→re-verify→apply-or-queue to existing
ledger events and evidence refs. It reuses openchrome, scenario verdicts, the
four gates plus design-integrity, journal.md, ledger.jsonl, loop-coverage
attribution, evolve, source-apply, and trust-gradient.

## Boundaries

- Autonomous path completes without human input.
- HIGH-RISK/source-affecting fixes are never auto-applied; they queue for thin
  human go/no-go through source-apply and trust-gradient.
- Fun/taste and release go/no-go remain human Ring 2.
- Rust remains the data plane; the Elixir executor remains unchanged.
- No new verification engine and no new data plane are introduced.
- #1 and #23 remain open.
