//! Contract test for Content Curation Gate v1 (#1652).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. These tests machine-check the curation gate contract: a
//! campaign that is fully solvable, balanced, novel, and curve-verified is
//! admitted; a campaign that fails any one dimension (unsolvable, imbalanced,
//! low-novelty, curve-spike) is rejected with the matching reason; malformed
//! evidence fails closed; and the gate composes into the existing
//! `declared-gate-and` aggregation as one more declared category — not a parallel
//! evaluator.
//!
//! The gate lives in `ouroforge-evaluator` (it reuses the evaluator gate
//! vocabulary and aggregation); `ouroforge-core` depends on the evaluator, so the
//! contract test exercises it here through the public evaluator API.

use std::path::PathBuf;

use ouroforge_evaluator::content_curation_gate::{
    compose_content_curation_into_categories, content_curation_gate_category,
    evaluate_content_curation_check, evaluate_content_curation_gate, ContentCurationCheck,
    ContentCurationGateState,
};
use ouroforge_evaluator::{evaluation_gate_categories, VisualGateState, VisualGateVerdict};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_check(name: &str) -> ContentCurationCheck {
    let path: PathBuf = repo_root().join("examples/content-curation").join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    ContentCurationCheck::from_json_str(&text).expect("fixture check parses")
}

#[test]
fn fully_curated_campaign_is_admitted() {
    let check = read_check("admit.json");
    let verdict = evaluate_content_curation_check(&check);
    assert_eq!(verdict.state, ContentCurationGateState::Pass);
    assert!(verdict.state.is_pass());
    assert_eq!(verdict.campaign_id, "curation-admit-v1");
    assert_eq!(verdict.evidence_refs.len(), 4);
}

#[test]
fn unsolvable_campaign_is_rejected() {
    let verdict = evaluate_content_curation_check(&read_check("reject-unsolvable.json"));
    assert_eq!(verdict.state, ContentCurationGateState::Unsolvable);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("not solvable"));
}

#[test]
fn imbalanced_campaign_is_rejected() {
    let verdict = evaluate_content_curation_check(&read_check("reject-imbalanced.json"));
    assert_eq!(verdict.state, ContentCurationGateState::Imbalanced);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("balanced"));
}

#[test]
fn low_novelty_campaign_is_rejected() {
    let verdict = evaluate_content_curation_check(&read_check("reject-low-novelty.json"));
    assert_eq!(verdict.state, ContentCurationGateState::LowNovelty);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("novelty"));
}

#[test]
fn curve_spike_campaign_is_rejected() {
    let verdict = evaluate_content_curation_check(&read_check("reject-curve-spike.json"));
    assert_eq!(verdict.state, ContentCurationGateState::CurveViolation);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("curve"));
}

#[test]
fn malformed_evidence_fails_closed() {
    let verdict = evaluate_content_curation_check(&read_check("malformed.json"));
    assert_eq!(verdict.state, ContentCurationGateState::MalformedEvidence);
    assert!(!verdict.state.is_pass());

    // levelsSolvable > levelsTotal is also malformed, not silently passed.
    let mut bad = read_check("admit.json");
    bad.levels_solvable = bad.levels_total + 1;
    let verdict = evaluate_content_curation_check(&bad);
    assert_eq!(verdict.state, ContentCurationGateState::MalformedEvidence);
}

#[test]
fn missing_evidence_dimension_fails_closed() {
    // A check with all-pass booleans but a missing required evidence dimension
    // (here: curve) must not be admitted; admission requires solver, balance,
    // novelty, AND curve evidence.
    let verdict =
        evaluate_content_curation_check(&read_check("malformed-missing-curve-evidence.json"));
    assert_eq!(verdict.state, ContentCurationGateState::MalformedEvidence);
    assert!(verdict.reason.contains("curve evidence"));

    // Dropping any one dimension from an otherwise-admittable check fails closed.
    for drop in ["solver", "balance", "novelty", "curve"] {
        let mut check = read_check("admit.json");
        match drop {
            "solver" => check.evidence.solver.clear(),
            "balance" => check.evidence.balance.clear(),
            "novelty" => check.evidence.novelty.clear(),
            _ => check.evidence.curve.clear(),
        }
        let verdict = evaluate_content_curation_check(&check);
        assert_eq!(
            verdict.state,
            ContentCurationGateState::MalformedEvidence,
            "missing {drop} evidence must fail closed"
        );
    }
}

#[test]
fn gate_category_aggregates_pass_and_fail() {
    let pass = evaluate_content_curation_gate(&[read_check("admit.json")]);
    let category = content_curation_gate_category(&pass).expect("declared gate has a category");
    assert_eq!(category["declared"], true);
    assert_eq!(category["status"], "pass");
    assert_eq!(category["resultCount"], 1);
    assert_eq!(category["failureCount"], 0);

    let mixed = evaluate_content_curation_gate(&[
        read_check("admit.json"),
        read_check("reject-unsolvable.json"),
    ]);
    let category = content_curation_gate_category(&mixed).expect("declared gate has a category");
    assert_eq!(category["status"], "fail");
    assert_eq!(category["resultCount"], 2);
    assert_eq!(category["failureCount"], 1);
}

#[test]
fn undeclared_gate_is_neutral() {
    assert!(content_curation_gate_category(&[]).is_none());
    let mut categories = serde_json::json!({ "aggregation": { "operator": "declared-gate-and" } });
    let added = compose_content_curation_into_categories(&mut categories, &[]);
    assert!(!added);
    assert!(categories.get("contentCuration").is_none());
}

#[test]
fn composes_into_the_existing_aggregation() {
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
    assert!(categories.get("contentCuration").is_none());

    // A passing curation gate composes as one more declared, passing category.
    let pass = evaluate_content_curation_gate(&[read_check("admit.json")]);
    assert!(compose_content_curation_into_categories(
        &mut categories,
        &pass
    ));
    assert_eq!(categories["contentCuration"]["declared"], true);
    assert_eq!(categories["contentCuration"]["status"], "pass");
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");

    // A failing curation gate marks the composed category fail (ANDs to a block).
    let mut categories2 = evaluation_gate_categories(1, 0, &[], &visual, &[]).unwrap();
    let fail = evaluate_content_curation_gate(&[read_check("reject-curve-spike.json")]);
    assert!(compose_content_curation_into_categories(
        &mut categories2,
        &fail
    ));
    assert_eq!(categories2["contentCuration"]["status"], "fail");
}

#[test]
fn docs_record_the_curation_gate_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/content-curation-gate-v1.md"))
        .expect("content-curation-gate doc exists");
    assert!(
        doc.contains("#1652"),
        "Content-Curation-Gate v1 doc records this issue (#1652)"
    );
    assert!(
        doc.contains("declared-gate-and"),
        "doc records composition via the declared-gate-and aggregation"
    );
}
