//! QA swarm regression suite (Scenario Coverage v13, #697).
//!
//! Re-validates every QA/playtest artifact that is present on `main` through its
//! public API (valid fixtures parse, invalid fixtures fail closed) and validates
//! the machine-checked coverage matrix that records where each area's regression
//! coverage lives.

use ouroforge_core::qa_regression_coverage::QaRegressionCoverageArtifact;
use ouroforge_core::runtime_frame_budget::RuntimeFrameBudgetEvidence;
use ouroforge_core::{
    AdversarialInputFuzzingPlanArtifact, QaScenarioCandidateArtifact, QaWorkerAssignmentArtifact,
    RouteAttemptEvidenceArtifact, RuntimeInvariantEvidence, RuntimeInvariantModel,
    VisualComparisonEvidenceArtifact,
};
use std::{fs, path::PathBuf};

fn examples(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples")
        .join(rel)
}

fn read(rel: &str) -> String {
    fs::read_to_string(examples(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn in_repo_qa_artifacts_accept_valid_fixtures() {
    QaScenarioCandidateArtifact::from_json_str(&read(
        "qa-scenario-candidate-v1/scenario-candidate.sample.json",
    ))
    .expect("scenario candidate sample");
    AdversarialInputFuzzingPlanArtifact::from_json_str(&read(
        "adversarial-input-fuzzing-v1/fuzzing-plan.sample.json",
    ))
    .expect("fuzzing plan sample");
    QaWorkerAssignmentArtifact::from_json_str(&read(
        "qa-worker-assignment-v1/worker-assignment.sample.json",
    ))
    .expect("worker assignment sample");
    RuntimeInvariantModel::from_json_str(&read(
        "runtime-invariant-checker-v1/invariant-model.sample.json",
    ))
    .expect("invariant model sample");
    RuntimeInvariantEvidence::from_json_str(&read(
        "runtime-invariant-checker-v1/invariant-evidence.sample.json",
    ))
    .expect("invariant evidence sample");
    RouteAttemptEvidenceArtifact::from_json_str(&read(
        "route-attempt-evidence-v1/route-attempt-success.sample.json",
    ))
    .expect("route attempt sample");
    VisualComparisonEvidenceArtifact::from_json_str(&read(
        "visual-comparison-evidence-v1/visual-comparison-unchanged.sample.json",
    ))
    .expect("visual comparison unchanged sample");
    VisualComparisonEvidenceArtifact::from_json_str(&read(
        "visual-comparison-evidence-v1/visual-comparison-changed.sample.json",
    ))
    .expect("visual comparison changed sample");
    RuntimeFrameBudgetEvidence::from_json_str(&read(
        "runtime-frame-budget-v1/valid/frame-budget.sample.json",
    ))
    .expect("frame budget sample");
    RuntimeFrameBudgetEvidence::from_json_str(&read(
        "runtime-frame-budget-v1/violation/frame-budget.slow.json",
    ))
    .expect("frame budget violation sample");
}

#[test]
fn in_repo_qa_artifacts_reject_malformed_missing_and_stale_fixtures() {
    assert!(QaScenarioCandidateArtifact::from_json_str(&read(
        "qa-scenario-candidate-v1/invalid/blocked-candidate.json"
    ))
    .is_err());
    assert!(QaScenarioCandidateArtifact::from_json_str(&read(
        "qa-scenario-candidate-v1/invalid/overbroad-candidate.json"
    ))
    .is_err());
    assert!(AdversarialInputFuzzingPlanArtifact::from_json_str(&read(
        "adversarial-input-fuzzing-v1/invalid/unbounded-fuzzing-plan.json"
    ))
    .is_err());
    assert!(AdversarialInputFuzzingPlanArtifact::from_json_str(&read(
        "adversarial-input-fuzzing-v1/invalid/unsupported-action-fuzzing-plan.json"
    ))
    .is_err());
    assert!(QaWorkerAssignmentArtifact::from_json_str(&read(
        "qa-worker-assignment-v1/invalid/unbounded-worker-assignment.json"
    ))
    .is_err());
    assert!(RuntimeInvariantModel::from_json_str(&read(
        "runtime-invariant-checker-v1/invalid/unsafe-expression.runtime-invariant.json"
    ))
    .is_err());
    assert!(RouteAttemptEvidenceArtifact::from_json_str(&read(
        "route-attempt-evidence-v1/invalid/malformed-route-attempt.json"
    ))
    .is_err());
    assert!(RouteAttemptEvidenceArtifact::from_json_str(&read(
        "route-attempt-evidence-v1/invalid/blocked-route-attempt.json"
    ))
    .is_err());
    assert!(VisualComparisonEvidenceArtifact::from_json_str(&read(
        "visual-comparison-evidence-v1/invalid/missing-thresholds.json"
    ))
    .is_err());
    assert!(VisualComparisonEvidenceArtifact::from_json_str(&read(
        "visual-comparison-evidence-v1/invalid/malformed-screenshot-ref.json"
    ))
    .is_err());
    assert!(RuntimeFrameBudgetEvidence::from_json_str(&read(
        "runtime-frame-budget-v1/invalid/negative-render-ms.json"
    ))
    .is_err());
    assert!(RuntimeFrameBudgetEvidence::from_json_str(&read(
        "runtime-frame-budget-v1/invalid/zero-total-budget.json"
    ))
    .is_err());
}

#[test]
fn coverage_matrix_validates_and_enumerates_every_area() {
    let artifact = QaRegressionCoverageArtifact::from_json_str(&read(
        "qa-swarm-regression-suite-v1/coverage.matrix.json",
    ))
    .expect("coverage matrix");
    let read_model = artifact.read_model();
    assert_eq!(read_model.area_count, 14);
    assert_eq!(
        read_model.coverage_status_counts.get("in-repo").copied(),
        Some(14)
    );
    assert!(
        !read_model
            .coverage_status_counts
            .contains_key("pending-merge"),
        "closing #697 must not depend on pending-merge coverage"
    );
    assert!(read_model.known_gap_count >= 1);
    assert!(read_model
        .validation_summary
        .iter()
        .any(|note| note.contains("regression coverage")));
}

#[test]
fn invalid_coverage_matrices_fail_closed() {
    for (name, expected) in [
        ("invalid/coverage.missing-area.json", "missing area"),
        (
            "invalid/coverage.unsupported-status.json",
            "unsupported coverage status",
        ),
        ("invalid/coverage.unsafe-ref.json", "forbidden traversal"),
        (
            "invalid/coverage.missing-known-gaps.json",
            "knownGaps must not be empty",
        ),
        (
            "invalid/coverage.unsafe-boundary.json",
            "forbidden QA regression coverage authority text",
        ),
    ] {
        let rel = format!("qa-swarm-regression-suite-v1/{name}");
        let error = QaRegressionCoverageArtifact::from_json_str(&read(&rel))
            .expect_err(&rel)
            .to_string();
        assert!(error.contains(expected), "{rel}: {error}");
    }
}

#[test]
fn regression_suite_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-swarm-regression-suite-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #697"));
    assert!(docs.contains("Coverage matrix"));
    assert!(docs.contains("Known gaps"));
    assert!(docs.contains("regression coverage"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
