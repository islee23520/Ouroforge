# QA Swarm Regression Suite v1 (Scenario Coverage v13)

Issue: #697 — Scenario Coverage v13: QA Swarm Regression Suite.

This adds regression coverage for the QA/playtest swarm artifacts so correctness
does not hide inside one demo. It re-validates every QA/playtest artifact that is
present in `main` through its public API (valid fixtures parse, invalid fixtures
fail closed) and records a machine-checked coverage matrix
(`ouroforge_core::qa_regression_coverage`) that maps each area to where its
regression coverage lives.

## Coverage matrix

| Area | Coverage | Where |
| --- | --- | --- |
| Scenario generation | in-repo | `examples/qa-scenario-candidate-v1` (re-validated) |
| Fuzzing plan | in-repo | `examples/adversarial-input-fuzzing-v1` (re-validated) |
| Worker budget/assignment | in-repo | `examples/qa-worker-assignment-v1` (re-validated) |
| Runtime invariant | in-repo | `examples/runtime-invariant-checker-v1` (re-validated) |
| Objective route attempt | in-repo | `examples/route-attempt-evidence-v1` (re-validated) |
| Visual comparison | in-repo | `examples/visual-comparison-evidence-v1` (re-validated) |
| Studio/dashboard read-model | in-repo | `examples/evidence-dashboard` + `examples/authoring-cockpit` node smokes |
| Malformed/missing/stale/unresolved-output | in-repo | invalid fixtures re-validated across artifacts |
| Performance budget | in-repo | `examples/qa-performance-budget-v1` + focused contract tests |
| Console/crash/runtime error classifier | in-repo | `examples/qa-error-classifier-v1` + `qa_error_classifier_contract` |
| Flake/rerun policy | in-repo | `examples/qa-flake-rerun-policy-v1` + `qa_flake_rerun_policy_contract` |
| Failure classification / backlog | in-repo | `examples/qa-failure-backlog-v1` + `qa_failure_backlog_contract` |
| Run matrix | in-repo | `examples/qa-swarm-run-matrix-v1` + `qa_run_matrix_contract` |
| Evidence bundle | in-repo | `examples/qa-swarm-evidence-bundle-v1` + `qa_evidence_bundle_contract` |

## Known gaps

- Live capture (running the engine to produce evidence) is out of scope for v1;
  the suite validates deterministic fixtures, schemas, and read models rather
  than live runtime behavior.
- Visual/performance/error signals are heuristic evidence inputs, not proof of
  fun, quality, production safety, accessibility compliance, market readiness,
  or release readiness.
- Studio/dashboard compatibility is covered through read-model fixtures and the
  existing evidence-dashboard/authoring-cockpit node smokes; no browser trusted
  write path is added.

## Boundary and governance

This regression coverage matrix records where QA/playtest regression coverage
lives; outputs are evidence inputs and remain review-gated. It adds regression
coverage, not auto-fix. There are no hidden workers, no remote swarm, no
auto-fix, no auto-apply, and no auto-merge. Browser/dashboard/Studio surfaces
remain read-only or draft-only.

#1 remains the roadmap/final-goal anchor and must stay open. #23 remains the
memory/governance anchor and must stay open. Public wording stays conservative:
this is bounded evidence-gated QA exploration, not a production game, a
shipped-game claim, a current Godot replacement, or a production-ready claim.
