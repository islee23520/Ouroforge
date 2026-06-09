# Optional Human Channel Demo v1

The demo fixture is
`examples/real-title-dogfood-v1/optional-human-channel-v1/demo.fixture.json`.
It compares two real-title dogfood runs for the existing Era I engine-builder
deckbuilder:

1. a no-human autonomous run using
   `cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2`;
2. the same run with optional human spot-check, non-blocking override
   provenance, and taste/fun-feedback provenance.

Both runs complete and the loop never waits for the human channel. The optional
human inputs are recorded as provenance only: the oversight view is read-only,
the override is a non-blocking nudge record, and taste feedback reuses M57
curation plus M58 playtest/fun-feel provenance. No surface performs trusted writes, source-apply, auto-apply, verifier execution, data-plane changes, or a new store; no new store is introduced.

The demo reuses openchrome, scenario verdicts, the four gates plus
design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve/source-apply, and trust-gradient evidence. High-risk/source-affecting
changes remain queued for thin human go/no-go only. Fun/taste verdicts and release go/no-go remain human Ring 2. Layer-3 remains DEFER. #1 and #23 remain open.

## Verification

```bash
cargo build --workspace --jobs 2
cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2 || true
```
