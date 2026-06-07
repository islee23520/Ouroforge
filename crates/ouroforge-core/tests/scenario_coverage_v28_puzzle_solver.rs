//! Scenario Coverage v28: Solver and Over-Solution Regression Suite (#1585).
//!
//! Locks Puzzle Solver and Over-Solution Detection v1 behavior with a
//! deterministic, fixture-scoped regression matrix. It enumerates solver
//! verdicts (solvable / unsolvable / bounded-search exhaustion), over-solution
//! detector verdicts (over-solution / clean / missing-intent fail-closed),
//! difficulty-metric computation (known level / stale evidence fail-closed), and
//! the design-integrity gate verdicts, plus a backward-compatibility golden
//! proving the existing four-gate aggregation is unchanged.
//!
//! Asserts states and shapes only — no flaky or timing-based assertions. The
//! suite reuses the level and evidence fixtures committed by #1580/#1581/#1582
//! and the existing test harness; it is a regression suite, not a new engine.

use std::fs;
use std::path::PathBuf;

use ouroforge_core::puzzle_difficulty_metric;
use ouroforge_core::puzzle_oversolution;
use ouroforge_core::puzzle_solver::{self, SolveBudget, SolveOutcome};
use ouroforge_evaluator::{evaluation_gate_categories, VisualGateState, VisualGateVerdict};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn matrix_root() -> PathBuf {
    repo_root().join("examples/puzzle-solver-oversolution-v1/scenario-coverage-v28")
}

fn read_json(path: PathBuf) -> Value {
    serde_json::from_str(
        &fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}")),
    )
    .unwrap_or_else(|error| panic!("parse {path:?}: {error}"))
}

fn matrix() -> Value {
    read_json(matrix_root().join("matrix.fixture.json"))
}

/// Read the `gridPuzzle` spec from a repo-relative scene fixture path.
fn spec_at(relative: &str) -> Value {
    read_json(repo_root().join(relative))["gridPuzzle"].clone()
}

/// Resolve a case's intent: either an inline object or `"fromScene"` (the
/// scene's own `intent` block).
fn intent_for(case: &Value) -> Value {
    match &case["intent"] {
        Value::String(s) if s == "fromScene" => {
            let scene = read_json(repo_root().join(case["spec"].as_str().unwrap()));
            scene["intent"].clone()
        }
        other => other.clone(),
    }
}

/// The design-integrity gate verdict per the scope contract (#1579): pass only
/// when intent is captured and satisfied AND no over-solution exists.
fn design_integrity_passes(spec: &Value, intent: &Value) -> bool {
    match puzzle_oversolution::detect_oversolutions(spec, intent, SolveBudget::default()) {
        Ok(report) => !report.has_oversolution(),
        Err(_) => false,
    }
}

#[test]
fn v28_matrix_header_is_pinned() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v28-puzzle-solver-matrix-v1"
    );
    assert_eq!(matrix["issue"], 1585);
}

#[test]
fn v28_solver_verdicts_are_enumerated() {
    for case in matrix()["solverCases"].as_array().expect("solver cases") {
        let id = case["id"].as_str().unwrap();
        let spec = spec_at(case["spec"].as_str().unwrap());
        let budget = SolveBudget {
            max_states: case["maxStates"].as_u64().unwrap() as usize,
        };
        let outcome = puzzle_solver::solve(&spec, budget).expect("valid spec");
        match case["expected"].as_str().unwrap() {
            "solvable" => {
                let SolveOutcome::Solvable { witness, .. } = &outcome else {
                    panic!("{id}: expected Solvable, got {outcome:?}");
                };
                assert!(
                    puzzle_solver::replay(&spec, witness).unwrap().is_won(),
                    "{id}: witness must replay to win"
                );
            }
            "unsolvable" => assert!(
                matches!(outcome, SolveOutcome::Unsolvable { .. }),
                "{id}: expected Unsolvable, got {outcome:?}"
            ),
            "exhausted" => assert!(
                matches!(outcome, SolveOutcome::Exhausted { .. }),
                "{id}: expected Exhausted, got {outcome:?}"
            ),
            other => panic!("{id}: unknown expected {other}"),
        }
    }
}

