# Scenario Coverage v8: 3D Capability Regression Suite

This directory contains source-like regression coverage fixtures for #606. The
suite proves bounded local 3D capability behavior independently from the demo
scene while preserving existing 2D compatibility and generated-state boundaries.
It is not production 3D readiness, a production-ready engine, broad 3D
compatibility, a secure sandbox, native export, plugin runtime, hosted/cloud
behavior, autonomous launch behavior, or a Godot replacement claim.

## 3D9.11.1 core 3D feature regressions

`core-3d-smoke.test.cjs` and `scenes/core-3d-regression.scene.json` cover the
focused transform hierarchy, active camera/projection, mesh/material refs,
primitive render smoke, and collision/trigger evidence paths.

Run:

```bash
node examples/3d-capability-regression-v8/core-3d-smoke.test.cjs
```

## 3D9.11.2 probe/evaluator/animation regressions

`probe-animation-evaluator-smoke.test.cjs` and
`scenes/probe-animation-regression.scene.json` cover the bounded 3D runtime probe,
transform animation playback, evaluator pass/fail behavior, and malformed 3D
fixture rejection. The invalid fixtures under `invalid/` are source-like negative
inputs; generated verdict JSON is written only to a temporary directory and then
removed.

Run:

```bash
node examples/3d-capability-regression-v8/probe-animation-evaluator-smoke.test.cjs
```

## 3D9.11.3 coverage matrix and read-model compatibility

`coverage-matrix.json` records each scoped 3D feature area, source-like evidence
refs, and known gaps. `coverage-read-model-compatibility.test.cjs` verifies the
matrix, dashboard/Studio read-model rendering for present/missing/malformed 3D
fields, escaped display behavior, and a legacy 2D runtime compatibility audit.

Run:

```bash
node examples/3d-capability-regression-v8/coverage-read-model-compatibility.test.cjs
```

Generated `runs/`, `dashboard-data/`, `target/`, `tmp/`, screenshots, previews,
and local tool state remain untracked. Rust/local validation owns trusted
persistence, source-like fixture validation, generated evidence writing, and CLI
contracts. Browser/dashboard/Studio surfaces may inspect exported evidence only;
they do not write trusted state, execute commands, auto-apply, auto-merge, bridge
to local servers, or self-approve.
