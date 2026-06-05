use ouroforge_core::qa_error_classifier::QaErrorClassifierArtifact;
use ouroforge_core::qa_evidence_bundle::QaEvidenceBundleArtifact;
use ouroforge_core::qa_failure_backlog::QaFailureBacklogArtifact;
use ouroforge_core::qa_flake_rerun_policy::FlakeRerunPolicyArtifact;
use ouroforge_core::qa_performance_budget::PerformanceBudgetArtifact;
use ouroforge_core::qa_run_matrix::QaRunMatrixArtifact;
use ouroforge_core::{
    AdversarialInputFuzzingPlanArtifact, QaScenarioCandidateArtifact, QaWorkerAssignmentArtifact,
    RouteAttemptEvidenceArtifact, RuntimeInvariantEvidence, RuntimeInvariantModel,
    VisualComparisonEvidenceArtifact,
};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn matrix() -> Value {
    serde_json::from_str(&read_repo(
        "examples/qa-swarm-regression-suite-v13/coverage-matrix.fixture.json",
    ))
    .expect("coverage matrix parses")
}

#[test]
fn qa_swarm_v13_matrix_covers_every_required_stage_and_boundary() {
    let matrix = matrix();
    assert_eq!(matrix["schemaVersion"], "qa-swarm-regression-suite-v13");
    assert_eq!(matrix["issue"], 697);
    assert_eq!(matrix["milestoneAnchor"], "#1");
    assert_eq!(matrix["memoryAnchor"], "#23");

    let stages = matrix["coverageStages"].as_array().expect("stages array");
    let stage_ids = stages
        .iter()
        .map(|stage| stage["id"].as_str().expect("stage id"))
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        "scenario-candidates",
        "fuzz-plans",
        "worker-assignments",
        "runtime-invariants",
        "route-attempts",
        "visual-comparisons",
        "performance-budgets",
        "error-classifications",
        "flake-rerun-policy",
        "failure-backlog",
        "run-matrix",
        "evidence-bundle",
        "dashboard-studio-read-models",
        "malformed-missing-stale-unresolved-output",
    ] {
        assert!(stage_ids.contains(required), "missing stage {required}");
    }

    let boundary = matrix["boundary"].as_str().expect("boundary");
    for required in [
        "Aggregate regression evidence only",
        "must not spawn workers",
        "auto-fix",
        "auto-apply",
        "auto-merge",
        "trusted mutation",
    ] {
        assert!(boundary.contains(required), "boundary missing {required}");
    }
    assert!(matrix["generatedStatePolicy"]
        .as_str()
        .expect("generated state policy")
        .contains("remain untracked"));
    assert!(!matrix["knownGaps"]
        .as_array()
        .expect("known gaps")
        .is_empty());
}

#[test]
fn qa_swarm_v13_positive_fixtures_validate_with_public_contracts() {
    QaScenarioCandidateArtifact::from_json_str(&read_repo(
        "examples/qa-scenario-candidate-v1/scenario-candidate.sample.json",
    ))
    .expect("scenario candidate fixture");
    AdversarialInputFuzzingPlanArtifact::from_json_str(&read_repo(
        "examples/adversarial-input-fuzzing-v1/fuzzing-plan.sample.json",
    ))
    .expect("fuzzing plan fixture");
    QaWorkerAssignmentArtifact::from_json_str(&read_repo(
        "examples/qa-worker-assignment-v1/worker-assignment.sample.json",
    ))
    .expect("worker assignment fixture");
    RuntimeInvariantModel::from_json_str(&read_repo(
        "examples/runtime-invariant-checker-v1/invariant-model.sample.json",
    ))
    .expect("runtime invariant model fixture");
    RuntimeInvariantEvidence::from_json_str(&read_repo(
        "examples/runtime-invariant-checker-v1/invariant-evidence.sample.json",
    ))
    .expect("runtime invariant evidence fixture");
    RouteAttemptEvidenceArtifact::from_json_str(&read_repo(
        "examples/route-attempt-evidence-v1/route-attempt-success.sample.json",
    ))
    .expect("route attempt fixture");
    VisualComparisonEvidenceArtifact::from_json_str(&read_repo(
        "examples/visual-comparison-evidence-v1/visual-comparison-unchanged.sample.json",
    ))
    .expect("visual comparison fixture");
    PerformanceBudgetArtifact::from_json_str(&read_repo(
        "examples/qa-performance-budget-v1/budget.pass.fixture.json",
    ))
    .expect("performance budget fixture");
    QaErrorClassifierArtifact::from_json_str(&read_repo(
        "examples/qa-error-classifier-v1/classifier.classified.fixture.json",
    ))
    .expect("error classifier fixture");
    FlakeRerunPolicyArtifact::from_json_str(&read_repo(
        "examples/qa-flake-rerun-policy-v1/policy.stable-pass.fixture.json",
    ))
    .expect("flake rerun policy fixture");
    QaFailureBacklogArtifact::from_json_str(&read_repo(
        "examples/qa-failure-backlog-v1/backlog.classified.fixture.json",
    ))
    .expect("failure backlog fixture");
    QaRunMatrixArtifact::from_json_str(&read_repo(
        "examples/qa-swarm-run-matrix-v1/matrix.complete.fixture.json",
    ))
    .expect("run matrix fixture");
    QaEvidenceBundleArtifact::from_json_str(&read_repo(
        "examples/qa-swarm-evidence-bundle-v1/bundle.complete.fixture.json",
    ))
    .expect("evidence bundle fixture");
}