#[test]
fn v28_detector_verdicts_are_enumerated() {
    for case in matrix()["detectorCases"]
        .as_array()
        .expect("detector cases")
    {
        let id = case["id"].as_str().unwrap();
        let spec = spec_at(case["spec"].as_str().unwrap());
        let intent = intent_for(case);
        let result =
            puzzle_oversolution::detect_oversolutions(&spec, &intent, SolveBudget::default());
        match case["expected"].as_str().unwrap() {
            "over-solution" => {
                let report = result.expect("valid spec + intent");
                assert!(report.has_oversolution(), "{id}: expected an over-solution");
                let bypass = &report.counterexamples[0];
                assert!(
                    puzzle_solver::replay(&spec, &bypass.trace)
                        .unwrap()
                        .is_won(),
                    "{id}: counterexample must replay to win"
                );
            }
            "clean" => {
                let report = result.expect("valid spec + intent");
                assert!(
                    !report.has_oversolution(),
                    "{id}: expected no over-solution, got {:?}",
                    report.counterexamples
                );
            }
            "fail-closed" => {
                assert!(result.is_err(), "{id}: missing intent must fail closed");
            }
            other => panic!("{id}: unknown expected {other}"),
        }
    }
}

#[test]
fn v28_difficulty_metrics_are_enumerated() {
    for case in matrix()["difficultyCases"]
        .as_array()
        .expect("difficulty cases")
    {
        let id = case["id"].as_str().unwrap();
        let spec = spec_at(case["spec"].as_str().unwrap());
        let evidence = read_json(repo_root().join(case["evidence"].as_str().unwrap()));
        let result =
            puzzle_difficulty_metric::compute_difficulty(&spec, &evidence, SolveBudget::default());
        match case["expected"].as_str().unwrap() {
            "computed" => {
                let metric = result.expect("valid evidence");
                assert_eq!(
                    metric.solution_length,
                    case["solutionLength"].as_u64().unwrap() as usize,
                    "{id}: solution length"
                );
                let expected_order: Vec<String> = case["mechanicOrder"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|m| m.as_str().unwrap().to_string())
                    .collect();
                assert_eq!(
                    metric.mechanic_introduction_order, expected_order,
                    "{id}: mechanic order"
                );
                assert!(metric.reachable_states > 0, "{id}: reachable states");
            }
            "fail-closed" => assert!(result.is_err(), "{id}: stale evidence must fail closed"),
            other => panic!("{id}: unknown expected {other}"),
        }
    }
}

#[test]
fn v28_design_integrity_gate_verdicts_are_enumerated() {
    for case in matrix()["gateCases"].as_array().expect("gate cases") {
        let id = case["id"].as_str().unwrap();
        let spec = spec_at(case["spec"].as_str().unwrap());
        let intent = intent_for(case);
        let passes = design_integrity_passes(&spec, &intent);
        match case["expected"].as_str().unwrap() {
            "pass" => assert!(passes, "{id}: gate must pass"),
            "fail" => assert!(!passes, "{id}: gate must fail"),
            other => panic!("{id}: unknown expected {other}"),
        }
    }
}

#[test]
fn v28_four_gate_aggregation_is_backward_compatible() {
    // The existing declared-gate-and four-gate aggregation must be unchanged by
    // the Milestone 28 work: compute it for a passing visual gate and compare to
    // the committed golden (order-independent value equality).
    let visual = vec![VisualGateVerdict {
        scenario_id: "scene-1".to_string(),
        checkpoint_id: "cp-1".to_string(),
        state: VisualGateState::Pass,
        reason: "ok".to_string(),
        comparison_ref: "compare/cp-1.json".to_string(),
        changed_pixels: Some(0),
        changed_percent_x1000: Some(0),
        changed_region_count: 0,
        threshold_summary: vec![],
        evidence_refs: vec!["evidence/visual/cp-1.json".to_string()],
        output_root: "runs/run-1".to_string(),
    }];
    let categories =
        evaluation_gate_categories(1, 0, &[], &visual, &[]).expect("four-gate categories present");

    let golden_ref = matrix()["compatibility"]["fourGateGolden"]
        .as_str()
        .unwrap()
        .to_string();
    let golden = read_json(matrix_root().join(golden_ref));
    assert_eq!(
        categories, golden,
        "the four-gate aggregation must remain backward-compatible"
    );
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
    assert_eq!(categories["aggregation"]["undeclaredGatePolicy"], "neutral");
}

#[test]
fn v28_doc_records_the_coverage_suite() {
    let doc = fs::read_to_string(repo_root().join("docs/scenario-coverage-v28.md"))
        .expect("v28 doc exists");
    assert!(doc.contains("#1585"), "doc cites the issue");
    assert!(
        doc.contains("declared-gate-and"),
        "doc records the backward-compat aggregation"
    );
}
