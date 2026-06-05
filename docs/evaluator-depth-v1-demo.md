# Evaluator Depth v1 demo

Issue: #1287

This fixture-scoped demo records two deterministic four-gate evaluator outcomes for Milestone 4.1. It demonstrates gate mechanics only: the fixtures do not claim fun, aesthetic quality, production readiness, current Godot replacement, source mutation authority, or any trusted browser action.

## Fixtures

- Seed: `seeds/evaluator-depth-v1-demo.yaml`
- Visual-fail recorded run: `examples/evaluator-depth-v1/demo/visual-fail-run/`
- Semantic-fail recorded run: `examples/evaluator-depth-v1/demo/semantic-fail-run/`
- Smoke test: `examples/evaluator-depth-v1/demo/demo-smoke.test.cjs`

The demo is reproducible from committed fixture JSON and tiny fixture screenshots. It requires no network access and no live browser to inspect the recorded verdicts.

## Expected verdicts

### Pure visual failure

`visual-fail-run/verdict.json` records:

- mechanical gate: `pass`
- runtime gate: `pass`
- visual gate: `fail`
- semantic gate: `pass`

The failure is evidence-linked to `evidence/scenarios/visual-mismatch/visual/visual-comparison.json`, with baseline and actual screenshot refs under the same fixture directory. The changed region is a one-pixel deterministic mismatch over a zero-pixel demo threshold.

### Pure semantic failure

`semantic-fail-run/verdict.json` records:

- mechanical gate: `pass`
- runtime gate: `pass`
- visual gate: `pass`
- semantic gate: `fail`

The failure is evidence-linked to `evidence/scenarios/semantic-invariant/semantic/runtime-invariant-model.json` and `evidence/scenarios/semantic-invariant/world-state.json`. The semantic invariant is intentionally narrow: `health-non-negative` fails because `player.health` is `-1`.

## Read-only and generated-state boundary

The committed files are fixture-scoped demo evidence. They are not generated run output under `runs/`, and they do not authorize browser writes, command bridges, auto-fix, auto-apply, auto-merge, self-approval, reviewer bypass, or trusted mutation.

Generated runs, screenshots, and diff artifacts outside this fixture remain ignored/untracked unless a future issue explicitly scopes them as fixtures.

## Verification

Run:

```bash
node examples/evaluator-depth-v1/demo/demo-smoke.test.cjs
```

The smoke test asserts both deterministic failures, evidence refs, and the four-category verdict shape.

Governance anchors #1 and #23 remain open.
