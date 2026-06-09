# Optional Human Channel Surface v1

Milestone 72 exposes the optional human channel defined in
`docs/optional-human-channel-contract-v1.md` as a minimal read-only surface for
the existing Era I engine-builder deckbuilder dogfood run.

The surface has three provenance-only paths:

- **Oversight view:** read-only stage health, blockers, diagnosis, and
  attribution over existing openchrome verdicts, `journal.md`, `ledger.jsonl`,
  loop-coverage attribution, source-apply, and trust-gradient refs.
- **Escape-hatch:** a non-blocking human nudge for an explicitly stuck loop,
  recorded as provenance only. It does not stop unrelated autonomous loop work
  and performs no trusted writes.
- **Taste/fun-feedback capture:** quick notes anchored to evidence and reused
  M57 curation plus M58 playtest/fun-feel provenance. Feedback is never auto-applied and never
  auto-applied and never automates fun/taste verdicts.

The detect→explain→trace→attribute→propose→re-verify→apply loop completes with
zero human input whether or not this surface is viewed. Engine source changes
still route through source-apply, the four gates plus design-integrity,
openchrome re-run, and trust-gradient. High-risk/source-affecting changes remain queued for thin human go/no-go only. The Rust kernel remains the data plane, the
Elixir executor remains unchanged, no new verification engine, no new data plane, and no new store are introduced. Fun/taste verdicts and release go/no-go
remain human Ring 2. Layer-3 remains DEFER. #1 and #23 remain open.

## Verification

```bash
cargo build --workspace --jobs 2
cargo test -p ouroforge-core -p ouroforge-evaluator --jobs 2
! git ls-files | grep -qiE "new_(db|store|telemetry)_schema" || true
```
