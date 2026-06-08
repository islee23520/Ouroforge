# Scenario Coverage v47: Deckbuilder UI Regression Suite

Issue: #1830  
Anchor: #1 Era I Milestone 52 (Deckbuilder UI Kit v1)

Scenario Coverage v47 locks the Deckbuilder UI Kit v1 with local,
deterministic state/shape checks only. It covers the hand/pipeline UI, the
shop/run-map UI, score-cascade display state, the deterministic demo smoke
shape, and an existing runtime UI/probe backward-compatibility golden.

The suite does not run a live browser, use the network, assert wall-clock timing,
mutate trusted sources, auto-apply fixes, auto-merge, self-approve, or claim fun,
quality, production readiness, shippability, or Godot replacement/parity.
Browser/Studio surfaces remain read-only/draft-only. Rust/local remains the
trusted authority for validation, scoring, persistence, and review/apply/trust-gradient writes. Generated runs/artifacts remain untracked unless fixture-scoped. Issues #1 and #23 remain open.

## Matrix

`examples/deckbuilder-ui-v1/scenario-coverage-v47/matrix.fixture.json`
enumerates these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V47.hand.pipeline` | #1826 card hand and pipeline UI | five hand cards from the existing deck-roguelike hand and three draft-only pipeline slots: `intent`, `modifier`, `commit`. |
| `V47.shop.run_map` | #1827 shop and run-map UI | two shop offers with `insufficient gold` for the unavailable offer; run map nodes `start`, `shop`, `elite`, `boss` with the elite path blocked. |
| `V47.score.cascade_display` | #1828 score-cascade display | ordered score phases, formatted final score, read-only evidence flags, and Rust/local authority boundary. |
| `V47.demo.smoke` | #1829 deterministic demo | fixture-scoped manifest, disabled network, no live browser dependency, and expected read-only probe state. |
| `V47.runtime.backcompat` | existing runtime UI/probe | `deckbuilder-ui.js`, `window.__OUROFORGE__`, draft-only probe methods, runtime event labels, and `trustedWrite: false` remain present. |
| `V47.boundary.negative_shapes` | invalid/stale UI boundaries | invalid hand index, missing slot, missing offer, unavailable offer, and blocked path fail closed without trusted mutation. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v47_deckbuilder_ui
node examples/game-runtime/deckbuilder-ui.test.cjs
node examples/deckbuilder-ui-v1/demo/demo-smoke.test.cjs
```

The runner inspects tracked fixtures, source shapes, and docs only. It avoids
flaky timing assertions and subjective feel/fun judgments. The backward-compatibility golden is a source-shape guard for the existing runtime UI/probe, not a generated browser snapshot.
