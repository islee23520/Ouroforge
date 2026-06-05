use ouroforge_core::gdd_scenario_acceptance_plan::GddScenarioAcceptancePlanArtifact;
use std::{fs, path::PathBuf};
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/gdd-scenario-acceptance-plan-v1")
        .join(name)
}
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}
#[test]
fn scenario_plan_fixtures_validate_and_export_read_models() {
    for name in [
        "scenario-plan.valid.fixture.json",
        "scenario-plan.partial.fixture.json",
        "scenario-plan.blocked.fixture.json",
        "scenario-plan.unsupported.fixture.json",
        "scenario-plan.stale.fixture.json",
    ] {
        let artifact = GddScenarioAcceptancePlanArtifact::from_json_str(&read_fixture(name))
            .unwrap_or_else(|error| panic!("{name}: {error:#}"));
        let read_model = artifact.read_model();
        assert_eq!(read_model.schema_version, artifact.schema_version);
        assert_eq!(
            read_model.scenario_draft_count,
            artifact.scenario_drafts.len()
        );
        assert!(read_model
            .validation_summary
            .iter()
            .any(|note| note.contains("requirement")));
        assert!(read_model
            .compatibility_notes
            .iter()
            .any(|note| note.contains("no trusted test creation")));
        assert!(!artifact
            .read_model_json()
            .unwrap()
            .contains("trusted tests enabled"));
    }
}
#[test]
fn invalid_scenario_plan_fixtures_fail_closed() {
    for (name, expected) in [
        (
            "invalid/scenario-plan.missing-requirement.fixture.json",
            "missing from declared GDD requirements",
        ),
        (
            "invalid/scenario-plan.unsupported-mechanic.fixture.json",
            "mechanics mapping ids",
        ),
        (
            "invalid/scenario-plan.unsupported-assertion.fixture.json",
            "unsupported assertion",
        ),
        (
            "invalid/scenario-plan.unsafe-scenario-ref.fixture.json",
            "forbidden traversal",
        ),
        (
            "invalid/scenario-plan.contradictory-acceptance.fixture.json",
            "contradictory acceptance criteria",
        ),
        (
            "invalid/scenario-plan.missing-evidence.fixture.json",
            "evidenceNeeded must not be empty",
        ),
        (
            "invalid/scenario-plan.stale-no-blocker.fixture.json",
            "stale targets require blockedReasons",
        ),
        (
            "invalid/scenario-plan.unsupported-no-blocker.fixture.json",
            "unsupported checks or stale targets require blockedReasons",
        ),
    ] {
        let error = GddScenarioAcceptancePlanArtifact::from_json_str(&read_fixture(name))
            .expect_err(name)
            .to_string();
        assert!(error.contains(expected), "{name}: {error}");
    }
}
#[test]
fn scenario_plan_docs_keep_governance_and_wording_boundaries() {
    let docs = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("docs/gdd-scenario-acceptance-plan-v1.md"),
    )
    .expect("docs");
    assert!(docs.contains("Issue: #653"));
    assert!(docs.contains("non-mutating"));
    assert!(docs.contains("scenario drafts"));
    assert!(docs.contains("not trusted tests"));
    assert!(docs.contains("No hidden implementation of unsupported checks"));
    assert!(docs.contains("#1 remains"));
    assert!(docs.contains("#23 remains"));
    for forbidden in [
        "trusted tests enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "production-ready claim enabled",
        "current Godot replacement is implemented",
        "autonomous unrestricted game creation enabled",
    ] {
        assert!(!docs.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
