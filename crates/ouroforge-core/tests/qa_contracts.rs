//! Consolidated QA contract tests.
//!
//! Merged from 7 individual qa_*_contract.rs files:
//! - qa_error_classifier_contract.rs
//! - qa_evidence_bundle_contract.rs
//! - qa_failure_backlog_contract.rs
//! - qa_flake_rerun_policy_contract.rs
//! - qa_performance_budget_contract.rs
//! - qa_playtest_demo_contract.rs
//! - qa_run_matrix_contract.rs

// ---------------------------------------------------------------------------
// Shared imports
// ---------------------------------------------------------------------------
use ouroforge_core::qa_error_classifier::QaErrorClassifierArtifact;
use ouroforge_core::qa_evidence_bundle::QaEvidenceBundleArtifact;
use ouroforge_core::qa_failure_backlog::QaFailureBacklogArtifact;
use ouroforge_core::qa_flake_rerun_policy::FlakeRerunPolicyArtifact;
use ouroforge_core::qa_performance_budget::PerformanceBudgetArtifact;
use ouroforge_core::qa_playtest_demo::QaPlaytestDemoManifest;
use ouroforge_core::qa_run_matrix::QaRunMatrixArtifact;
use std::{fs, path::PathBuf};

// ---------------------------------------------------------------------------
// Per-module fixture helpers (each module has its own example directory)
// ---------------------------------------------------------------------------

// -- qa_error_classifier helpers --
fn error_classifier_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-error-classifier-v1")
        .join(name)
}

fn error_classifier_read_fixture(name: &str) -> String {
    fs::read_to_string(error_classifier_fixture(name)).expect(name)
}

// -- qa_evidence_bundle helpers --
fn evidence_bundle_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-swarm-evidence-bundle-v1")
        .join(name)
}

fn evidence_bundle_read_fixture(name: &str) -> String {
    fs::read_to_string(evidence_bundle_fixture(name)).expect(name)
}

// -- qa_failure_backlog helpers --
fn failure_backlog_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-failure-backlog-v1")
        .join(name)
}

fn failure_backlog_read_fixture(name: &str) -> String {
    fs::read_to_string(failure_backlog_fixture(name)).expect(name)
}

// -- qa_flake_rerun_policy helpers --
fn flake_rerun_policy_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-flake-rerun-policy-v1")
        .join(name)
}

fn flake_rerun_policy_read_fixture(name: &str) -> String {
    fs::read_to_string(flake_rerun_policy_fixture(name)).expect(name)
}

// -- qa_performance_budget helpers --
fn performance_budget_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-performance-budget-v1")
        .join(name)
}

fn performance_budget_read_fixture(name: &str) -> String {
    fs::read_to_string(performance_budget_fixture(name)).expect(name)
}

// -- qa_playtest_demo helpers --
fn playtest_demo_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-playtest-demo-v1")
        .join(name)
}

fn playtest_demo_read_fixture(name: &str) -> String {
    fs::read_to_string(playtest_demo_fixture(name)).expect(name)
}

// -- qa_run_matrix helpers --
fn run_matrix_fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-swarm-run-matrix-v1")
        .join(name)
}

fn run_matrix_read_fixture(name: &str) -> String {
    fs::read_to_string(run_matrix_fixture(name)).expect(name)
}

// ===========================================================================
// qa_error_classifier_contract.rs
// ===========================================================================

#[test]
fn classifier_fixtures_validate_and_roll_up_status() {
    for (name, expected_status) in [
        ("classifier.classified.fixture.json", "classified"),
        ("classifier.inconclusive.fixture.json", "blocked"),
        ("classifier.stale.fixture.json", "stale"),
    ] {
        let artifact =
            QaErrorClassifierArtifact::from_json_str(&error_classifier_read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), expected_status, "{name} status");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, expected_status);
        assert_eq!(read_model.entry_count, artifact.entries.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("evidence inputs")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no auto-fix")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
    }
}

