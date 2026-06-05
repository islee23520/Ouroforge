use ouroforge_core::qa_error_classifier::QaErrorClassifierArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-error-classifier-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn classifier_fixtures_validate_and_roll_up_status() {
    for (name, expected_status) in [
        ("classifier.classified.fixture.json", "classified"),
        ("classifier.inconclusive.fixture.json", "blocked"),
        ("classifier.stale.fixture.json", "stale"),
    ] {
        let artifact = QaErrorClassifierArtifact::from_json_str(&read_fixture(name))
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
    let artifact = QaErrorClassifierArtifact::from_json_str(&read_fixture(
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
        let error = QaErrorClassifierArtifact::from_json_str(&read_fixture(name))
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
