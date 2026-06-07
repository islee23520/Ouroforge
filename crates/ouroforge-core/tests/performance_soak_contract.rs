use ouroforge_core::performance_soak::PerformanceSoakArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/performance-soak-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_fixtures_validate_and_roll_up_verdict() {
    for (name, status, verdict) in [
        ("suite.budget-pass.fixture.json", "complete", "pass"),
        ("suite.regression.fixture.json", "complete", "regressed"),
        ("suite.soak-unstable.fixture.json", "complete", "unstable"),
    ] {
        let artifact = PerformanceSoakArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        assert_eq!(artifact.computed_status(), status, "{name}");
        assert_eq!(artifact.computed_verdict(), verdict, "{name}");
        let read_model = artifact.read_model();
        assert_eq!(read_model.verdict, verdict, "{name}");
        assert_eq!(read_model.segment_count, artifact.segments.len(), "{name}");
        assert!(
            artifact.dashboard_compat.read_only,
            "{name} must be read-only"
        );
        // Reuse statement: soak over the existing frame-budget surface.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no new profiler")));
        // Deterministic, never flaky.
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("never flaky")));
    }
}

#[test]
fn budget_pass_has_no_breach_and_is_stable() {
    let artifact =
        PerformanceSoakArtifact::from_json_str(&read_fixture("suite.budget-pass.fixture.json"))
            .expect("pass");
    let read_model = artifact.read_model();
    assert_eq!(read_model.verdict, "pass");
    assert_eq!(read_model.breach_count, 0);
    assert_eq!(read_model.within_budget_count, read_model.segment_count);
    assert!(read_model.soak_stable);
}

#[test]
fn planted_performance_regression_is_detected() {
    let artifact =
        PerformanceSoakArtifact::from_json_str(&read_fixture("suite.regression.fixture.json"))
            .expect("regression");
    let read_model = artifact.read_model();
    assert_eq!(read_model.verdict, "regressed");
    assert_eq!(read_model.breach_count, 1);
    assert_eq!(
        read_model.budget_breaches[0].segment_id,
        "seg-large-content"
    );
    assert!(read_model.budget_breaches[0].value_x1000 > read_model.budget_breaches[0].budget_x1000);
}

#[test]
fn soak_drift_beyond_tolerance_is_unstable() {
    let artifact =
        PerformanceSoakArtifact::from_json_str(&read_fixture("suite.soak-unstable.fixture.json"))
            .expect("unstable");
    let read_model = artifact.read_model();
    assert_eq!(read_model.verdict, "unstable");
    // No budget breach — the instability is pure drift over the session.
    assert_eq!(read_model.breach_count, 0);
    assert!(!read_model.soak_stable);
    assert!(read_model.drift_x1000 > 0);
}

#[test]
fn read_model_is_deterministic_regardless_of_segment_order() {
    let raw = read_fixture("suite.budget-pass.fixture.json");
    let forward = PerformanceSoakArtifact::from_json_str(&raw).expect("forward");
    let mut reversed = forward.clone();
    reversed.segments.reverse();
    reversed.validate().expect("reversed validates");
    assert_eq!(
        forward.read_model_json().unwrap(),
        reversed.read_model_json().unwrap(),
        "read model must be deterministic regardless of segment order"
    );
}

#[test]
fn invalid_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/suite.duplicate-segment.fixture.json",
            "duplicate segment id",
        ),
        (
            "invalid/suite.duplicate-sequence.fixture.json",
            "duplicate segment sequence",
        ),
        (
            "invalid/suite.unknown-metric.fixture.json",
            "unknown metric kind",
        ),
        (
            "invalid/suite.unsupported-comparator.fixture.json",
            "unsupported comparator",
        ),
        (
            "invalid/suite.zero-budget.fixture.json",
            "budgetX1000 must be greater than 0",
        ),
        (
            "invalid/suite.missing-frame-ref.fixture.json",
            "frameBudgetRef",
        ),
        (
            "invalid/suite.verdict-mismatch.fixture.json",
            "does not match computed verdict",
        ),
        (
            "invalid/suite.not-read-only.fixture.json",
            "must remain read-only or draft-only",
        ),
        (
            "invalid/suite.unsafe-boundary.fixture.json",
            "boundary must state",
        ),
        (
            "invalid/suite.stale-no-blocker.fixture.json",
            "requires visible blockedReasons",
        ),
        (
            "invalid/suite.forbidden-wording.fixture.json",
            "forbidden performance soak authority text",
        ),
    ] {
        let error = PerformanceSoakArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn scope_doc_mentions_soak_and_performance() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/production-qa-matrix-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Soak and performance testing"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
}
