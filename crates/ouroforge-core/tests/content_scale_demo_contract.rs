//! Demo smoke test for Content-at-Scale Generation and Curation Demo v1 (#1653).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. This is the deterministic end-to-end demo for one genre
//! (grid-puzzle): a campaign is generated through the front door (#1649), its
//! novelty is measured (#1650) and its whole-game difficulty curve verified
//! (#1651), and the curation gate (#1652) admits a solvable/balanced/novel set
//! with a verified curve while rejecting weak content (low-novelty and a
//! curve-spike campaign).
//!
//! It reuses the existing surfaces only — generation, the engine-room solver
//! (Milestone 28), the novelty metric, the difficulty-curve verifier, and the
//! evaluator curation gate. It reproduces deterministically, with no network or
//! live browser, over fixture-scoped inputs under
//! `examples/content-scale-v1/demo/`. It asserts gate states, not subjective
//! quality.

use std::path::PathBuf;

use ouroforge_core::content_difficulty_curve::{verify_curve, CurveInput};
use ouroforge_core::content_novelty::{compute_novelty, DEFAULT_NOVELTY_THRESHOLD};
use ouroforge_core::content_scale_generation::{
    generate_campaign, CampaignBrief, CampaignProposalSet,
};
use ouroforge_core::puzzle_solver::{solve, SolveBudget};
use ouroforge_evaluator::content_curation_gate::{
    evaluate_content_curation_check, ContentCurationCheck, ContentCurationGateState,
    CurationEvidence, CONTENT_CURATION_GATE_SCHEMA_VERSION,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn demo_path(name: &str) -> PathBuf {
    repo_root()
        .join("examples/content-scale-v1/demo")
        .join(name)
}

fn generated_set(name: &str) -> CampaignProposalSet {
    let text =
        std::fs::read_to_string(demo_path(name)).unwrap_or_else(|_| panic!("fixture: {name}"));
    let brief = CampaignBrief::from_json_str(&text).expect("campaign brief parses");
    generate_campaign(&brief, FIXED_NOW_MS).expect("campaign generates")
}

fn curve_input(name: &str) -> CurveInput {
    let text =
        std::fs::read_to_string(demo_path(name)).unwrap_or_else(|_| panic!("fixture: {name}"));
    CurveInput::from_json_str(&text).expect("curve input parses")
}

/// Count how many of a generated grid-puzzle set's proposals are solvable on the
/// engine-room solver — real solvability evidence, not an assertion.
fn count_solvable(set: &CampaignProposalSet) -> usize {
    set.proposals
        .iter()
        .filter(|g| {
            let artifact: Value = serde_json::from_str(&g.proposal.to).expect("artifact parses");
            solve(&artifact, SolveBudget::default())
                .map(|o| o.is_solvable())
                .unwrap_or(false)
        })
        .count()
}

/// Build a curation check from the composed pipeline facts, with all four
/// evidence dimensions declared.
fn curation_check(
    campaign_id: &str,
    levels_total: usize,
    levels_solvable: usize,
    balanced: bool,
    low_novelty: bool,
    curve_passed: bool,
    curve_finding_count: usize,
) -> ContentCurationCheck {
    ContentCurationCheck {
        schema_version: CONTENT_CURATION_GATE_SCHEMA_VERSION.to_string(),
        campaign_id: campaign_id.to_string(),
        levels_total,
        levels_solvable,
        balanced,
        low_novelty,
        curve_passed,
        curve_finding_count,
        evidence: CurationEvidence {
            solver: vec!["evidence/demo/solver.json".to_string()],
            balance: vec!["evidence/demo/balance.json".to_string()],
            novelty: vec!["evidence/demo/novelty.json".to_string()],
            curve: vec!["evidence/demo/curve.json".to_string()],
        },
    }
}

#[test]
fn admitted_campaign_passes_generation_curation_and_curve() {
    // Generation -> a set of distinct, solvable grid-puzzle proposals.
    let set = generated_set("admitted-campaign.json");
    assert_eq!(set.proposals.len(), 3);
    let solvable = count_solvable(&set);
    assert_eq!(solvable, 3, "every admitted level must be solvable");

    // Novelty -> a distinct set is not low-novelty.
    let novelty = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &Default::default())
        .expect("novelty computes");
    assert!(!novelty.low_novelty);
    assert_eq!(novelty.duplicate_count, 0);

    // Curve -> the declared whole-game curve is verified.
    let curve = verify_curve(&curve_input("admitted-curve.json")).expect("curve verifies");
    assert!(
        curve.passed,
        "admitted curve must be verified: {:?}",
        curve.findings
    );

    // Curation -> the campaign is admitted.
    let check = curation_check(
        &set.campaign_id,
        set.proposals.len(),
        solvable,
        true,
        novelty.low_novelty,
        curve.passed,
        curve.findings.len(),
    );
    let verdict = evaluate_content_curation_check(&check);
    assert_eq!(verdict.state, ContentCurationGateState::Pass);
}

#[test]
fn low_novelty_campaign_is_curated_out() {
    // Generation -> a repetitive set; novelty flags it low.
    let set = generated_set("low-novelty-campaign.json");
    assert_eq!(
        count_solvable(&set),
        3,
        "the repeated level is still solvable"
    );
    let novelty = compute_novelty(&set, DEFAULT_NOVELTY_THRESHOLD, &Default::default())
        .expect("novelty computes");
    assert!(novelty.low_novelty, "repeated content must be low-novelty");

    // Curation -> rejected for low novelty (curve verified, but novelty fails).
    let curve = verify_curve(&curve_input("admitted-curve.json")).expect("curve verifies");
    let check = curation_check(
        &set.campaign_id,
        set.proposals.len(),
        count_solvable(&set),
        true,
        novelty.low_novelty,
        curve.passed,
        curve.findings.len(),
    );
    let verdict = evaluate_content_curation_check(&check);
    assert_eq!(verdict.state, ContentCurationGateState::LowNovelty);
}

#[test]
fn curve_spike_campaign_is_curated_out() {
    // A campaign whose whole-game curve has a spike/regression is rejected even
    // when its levels are solvable and novel.
    let curve = verify_curve(&curve_input("spike-curve.json")).expect("curve verifies");
    assert!(!curve.passed);
    assert!(curve.spikes >= 1 && curve.regressions >= 1);

    let check = curation_check(
        "content-scale-demo-spike-v1",
        3,
        3,
        true,
        false,
        curve.passed,
        curve.findings.len(),
    );
    let verdict = evaluate_content_curation_check(&check);
    assert_eq!(verdict.state, ContentCurationGateState::CurveViolation);
}

#[test]
fn demo_doc_records_the_flow() {
    let doc = std::fs::read_to_string(repo_root().join("docs/content-scale-v1-demo.md"))
        .expect("content-scale demo doc exists");
    assert!(doc.contains("#1653"), "demo doc records this issue (#1653)");
    assert!(
        doc.contains("curation") && doc.contains("curve"),
        "demo doc records the curation + curve flow"
    );
}
