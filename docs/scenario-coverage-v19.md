# Scenario Coverage v19: Evaluator Depth Regression Suite

Issue: #1288  
Scope: Evaluator Depth v1 (#1279) under #1 Milestone 4.1.  
Status: fixture-scoped deterministic regression coverage; not generated run output, browser automation, subjective quality scoring, release-readiness evidence, or engine-replacement evidence.

Scenario Coverage v19 locks the four-gate evaluator depth behavior with a deterministic matrix under `examples/evaluator-depth-v1/scenario-coverage-v19/` and the runner `examples/evaluator-depth-v1/scenario-coverage-v19-evaluator-depth.test.cjs`.

## Matrix files

- `examples/evaluator-depth-v1/scenario-coverage-v19/matrix.json` enumerates the required visual and semantic gate states, expected evaluator classifications, fixture refs, and guardrails.
- `examples/evaluator-depth-v1/scenario-coverage-v19/legacy-two-gate-verdict.golden.json` records the legacy two-gate verdict snapshot that must remain unchanged when no visual or semantic gates are declared.
- Existing gate fixtures under `examples/evaluator-depth-v1/visual/` and `examples/evaluator-depth-v1/semantic/` remain the fixture source for Rust/local evaluator tests and the v19 Node matrix runner.
- Recorded #1287 demo verdicts under `examples/evaluator-depth-v1/demo/` are audited for the four-category verdict shape.

## Required visual gate states

| State | Fixture | Expected gate state | Notes |
| --- | --- | --- | --- |
| unchanged | `visual/visual-gate-pass-unchanged.json` | `pass` | Zero changed pixels with declared thresholds. |
| changed-over-threshold | `visual/visual-gate-fail-changed-over-threshold.json` | `fail` | Changed pixels exceed the declared mechanical threshold. |
| under-threshold | `visual/visual-gate-pass-under-threshold.json` | `pass` | Changed pixels remain within declared threshold bounds. |
| missing-baseline | `visual/visual-gate-missing-baseline.json` | `missing-baseline` | Baseline side is explicitly missing. |
| missing-screenshot | `visual/visual-gate-missing-screenshot.json` | `missing-screenshot` | Actual screenshot side is explicitly missing. |
| threshold-not-declared | `visual/invalid/visual-gate-missing-threshold.json` | `threshold-not-declared` | Comparison exists but no threshold is declared. |
| stale-ref | `visual/invalid/visual-gate-stale-ref.json` | `stale-ref` | Fixture models an unindexed/stale evidence reference. |

Additional visual regression fixtures remain covered for `dimension-mismatch` and `malformed-screenshot` so future evaluator changes do not silently weaken existing state coverage.

## Required semantic gate states

| State | Fixture | Expected gate state | Notes |
| --- | --- | --- | --- |
| pass | `semantic/semantic-gate-pass-health.json` | `pass` | `health_non_negative` succeeds against `player.safeHealth`. |
| fail-violation | `semantic/semantic-gate-fail-health.json` | `fail` | `health_non_negative` fails against negative `player.health`. |
| unsupported | `semantic/invalid/semantic-gate-unsupported-type.json` | `unsupported` | Unsupported invariant type remains visible. |
| missing-target-state | `semantic/semantic-gate-missing-target-state.json` | `missing-target-state` | Required entity target is absent from the state. |
| malformed | `semantic/invalid/semantic-gate-malformed-invariant.json` | `malformed-invariant` | Malformed invariant shape is classified instead of hidden. |
| unsafe-expression | `semantic/invalid/semantic-gate-unsafe-expression.json` | `unsafe-expression` | Unsafe expression-like field is rejected as fixture data. |
| stale-ref | `semantic/invalid/semantic-gate-stale-ref.json` | `stale-ref` | Stale run reference remains visible. |

## Verdict shape and compatibility

Coverage asserts the four sibling categories:

- `mechanical`
- `runtime`
- `visual`
- `semantic`

The aggregation policy stays `declared-gate-and`, and undeclared visual/semantic gates remain neutral. The legacy two-gate verdict golden remains unchanged: it omits `gateCategories`, `visual`, and `semantic` when no visual or semantic gate is declared.

## Reproduction

Run:

```bash
node examples/evaluator-depth-v1/scenario-coverage-v19-evaluator-depth.test.cjs
```

The runner is deterministic and fixture-scoped. It requires no network access, live browser, timing assertions, local server, generated run directory, or screenshot capture.

## Boundaries

Scenario Coverage v19 asserts gate states and verdict shape only. It does not assert fun, aesthetics, release readiness, shipped-game quality, compatibility-stable engine behavior, engine-replacement status, source mutation authority, browser trusted writes, command bridges, auto-fix, auto-apply, auto-merge, self-approval, or reviewer bypass.

Generated runs and artifacts remain ignored unless a future issue explicitly scopes them as deterministic checked-in fixtures. #1 and #23 remain open governance anchors.
