# Loop Coverage Metric v1 Demo

Issue context: #1463 and the Loop Coverage Metric v1 chain.

This demo is fixture-scoped and local-only. It shows how `loop-coverage-metric-v1` evidence can be read from committed JSON fixtures and displayed by read-only surfaces. It does not create trusted evidence, mutate source, run a worker, launch a command bridge, or promote any result.

## Fixture Set

The demo fixtures live under `examples/loop-coverage-v1/fixtures/`:

- `baseline-loop-covered.json` records a stable baseline for comparison.
- `computed-current.json` records a non-regressed computed state.
- `manual-drop-regressed.json` records a current state where manual artifacts reduce the loop-covered fraction below the baseline threshold.
- `insufficient-no-baseline.json` records supported inputs without a usable baseline.
- `stale-ref.json` records a stale attribution reference and therefore remains insufficient.
- `unsupported-kind.json` records an artifact kind outside the metric contract.

The demo smoke test reads only those fixtures. It is offline, deterministic, and fixture-scoped.

## Boundary

Loop coverage is a descriptive metric only. It describes authorship and verification coverage for trusted artifacts; it is not a quality guarantee, no production-ready status is implied, and no Godot replacement claim is made.

Dashboard and Studio surfaces are read-only. They may inspect already-exported JSON, but they must not compute trusted attribution, write files, mutate trusted state, provide mutation controls, use no auto-apply path, and use no auto-merge path.

#1 and #23 remain open. This demo does not close, modify, or replace either governance anchor.
