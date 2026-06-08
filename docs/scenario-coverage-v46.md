# Scenario Coverage v46: Game-Feel and Juice Regression Suite

Issue: #1823  
Anchor: #1 Era I Milestone 51 (Game feel and juice)

Scenario Coverage v46 locks the Game-Feel and Juice Toolkit v1 with local,
deterministic state/shape checks only. It covers juice primitive declarations,
score-cascade feedback ordering, responsiveness pass/fail evidence, the
fixture-scoped Game-Feel and Juice demo, and an existing runtime feedback backward-compatibility golden.

The suite does not run a live browser, use the network, assert wall-clock timing,
mutate trusted sources, auto-apply fixes, auto-merge, self-approve, or claim fun,
quality, production readiness, shippability, or Godot replacement/parity.
Browser/Studio surfaces remain read-only. Generated runs/artifacts remain untracked unless fixture-scoped. Issues #1 and #23 remain open.

## Matrix

`examples/game-feel-juice-v1/scenario-coverage-v46/matrix.fixture.json`
enumerates these rows:

| Row | Surface | Expected state/shape |
| --- | --- | --- |
| `V46.juice.primitives` | #1819 runtime juice primitives | four deterministic primitive declarations: tween, shake, hit-stop, and SFX. |
| `V46.cascade.order` | #1820 score-cascade payoff feedback | ordered feedback sequence with `score_cascade` trigger and final score `24`. |
| `V46.responsiveness.pass` | #1821 responsiveness verifier | `responsiveness-within-budget` reports `80ms` and passes the `100ms` budget. |
| `V46.responsiveness.fail` | #1821 responsiveness verifier | `responsiveness-over-budget` reports `112ms` and fails the `100ms` budget. |
| `V46.demo.smoke` | #1822 demo | demo manifest references existing deterministic fixtures and no live browser dependency. |
| `V46.runtime.backcompat` | existing runtime feedback | `juice.js`, `runtime.juice.feedback`, feedback updates, and the read-only probe wiring remain present. |

## Reproducibility

Run:

```bash
cargo test -p ouroforge-core --test scenario_coverage_v46_game_feel_juice
```

The runner recomputes Rust/local cascade and responsiveness states from tracked
fixtures and inspects the JavaScript/runtime source shape for backward
compatibility. It intentionally avoids flaky timing assertions and subjective
feel/fun judgments.
