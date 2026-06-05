use ouroforge_core::gdd_asset_placeholder_plan::GddAssetPlaceholderPlanArtifact;
use std::{fs, path::PathBuf};
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-asset-placeholder-plan-v1")
        .join(name)
}
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}
#[test]
fn asset_plan_fixtures_validate_and_export_read_models() {
    for name in [
        "asset-plan.valid.fixture.json",
        "asset-plan.missing.fixture.json",
        "asset-plan.stale.fixture.json",
        "asset-plan.unsupported.fixture.json",
    ] {
        let artifact = GddAssetPlaceholderPlanArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.asset_entry_count, artifact.asset_entries.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("license/source")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no asset generation")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("remote fetch enabled"));
    }
}
#[test]
fn invalid_asset_plan_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/asset-plan.missing-license.fixture.json",
            "license/source notes",
        ),
        ("invalid/asset-plan.remote-ref.fixture.json", "remote refs"),
        (
            "invalid/asset-plan.generated-root.fixture.json",
            "generated-root",
        ),
        (
            "invalid/asset-plan.unsupported-type.fixture.json",
            "unsupported asset type",
        ),
        (
            "invalid/asset-plan.stale-manifest-ref.fixture.json",
            "missing from declared manifestRefs",
        ),
        (
            "invalid/asset-plan.proprietary-ambiguous.fixture.json",
            "proprietary/copyright ambiguity",
        ),
        (
            "invalid/asset-plan.unsafe-path.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/asset-plan.stale-no-blocker.fixture.json",
            "stale manifest refs",
        ),
        (
            "invalid/asset-plan.manifest-generated-root.fixture.json",
            "generated-root or evidence output",
        ),
        (
            "invalid/asset-plan.boundary-negation-bypass.fixture.json",
            "forbidden GDD asset authority text",
        ),
    ] {
        let error = GddAssetPlaceholderPlanArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}
#[test]
fn asset_plan_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-asset-placeholder-plan-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #652"));
    assert!(docs.contains("non-mutating"));
    assert!(docs.contains("placeholder assets or known local refs"));
    assert!(docs.contains("No asset generation"));
    assert!(docs.contains("license/source"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "remote fetch enabled",
        "asset generation enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
