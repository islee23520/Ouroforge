use ouroforge_core::qa_evidence_bundle::QaEvidenceBundleArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-swarm-evidence-bundle-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn bundle_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("bundle.complete.fixture.json", "complete"),
        ("bundle.partial.fixture.json", "partial"),
        ("bundle.blocked.fixture.json", "blocked"),
        ("bundle.flaky.fixture.json", "partial"),
        ("bundle.stale.fixture.json", "stale"),
    ] {
        let artifact = QaEvidenceBundleArtifact::from_json_str(&read_fixture(name))
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
    let artifact =
        QaEvidenceBundleArtifact::from_json_str(&read_fixture("bundle.complete.fixture.json"))
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
        let error = QaEvidenceBundleArtifact::from_json_str(&read_fixture(name))
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
