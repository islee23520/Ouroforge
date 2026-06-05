use ouroforge_core::gdd_prototype_evidence_bundle::GddPrototypeEvidenceBundleArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-prototype-evidence-bundle-v1")
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
        "evidence.unsupported.fixture.json",
        "evidence.stale.fixture.json",
    ] {
        let artifact = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.scenario_count, artifact.scenario_verdicts.len());
        assert_eq!(
            read_model.requirement_count,
            artifact.requirement_coverage.len()
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("GDD requirements")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("remain separate")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("trusted writes enabled"));
    }
}

#[test]
fn invalid_prototype_evidence_fixtures_fail_closed() {
    for (name, expected) in [
        ("invalid/evidence.unsafe-ref.fixture.json", "forbidden"),
        (
            "invalid/evidence.malformed-missing-run.fixture.json",
            "requires blockedReasons",
        ),
        (
            "invalid/evidence.partial-no-blocker.fixture.json",
            "non-passing",
        ),
        (
            "invalid/evidence.unsupported-no-blocker.fixture.json",
            "requires blockedReasons",
        ),
        (
            "invalid/evidence.stale-no-blocker.fixture.json",
            "non-passing",
        ),
        (
            "invalid/evidence.missing-requirement-link.fixture.json",
            "missing from requirementCoverage",
        ),
        (
            "invalid/evidence.missing-scenario-link.fixture.json",
            "links missing scenarioId",
        ),
        (
            "invalid/evidence.fail-without-failure.fixture.json",
            "requires failed scenario",
        ),
        ("invalid/evidence.unsafe-boundary.fixture.json", "forbidden"),
    ] {
        let error = GddPrototypeEvidenceBundleArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn prototype_evidence_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-prototype-evidence-bundle-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #657"));
    assert!(docs.contains("evidence-gated prototype generation"));
    assert!(docs.contains("not autonomous unrestricted game creation"));
    assert!(docs.contains("Generated run/evidence output remains untracked"));
    assert!(docs.contains("#1 remains open"));
    assert!(docs.contains("#23 remains open"));
    for forbidden in [
        "trusted writes enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
        "autonomous unrestricted game creation enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
