# Deckbuilder UI Demo v1

Issue: #1829 — Deckbuilder UI Demo v1  
Anchor: #1 Era I Milestone 52 (Deckbuilder UI surface)

This fixture-scoped demo shows the full deckbuilder UI over a deterministic sample run: hand, pipeline, shop offers, run-map choices, and score cascade display. It reuses the existing JavaScript runtime UI and `window.__OUROFORGE__` probe surface. It is not a new engine, renderer, UI framework, browser command bridge, trusted write path, release gate, or fun/quality verdict.

## Demo assets

- `examples/deckbuilder-ui-v1/demo/demo-manifest.json` — expected state/probe shape and conservative governance boundaries.
- `examples/deckbuilder-ui-v1/demo/deckbuilder-ui-demo-scene-v1.json` — fixture-scoped deterministic scene/run.
- `examples/deckbuilder-ui-v1/demo/demo-smoke.test.cjs` — Node smoke that loads the existing runtime with network disabled and no live browser.

## Reproduction

```bash
node examples/deckbuilder-ui-v1/demo/demo-smoke.test.cjs
```

The smoke asserts deterministic probe-observable state for:

- hand card ids and play pipeline slots;
- shop offers, including unavailable choices;
- run-map nodes and draft path planning;
- score cascade phase order, formatted final score, and tooltips;
- read-only/draft-only interaction proposals with `trustedWrite: false`.

## Boundaries

Browser and Studio surfaces remain read-only/draft-only for this demo. Trusted writes stay on the existing Rust/local review/apply/trust-gradient path. Generated runs/artifacts remain untracked unless explicitly fixture-scoped. This demo asserts mechanical UI/probe behavior only; it does not claim production readiness, shippability, quality, fun, market demand, autonomous release, or Godot replacement/parity. #1 and #23 remain open governance anchors.
