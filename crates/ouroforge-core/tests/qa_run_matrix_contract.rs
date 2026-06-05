use ouroforge_core::qa_run_matrix::QaRunMatrixArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/qa-swarm-run-matrix-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

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
        let artifact = QaRunMatrixArtifact::from_json_str(&read_fixture(name))
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
        let artifact = QaRunMatrixArtifact::from_json_str(&read_fixture(name)).expect(name);
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
        let error = QaRunMatrixArtifact::from_json_str(&read_fixture(name))
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
