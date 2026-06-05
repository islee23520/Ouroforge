use ouroforge_core::gdd_prototype_draft_bundle::GddPrototypeDraftBundleArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-prototype-draft-bundle-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn prototype_draft_bundle_fixtures_validate_and_export_read_models() {
    for name in [
        "bundle.valid.fixture.json",
        "bundle.incomplete.fixture.json",
        "bundle.stale.fixture.json",
        "bundle.unsupported.fixture.json",
        "bundle.blocked.fixture.json",
    ] {
        let artifact = GddPrototypeDraftBundleArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.component_count, artifact.components.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("review surface")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("display-only")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("direct trusted writes enabled"));
    }
}

#[test]
fn invalid_prototype_draft_bundle_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/bundle.unsafe-ref.fixture.json",
            "fixture/reference",
        ),
        (
            "invalid/bundle.missing-component.fixture.json",
            "missing required component",
        ),
        (
            "invalid/bundle.duplicate-component.fixture.json",
            "duplicated",
        ),
        (
            "invalid/bundle.missing-scenario-no-blocker.fixture.json",
            "blockedReasons",
        ),
        (
            "invalid/bundle.stale-target-no-blocker.fixture.json",
            "stale target",
        ),
        (
            "invalid/bundle.missing-source-note.fixture.json",
            "sourceNoteRefs must not be empty",
        ),
        ("invalid/bundle.malformed-hash.fixture.json", "sha256"),
        ("invalid/bundle.unsafe-boundary.fixture.json", "forbidden"),
        ("invalid/bundle.overbroad.fixture.json", "overbroad"),
    ] {
        let error = GddPrototypeDraftBundleArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn prototype_draft_bundle_docs_keep_review_and_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-prototype-draft-bundle-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #655"));
    assert!(docs.contains("review surface only"));
    assert!(docs.contains("No direct trusted writes"));
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
