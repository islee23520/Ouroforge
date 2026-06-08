# Scoring-Engine Demo v1

Status: deterministic fixture-scoped demo evidence for issue #1802 under Era I Milestone 48.

This demo shows readable card modifiers composing into a large deterministic score while preserving the existing card-roguelite substrate and engine-builder deckbuilder config shape. It is mechanical scoring evidence only: it does not score fun, quality, release readiness, market demand, or engine parity.

## Fixtures

The demo manifest is `examples/scoring-engine-v1/demo/demo.manifest.json` and references two local configs:

- `readable-composed-score.config.json` — a base score of 10 receives a readable `+5` tuning modifier and a `×2` overdrive modifier, producing a composed score of 30 without a degenerate finding.
- `degenerate-combo.config.json` — the same readable pieces add `reactor-loop` (`×3`) and replay seed `5803`, producing `(10 + 5) × 2 × 3 = 90` and a reproducible degenerate combo finding.

The combo remains deterministic by replaying the same fixture twice and comparing the composition digest. No network, live browser, cloud service, generated run directory, or Studio write path is required.

## Reproduction

```bash
cargo test -p ouroforge-core --test scoring_engine_demo_contract --jobs 2
```

The smoke test validates the manifest, recomputes both scores through Rust/local composition analysis, asserts the documented modifier order and degenerate state, and verifies replay digest stability for the degenerate combo.

## Boundaries

- Rust/local owns scoring validation, composition analysis, replay digesting, and fixture verification.
- Browser, dashboard, cockpit, and Studio surfaces remain read-only inspection surfaces.
- No direct trusted writes, hidden trusted writes, auto-apply, auto-merge, self-approval, reviewer bypass, or release action are introduced.
- Generated runs/artifacts remain untracked unless fixture-scoped.
- The fun/feel verdict remains the human Era J gate.
- Steam account creation, signing, release buttons, hosted/cloud/mobile Layer-3 work, and market demand are out of scope.
- Issues #1 and #23 remain open governance anchors.
