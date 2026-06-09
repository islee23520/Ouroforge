# Scenario Coverage v64: Optional Human Channel

Coverage v64 locks Milestone 72 optional oversight and taste-feedback behavior
for the existing Era I engine-builder deckbuilder dogfood subject.

Matrix fixture:
`examples/real-title-dogfood-v1/scenario-coverage-v64/matrix.fixture.json`.

The suite is test-only. It verifies:

- the oversight surface is read-only and renders stage health, blockers,
  diagnosis, and attribution from existing evidence;
- the autonomous real-title loop completes with zero human input;
- the optional human spot-check/override/taste-feedback run also completes and
  the loop never waits;
- override records are provenance-only and cannot perform trusted writes,
  source-apply, auto-apply, or block the loop;
- taste/fun feedback reuses M57 curation and M58 playtest/fun-feel provenance,
  remains human Ring 2, and is never auto-applied.

Coverage v64 reuses openchrome, scenario verdicts, the four gates plus
design-integrity, `journal.md`, `ledger.jsonl`, loop-coverage attribution,
evolve, source-apply, and trust-gradient. It introduces no new verification engine, no new data plane, and no new persistent store. Engine source changes
still route through source-apply plus gates plus trust-gradient; the
high-risk/source-affecting tail remains queued for thin human go/no-go only.
Fun/taste verdicts and release go/no-go remain human Ring 2. Layer-3 remains
DEFER. #1 and #23 remain open.

## Verification

```bash
cargo test --workspace --jobs 2
```
