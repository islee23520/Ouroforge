use ouroforge_core::gdd_gameplay_behavior_plan::GddGameplayBehaviorPlanArtifact;
use std::{fs, path::PathBuf};
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-gameplay-behavior-plan-v1")
        .join(name)
}
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}
#[test]
fn behavior_plan_fixtures_validate_and_export_read_models() {
    for name in [
        "behavior-plan.valid.fixture.json",
        "behavior-plan.unsupported.fixture.json",
        "behavior-plan.script-needed.fixture.json",
        "behavior-plan.partial.fixture.json",
        "behavior-plan.blocked.fixture.json",
        "behavior-plan.stale.fixture.json",
    ] {
        let artifact = GddGameplayBehaviorPlanArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(
            read_model.behavior_model_count,
            artifact.behavior_models.len()
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("requirement")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("non-mutating")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("arbitrary script generation enabled"));
    }
}
#[test]
fn invalid_behavior_plan_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/behavior-plan.missing-requirement.fixture.json",
            "missing from declared GDD requirements",
        ),
        (
            "invalid/behavior-plan.unsupported-no-blocker.fixture.json",
            "blockedReasons",
        ),
        (
            "invalid/behavior-plan.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/behavior-plan.contradictory.fixture.json",
            "contradictory core loop behavior",
        ),
        (
            "invalid/behavior-plan.missing-proof.fixture.json",
            "proof expectation",
        ),
        (
            "invalid/behavior-plan.stale-no-blocker.fixture.json",
            "stale ref",
        ),
        (
            "invalid/behavior-plan.script-need-no-blocker.fixture.json",
            "blockedReasons",
        ),
    ] {
        let error = GddGameplayBehaviorPlanArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}
#[test]
fn behavior_plan_docs_keep_governance_and_script_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-gameplay-behavior-plan-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #651"));
    assert!(docs.contains("non-mutating"));
    assert!(docs.contains("gameplay-behavior-model-v1"));
    assert!(docs.contains("No arbitrary script generation"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "arbitrary script generation enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "production-ready claim enabled",
        "browser trusted write enabled",
        "autonomous unrestricted game creation enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
