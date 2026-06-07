//! Contract test for Asset-QA Gate v1 (#1636).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! These tests machine-check the asset-QA gate contract: a clean asset passes;
//! a style-inconsistent, regressing, format-invalid, or missing-provenance asset
//! fails; malformed evidence fails closed; a missing/non-comparable baseline is
//! insufficient-data (never a silent pass); and the gate composes into the
//! existing four-gate `declared-gate-and` aggregation.
//!
//! The gate lives in `ouroforge-evaluator` (it reuses the visual gate and the
//! evaluator gate vocabulary); `ouroforge-core` depends on the evaluator, so the
//! contract test exercises it here through the public evaluator API.

use std::path::PathBuf;

use ouroforge_evaluator::asset_qa_gate::{
    asset_qa_gate_category, compose_asset_qa_into_categories, evaluate_asset_qa_check,
    evaluate_asset_qa_gate, AssetQaCheck, AssetQaGateState,
};
use ouroforge_evaluator::{evaluation_gate_categories, VisualGateState, VisualGateVerdict};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_check(name: &str) -> AssetQaCheck {
    let path: PathBuf = repo_root().join("examples/asset-qa").join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    AssetQaCheck::from_json_str(&text).expect("fixture check parses")
}

#[test]
fn clean_asset_passes_the_gate() {
    let check = read_check("pass.json");
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::Pass);
    assert!(verdict.state.is_pass());
    assert_eq!(verdict.asset_id, "hero-idle");
    assert_eq!(verdict.evidence_refs.len(), 3);
}

#[test]
fn style_inconsistent_asset_fails() {
    let check = read_check("style-inconsistent.json");
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::StyleInconsistent);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("style baseline"));
}

#[test]
fn missing_provenance_asset_fails() {
    let check = read_check("missing-provenance.json");
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::MissingProvenance);
    assert!(verdict.reason.contains("license/provenance"));
}

#[test]
fn malformed_evidence_fails_closed() {
    let check = read_check("malformed-evidence.json");
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::MalformedEvidence);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("evidence references"));
}

#[test]
fn visual_regression_fails() {
    let mut check = read_check("pass.json");
    check.visual_regression = VisualGateState::Fail;
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::VisualRegression);
    assert!(verdict
        .reason
        .contains("regresses against the prior accepted baseline"));
}

#[test]
fn invalid_format_fails() {
    let mut check = read_check("pass.json");
    check.format_resolution_valid = false;
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::FormatInvalid);
}

#[test]
fn missing_baseline_is_insufficient_data_not_a_silent_pass() {
    let mut check = read_check("pass.json");
    check.visual_regression = VisualGateState::MissingBaseline;
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::InsufficientData);
    assert!(!verdict.state.is_pass());
}

#[test]
fn wrong_schema_version_fails_closed() {
    let mut check = read_check("pass.json");
    check.schema_version = "ouroforge.asset-qa-gate.v0".to_string();
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::MalformedEvidence);
}

#[test]
fn provenance_precedence_over_style_and_format() {
    // A check failing multiple dimensions reports the highest-precedence
    // failure: missing-provenance before format before style.
    let mut check = read_check("pass.json");
    check.license_provenance_complete = false;
    check.format_resolution_valid = false;
    check.style_consistency = VisualGateState::Fail;
    let verdict = evaluate_asset_qa_check(&check);
    assert_eq!(verdict.state, AssetQaGateState::MissingProvenance);
}

#[test]
fn gate_category_aggregates_pass_and_fail() {
    let pass = evaluate_asset_qa_gate(&[read_check("pass.json")]);
    let category = asset_qa_gate_category(&pass).expect("declared gate has a category");
    assert_eq!(category["declared"], true);
    assert_eq!(category["status"], "pass");
    assert_eq!(category["resultCount"], 1);
    assert_eq!(category["failureCount"], 0);

    let mixed = evaluate_asset_qa_gate(&[
        read_check("pass.json"),
        read_check("style-inconsistent.json"),
    ]);
    let category = asset_qa_gate_category(&mixed).expect("declared gate has a category");
    assert_eq!(category["status"], "fail");
    assert_eq!(category["resultCount"], 2);
    assert_eq!(category["failureCount"], 1);
}

#[test]
fn undeclared_gate_is_neutral() {
    // No asset-QA checks: the gate is undeclared and contributes no category,
    // preserving undeclaredGatePolicy: neutral.
    assert!(asset_qa_gate_category(&[]).is_none());
    let mut categories = serde_json::json!({ "aggregation": { "operator": "declared-gate-and" } });
    let added = compose_asset_qa_into_categories(&mut categories, &[]);
    assert!(!added);
    assert!(categories.get("assetQa").is_none());
}

#[test]
fn composes_into_the_existing_four_gate_aggregation() {
    // Build a real four-gate categories object with a passing visual gate, then
    // compose the asset-QA gate into the same declared-gate-and aggregation.
    let visual = vec![VisualGateVerdict {
        scenario_id: "scene-1".to_string(),
        checkpoint_id: "cp-1".to_string(),
        state: VisualGateState::Pass,
        reason: "ok".to_string(),
        comparison_ref: "compare/cp-1.json".to_string(),
        changed_pixels: Some(0),
        changed_percent_x1000: Some(0),
        changed_region_count: 0,
        threshold_summary: vec![],
        evidence_refs: vec!["evidence/visual/cp-1.json".to_string()],
        output_root: "runs/run-1".to_string(),
    }];
    let mut categories =
        evaluation_gate_categories(1, 0, &[], &visual, &[]).expect("four-gate categories present");
    // The base aggregation is the existing declared-gate-and / neutral policy.
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
    assert_eq!(categories["aggregation"]["undeclaredGatePolicy"], "neutral");
    assert!(categories.get("assetQa").is_none());

    let verdicts = evaluate_asset_qa_gate(&[read_check("pass.json")]);
    let added = compose_asset_qa_into_categories(&mut categories, &verdicts);
    assert!(added);
    // The asset-QA gate is now one more declared category ANDed with the four.
    assert_eq!(categories["assetQa"]["declared"], true);
    assert_eq!(categories["assetQa"]["status"], "pass");
    assert_eq!(categories["visual"]["status"], "pass");
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
}

#[test]
fn docs_record_the_asset_qa_gate_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/asset-pipeline-design.md"))
        .expect("asset pipeline design doc exists");
    assert!(
        doc.contains("#1636"),
        "design gate doc records the asset-QA gate follow-up (#1636)"
    );
    assert!(
        doc.contains("declared-gate-and"),
        "doc records composition with the existing aggregation"
    );
    assert!(
        doc.contains("fails closed"),
        "doc records the fail-closed contract"
    );
}
