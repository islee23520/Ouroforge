# Scenario Coverage v42 — Card-Roguelite Substrate Regression Suite

Issue: #1796

Scenario Coverage v42 locks Card-Roguelite Substrate v1 behavior with state/shape regressions only. It covers substrate determinism, deck-roguelike config golden parity, engine-builder deckbuilder config parity, demo manifest shape, generated-state boundaries, and a backward-compatibility golden proving the pre-substrate deck-roguelike regression surface remains valid.

Fixtures live under `examples/card-roguelite-substrate-v1/scenario-coverage-v42/` and reference the existing substrate, parity, engine-builder, demo, and v31 deck-roguelike fixtures. The runner is Rust/local owned and requires no network, no live browser, no browser command bridge, and no trusted writes.

Browser, dashboard, and Studio surfaces may inspect read-only substrate state only. Generation remains proposal-only through the existing review/apply/trust-gradient path. Generated runs, assets, builds, coverage output, and other artifacts stay untracked unless explicitly fixture-scoped.

Conservative wording is preserved: no auto-merge, no auto-apply, self-approval, reviewer bypass, production-ready claim, Godot replacement/parity claim, autonomous shipping claim, quality guarantee, or automated fun score is introduced. The fun/feel verdict remains a human Era J gate.

Issues #1 and #23 remain open governance anchors.
