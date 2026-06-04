# Production 2D Regression Coverage v7

This directory contains source-like regression coverage fixtures for #592. The
suite isolates Production 2D behavior that would otherwise be hidden inside the
integrated collect-and-exit demo. It is intentionally local-first and does not
commit generated `runs/`, dashboard exports, screenshots, temp projects, or local
tool state.

## P2D8.12.1 runtime feature scenarios

`runtime-feature-regressions.test.cjs` covers renderer ordering, camera
follow/parallax/clamp evidence, collision/trigger outcomes, input action mapping,
save/load, snapshot restore, and replay divergence using existing runtime
fixtures. `scenarios/runtime-feature-regressions.json` documents the bounded
scenario paths as source-like metadata for the smoke.

Run:

```bash
node examples/production-2d-regression-v7/runtime-feature-regressions.test.cjs
```

## P2D8.12.2 media and performance scenarios

`media-performance-regressions.test.cjs` covers animation state changes, VFX
intent events, audio intent evidence, and runtime frame-budget/profiler evidence
using existing runtime fixtures. `scenarios/media-performance-regressions.json`
documents the bounded media/performance scenario paths as source-like metadata for
the smoke.

Run:

```bash
node examples/production-2d-regression-v7/media-performance-regressions.test.cjs
```

## P2D8.12.3 coverage matrix and read-model compatibility

`coverage-read-model-compatibility.test.cjs` verifies the coverage matrix and
checks dashboard/Studio compatibility for present, missing, and malformed
Production 2D evidence fields. `coverage-matrix.json` records each scoped feature
area, the source-like evidence paths, and known gaps without tracking generated
run output.

Run:

```bash
node examples/production-2d-regression-v7/coverage-read-model-compatibility.test.cjs
```

This is not a shipped-game, production-ready engine, Godot replacement, native
export, hosted service, plugin runtime, or browser-trusted mutation workflow.
