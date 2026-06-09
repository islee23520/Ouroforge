# Self-Improvement Loop Demo v1

The M71 demo fixture is
`examples/real-title-dogfood-v1/self-improvement-loop-demo-v1/demo.fixture.json`.
It demonstrates the existing Era I engine-builder deckbuilder as the dogfood
subject and keeps the loop on the existing evidence pipeline.

The demo chain is:

1. detect through the openchrome real-title run and scenario verdicts;
2. explain in `journal.md`;
3. trace in `ledger.jsonl`;
4. attribute through loop-coverage attribution;
5. propose through existing source-apply patch-preview evidence;
6. re-verify through the four gates, design-integrity, rollback, kill-switch,
   and trust-gradient;
7. apply the low-risk reversible fix only when before/after evidence improves;
8. queue the high-risk/source-affecting reversible fix for thin one-click human
   go/no-go provenance.

The high-risk item does not block unrelated autonomous loop work while it awaits
the click. The fixture is a demo evidence contract only: it does not create a new store, telemetry schema, verification engine, browser executor, or data plane.
Rust remains the data plane; the Elixir executor remains the unchanged control
plane. Fun/taste and release go/no-go stay human Ring 2.

## Verification

```bash
set -euo pipefail
cargo build --workspace --jobs 2
cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2 || true
```
