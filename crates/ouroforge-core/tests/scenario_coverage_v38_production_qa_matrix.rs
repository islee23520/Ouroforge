//! Scenario Coverage v38 — Production-Scale QA Matrix Regression Suite (#1671).
//!
//! Locks Production-Scale QA Matrix v1 behavior: the content x seed x target
//! regression matrix (#1666), visual-regression at scale (#1667),
//! performance/soak testing (#1668), and the consolidated production-QA verdict
//! with crash/accessibility checks (#1669), plus the backward-compatibility
//! guarantee that the existing per-artifact QA gates remain valid. State/shape
//! assertions only — no flaky or timing-based checks — so a breaking change
//! fails CI.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::performance_soak::PerformanceSoakArtifact;
use ouroforge_core::production_qa_matrix::ProductionQaMatrixArtifact;
use ouroforge_core::production_qa_verdict::ProductionQaVerdictArtifact;
use ouroforge_core::qa_performance_budget::PerformanceBudgetArtifact;
use ouroforge_core::qa_run_matrix::QaRunMatrixArtifact;
use ouroforge_core::visual_regression_scale::VisualRegressionScaleArtifact;
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn read_text(relative: &str) -> String {
    let path = repo_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn example(relative: &str) -> String {
    read_text(&format!("examples/{relative}"))
}

#[test]
fn v38_matrix_is_enumerated() {
    let matrix: Value = serde_json::from_str(&read_text(
        "examples/production-qa-matrix-v1/scenario-coverage-v38/matrix.fixture.json",
    ))
    .expect("matrix parses");
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v38");
    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 12,
        "v38 enumerates the production-scale QA matrix behaviors"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(scenario["system"].as_str().expect("system").to_string());
        assert!(scenario["kind"].is_string());
        assert!(scenario["expect"].is_string());
    }
    for system in ["matrix", "visual", "performance", "verdict", "backcompat"] {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("backcompat-per-artifact-gates"));
}

#[test]
fn v38_matrix_aggregation_and_regression() {
    let complete = ProductionQaMatrixArtifact::from_json_str(&example(
        "production-qa-matrix-v1/matrix.complete.fixture.json",
    ))
    .expect("complete matrix");
    let rm = complete.read_model();
    assert_eq!(rm.status, "complete");
    assert!(rm.coverage_complete);
    assert_eq!(rm.regression_count, 0);

    let regression = ProductionQaMatrixArtifact::from_json_str(&example(
        "production-qa-matrix-v1/matrix.regression.fixture.json",
    ))
    .expect("regression matrix");
    assert_eq!(regression.read_model().regression_count, 2);

    // Fail-closed: a malformed verdict is rejected.
    assert!(ProductionQaMatrixArtifact::from_json_str(&example(
        "production-qa-matrix-v1/invalid/matrix.malformed-verdict.fixture.json",
    ))
    .is_err());
}

#[test]
fn v38_visual_match_diff_and_missing_baseline() {
    let m = VisualRegressionScaleArtifact::from_json_str(&example(
        "visual-regression-scale-v1/suite.match.fixture.json",
    ))
    .expect("match");
    assert_eq!(m.read_model().regression_count, 0);

    let d = VisualRegressionScaleArtifact::from_json_str(&example(
        "visual-regression-scale-v1/suite.diff.fixture.json",
    ))
    .expect("diff");
    assert_eq!(d.read_model().regression_count, 1);

    let missing = VisualRegressionScaleArtifact::from_json_str(&example(
        "visual-regression-scale-v1/suite.missing-baseline.fixture.json",
    ))
    .expect("missing");
    let mr = missing.read_model();
    assert_eq!(mr.status, "partial");
    assert_eq!(mr.missing_baseline_count, 1);
}

#[test]
fn v38_performance_pass_regression_and_soak() {
    let pass = PerformanceSoakArtifact::from_json_str(&example(
        "performance-soak-v1/suite.budget-pass.fixture.json",
    ))
    .expect("pass");
    assert_eq!(pass.computed_verdict(), "pass");

    let regressed = PerformanceSoakArtifact::from_json_str(&example(
        "performance-soak-v1/suite.regression.fixture.json",
    ))
    .expect("regressed");
    assert_eq!(regressed.computed_verdict(), "regressed");

    let unstable = PerformanceSoakArtifact::from_json_str(&example(
        "performance-soak-v1/suite.soak-unstable.fixture.json",
    ))
    .expect("unstable");
    assert_eq!(unstable.computed_verdict(), "unstable");
}

#[test]
fn v38_verdict_pass_fail_inconclusive_and_checks() {
    let pass = ProductionQaVerdictArtifact::from_json_str(&example(
        "production-qa-verdict-v1/verdict.pass.fixture.json",
    ))
    .expect("pass");
    assert_eq!(pass.computed_verdict(), "pass");
    // Crash and accessibility checks are part of the composed verdict.
    let kinds: BTreeSet<&str> = pass.checks.iter().map(|c| c.kind.as_str()).collect();
    assert!(kinds.contains("crash"));
    assert!(kinds.contains("accessibility"));

    let fail = ProductionQaVerdictArtifact::from_json_str(&example(
        "production-qa-verdict-v1/verdict.fail.fixture.json",
    ))
    .expect("fail");
    assert_eq!(fail.computed_verdict(), "fail");

    let inconclusive = ProductionQaVerdictArtifact::from_json_str(&example(
        "production-qa-verdict-v1/verdict.inconclusive.fixture.json",
    ))
    .expect("inconclusive");
    assert_eq!(inconclusive.computed_verdict(), "inconclusive");
}

#[test]
fn v38_backward_compatibility_per_artifact_gates_remain_valid() {
    // The production-scale matrix composes existing per-artifact gates; those
    // gates must keep validating unchanged (backward-compatibility golden).
    let run_matrix = QaRunMatrixArtifact::from_json_str(&example(
        "qa-swarm-run-matrix-v1/matrix.complete.fixture.json",
    ))
    .expect("existing QA run matrix gate remains valid");
    assert_eq!(run_matrix.computed_status(), "complete");

    let budget = PerformanceBudgetArtifact::from_json_str(&example(
        "qa-performance-budget-v1/budget.pass.fixture.json",
    ))
    .expect("existing QA performance budget gate remains valid");
    assert_eq!(budget.status, "pass");
}

#[test]
fn v38_doc_documents_the_suite() {
    let doc = read_text("docs/scenario-coverage-v38.md");
    assert!(doc.contains("Scenario Coverage v38"));
    assert!(doc.contains("#1671"));
    assert!(doc.contains("backward-compat"));
    assert!(doc.contains("states/shapes"));
    assert!(doc.contains("#1 and #23 remain open"));
    for forbidden in [
        "auto-merge enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!doc.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
