# Scenario Coverage v9: Gameplay Logic Regression Suite

This directory contains source-like regression fixtures for #624. The suite proves
structured gameplay logic behavior independently from the Gameplay Logic Demo v1
fixture while preserving the same safety boundary: data-first behavior authoring,
Rust/local validation, and read-only exported evidence inspection.

It is not arbitrary executable scripting, a plugin runtime, a command bridge, a
hosted/cloud feature, a production-stable scripting API, production-ready engine
maturity, native export, or a Godot replacement claim.

## GL10.14.1 behavior model/runtime regressions

`behavior-model-runtime-smoke.test.cjs`,
`behaviors/gameplay-logic-regression-v9.behavior.json`, and
`scenarios/gameplay-logic-regression-v9.behavior-assertions.json` cover:

- behavior artifact validation shape and Rust/local validation authority;
- event ordering and signal routing (`platePressed` before `gateSignal`);
- state-machine transitions for pressure-plate and hazard states;
- ability, cooldown, movement, audio, animation, damage, and terminal outcomes;
- no-arbitrary-script/no-command/no-plugin-loader action boundaries;
- generated-state hygiene for `runs/`, `dashboard-data/`, `target/`, and `tmp/`.

Run:

```bash
node examples/gameplay-logic-regression-v9/behavior-model-runtime-smoke.test.cjs
cargo test -p ouroforge-core --test behavior_runtime_contract gameplay_logic_regression_v9_behavior_model_runtime_fixture_covers_gl10_14_1
```

Later #624 PR units should add draft/apply/evidence regression fixtures and then
the coverage matrix/read-model compatibility gate. Generated behavior drafts,
review/apply outputs, runs, dashboard exports, screenshots, temp files, and local
tool state remain untracked unless explicitly fixture-scoped.

## GL10.14.2 draft/apply/evidence regressions

`draft-apply-evidence-smoke.test.cjs`, the `drafts/`, `applies/`,
`evidence/`, and `journal/` fixtures cover behavior draft validation,
review-gated apply transaction readiness, rollback metadata, rerun comparison,
stale evidence visibility, and read-only lifecycle journal evidence.

Run:

```bash
node examples/gameplay-logic-regression-v9/draft-apply-evidence-smoke.test.cjs
cargo test -p ouroforge-core --test behavior_runtime_contract gameplay_logic_regression_v9_draft_apply_fixtures_cover_gl10_14_2
cargo test -p ouroforge-core --test behavior_evidence_bundle gameplay_logic_regression_v9_evidence_bundle_covers_gl10_14_2_lifecycle_refs
```
