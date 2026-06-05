use ouroforge_core::gdd_requirement_extraction::GddRequirementExtractionArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-requirement-extraction-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_partial_and_blocked_extraction_fixtures_validate() {
    for name in [
        "requirements.valid.fixture.json",
        "requirements.partial.fixture.json",
        "requirements.blocked.fixture.json",
    ] {
        let artifact = GddRequirementExtractionArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.requirement_count, artifact.requirements.len());
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("display-only")));
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("source section")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("auto-apply enabled"));
    }
}

#[test]
fn invalid_extraction_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/requirements.missing-source-ref.fixture.json",
            "missing sourceSectionRef",
        ),
        (
            "invalid/requirements.duplicate-id.fixture.json",
            "duplicated",
        ),
        (
            "invalid/requirements.invented-no-excerpt.fixture.json",
            "must include sourceExcerpt",
        ),
        (
            "invalid/requirements.invented-unlinked-excerpt.fixture.json",
            "sourceExcerpt is not present",
        ),
        (
            "invalid/requirements.conflict-no-blocker.fixture.json",
            "must include blockedReasons",
        ),
        (
            "invalid/requirements.low-confidence-no-blocker.fixture.json",
            "low-confidence",
        ),
        (
            "invalid/requirements.unsafe-boundary.fixture.json",
            "forbidden",
        ),
    ] {
        let error = GddRequirementExtractionArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn requirement_extraction_docs_keep_boundaries_and_governance() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-requirement-extraction-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #646"));
    assert!(docs.contains("LLM extraction is advisory only"));
    assert!(docs.contains("not a prototype generator"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
        "browser trusted write enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
