use ouroforge_core::gdd_scene_level_plan::GddSceneLevelPlanArtifact;
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-scene-level-plan-v1")
        .join(name)
}
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

#[test]
fn scene_level_plan_fixtures_validate_and_export_read_models() {
    for name in [
        "scene-level-plan.valid.fixture.json",
        "scene-level-plan.unsupported.fixture.json",
        "scene-level-plan.partial.fixture.json",
        "scene-level-plan.blocked.fixture.json",
        "scene-level-plan.stale.fixture.json",
    ] {
        let artifact = GddSceneLevelPlanArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(read_model.level_intent_count, artifact.level_intents.len());
        assert_eq!(
            read_model.scene_plan_count,
            artifact.scene_generation_plans.len()
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
            .contains("direct scene write enabled"));
    }
}

#[test]
fn invalid_scene_level_plan_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/scene-level-plan.missing-requirement.fixture.json",
            "missing from declared GDD requirements",
        ),
        (
            "invalid/scene-level-plan.unsupported-no-blocker.fixture.json",
            "blockedReasons",
        ),
        (
            "invalid/scene-level-plan.unsafe-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/scene-level-plan.contradictory.fixture.json",
            "contradictory level goals",
        ),
        (
            "invalid/scene-level-plan.missing-proof.fixture.json",
            "objective proof expectation",
        ),
        (
            "invalid/scene-level-plan.stale-no-blocker.fixture.json",
            "stale target",
        ),
    ] {
        let error = GddSceneLevelPlanArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}

#[test]
fn scene_level_plan_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-scene-level-plan-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #650"));
    assert!(docs.contains("non-mutating"));
    assert!(docs.contains("level-intent-v1"));
    assert!(docs.contains("scene-generation-plan-v1"));
    assert!(docs.contains("No direct scene or tilemap writes"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "current Godot replacement is implemented",
        "production-ready claim enabled",
        "browser trusted write enabled",
        "autonomous unrestricted game creation enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
