//! Deterministic demo smoke test for the Generative Front Door v1 (#1596).
//!
//! Reproduces, from a fresh clone and with no network or live browser, the
//! generative front door over the verification engine room end to end over
//! fixture-scoped briefs in `examples/generative-front-door-v1/demo/`:
//!
//! - the intake (#1593) turns each authoring brief into a grid-puzzle
//!   *proposal* — proposal-only, never a trusted write or promotion;
//! - the engine room (the promotion guard, #1594) BLOCKS the brief whose level
//!   admits an unintended over-solution, with an evidence-linked reason, and
//!   withholds it from the apply path;
//! - the engine room marks the clean brief PROMOTABLE and routes the proposal
//!   UNCHANGED into the existing review/apply/trust-gradient path.

use std::path::{Path, PathBuf};

use ouroforge_core::generative_intake::{intake_brief, GenerativeBrief, GenerativeProposal};
use ouroforge_core::generative_promotion_guard::{evaluate_promotion, PromotionGuardOutcome};
use ouroforge_core::puzzle_solver::SolveBudget;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

const PROMOTABLE: &str = "examples/generative-front-door-v1/demo/brief-promotable.json";
const BLOCKED: &str = "examples/generative-front-door-v1/demo/brief-blocked.json";

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn read_brief(relative: &str) -> GenerativeBrief {
    let text = std::fs::read_to_string(workspace_path(relative)).expect("demo fixture exists");
    GenerativeBrief::from_json_str(&text).expect("demo brief parses")
}

fn proposal(relative: &str) -> GenerativeProposal {
    intake_brief(&read_brief(relative), FIXED_NOW_MS).expect("brief produces a proposal")
}

#[test]
fn brief_becomes_a_proposal_only_artifact() {
    // Intake is proposal-only for BOTH briefs: it produces a proposed/pending/
    // unverified proposal with provenance, and never promotes or writes.
    for fixture in [PROMOTABLE, BLOCKED] {
        let generative = proposal(fixture);
        generative.validate().expect("proposal validates");
        assert_eq!(generative.proposal.status, "proposed");
        assert_eq!(generative.proposal.verdict_status, "pending");
        assert_eq!(generative.proposal.confidence, "unverified");
        assert!(generative.provenance.proposal_only);
    }
}

#[test]
fn failing_proposal_is_blocked_by_the_engine_room() {
    let generative = proposal(BLOCKED);
    let verdict = evaluate_promotion(&generative, SolveBudget::default()).expect("guard evaluates");

    assert!(
        !verdict.is_promotable(),
        "the over-solution brief must be blocked"
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
    // A blocked proposal is withheld from the apply path.
    assert!(verdict.promotable_proposal().is_none());
}

#[test]
fn passing_proposal_is_promotable_through_the_existing_path() {
    let generative = proposal(PROMOTABLE);
    let verdict = evaluate_promotion(&generative, SolveBudget::default()).expect("guard evaluates");

    assert!(
        verdict.is_promotable(),
        "the clean brief must be promotable"
    );
    // The proposal routed into the existing review/apply/trust-gradient path is
    // the same proposal, UNCHANGED (still proposal-only).
    let routed = verdict
        .promotable_proposal()
        .expect("promotable proposal routed");
    assert_eq!(routed, &generative.proposal);
    assert_eq!(routed.status, "proposed");
    assert_eq!(routed.verdict_status, "pending");
    assert_eq!(routed.confidence, "unverified");
}

#[test]
fn demo_is_deterministic_across_runs() {
    for fixture in [PROMOTABLE, BLOCKED] {
        let generative = proposal(fixture);
        let first = evaluate_promotion(&generative, SolveBudget::default()).expect("guard");
        let second = evaluate_promotion(&generative, SolveBudget::default()).expect("guard");
        assert_eq!(
            first.is_promotable(),
            second.is_promotable(),
            "{fixture}: promotion verdict must be deterministic"
        );
        assert_eq!(
            first.to_json().unwrap(),
            second.to_json().unwrap(),
            "{fixture}: serialized verdict must be deterministic"
        );
    }
}

#[test]
fn demo_doc_records_the_front_door_demo() {
    let doc = std::fs::read_to_string(workspace_path("docs/generative-front-door-v1-demo.md"))
        .expect("demo doc exists");
    assert!(doc.contains("#1596"), "doc references the demo issue");
    assert!(doc.contains("brief-promotable.json"));
    assert!(doc.contains("brief-blocked.json"));
    assert!(
        doc.contains("cargo test --test generative_front_door_demo"),
        "doc records the reproduce command"
    );
}
