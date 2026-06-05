# Evolve Loop Depth v1 demo

Issue: #1296
Roadmap anchor: #1 Milestone 5.1, Evolve Loop Depth v1.

This fixture-scoped demo records one deterministic full-loop slice:

1. `examples/evolve-loop-depth-v1/demo/before-run/verdict.json` fails the declared `visual` gate while `mechanical`, `runtime`, and `semantic` pass.
2. `examples/evolve-loop-depth-v1/demo/before-run/mutation/proposals.json` records an evidence-linked proposal whose rationale cites the failing `visual` gate and `evidence/scenarios/visual-depth-demo/visual/visual-comparison.json`.
3. `examples/evolve-loop-depth-v1/demo/apply/operation.scene-only.json` uses the #215 `scene-only-mutation-v1` operation shape to express a bounded `components.transform.x` scene edit for `player`.
4. `examples/evolve-loop-depth-v1/demo/after-run/transactions/scene-edit-depth-demo-align-player-x.json` and `before-run/mutation/scene-applications.json` capture the manual-review scene-only apply evidence.
5. `examples/evolve-loop-depth-v1/demo/comparison/run-comparison-before-run--after-run.json` records the four-gate delta: `visual` transitions `fail_to_pass`; `mechanical`, `runtime`, and `semantic` remain `unchanged_pass`.

## Determinism boundary

The demo is offline and fixture-scoped. It does not require network access, external services or a live browser to reproduce the recorded artifacts. Placeholder image files are committed only as tiny fixture refs so the visual evidence paths resolve deterministically from a fresh clone.

The seed is `seeds/evolve-loop-depth-v1-demo.yaml`. The tracked fixtures are intentionally small source-like examples for this issue, not generated local run output. New run directories, proposals, comparison artifacts, screenshots, and dashboard exports produced during local experimentation remain generated state and should stay ignored unless a later issue explicitly promotes a bounded fixture.

## Trusted ownership and reuse

The trusted verdict, proposal rationale, scene-only apply contract, and four-gate comparison shapes are owned by Rust/local contracts from Evolve Loop Depth v1:

- evidence-linked proposals deepen `evolve_run` and `MutationProposalRationale`;
- bounded apply reuses the #215 `mutation apply-scene` / `scene-only-mutation-v1` path;
- rerun comparison reuses the existing `compare_runs`/`write_run_comparison_artifact` four-gate model;
- this demo adds no proposal engine, apply engine, comparison engine, browser write path, command bridge, self-approval path, unattended trusted-write path, unattended repair path, or unattended integration path.

## Smoke test

Run from the repository root:

```bash
node examples/evolve-loop-depth-v1/demo/demo-smoke.test.cjs
```

The smoke test asserts that the fixture proposal cites the failing visual gate, the scene-only apply operation is bounded to an allowed scene path, the transaction/application record links back to the proposal, the before/after four-gate comparison contains `visual: fail_to_pass`, all fixture evidence refs exist, and public wording remains conservative.

## Conservative wording

This is a mechanics demonstration only. It does not claim subjective gameplay quality, release maturity, engine-substitution status, autonomous repair, unattended trusted writes, unattended integration, extension distribution, platform packaging, or browser-side trusted writes. Manual review remains the boundary before any scene mutation is accepted in real workflows.
