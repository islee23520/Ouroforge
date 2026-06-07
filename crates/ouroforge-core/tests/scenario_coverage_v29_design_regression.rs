//! Scenario Coverage v29 — Design Regression Harness regression suite (#1590).
//!
//! CI-gated mirror of `examples/design-regression-harness-v1/scenario-coverage-v29-design-regression.test.cjs`.
//! Enumerates harness classification, replayable trace linkage, stale baseline fail-closed,
//! and backward compatibility with single-run solver (#1580).

use std::path::{Path, PathBuf};

use ouroforge_core::design_regression_harness::{DesignRegressionHarness, RegressionOutcome};
use ouroforge_core::puzzle_solver::{self, SolveBudget, SolveOutcome};
use serde_json::Value;

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_json(relative: &str) -> Value {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

const MATRIX: &str =
    "examples/design-regression-harness-v1/scenario-coverage-v29/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v29.md";

fn load_harness(relative: &str) -> DesignRegressionHarness {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("harness exists");
    DesignRegressionHarness::from_json_str(&text).expect("valid harness")
}

fn outcome_from_str(s: &str) -> RegressionOutcome {
    match s {
        "newly-broken" => RegressionOutcome::NewlyBroken,
        "improved" => RegressionOutcome::Improved,
        "unchanged" => RegressionOutcome::Unchanged,
        "inconclusive" => RegressionOutcome::Inconclusive,
        other => panic!("unknown outcome {other}"),
    }
}

#[test]
fn v29_matrix_header_is_pinned() {
    let matrix = read_json(MATRIX);
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v29-design-regression-matrix-v1"
    );
    assert_eq!(matrix["issue"], 1590);
}

#[test]
fn v29_harness_classifications_are_enumerated() {
    let matrix = read_json(MATRIX);
    for case in matrix["harnessCases"].as_array().expect("cases") {
        let id = case["id"].as_str().unwrap();
        let harness_path = case["harness"].as_str().unwrap();
        let level_id = case["levelId"].as_str().unwrap();
        let expected = outcome_from_str(case["expectedOutcome"].as_str().unwrap());
        let harness = load_harness(harness_path);
        let report = harness.run().expect("harness runs");
        if let Some(verdict) = case.get("overallVerdict") {
            assert_eq!(report.overall_verdict, verdict.as_str().unwrap(), "{id}: verdict");
        }
        if case.get("promotionBlocked") == Some(&Value::Bool(true)) {
            assert!(report.promotion_blocked(), "{id}: promotion blocked");
        }
        let level = report
            .levels
            .iter()
            .find(|l| l.level_id == level_id)
            .unwrap_or_else(|| panic!("{id}: level {level_id}"));
        assert_eq!(level.outcome, expected, "{id}: outcome");
        let expects_trace = case["expectsTrace"].as_bool().unwrap();
        if expects_trace {
            let trace = level.trace.as_ref().expect("{id}: trace");
            let expected_trace: Vec<String> = case["trace"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
            assert_eq!(trace, &expected_trace, "{id}: trace golden");
            let spec = harness
                .levels
                .iter()
                .find(|l| l.level_id == level_id)
                .expect("spec")
                .spec
                .clone();
            assert!(
                puzzle_solver::replay(&spec, trace).expect("replays").is_won(),
                "{id}: trace wins"
            );
        } else {
            assert!(level.trace.is_none(), "{id}: no trace");
        }
    }
}

#[test]
fn v29_single_run_solver_remains_valid_outside_harness() {
    let matrix = read_json(MATRIX);
    let bc = &matrix["backwardCompat"];
    let spec_path = bc["spec"].as_str().unwrap();
    let spec = read_json(spec_path)["gridPuzzle"].clone();
    let budget = SolveBudget {
        max_states: bc["maxStates"].as_u64().unwrap() as usize,
    };
    let outcome = puzzle_solver::solve(&spec, budget).expect("valid spec");
    assert!(
        matches!(outcome, SolveOutcome::Solvable { .. }),
        "single-run solver still solvable outside harness"
    );
}

#[test]
fn v29_doc_preserves_governance_and_boundary() {
    let doc = std::fs::read_to_string(workspace_path(DOC)).expect("doc");
    assert!(doc.contains("#1") && doc.contains("#23"));
    let lowered = doc.to_lowercase();
    assert!(lowered.contains("fixture-scoped") && lowered.contains("read-only"));
}
