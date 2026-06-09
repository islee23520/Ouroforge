# Scenario Coverage v65: Autonomous Self-Improvement End-to-End

Coverage v65 locks the Era L end-to-end autonomous self-validation and
improvement loop for the existing Era I engine-builder deckbuilder.

Matrix fixture:
`examples/real-title-dogfood-v1/scenario-coverage-v65/matrix.fixture.json`.

The test-only suite composes the completed Milestones 68 through 72:

1. **Dogfood run / detect:** use the existing real-title openchrome run,
   scenario verdicts, `journal.md`, and `ledger.jsonl` evidence.
2. **Self-audit / attribute:** reuse loop-coverage attribution and acceptance
   audit evidence.
3. **Diagnose / propose:** reuse the diagnosis contract and source-apply patch
   preview. No source mutation happens at diagnosis time.
4. **Re-verify / apply:** low-risk reversible fixes auto-apply only after
   openchrome, scenario verdicts, the four gates plus design-integrity,
   rollback, kill-switch, and trust-gradient pass and after-evidence improves.
5. **High-risk tail:** high-risk/source-affecting fixes are never auto-applied;
   they queue for thin human go/no-go provenance.
6. **Optional human channel:** oversight, override, and taste feedback stay
   optional, read-only/provenance-only, and never block the loop.

The autonomous path completes with zero human input. Coverage v65 introduces no new verification engine, no new data plane, and no new persistent store. Rust remains the data plane, the Elixir executor remains unchanged, fun/taste verdicts
and release go/no-go remain human Ring 2, Layer-3 remains DEFER, and #1 and #23 remain open.

## Verification

```bash
cargo test --workspace --jobs 2
```
