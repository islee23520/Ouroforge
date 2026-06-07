//! Contract test for Engine-Room Promotion Guard v1 (#1594).
//!
//! Part of Generative Front Door v1 (#1592) under #1 Era F Milestone 30. These
//! tests machine-check the promotion precondition: a generated proposal (the
//! front door, #1593) may only be promoted once it passes the ENGINE ROOM — the
//! deterministic solver (#1580) and over-solution detector (#1581) produce the
//! facts, the Milestone 28 design-integrity gate (#1583) turns them into a
//! declared verdict, and that verdict is ANDed into the existing four-gate
//! `declared-gate-and` aggregation.
//!
//! Three cases are exercised end to end through the real front-door intake:
//!   1. a passing proposal is promotable and routed UNCHANGED to the existing
//!      review/apply/trust-gradient path;
//!   2. a gate-failing proposal (intent not satisfied) is blocked with evidence;
//!   3. an over-solution proposal is blocked with counterexample evidence.
//!
//! Boundary checks assert the proposal-only model: the guard never mutates the
//! proposal, never approves or applies it, and withholds every blocked proposal
//! from the apply path.

use std::path::PathBuf;

use ouroforge_core::generative_intake::{intake_brief, GenerativeBrief, GenerativeProposal};
use ouroforge_core::generative_promotion_guard::{
    evaluate_promotion, PromotionGuardOutcome, GENERATIVE_PROMOTION_GUARD_SCHEMA_VERSION,
};
use ouroforge_core::puzzle_solver::SolveBudget;
use ouroforge_evaluator::design_integrity_gate::DesignIntegrityGateState;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn proposal_from_brief(name: &str) -> GenerativeProposal {
    let path = repo_root()
        .join("examples/generative-front-door")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    let brief = GenerativeBrief::from_json_str(&text).expect("fixture brief parses");
    intake_brief(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted")
}

#[test]
fn passing_proposal_is_promotable_and_routed_unchanged() {
    let generative = proposal_from_brief("generative-promotion-pass-brief-v1.json");
    let verdict = evaluate_promotion(&generative, SolveBudget::default()).expect("guard evaluates");

    // The engine room passed.
    assert_eq!(verdict.gate_state(), DesignIntegrityGateState::Pass);
    assert!(verdict.is_promotable(), "proposal should be promotable");
    assert_eq!(
        verdict.schema_version,
        GENERATIVE_PROMOTION_GUARD_SCHEMA_VERSION
    );
    assert_eq!(verdict.proposal_id, generative.proposal.id);
    assert_eq!(verdict.brief_id, generative.provenance.brief_id);

    // The design-integrity gate is ANDed into the existing four-gate
    // declared-gate-and aggregation as one more declared category.
    assert_eq!(
        verdict.engine_room_categories["aggregation"]["operator"],
        "declared-gate-and"
    );
    // Promotion requires the design-integrity gate to be DECLARED and passing in
    // the composed four-gate aggregation — it is never neutralized away.
    assert_eq!(
        verdict.engine_room_categories["designIntegrity"]["declared"],
        true
    );
    assert_eq!(
        verdict.engine_room_categories["designIntegrity"]["status"],
        "pass"
    );

    match &verdict.outcome {
        PromotionGuardOutcome::Promotable { gate_evidence_refs } => {
            assert!(
                !gate_evidence_refs.is_empty(),
                "pass carries engine-room evidence refs"
            );
        }
        other => panic!("expected Promotable, got {other:?}"),
    }

    // Routing: the proposal handed to the existing review/apply/trust-gradient
    // path is the *same* proposal, UNCHANGED. The guard removes only the
    // engine-room block; it does not approve, apply, or alter status.
    let routed = verdict
        .promotable_proposal()
        .expect("promotable proposal is routed");
    assert_eq!(routed, &generative.proposal, "proposal routed unchanged");
    assert_eq!(routed.status, "proposed");
    assert_eq!(routed.verdict_status, "pending");
    assert_eq!(routed.confidence, "unverified");
}

#[test]
fn gate_failing_proposal_is_blocked_with_evidence() {
    let generative = proposal_from_brief("generative-promotion-gate-fail-brief-v1.json");
    let verdict = evaluate_promotion(&generative, SolveBudget::default()).expect("guard evaluates");

    assert!(
        !verdict.is_promotable(),
        "gate-failing proposal is not promotable"
    );
    assert_eq!(
        verdict.gate_state(),
        DesignIntegrityGateState::IntentUnsatisfied,
        "intended solution does not reach win"
    );
    assert_eq!(
        verdict.engine_room_categories["designIntegrity"]["status"],
        "fail"
    );

    match &verdict.outcome {
        PromotionGuardOutcome::Blocked {
            reason,
            evidence_refs,
        } => {
            assert!(reason.contains("blocked"), "reason states it is blocked");
            assert!(
                reason.contains("intent-unsatisfied"),
                "reason names the failing gate dimension"
            );
            assert!(
                !evidence_refs.is_empty(),
                "block carries an evidence reference"
            );
        }
        other => panic!("expected Blocked, got {other:?}"),
    }

    // A blocked proposal is withheld from the apply path.
    assert!(
        verdict.promotable_proposal().is_none(),
        "blocked proposal must not be routed"
    );
}

#[test]
fn over_solution_proposal_is_blocked_with_counterexamples() {
    let generative = proposal_from_brief("generative-promotion-oversolution-brief-v1.json");
    let verdict = evaluate_promotion(&generative, SolveBudget::default()).expect("guard evaluates");

    assert!(
        !verdict.is_promotable(),
        "over-solution proposal is not promotable"
    );
    assert_eq!(verdict.gate_state(), DesignIntegrityGateState::OverSolution);
    assert_eq!(
        verdict.engine_room_categories["designIntegrity"]["status"],
        "fail"
    );

    match &verdict.outcome {
        PromotionGuardOutcome::Blocked {
            reason,
            evidence_refs,
        } => {
            assert!(
                reason.contains("over-solution"),
                "reason names the over-solution"
            );
            assert!(
                evidence_refs.iter().any(|r| r.contains("counterexample")),
                "block links to replayable counterexample evidence"
            );
        }
        other => panic!("expected Blocked, got {other:?}"),
    }

    assert!(
        verdict.promotable_proposal().is_none(),
        "blocked proposal must not be routed"
    );
}

#[test]
fn verdict_serializes_to_evidence_json() {
    let generative = proposal_from_brief("generative-promotion-pass-brief-v1.json");
    let verdict = evaluate_promotion(&generative, SolveBudget::default()).expect("guard evaluates");

    let json = verdict.to_json().expect("verdict serializes");
    assert!(json.contains(GENERATIVE_PROMOTION_GUARD_SCHEMA_VERSION));
    assert!(json.contains("promotable"));
    assert!(json.contains("declared-gate-and"));
}
