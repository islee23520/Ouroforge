use ouroforge_core::{
    gdd_asset_placeholder_plan::GddAssetPlaceholderPlanArtifact,
    gdd_design_brief::GddDesignBriefArtifact, gdd_feasibility_gate::GddFeasibilityGateArtifact,
    gdd_gameplay_behavior_plan::GddGameplayBehaviorPlanArtifact,
    gdd_mechanics_mapping::GddMechanicsMappingArtifact,
    gdd_project_scaffold_plan::GddProjectScaffoldPlanArtifact,
    gdd_prototype_apply::GddPrototypeApplyArtifact,
    gdd_prototype_draft_bundle::GddPrototypeDraftBundleArtifact,
    gdd_prototype_evidence::GddPrototypeEvidenceBundleArtifact as GddPrototypeRunEvidenceArtifact,
    gdd_prototype_evidence_bundle::GddPrototypeEvidenceBundleArtifact as GddPrototypeEvidenceJournalBundleArtifact,
    gdd_prototype_task_graph::GddPrototypeTaskGraphArtifact,
    gdd_requirement_extraction::GddRequirementExtractionArtifact,
    gdd_scenario_acceptance_plan::GddScenarioAcceptancePlanArtifact,
    gdd_scene_level_plan::GddSceneLevelPlanArtifact,
};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn repo_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn read_repo(path: &str) -> String {
    fs::read_to_string(repo_path(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn manifest() -> Value {
    serde_json::from_str(&read_repo(
        "examples/gdd-to-prototype-demo-v1/demo.manifest.fixture.json",
    ))
    .expect("demo manifest json")
}

fn ref_at<'a>(manifest: &'a Value, key: &str) -> &'a str {
    manifest["artifactRefs"][key]
        .as_str()
        .unwrap_or_else(|| panic!("missing artifact ref {key}"))
}

#[test]
fn gdd_to_prototype_demo_manifest_links_validated_artifacts_end_to_end() {
    let manifest = manifest();
    assert_eq!(manifest["schemaVersion"], "gdd-to-prototype-demo-v1");
    assert_eq!(manifest["issue"], 659);
    assert_eq!(manifest["status"], "evidence-gated-pass");
    assert!(repo_path(manifest["gddRef"].as_str().unwrap()).exists());

    GddDesignBriefArtifact::from_json_str(&read_repo(ref_at(&manifest, "designBrief"))).unwrap();
    GddRequirementExtractionArtifact::from_json_str(&read_repo(ref_at(&manifest, "requirements")))
        .unwrap();
    GddMechanicsMappingArtifact::from_json_str(&read_repo(ref_at(&manifest, "mechanicsMapping")))
        .unwrap();
    GddFeasibilityGateArtifact::from_json_str(&read_repo(ref_at(&manifest, "feasibilityGate")))
        .unwrap();
    GddProjectScaffoldPlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "scaffoldPlan")))
        .unwrap();
    GddSceneLevelPlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "sceneLevelPlan")))
        .unwrap();
    GddGameplayBehaviorPlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "behaviorPlan")))
        .unwrap();
    GddAssetPlaceholderPlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "assetPlan")))
        .unwrap();
    GddScenarioAcceptancePlanArtifact::from_json_str(&read_repo(ref_at(&manifest, "scenarioPlan")))
        .unwrap();
    GddPrototypeTaskGraphArtifact::from_json_str(&read_repo(ref_at(&manifest, "taskGraph")))
        .unwrap();
    GddPrototypeDraftBundleArtifact::from_json_str(&read_repo(ref_at(&manifest, "draftBundle")))
        .unwrap();
    GddPrototypeApplyArtifact::from_json_str(&read_repo(ref_at(&manifest, "reviewApply"))).unwrap();
    GddPrototypeRunEvidenceArtifact::from_json_str(&read_repo(ref_at(&manifest, "runEvidence")))
        .unwrap();
    GddPrototypeEvidenceJournalBundleArtifact::from_json_str(&read_repo(ref_at(
        &manifest,
        "evidenceJournalBundle",
    )))
    .unwrap();
}

#[test]
fn gdd_to_prototype_demo_records_inert_commands_and_cleanup_policy() {
    let manifest = manifest();
    let commands = manifest["inertCommands"]
        .as_array()
        .expect("commands array");
    assert!(commands
        .iter()
        .all(|command| command.as_str().unwrap().starts_with("cargo ")
            || command.as_str().unwrap().starts_with("node ")));
    assert!(manifest["expectedEvidence"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item
            .as_str()
            .unwrap()
            .contains("read-only/read-model compatible")));
    assert!(manifest["cleanupPolicy"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item.as_str().unwrap().contains("remain untracked")));
    assert!(manifest["assetSourceNotes"]
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item.as_str().unwrap().contains("placeholder/local fixture")));
}

#[test]
fn gdd_to_prototype_demo_docs_keep_governance_and_wording_boundaries() {
    let docs = read_repo("docs/gdd-to-prototype-demo-v1.md");
    let manifest = read_repo("examples/gdd-to-prototype-demo-v1/demo.manifest.fixture.json");
    for text in [&docs, &manifest] {
        assert!(text.contains("Issue: #659") || text.contains("\"issue\": 659"));
        assert!(
            text.contains("Generated prototype drafts")
                || text.contains("generated prototype drafts")
        );
        assert!(text.contains("placeholder") || text.contains("placeholders"));
        assert!(text.contains("#1 remains open"));
        assert!(text.contains("#23 remains open"));
        for forbidden in [
            "browser trusted write enabled",
            "auto-apply enabled",
            "auto-merge enabled",
            "autonomous unrestricted game creation enabled",
            "production-ready claim enabled",
            "current Godot replacement is implemented",
            "native export enabled",
            "plugin runtime enabled",
        ] {
            assert!(!text.contains(forbidden), "forbidden wording: {forbidden}");
        }
    }
}
