//! Deterministic demo smoke test for Asset Generation and QA Demo v1 (#1638).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! This composes the already-merged Asset Generation Proposal Model (#1635), the
//! Asset-QA Gate (#1636), and the Asset Import/Atlas Path (#1637) into one
//! end-to-end walkthrough and asserts the documented behavior: an asset is
//! generated as a proposal, is blocked by asset-QA when style-inconsistent, and
//! is promotable only when it passes — with promotion routed through the asset-QA
//! gate. It asserts behavior and gate states, never subjective quality, and runs
//! with no network and no live browser.

use std::path::PathBuf;

use ouroforge_core::asset_generation_proposal::{generate_asset_proposal, AssetGenerationBrief};
use ouroforge_core::asset_import::{
    enforce_asset_qa, AssetImportReport, ImportedAsset, ASSET_IMPORT_SCHEMA_VERSION,
};
use ouroforge_core::{ProjectAssetClassification, ProjectAssetType};
use ouroforge_evaluator::asset_qa_gate::{evaluate_asset_qa_check, AssetQaCheck, AssetQaGateState};

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn demo_dir() -> PathBuf {
    repo_root().join("examples/asset-pipeline-v1/demo")
}

fn read_brief(name: &str) -> AssetGenerationBrief {
    let text = std::fs::read_to_string(demo_dir().join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {name}"));
    AssetGenerationBrief::from_json_str(&text).expect("demo brief parses")
}

fn read_check(name: &str) -> AssetQaCheck {
    let text = std::fs::read_to_string(demo_dir().join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {name}"));
    AssetQaCheck::from_json_str(&text).expect("demo check parses")
}

/// A runtime-shaped import report carrying one generated asset under the given
/// id (what the import promotion gate from #1637 enforces asset-QA over).
fn generated_import(asset_id: &str) -> AssetImportReport {
    AssetImportReport {
        schema_version: ASSET_IMPORT_SCHEMA_VERSION.to_string(),
        manifest_id: "demo".to_string(),
        assets: vec![ImportedAsset {
            id: asset_id.to_string(),
            asset_type: ProjectAssetType::Image,
            classification: ProjectAssetClassification::Generated,
            atlas_frame_count: 0,
            atlas_animation_count: 0,
        }],
        atlas_frame_total: 0,
        atlas_animation_total: 0,
    }
}

#[test]
fn demo_generates_a_proposal_then_blocks_then_promotes() {
    // 1. Generate: the brief becomes a proposal carrying license/provenance,
    //    proposal-only (proposed / pending / unverified). Never auto-promoted.
    let brief = read_brief("asset-brief.json");
    let generated =
        generate_asset_proposal(&brief, FIXED_NOW_MS).expect("demo brief generates a proposal");
    assert_eq!(generated.proposal.status, "proposed");
    assert_eq!(generated.proposal.verdict_status, "pending");
    assert_eq!(generated.proposal.confidence, "unverified");
    assert!(generated.provenance.proposal_only);
    assert_eq!(generated.provenance.license.license, "CC-BY-4.0");
    let asset_id = "demo-hero-idle";

    // 2. Asset-QA gate — blocked: the style-inconsistent check fails the gate.
    let blocked_check = read_check("asset-qa-blocked.json");
    let blocked_verdict = evaluate_asset_qa_check(&blocked_check);
    assert_eq!(blocked_verdict.state, AssetQaGateState::StyleInconsistent);
    assert!(!blocked_verdict.state.is_pass());

    // The blocked verdict cannot promote the generated asset: import routing
    // through the asset-QA gate (#1637) fails closed.
    let import = generated_import(asset_id);
    let blocked = enforce_asset_qa(&import, std::slice::from_ref(&blocked_verdict));
    assert!(
        blocked.is_err(),
        "a style-inconsistent asset must not be promotable"
    );
    // With no asset-QA verdict at all, promotion is likewise blocked.
    assert!(enforce_asset_qa(&import, &[]).is_err());

    // 3. Asset-QA gate — pass — promotion routing: the passing check clears the
    //    gate and the same generated asset becomes promotable.
    let pass_check = read_check("asset-qa-pass.json");
    let pass_verdict = evaluate_asset_qa_check(&pass_check);
    assert_eq!(pass_verdict.state, AssetQaGateState::Pass);
    assert_eq!(pass_verdict.asset_id, asset_id);

    enforce_asset_qa(&import, std::slice::from_ref(&pass_verdict))
        .expect("a verified asset with a passing asset-QA verdict is promotable");
}

#[test]
fn demo_doc_records_the_walkthrough() {
    let doc = std::fs::read_to_string(repo_root().join("docs/asset-pipeline-v1-demo.md"))
        .expect("demo doc exists");
    assert!(doc.contains("#1638"), "doc records the demo issue");
    assert!(
        doc.contains("proposal-only") && doc.contains("fails closed"),
        "doc records proposal-only generation and the fail-closed gate"
    );
    assert!(
        doc.contains("#1635") && doc.contains("#1636") && doc.contains("#1637"),
        "doc records the composed follow-ups"
    );
}
