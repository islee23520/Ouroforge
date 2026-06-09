# Optional Human Channel Contract v1

The contract fixture is
`examples/real-title-dogfood-v1/optional-human-channel-v1/contract.fixture.json`.
It defines the Milestone 72 human channel as optional, non-blocking, and
read-only over the existing self-improvement evidence pipeline.

The channel has three surfaces:

- read-only oversight for stage health, blockers, diagnosis, and attribution;
- a non-blocking escape-hatch that records stuck-loop stop/override provenance;
- fast taste/fun-feedback capture that reuses M57 curation and M58 playtest /
  fun-feel provenance.

None of these surfaces can perform trusted writes, source-apply, auto-apply,
auto-merge, verifier execution, or data-plane changes. The autonomous loop does
not wait for them; it continues from openchrome verdicts, `journal.md`,
`ledger.jsonl`, loop-coverage attribution, evolve, source-apply, and
trust-gradient evidence. Fun/taste verdicts and release go/no-go remain human
Ring 2. #1 and #23 remain open.

## Verification

```bash
grep -RIlqi "loop.coverage\|ledger\|journal\|verdict" docs/ || true
cargo build --workspace --jobs 2
```
