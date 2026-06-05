use ouroforge_core::gdd_mechanics_mapping::GddMechanicsMappingArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-mechanics-mapping-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn valid_mechanics_mapping_fixtures_validate_and_export_read_models() {
    for name in [
        "mechanics.supported.fixture.json",
        "mechanics.unsupported.fixture.json",
        "mechanics.partial.fixture.json",
        "mechanics.contradictory.fixture.json",
        "mechanics.deferred.fixture.json",
    ] {
        let artifact = GddMechanicsMappingArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.mapping_count, artifact.mappings.len());
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("display-only")));
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("requirement")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("auto-apply enabled"));
    }
}

#[test]
fn invalid_mechanics_mapping_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/mechanics.supported-missing-behavior.fixture.json",
            "supported behaviorModelRefs",
        ),
        (
            "invalid/mechanics.unsupported-no-recommendation.fixture.json",
            "recommendations must not be empty",
        ),
        (
            "invalid/mechanics.contradictory-no-blocker.fixture.json",
            "contradictory mapping",
        ),
        (
            "invalid/mechanics.overbroad-core-loop.fixture.json",
            "overbroad",
        ),
        (
            "invalid/mechanics.unsafe-boundary.fixture.json",
            "forbidden",
        ),
        (
            "invalid/mechanics.unknown-capability.fixture.json",
            "unknown capability",
        ),
    ] {
        let error = GddMechanicsMappingArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn mechanics_mapping_docs_keep_boundaries_and_governance() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-mechanics-mapping-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #647"));
    assert!(docs.contains("requirement ids"));
    assert!(docs.contains("unsupported mechanics"));
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
