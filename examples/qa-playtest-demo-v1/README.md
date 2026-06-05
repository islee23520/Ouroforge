# Autonomous QA Playtest Demo v1 fixtures

Issue: #696 — Autonomous QA Playtest Demo v1.

`demo.manifest.json` is validated by `ouroforge_core::qa_playtest_demo`. It wires
the full bounded QA/playtest pipeline on a fixture project. Outputs are evidence
and backlog inputs only; there is no auto-fix, auto-apply, or hidden worker.

Generated runs/fuzz/screenshots/bundle exports are written only under
`runs/qa-playtest-demo/...` and stay untracked.

## Valid fixture

- `demo.manifest.json` — bounded fuzz/worker budgets, disjoint output roots, all
  thirteen stages present, explicit cleanup policy and known gaps.

## Invalid fixtures (fail closed)

- `invalid/demo.unbounded-fuzz.manifest.json`
- `invalid/demo.unbounded-worker.manifest.json`
- `invalid/demo.overlapping-outputs.manifest.json`
- `invalid/demo.missing-cleanup.manifest.json`
- `invalid/demo.missing-known-gaps.manifest.json`
- `invalid/demo.missing-stage.manifest.json`
- `invalid/demo.unsafe-boundary.manifest.json`
