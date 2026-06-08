# Run and Shop Demo v1

Issue: #1808, under Escalating Run Structure and Shop Economy v1 (#1805) and #1 Era I Milestone 49.

This demo is a deterministic, fixture-scoped Rust/local smoke of the Era I run/shop spine:

- `examples/run-shop-v1/fixtures/escalating.win.json` demonstrates a bounded escalating run with quota curve `40 -> 64 -> 84`, terminal `won`, total score `84`, and final gold `47`.
- `examples/run-shop-v1/fixtures/shop.remove.json` demonstrates shop probability levers: a seeded reroll changes the draft deterministically, then removal trims the `wound` card from the seeded deck.
- `examples/run-shop-v1/demo/run-shop-demo.manifest.json` records the demo wiring and links the existing deckbuilder demo tree at `examples/deckbuilder-ui-v1/demo/demo-manifest.json`.

## Boundaries

- Deterministic fixture demo only; no network and no live browser are required.
- Rust/local owns run resolution, shop transaction validation, and deterministic evidence checks.
- Browser/Studio surfaces remain read-only inspection surfaces and receive no trusted write authority.
- The demo asserts mechanical behavior and gate-safe state only. It does not assert subjective fun, production readiness, auto-merge quality, release readiness, or Godot replacement/parity.
- Generated runs and artifacts stay untracked unless explicitly fixture-scoped under `examples/run-shop-v1/`.
- #1 and #23 remain governance anchors and must stay open.

## Reproduce

```bash
cargo test -p ouroforge-core run_shop_demo --jobs 2
```

The broader issue verification still uses the full repository gate from #1808.
