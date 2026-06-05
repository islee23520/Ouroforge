use ouroforge_core::gdd_project_scaffold_plan::GddProjectScaffoldPlanArtifact;
use std::{fs, path::PathBuf};
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-project-scaffold-plan-v1")
        .join(name)
}
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}
#[test]
fn scaffold_plan_fixtures_validate_and_export_read_models() {
    for name in [
        "scaffold.valid.fixture.json",
        "scaffold.stale.fixture.json",
        "scaffold.blocked.fixture.json",
        "scaffold.deferred.fixture.json",
    ] {
        let artifact = GddProjectScaffoldPlanArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.planned_file_count, artifact.files.len());
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("preview-only")));
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
fn invalid_scaffold_plan_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/scaffold.unsafe-path.fixture.json",
            "fixture/reference",
        ),
        (
            "invalid/scaffold.generated-root-collision.fixture.json",
            "collides",
        ),
        ("invalid/scaffold.duplicate-file.fixture.json", "duplicated"),
        (
            "invalid/scaffold.unsupported-template-no-blocker.fixture.json",
            "blockedReasons",
        ),
        (
            "invalid/scaffold.missing-feasibility-pass.fixture.json",
            "feasibilityState pass",
        ),
        (
            "invalid/scaffold.stale-no-blocker.fixture.json",
            "stale target",
        ),
        ("invalid/scaffold.overbroad.fixture.json", "overbroad"),
        (
            "invalid/scaffold.direct-write-command.fixture.json",
            "preview-only",
        ),
    ] {
        let error = GddProjectScaffoldPlanArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}
#[test]
fn scaffold_plan_docs_keep_preview_and_governance_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-project-scaffold-plan-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #649"));
    assert!(docs.contains("Preview first"));
    assert!(docs.contains("no direct trusted writes"));
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
