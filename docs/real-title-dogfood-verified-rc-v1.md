# Real-Title End-to-End Run to Verified Release Candidate v1

Era L Milestone 68 carries the existing Era I engine-builder deckbuilder to a
fixture-scoped verified release-candidate evidence state. The canonical seed is
`seeds/dogfood-deckbuilder.yaml`; the real-title command remains:

```bash
cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2
```

The committed evidence bundle is under `examples/real-title-dogfood-v1/`. It
uses the existing release provenance bundle validator, the existing Milestone 25
per-change provenance bundle shape, the dogfood harness ledger, `journal.md`,
`verdict.json`, loop-coverage attribution, source-apply/trust-gradient boundary
refs, and scenario coverage v60. It introduces no new verification engine and no new data plane.

## Verified RC chain

| Stage | Evidence |
| --- | --- |
| substrate | `examples/card-roguelite-substrate-v1/engine-builder/deckbuilder.config.golden.json` |
| scoring | `examples/game-runtime/score-cascade-feedback-v1.json` |
| run/shop | `examples/game-runtime/deckbuilder-ui-scene-v1.json` |
| balance | `docs/engine-builder-balance-v1.md` |
| juice | `examples/game-feel-juice-v1/demo/game-feel-juice-demo-v1.json` |
| UI | `examples/deckbuilder-ui-v1/demo/demo-manifest.json` |
| localization | `docs/localization-v1-demo.md` |
| Steam-export | `docs/steam-desktop-export-v1-demo.md` |

## Friction policy

The verified RC fixture records `dogfood.friction.none` in `ledger.jsonl` so the
run explicitly states that no friction was hidden. Future friction must be one of
the harness taxonomy events (`stall`, `retry`, `manual-intervention`,
`budget-halt`, `gate-fail`) with stage attribution and evidence refs.

## Boundaries

- Autonomous path completes without human input.
- HIGH-RISK/source-affecting changes are never auto-applied; they queue for thin
  human go/no-go through source-apply and trust-gradient.
- Fun/taste and release go/no-go remain human Ring 2.
- Rust remains the data plane; the Elixir executor remains unchanged.
- Generated run artifacts stay untracked unless fixture-scoped.
- #1 and #23 remain open.
