# Card-Roguelite Substrate v1 Demo

Issue: #1795, under Card-Roguelite Substrate v1 (#1791) and #1 Era I Milestone 47.

This demo is a fixture-scoped, Rust-local smoke of two configurations running over one substrate:

- `deck-roguelike-parity` loads the migrated deck-roguelike golden config from `examples/card-roguelite-substrate-v1/parity/deck-roguelike-classic.substrate.golden.json`.
- `engine-builder-deckbuilder` loads the engine-builder deckbuilder config from `examples/card-roguelite-substrate-v1/engine-builder/deckbuilder.config.golden.json`.

The manifest lives at `examples/card-roguelite-substrate-v1/demo/substrate-demo.manifest.json`. The smoke contract validates both configs, resolves each config twice, checks probe visibility, and confirms that the deck-roguelike demo run still matches the golden migrated config by value.

## Boundaries

- Deterministic fixture demo only; no network and no live browser are required.
- Rust/local owns validation and substrate resolution.
- Browser/Studio surfaces remain read-only inspection surfaces.
- The demo asserts mechanical behavior, parity, and determinism only. It does not assert subjective fun, production readiness, auto-merge quality, or Godot replacement/parity.
- Generated runs and artifacts stay untracked unless explicitly fixture-scoped under this demo tree.
- #1 and #23 remain governance anchors and must stay open.

## Reproduce

```bash
cargo test -p ouroforge-core card_roguelite_substrate_demo --jobs 2
```

The broader issue verification still uses the full repository gate from #1795.