#[test]
fn classified_fixture_covers_all_error_kinds() {
    let artifact = QaErrorClassifierArtifact::from_json_str(&error_classifier_read_fixture(
        "classifier.classified.fixture.json",
    ))
    .expect("classified");
    let read_model = artifact.read_model();
    for kind in [
        "console",
        "exception",
        "crash",
        "probe-unavailable",
        "asset-load-failure",
        "scenario-timeout",
    ] {
        assert!(
            read_model.kind_counts.contains_key(kind),
            "missing kind {kind}"
        );
    }
    assert!(read_model.failure_class_counts.contains_key("unknown"));
}

#[test]
fn invalid_classifier_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/classifier.missing-console-evidence.fixture.json",
            "missing console/probe evidence",
        ),
        (
            "invalid/classifier.missing-probe-evidence.fixture.json",
            "missing console/probe evidence",
        ),
        (
            "invalid/classifier.missing-entry-evidence.fixture.json",
            "missing supporting evidence",
        ),
        (
            "invalid/classifier.malformed-payload.fixture.json",
            "malformed error payload",
        ),
        (
            "invalid/classifier.unknown-severity.fixture.json",
            "unknown severity",
        ),
        (
            "invalid/classifier.unsupported-kind.fixture.json",
            "unsupported error kind",
        ),
        (
            "invalid/classifier.missing-classification.fixture.json",
            "missing classification",
        ),
        (
            "invalid/classifier.kind-class-mismatch.fixture.json",
            "inconsistent with kind",
        ),
        (
            "invalid/classifier.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/classifier.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/classifier.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error = QaErrorClassifierArtifact::from_json_str(&error_classifier_read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn classifier_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-error-classifier-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #690"));
    assert!(docs.contains("evidence inputs"));
    assert!(docs.contains("not trusted truth"));
    assert!(docs.contains("review-gated"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "production safety guaranteed",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

// ===========================================================================
// qa_evidence_bundle_contract.rs
// ===========================================================================

#[test]
fn bundle_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("bundle.complete.fixture.json", "complete"),
        ("bundle.partial.fixture.json", "partial"),
        ("bundle.blocked.fixture.json", "blocked"),
        ("bundle.flaky.fixture.json", "partial"),
        ("bundle.stale.fixture.json", "stale"),
    ] {
        let artifact = QaEvidenceBundleArtifact::from_json_str(&evidence_bundle_read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, expected);
        assert_eq!(read_model.component_count, artifact.components.len());
        assert!(
            artifact.dashboard_export.read_only,
            "{name} export must be read-only"
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("run matrix")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
    }
}

#[test]
fn complete_bundle_covers_every_component_type() {
    let artifact = QaEvidenceBundleArtifact::from_json_str(&evidence_bundle_read_fixture(
        "bundle.complete.fixture.json",
    ))
    .expect("complete");
    let read_model = artifact.read_model();
    for component_type in [
        "scenario-candidates",
        "fuzz-plans",
        "worker-assignments",
        "invariant-checks",
        "route-attempts",
        "visual-comparisons",
        "performance-budgets",
        "error-classifications",
        "flake-policy",
        "failure-classifications",
        "mutation-backlog",
        "run-matrix",
    ] {
        assert!(
            read_model
                .component_status_by_type
                .contains_key(component_type),
            "missing component type {component_type}"
        );
    }
}

#[test]
fn invalid_bundle_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/bundle.malformed.fixture.json",
            "malformed artifact",
        ),
        (
            "invalid/bundle.unresolved-output.fixture.json",
            "unresolved output roots",
        ),
        (
            "invalid/bundle.missing-evidence.fixture.json",
            "missing component",
        ),
        (
            "invalid/bundle.missing-cleanup.fixture.json",
            "missing cleanup confirmation",
        ),
        (
            "invalid/bundle.missing-budget.fixture.json",
            "missing budgets",
        ),
        (
            "invalid/bundle.inconsistent-matrix.fixture.json",
            "inconsistent matrix rows",
        ),
        (
            "invalid/bundle.not-read-only.fixture.json",
            "must remain read-only or draft-only",
        ),
        (
            "invalid/bundle.status-mismatch.fixture.json",
            "does not match computed classification",
        ),
        (
            "invalid/bundle.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error = QaEvidenceBundleArtifact::from_json_str(&evidence_bundle_read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn bundle_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-swarm-evidence-bundle-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #694"));
    assert!(docs.contains("evidence inputs"));
    assert!(docs.contains("not trusted truth"));
    assert!(docs.contains("review-gated"));
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

