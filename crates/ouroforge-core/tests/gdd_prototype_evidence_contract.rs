use ouroforge_core::gdd_prototype_evidence::GddPrototypeEvidenceBundleArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-prototype-evidence-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn prototype_evidence_fixtures_validate_and_export_read_models() {
    for name in [
        "evidence.pass.fixture.json",
        "evidence.fail.fixture.json",
        "evidence.missing-run.fixture.json",
        "evidence.partial.fixture.json",
        "evidence.stale.fixture.json",
    ] {
        let artifact = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(
            read_model.dashboard_summary[0],
            format!("scenarios:{}", artifact.scenario_verdicts.len())
        );
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("read-only prototype evidence")));
        assert!(artifact
            .journal_markdown()
            .contains("GDD Prototype Evidence Journal"));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("auto-apply enabled"));
    }
}

#[test]
fn prototype_evidence_read_model_surfaces_failures_and_unsupported_requirements() {
    let failed = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(
        "evidence.fail.fixture.json",
    ))
    .expect("failed fixture validates");
    assert_eq!(failed.read_model().failed_requirements, ["req-exit"]);
    assert!(failed.journal_markdown().contains("exit trigger missing"));

    let partial = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(
        "evidence.partial.fixture.json",
    ))
    .expect("partial fixture validates");
    assert_eq!(
        partial.read_model().unsupported_requirements,
        ["req-hazard"]
    );
    assert!(partial
        .read_model()
        .dashboard_summary
        .iter()
        .any(|row| row == "requirements:3"));
}

#[test]
fn invalid_prototype_evidence_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/evidence.pass-with-failure.fixture.json",
            "passing GDD prototype evidence requires",
        ),
        (
            "invalid/evidence.fail-without-failed-scenario.fixture.json",
            "failed scenario verdicts",
        ),
        (
            "invalid/evidence.missing-run-no-blocker.fixture.json",
            "missing-run GDD prototype evidence requires",
        ),
        (
            "invalid/evidence.unsupported-scenario-no-blocker.fixture.json",
            "skipped or unsupported GDD prototype scenario verdict requires",
        ),
        (
            "invalid/evidence.unsupported-requirement-no-blocker.fixture.json",
            "unsupported GDD prototype requirements require",
        ),
        (
            "invalid/evidence.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        ("invalid/evidence.unsafe-boundary.fixture.json", "forbidden"),
        (
            "invalid/evidence.empty-journal.fixture.json",
            "must not be empty",
        ),
    ] {
        let error = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn prototype_evidence_docs_keep_journal_dashboard_and_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-prototype-evidence-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #657"));
    assert!(docs.contains("requirement coverage"));
    assert!(docs.contains("Dashboard/Studio compatibility is read-only"));
    assert!(docs.contains("journal summary"));
    assert!(docs.contains("Generated run/evidence output remains untracked"));
    assert!(docs.contains("#1 remains open"));
    assert!(docs.contains("#23 remains open"));
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "browser trusted write enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