#[test]
fn qa_swarm_v13_negative_fixtures_fail_closed() {
    for (name, error) in [
        (
            "scenario candidate overbroad",
            QaScenarioCandidateArtifact::from_json_str(&read_repo(
                "examples/qa-scenario-candidate-v1/invalid/overbroad-candidate.json",
            ))
            .expect_err("overbroad scenario candidate rejected")
            .to_string(),
        ),
        (
            "worker unbounded",
            QaWorkerAssignmentArtifact::from_json_str(&read_repo(
                "examples/qa-worker-assignment-v1/invalid/unbounded-worker-assignment.json",
            ))
            .expect_err("unbounded worker assignment rejected")
            .to_string(),
        ),
        (
            "unsafe runtime invariant",
            RuntimeInvariantModel::from_json_str(&read_repo(
                "examples/runtime-invariant-checker-v1/invalid/unsafe-expression.runtime-invariant.json",
            ))
            .expect_err("unsafe runtime invariant rejected")
            .to_string(),
        ),
        (
            "malformed route attempt",
            RouteAttemptEvidenceArtifact::from_json_str(&read_repo(
                "examples/route-attempt-evidence-v1/invalid/malformed-route-attempt.json",
            ))
            .expect_err("malformed route attempt rejected")
            .to_string(),
        ),
        (
            "visual missing thresholds",
            VisualComparisonEvidenceArtifact::from_json_str(&read_repo(
                "examples/visual-comparison-evidence-v1/invalid/missing-thresholds.json",
            ))
            .expect_err("visual missing thresholds rejected")
            .to_string(),
        ),
        (
            "performance status mismatch",
            PerformanceBudgetArtifact::from_json_str(&read_repo(
                "examples/qa-performance-budget-v1/invalid/budget.status-mismatch.fixture.json",
            ))
            .expect_err("performance status mismatch rejected")
            .to_string(),
        ),
        (
            "error classifier missing console evidence",
            QaErrorClassifierArtifact::from_json_str(&read_repo(
                "examples/qa-error-classifier-v1/invalid/classifier.missing-console-evidence.fixture.json",
            ))
            .expect_err("classifier missing console evidence rejected")
            .to_string(),
        ),
        (
            "flake unbounded reruns",
            FlakeRerunPolicyArtifact::from_json_str(&read_repo(
                "examples/qa-flake-rerun-policy-v1/invalid/policy.unbounded-reruns.fixture.json",
            ))
            .expect_err("unbounded flake reruns rejected")
            .to_string(),
        ),
        (
            "backlog auto apply",
            QaFailureBacklogArtifact::from_json_str(&read_repo(
                "examples/qa-failure-backlog-v1/invalid/backlog.auto-fix-attempt.fixture.json",
            ))
            .expect_err("auto apply backlog rejected")
            .to_string(),
        ),
        (
            "run matrix missing evidence",
            QaRunMatrixArtifact::from_json_str(&read_repo(
                "examples/qa-swarm-run-matrix-v1/invalid/matrix.missing-evidence.fixture.json",
            ))
            .expect_err("missing run matrix evidence rejected")
            .to_string(),
        ),
        (
            "bundle unresolved output",
            QaEvidenceBundleArtifact::from_json_str(&read_repo(
                "examples/qa-swarm-evidence-bundle-v1/invalid/bundle.unresolved-output.fixture.json",
            ))
            .expect_err("unresolved output bundle rejected")
            .to_string(),
        ),
    ] {
        assert!(!error.is_empty(), "{name} must explain rejection");
    }

    let mut over_budget_fuzz: Value = serde_json::from_str(&read_repo(
        "examples/adversarial-input-fuzzing-v1/fuzzing-plan.sample.json",
    ))
    .expect("fuzz json");
    over_budget_fuzz["budget"]["maxRuns"] = Value::from(101);
    let error = AdversarialInputFuzzingPlanArtifact::from_json_str(&over_budget_fuzz.to_string())
        .expect_err("over-budget fuzz plan rejected")
        .to_string();
    assert!(error.contains("budget.maxRuns"));
}

#[test]
fn qa_swarm_v13_docs_keep_dashboard_studio_and_governance_boundaries() {
    let docs = read_repo("docs/scenario-coverage-v13-qa-swarm.md");
    let studio_docs = read_repo("docs/studio-qa-swarm-inspection-surface-v1.md");
    for required in [
        "Issue: #697",
        "#1 remains open",
        "#23 remains open",
        "read-only or draft-only",
        "Generated runs",
        "remain untracked",
        "known gaps",
    ] {
        assert!(docs.contains(required), "docs missing {required}");
    }
    assert!(studio_docs.contains("read-only") || studio_docs.contains("Read-only"));
    assert!(studio_docs.contains("spawn QA workers"));
    for forbidden in [
        "auto-fix enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "hidden workers enabled",
        "remote/cloud swarm enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
        assert!(
            !studio_docs.contains(forbidden),
            "forbidden Studio wording: {forbidden}"
        );
    }
}
