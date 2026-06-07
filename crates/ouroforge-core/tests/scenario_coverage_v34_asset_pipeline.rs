//! Scenario Coverage v34 — Asset Pipeline Regression Suite (#1639).
//!
//! Locks Asset Generation and Asset-QA v1 behavior: asset proposal generation
//! (#1635), the asset-QA gate (#1636), asset import/atlas (#1637), and the
//! backward-compatibility guarantee that the existing four-gate
//! `declared-gate-and` aggregation and visual gate remain valid. State/shape
//! assertions only — no flaky or timing-based checks.

use std::fs;
use std::path::PathBuf;

use ouroforge_core::asset_generation_proposal::{generate_asset_proposal, AssetGenerationBrief};
use ouroforge_core::asset_import::{import_validated_assets, validate_manifest};
use ouroforge_core::ProjectAssetManifest;
use ouroforge_evaluator::asset_qa_gate::{
    compose_asset_qa_into_categories, evaluate_asset_qa_check, evaluate_asset_qa_gate, AssetQaCheck,
};
use ouroforge_evaluator::{evaluation_gate_categories, VisualGateState, VisualGateVerdict};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn coverage_root() -> PathBuf {
    repo_root().join("examples/asset-pipeline-v1/scenario-coverage-v34")
}

fn import_fixture_root() -> PathBuf {
    repo_root().join("examples/asset-pipeline-v1-regression")
}

fn read_text(relative: &str) -> String {
    let path = coverage_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn matrix() -> Value {
    serde_json::from_str(&read_text("matrix.fixture.json")).expect("matrix parses")
}

fn reject_reason(case: &Value) -> &str {
    case["rejectReason"]
        .as_str()
        .expect("rejected case has a reason")
}

#[test]
fn v34_matrix_header_is_enumerated() {
    let matrix = matrix();
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v34-asset-pipeline-matrix-v1"
    );
    assert_eq!(matrix["issue"], 1639);
    assert_eq!(
        matrix["proposalCases"].as_array().expect("proposal").len(),
        3
    );
    assert_eq!(matrix["qaCases"].as_array().expect("qa").len(), 3);
    assert_eq!(matrix["importCases"].as_array().expect("import").len(), 3);
}

#[test]
fn v34_asset_proposal_cases() {
    for case in matrix()["proposalCases"]
        .as_array()
        .expect("proposal cases")
    {
        let fixture = case["fixture"].as_str().expect("fixture");
        let brief = AssetGenerationBrief::from_json_str(&read_text(fixture))
            .unwrap_or_else(|error| panic!("{}: parse {error}", case["id"]));
        let result = generate_asset_proposal(&brief, FIXED_NOW_MS);
        match case["expectedOutcome"].as_str().expect("outcome") {
            "proposed" => {
                let generated = result.unwrap_or_else(|error| panic!("{}: {error}", case["id"]));
                assert_eq!(generated.proposal.status, "proposed");
                assert_eq!(generated.proposal.verdict_status, "pending");
                assert!(generated.provenance.proposal_only);
            }
            "rejected" => {
                let error = result.expect_err(&format!("{} must be rejected", case["id"]));
                assert!(
                    error.to_string().contains(reject_reason(case)),
                    "{}: unexpected error {error}",
                    case["id"]
                );
            }
            other => panic!("unknown proposal outcome {other}"),
        }
    }
}

#[test]
fn v34_asset_qa_cases() {
    for case in matrix()["qaCases"].as_array().expect("qa cases") {
        let fixture = case["fixture"].as_str().expect("fixture");
        let check: AssetQaCheck =
            serde_json::from_str(&read_text(fixture)).expect("qa check parses");
        let verdict = evaluate_asset_qa_check(&check);
        let state = serde_json::to_value(verdict.state).expect("state serializes");
        assert_eq!(
            state, case["expectedState"],
            "{}: unexpected asset-QA state",
            case["id"]
        );
    }
}

#[test]
fn v34_asset_import_cases() {
    for case in matrix()["importCases"].as_array().expect("import cases") {
        let manifest_rel = case["manifest"].as_str().expect("manifest");
        let text = fs::read_to_string(import_fixture_root().join(manifest_rel))
            .unwrap_or_else(|error| panic!("read manifest {manifest_rel}: {error}"));
        let manifest = ProjectAssetManifest::from_json_str(&text).expect("manifest schema parses");
        match case["expectedOutcome"].as_str().expect("outcome") {
            "imported" => {
                let report = import_validated_assets(&manifest, &import_fixture_root(), &[])
                    .unwrap_or_else(|error| panic!("{}: {error}", case["id"]));
                assert_eq!(
                    report.assets.len() as u64,
                    case["expectedAssetCount"].as_u64().expect("count"),
                    "{}: asset count",
                    case["id"]
                );
            }
            "rejected" => {
                let error = validate_manifest(&manifest, &import_fixture_root())
                    .expect_err(&format!("{} must be rejected", case["id"]));
                assert!(
                    error.to_string().contains(reject_reason(case)),
                    "{}: unexpected error {error}",
                    case["id"]
                );
            }
            other => panic!("unknown import outcome {other}"),
        }
    }
}

#[test]
fn v34_backward_compatibility_four_gate_and_visual_remain_valid() {
    // The existing four-gate aggregation with a passing visual gate is unchanged.
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
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
    assert_eq!(categories["aggregation"]["undeclaredGatePolicy"], "neutral");
    assert_eq!(categories["visual"]["declared"], true);
    assert_eq!(categories["visual"]["status"], "pass");
    assert!(categories.get("assetQa").is_none());

    // Composing the asset-QA gate is additive: the base aggregation is unchanged
    // and a new declared `assetQa` category is ANDed in.
    let verdicts =
        evaluate_asset_qa_gate(&[
            serde_json::from_str::<AssetQaCheck>(&read_text("qa/pass.json"))
                .expect("pass check parses"),
        ]);
    assert!(compose_asset_qa_into_categories(&mut categories, &verdicts));
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
    assert_eq!(categories["visual"]["status"], "pass");
    assert_eq!(categories["assetQa"]["declared"], true);
    assert_eq!(categories["assetQa"]["status"], "pass");
}

#[test]
fn v34_docs_and_fixtures_preserve_generated_state_wording_and_governance() {
    let docs =
        fs::read_to_string(repo_root().join("docs/scenario-coverage-v34.md")).expect("v34 docs");
    assert!(docs.contains("#1639"), "docs record the coverage issue");
    assert!(
        docs.contains("#1") && docs.contains("#23"),
        "docs record the governance anchors"
    );

    // Conservative wording across the docs and the matrix: no quality/fun/parity
    // or auto-merge claims.
    let mut corpus = docs;
    corpus.push_str(&read_text("matrix.fixture.json"));
    // Overclaim phrases that must never appear (negative-context mentions like
    // "no auto-merge" are governance language and are intentionally allowed).
    for forbidden in [
        "production-ready",
        "Godot replacement",
        "Godot parity",
        "is fun",
        "looks good",
    ] {
        assert!(
            !corpus.contains(forbidden),
            "forbidden wording present: {forbidden}"
        );
    }
}