// ===========================================================================
// qa_failure_backlog_contract.rs
// ===========================================================================

#[test]
fn backlog_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("backlog.classified.fixture.json", "classified"),
        ("backlog.unknown.fixture.json", "classified"),
        ("backlog.flaky.fixture.json", "classified"),
        ("backlog.stale.fixture.json", "stale"),
        ("backlog.blocked.fixture.json", "blocked"),
    ] {
        let artifact = QaFailureBacklogArtifact::from_json_str(&failure_backlog_read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, expected);
        assert_eq!(read_model.item_count, artifact.items.len());
        // No review status is ever an applied/fixed state.
        for status in read_model.review_status_counts.keys() {
            assert!(
                !status.contains("applied") && !status.contains("fixed"),
                "{name}: backlog must not carry an applied/fixed review status"
            );
        }
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("not automatic fixes")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-apply enabled"));
    }
}

#[test]
fn invalid_backlog_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/backlog.missing-evidence.fixture.json",
            "is missing evidence",
        ),
        (
            "invalid/backlog.invalid-owner-lane.fixture.json",
            "invalid owner lane",
        ),
        (
            "invalid/backlog.unsupported-class.fixture.json",
            "unsupported failure class",
        ),
        (
            "invalid/backlog.missing-reproduction.fixture.json",
            "missing reproduction context",
        ),
        (
            "invalid/backlog.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/backlog.duplicate-id.fixture.json",
            "duplicate backlog id",
        ),
        (
            "invalid/backlog.auto-fix-attempt.fixture.json",
            "auto-apply",
        ),
        (
            "invalid/backlog.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/backlog.status-mismatch.fixture.json",
            "does not match computed classification",
        ),
        (
            "invalid/backlog.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error = QaFailureBacklogArtifact::from_json_str(&failure_backlog_read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn backlog_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-failure-backlog-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #692"));
    assert!(docs.contains("not automatic fixes"));
    assert!(docs.contains("review-gated"));
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

// ===========================================================================
// qa_flake_rerun_policy_contract.rs
// ===========================================================================

