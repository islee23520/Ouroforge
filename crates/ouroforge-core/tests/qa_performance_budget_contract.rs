use ouroforge_core::qa_performance_budget::PerformanceBudgetArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-performance-budget-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

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
        let artifact = PerformanceBudgetArtifact::from_json_str(&read_fixture(name))
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
    let artifact = PerformanceBudgetArtifact::from_json_str(&read_fixture(
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
        let error = PerformanceBudgetArtifact::from_json_str(&read_fixture(name))
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
