# Localization Demo v1

Issue: #1834
Anchor: #1 Era I Milestone 53

This deterministic demo shows a Deckbuilder UI title localized through the
Localization Pipeline v1 catalog and validates the gate state for both a complete
locale and a rejected incomplete locale.

Fixtures:

- `examples/localization-v1/demo/demo-manifest.json` — demo expectations and
  governance boundary.
- `examples/localization-v1/string-catalog.complete.fixture.json` — externalized
  source strings.
- `examples/localization-v1/locale.es.fixture.json` — complete `es-ES` locale
  proposal; the title resolves to `IU de cartas / mano / canalización`.
- `examples/localization-v1/invalid/locale.missing.fixture.json` — rejected
  incomplete `fr-FR` locale.

Run:

```bash
cargo test -p ouroforge-core --test localization_demo_contract
```

The demo is local and deterministic: no network, no live browser, no generated
untracked output, and no subjective fun/quality assertion. Rust/local owns the
completeness and placeholder validation. Browser/Studio surfaces remain read-only
or draft-only, and generated localization text remains proposal-only until the
existing review/apply/trust-gradient path accepts it. This is not a
production-ready, quality, fun, shippable, release, market, or Godot
replacement/parity claim. #1 and #23 remain open.
