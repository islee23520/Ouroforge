use ouroforge_core::qa_failure_backlog::QaFailureBacklogArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-failure-backlog-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn backlog_fixtures_validate_and_roll_up_status() {
    for (name, expected) in [
        ("backlog.classified.fixture.json", "classified"),
        ("backlog.unknown.fixture.json", "classified"),
        ("backlog.flaky.fixture.json", "classified"),
        ("backlog.stale.fixture.json", "stale"),
        ("backlog.blocked.fixture.json", "blocked"),
    ] {
        let artifact = QaFailureBacklogArtifact::from_json_str(&read_fixture(name))
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
        let error = QaFailureBacklogArtifact::from_json_str(&read_fixture(name))
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
