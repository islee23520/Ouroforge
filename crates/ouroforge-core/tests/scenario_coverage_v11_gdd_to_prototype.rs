//! Scenario Coverage v11: GDD-to-Prototype regression suite (#660).
//!
//! This composes existing stage-level GDD contracts into one matrix so the
//! end-to-end demo does not hide missing malformed/stale/unsupported coverage.

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

const COVERAGE_DOC: &str = include_str!("../../../docs/scenario-coverage-v11-gdd-to-prototype.md");
const COVERAGE_FIXTURE: &str = include_str!(
    "../../../examples/gdd-prototype-regression-suite-v11/coverage-matrix.fixture.json"
);

fn repo_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}

fn read_repo(path: &str) -> String {
    fs::read_to_string(repo_path(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn coverage_fixture() -> Value {
    serde_json::from_str(COVERAGE_FIXTURE).expect("coverage matrix fixture parses")
}

fn fixture_paths(stage: &Value, key: &str) -> Vec<String> {
    stage[key]
        .as_array()
        .unwrap_or_else(|| panic!("{} missing {key}", stage["id"]))
        .iter()
        .map(|value| value.as_str().expect("fixture path string").to_string())
        .collect()
}

fn parse_stage(stage_id: &str, path: &str, text: &str) -> Result<(), String> {
    match stage_id {
        "GDD11.design-brief-schema" => GddDesignBriefArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.requirements" => GddRequirementExtractionArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.mechanics-core-loop" => GddMechanicsMappingArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.feasibility" => GddFeasibilityGateArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.scaffold" => GddProjectScaffoldPlanArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.scene-level" => GddSceneLevelPlanArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.behavior" => GddGameplayBehaviorPlanArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.assets" => GddAssetPlaceholderPlanArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.scenarios" => GddScenarioAcceptancePlanArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.task-graph" => GddPrototypeTaskGraphArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string()),
        "GDD11.bundle-apply-evidence-read-models" => parse_bundle_apply_or_evidence(path, text),
        other => panic!("unknown stage id {other}"),
    }
}

fn parse_bundle_apply_or_evidence(path: &str, text: &str) -> Result<(), String> {
    if path.contains("gdd-prototype-draft-bundle-v1") {
        GddPrototypeDraftBundleArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string())
    } else if path.contains("gdd-prototype-apply-v1") {
        GddPrototypeApplyArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string())
    } else if path.contains("gdd-prototype-evidence-bundle-v1") {
        GddPrototypeEvidenceJournalBundleArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string())
    } else if path.contains("gdd-prototype-evidence-v1") {
        GddPrototypeRunEvidenceArtifact::from_json_str(text)
            .map(|_| ())
            .map_err(|error| error.to_string())
    } else if path.contains("gdd-to-prototype-demo-v1") {
        serde_json::from_str::<Value>(text)
            .map(|_| ())
            .map_err(|error| error.to_string())
    } else {
        panic!("unknown bundle/apply/evidence path {path}");
    }
}

#[test]
fn scenario_coverage_v11_matrix_declares_all_gdd_pipeline_stages() {
    let fixture = coverage_fixture();
    assert_eq!(
        fixture["schemaVersion"],
        "scenario-coverage-v11-gdd-to-prototype-regression-v1"
    );
    assert_eq!(fixture["issue"], 660);

    let stages = fixture["stages"].as_array().expect("stages array");
    let ids: Vec<&str> = stages
        .iter()
        .map(|stage| stage["id"].as_str().expect("stage id"))
        .collect();
    for required in [
        "GDD11.design-brief-schema",
        "GDD11.requirements",
        "GDD11.mechanics-core-loop",
        "GDD11.feasibility",
        "GDD11.scaffold",
        "GDD11.scene-level",
        "GDD11.behavior",
        "GDD11.assets",
        "GDD11.scenarios",
        "GDD11.task-graph",
        "GDD11.bundle-apply-evidence-read-models",
    ] {
        assert!(ids.contains(&required), "missing stage {required}");
        assert!(COVERAGE_DOC.contains(required), "doc missing {required}");
    }

    assert!(stages.len() >= 11);
    let total_positive: usize = stages
        .iter()
        .map(|stage| fixture_paths(stage, "positiveFixtures").len())
        .sum();
    let total_negative: usize = stages
        .iter()
        .map(|stage| fixture_paths(stage, "negativeFixtures").len())
        .sum();
    assert!(
        total_positive >= 40,
        "positive coverage too low: {total_positive}"
    );
    assert!(
        total_negative >= 30,
        "negative coverage too low: {total_negative}"
    );
}

#[test]
fn scenario_coverage_v11_stage_fixtures_validate_and_fail_closed() {
    let fixture = coverage_fixture();
    for stage in fixture["stages"].as_array().expect("stages array") {
        let stage_id = stage["id"].as_str().expect("stage id");
        let doc = stage["doc"].as_str().expect("doc path");
        let contract_test = stage["contractTest"].as_str().expect("contract test path");
        assert!(repo_path(doc).is_file(), "missing doc {doc}");
        assert!(
            repo_path(contract_test).is_file(),
            "missing test {contract_test}"
        );

        for path in fixture_paths(stage, "positiveFixtures") {
            let text = read_repo(&path);
            parse_stage(stage_id, &path, &text)
                .unwrap_or_else(|error| panic!("{stage_id} positive fixture {path}: {error}"));
        }

        for path in fixture_paths(stage, "negativeFixtures") {
            let text = read_repo(&path);
            let error = parse_stage(stage_id, &path, &text)
                .expect_err(&format!("{stage_id} negative fixture should fail: {path}"));
            assert!(!error.trim().is_empty(), "{path} should explain failure");
        }
    }
}

#[test]
fn scenario_coverage_v11_documents_boundaries_governance_and_read_models() {
    let fixture_text = COVERAGE_FIXTURE.to_ascii_lowercase();
    let doc_text = COVERAGE_DOC.to_ascii_lowercase();
    for required in [
        "#1 remains open",
        "#23 remains open",
        "evidence-gated",
        "read-only",
        "draft-only",
        "fixture-scoped",
        "generated",
        "untracked",
        "asset generation remains out of scope",
        "rust/local validators own trusted",
        "studio/cockpit inspection stays escaped and read-only",
        "dashboard read models stay display-only/read-only",
    ] {
        assert!(doc_text.contains(required), "doc missing {required}");
    }
    for required in [
        "no autonomous unrestricted game creation",
        "no arbitrary source mutation",
        "no generated proprietary asset claim",
        "no production game",
        "no current godot replacement",
        "no native export",
        "no plugin runtime",
        "no hosted/cloud",
        "auto-apply",
        "auto-merge",
    ] {
        assert!(
            fixture_text.contains(required),
            "fixture missing {required}"
        );
        assert!(doc_text.contains(required), "doc missing {required}");
    }

    for forbidden in [
        "browser trusted write enabled",
        "auto-apply enabled",
        "auto-merge enabled",
        "autonomous unrestricted game creation enabled",
        "generated proprietary assets are included",
        "production-ready claim enabled",
        "current godot replacement is implemented",
        "native export enabled",
        "plugin runtime enabled",
        "hosted/cloud service enabled",
    ] {
        assert!(
            !COVERAGE_DOC.contains(forbidden),
            "forbidden wording: {forbidden}"
        );
        assert!(
            !COVERAGE_FIXTURE.contains(forbidden),
            "forbidden fixture wording: {forbidden}"
        );
    }
}