#[test]
fn policy_fixtures_validate_and_classify() {
    for (name, expected) in [
        ("policy.stable-pass.fixture.json", "stable-pass"),
        ("policy.stable-fail.fixture.json", "stable-fail"),
        ("policy.flaky.fixture.json", "flaky"),
        ("policy.flaky-permissive.fixture.json", "flaky"),
        ("policy.inconclusive.fixture.json", "inconclusive"),
        ("policy.exhausted.fixture.json", "exhausted"),
        ("policy.unsupported.fixture.json", "unsupported"),
        ("policy.stale.fixture.json", "stale"),
    ] {
        let artifact =
            FlakeRerunPolicyArtifact::from_json_str(&flake_rerun_policy_read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_classification(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.classification, expected);
        assert!(read_model.max_reruns >= 1 && read_model.max_reruns <= 10);
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("evidence inputs")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no auto-fix")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
    }
}

#[test]
fn invalid_policy_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/policy.unbounded-reruns.fixture.json",
            "unbounded reruns are not allowed",
        ),
        (
            "invalid/policy.missing-threshold.fixture.json",
            "consistency threshold must be between 0 and 1",
        ),
        (
            "invalid/policy.overlapping-outputs.fixture.json",
            "overlapping output roots",
        ),
        (
            "invalid/policy.missing-cleanup.fixture.json",
            "cleanup policy must not be empty",
        ),
        (
            "invalid/policy.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/policy.malformed-comparison.fixture.json",
            "malformed comparison",
        ),
        (
            "invalid/policy.missing-original-evidence.fixture.json",
            "missing original evidence",
        ),
        (
            "invalid/policy.classification-mismatch.fixture.json",
            "does not match computed classification",
        ),
        (
            "invalid/policy.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error = FlakeRerunPolicyArtifact::from_json_str(&flake_rerun_policy_read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn policy_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-flake-rerun-policy-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #691"));
    assert!(docs.contains("evidence inputs"));
    assert!(docs.contains("bounded reruns"));
    assert!(docs.contains("review-gated"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "quality guaranteed",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

// ===========================================================================
// qa_performance_budget_contract.rs
// ===========================================================================

#[test]
fn budget_fixtures_validate_and_classify() {
    for (name, expected_status) in [
        ("budget.pass.fixture.json", "pass"),
        ("budget.fail.fixture.json", "fail"),
        ("budget.inconclusive.fixture.json", "inconclusive"),
        ("budget.missing.fixture.json", "missing"),
        ("budget.stale.fixture.json", "stale"),
        ("budget.unsupported.fixture.json", "unsupported"),
        ("budget.baseline-change.fixture.json", "fail"),
    ] {
        let artifact =
            PerformanceBudgetArtifact::from_json_str(&performance_budget_read_fixture(name))
                .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(
            artifact.computed_status().as_str(),
            expected_status,
            "{name} computed status"
        );
        let read_model = artifact.read_model();
        assert_eq!(
            read_model.status, expected_status,
            "{name} read model status"
        );
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.metric_count, artifact.metrics.len());
        assert_eq!(read_model.threshold_count, artifact.thresholds.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("evidence inputs")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no auto-fix")));
        // Read model JSON never leaks trusted-mutation wording.
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
        assert!(!json.contains("production performance guaranteed"));
    }
}

#[test]
fn baseline_change_fixture_carries_baseline_refs() {
    let artifact = PerformanceBudgetArtifact::from_json_str(&performance_budget_read_fixture(
        "budget.baseline-change.fixture.json",
    ))
    .expect("baseline-change");
    assert!(
        !artifact.baseline_refs.is_empty(),
        "baseline-change fixture must carry baseline refs"
    );
    assert!(
        !artifact.violations().is_empty(),
        "baseline-change fixture must record a regression"
    );
}

#[test]
fn invalid_budget_fixtures_fail_closed() {
    for (name, expected) in [
        ("invalid/budget.malformed-metric.fixture.json", "malformed"),
        (
            "invalid/budget.missing-baseline.fixture.json",
            "baseline required",
        ),
        (
            "invalid/budget.unsupported-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/budget.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/budget.browser-no-warning.fixture.json",
            "browser trust warning",
        ),
        (
            "invalid/budget.status-mismatch.fixture.json",
            "does not match computed classification",
        ),
        (
            "invalid/budget.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/budget.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error =
            PerformanceBudgetArtifact::from_json_str(&performance_budget_read_fixture(name))
                .expect_err(name)
                .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn budget_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-performance-budget-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #689"));
    assert!(docs.contains("evidence inputs"));
    assert!(docs.contains("not trusted truth"));
    assert!(docs.contains("review-gated"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "production performance guaranteed",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

// ===========================================================================
// qa_playtest_demo_contract.rs
// ===========================================================================

#[test]
fn demo_manifest_validates_and_wires_all_stages() {
    let manifest =
        QaPlaytestDemoManifest::from_json_str(&playtest_demo_read_fixture("demo.manifest.json"))
            .unwrap_or_else(|error| panic!("demo.manifest.json: {error:#}"));
    let read_model = manifest.read_model();
    assert_eq!(read_model.stage_count, 13);
    assert_eq!(read_model.present_stage_count, 13);
    assert!(
        read_model.known_gap_count >= 1,
        "a demo must state known gaps"
    );
    // Generated output roots must stay under runs/.
    for root in &manifest.generated_output_roots {
        assert!(
            root.starts_with("runs/"),
            "generated output must stay under runs/: {root}"
        );
    }
    assert!(read_model
        .compatibility_notes
        .iter()
        .any(|note| note.contains("evidence and backlog inputs")));
    let json = manifest.read_model_json().unwrap();
    assert!(!json.contains("auto-merge enabled"));
}

#[test]
fn invalid_demo_manifests_fail_closed() {
    for (name, expected) in [
        (
            "invalid/demo.unbounded-fuzz.manifest.json",
            "unbounded fuzzing is not allowed",
        ),
        (
            "invalid/demo.unbounded-worker.manifest.json",
            "unbounded workers are not allowed",
        ),
        (
            "invalid/demo.overlapping-outputs.manifest.json",
            "overlapping output roots",
        ),
        (
            "invalid/demo.missing-cleanup.manifest.json",
            "cleanup policy must not be empty",
        ),
        (
            "invalid/demo.missing-known-gaps.manifest.json",
            "knownGaps must not be empty",
        ),
        (
            "invalid/demo.missing-stage.manifest.json",
            "missing stage `run-matrix`",
        ),
        (
            "invalid/demo.unsafe-boundary.manifest.json",
            "boundary must state",
        ),
    ] {
        let error = QaPlaytestDemoManifest::from_json_str(&playtest_demo_read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn demo_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-playtest-demo-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #696"));
    assert!(docs.contains("Known gaps"));
    assert!(docs.contains("Cleanup policy"));
    assert!(docs.contains("evidence and backlog inputs"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-fix enabled",
        "auto-merge enabled",
        "hidden workers enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

// ===========================================================================
// qa_run_matrix_contract.rs
// ===========================================================================

#[test]
fn run_matrix_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("matrix.complete.fixture.json", "complete"),
        ("matrix.partial.fixture.json", "partial"),
        ("matrix.flaky.fixture.json", "partial"),
        ("matrix.inconclusive.fixture.json", "partial"),
        ("matrix.missing.fixture.json", "partial"),
        ("matrix.stale.fixture.json", "stale"),
        ("matrix.unsupported.fixture.json", "partial"),
    ] {
        let artifact = QaRunMatrixArtifact::from_json_str(&run_matrix_read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), expected, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.status, expected);
        assert_eq!(read_model.row_count, artifact.rows.len());
        assert!(
            artifact.dashboard_compat.read_only,
            "{name} must be read-only"
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("evidence inputs")));
        let json = artifact.read_model_json().unwrap();
        assert!(!json.contains("auto-merge enabled"));
    }
}

#[test]
fn run_matrix_verdicts_cover_full_taxonomy() {
    // Across the valid fixtures, every supported verdict should appear at least once.
    let mut seen = std::collections::BTreeSet::new();
    for name in [
        "matrix.complete.fixture.json",
        "matrix.partial.fixture.json",
        "matrix.flaky.fixture.json",
        "matrix.inconclusive.fixture.json",
        "matrix.missing.fixture.json",
        "matrix.stale.fixture.json",
        "matrix.unsupported.fixture.json",
    ] {
        let artifact =
            QaRunMatrixArtifact::from_json_str(&run_matrix_read_fixture(name)).expect(name);
        for row in &artifact.rows {
            seen.insert(row.verdict.clone());
        }
    }
    for verdict in [
        "passed",
        "failed",
        "flaky",
        "inconclusive",
        "skipped",
        "unsupported",
        "missing_evidence",
    ] {
        assert!(
            seen.contains(verdict),
            "missing verdict coverage: {verdict}"
        );
    }
}

#[test]
fn invalid_run_matrix_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/matrix.malformed-verdict.fixture.json",
            "malformed verdict",
        ),
        ("invalid/matrix.duplicate-row.fixture.json", "duplicate row"),
        (
            "invalid/matrix.invalid-worker-id.fixture.json",
            "workerId must be a bounded local id",
        ),
        (
            "invalid/matrix.missing-run-ref.fixture.json",
            "missing run ref",
        ),
        (
            "invalid/matrix.missing-budget.fixture.json",
            "budget must be bounded",
        ),
        (
            "invalid/matrix.inconsistent-rerun-group.fixture.json",
            "inconsistent rerun group",
        ),
        (
            "invalid/matrix.missing-evidence.fixture.json",
            "is missing evidence for verdict",
        ),
        (
            "invalid/matrix.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/matrix.not-read-only.fixture.json",
            "must remain read-only or draft-only",
        ),
        (
            "invalid/matrix.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
    ] {
        let error = QaRunMatrixArtifact::from_json_str(&run_matrix_read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn run_matrix_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/qa-swarm-run-matrix-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #693"));
    assert!(docs.contains("evidence inputs"));
    assert!(docs.contains("not trusted truth"));
    assert!(docs.contains("review-gated"));
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
