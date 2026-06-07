//! Contract test for the Non-Developer Accessibility Path v1 (#1595).
//!
//! Part of Generative Front Door v1 (#1592) under #1 Era F Milestone 30. These
//! tests machine-check the non-developer flow: a plain authoring brief becomes a
//! generated proposal (#1593), the engine room verifies it (#1594), and the
//! result — proposal, provenance, verdicts, and solver trace — is surfaced
//! READ-ONLY.
//!
//! Two cases are exercised end to end:
//!   1. a non-dev brief yields a VERIFIED-solvable proposal with attached
//!      provenance, verdicts, and a solver trace; the verified proposal is
//!      eligible to enter the existing review/apply/trust-gradient path,
//!      unchanged;
//!   2. a non-dev brief that fails verification (an over-solution shortcut) is
//!      REPORTED read-only and never promoted.

use std::path::PathBuf;

use ouroforge_core::generative_accessibility::{
    accessibility_intake, AccessibilityOutcome, GENERATIVE_ACCESSIBILITY_SCHEMA_VERSION,
};
use ouroforge_core::generative_intake::GenerativeBrief;
use ouroforge_core::puzzle_solver::SolveBudget;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_brief(name: &str) -> GenerativeBrief {
    let path = repo_root()
        .join("examples/generative-front-door")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    GenerativeBrief::from_json_str(&text).expect("fixture brief parses")
}

#[test]
fn non_dev_brief_yields_verified_proposal_with_attached_evidence() {
    let brief = read_brief("generative-accessibility-verified-brief-v1.json");
    let result =
        accessibility_intake(&brief, SolveBudget::default(), FIXED_NOW_MS).expect("intake runs");

    assert_eq!(
        result.schema_version,
        GENERATIVE_ACCESSIBILITY_SCHEMA_VERSION
    );
    assert_eq!(result.brief_id, brief.brief_id);
    assert!(
        result.is_verified(),
        "brief should verify: {:?}",
        result.outcome
    );
    assert!(
        result.headline.to_lowercase().contains("verified"),
        "headline reports verified"
    );

    // Provenance is attached and links to the brief.
    assert_eq!(result.provenance.brief_id, brief.brief_id);
    assert!(result.provenance.proposal_only);

    // The engine-room verdict (verdicts) is attached and passing.
    assert!(result.engine_room.is_promotable());

    // The solver trace is attached: the level is solvable with no over-solution,
    // and the witness matches the author's intended solution.
    assert!(result.solver_trace.solvable, "solver finds a winning path");
    assert!(
        !result.solver_trace.over_solution,
        "verified level has no shorter bypass"
    );
    assert_eq!(
        result.solver_trace.shortest_witness.as_deref(),
        Some(result.solver_trace.intended_solution.as_slice()),
        "the shortest witness is the intended solution"
    );

    // Verified evidence is surfaced, and the proposal eligible for the existing
    // review/apply/trust-gradient path is present and UNCHANGED (proposal-only).
    match &result.outcome {
        AccessibilityOutcome::Verified { gate_evidence_refs } => {
            assert!(!gate_evidence_refs.is_empty(), "verified carries evidence");
        }
        other => panic!("expected Verified, got {other:?}"),
    }
    let proposal = result
        .verified_proposal()
        .expect("verified proposal present");
    assert_eq!(proposal.status, "proposed");
    assert_eq!(proposal.verdict_status, "pending");
    assert_eq!(proposal.confidence, "unverified");
}

#[test]
fn non_dev_brief_that_fails_verification_is_reported_not_promoted() {
    let brief = read_brief("generative-accessibility-unverified-brief-v1.json");
    let result =
        accessibility_intake(&brief, SolveBudget::default(), FIXED_NOW_MS).expect("intake runs");

    assert!(!result.is_verified(), "shortcut brief must not verify");
    assert!(
        result.headline.to_lowercase().contains("not verified"),
        "headline reports it is not verified"
    );

    // The solver trace surfaces the shortcut: a winning path strictly shorter
    // than the author's intended solution exists.
    assert!(result.solver_trace.solvable);
    assert!(
        result.solver_trace.over_solution,
        "an over-solution shortcut exists"
    );
    assert!(
        result.solver_trace.shortest_witness.as_ref().unwrap().len()
            < result.solver_trace.intended_solution.len()
    );

    // It is reported read-only with an evidence-linked reason; it is NOT
    // promoted.
    match &result.outcome {
        AccessibilityOutcome::Unverified {
            reason,
            evidence_refs,
        } => {
            assert!(reason.to_lowercase().contains("over-solution"));
            assert!(!evidence_refs.is_empty(), "report carries evidence");
        }
        other => panic!("expected Unverified, got {other:?}"),
    }
    assert!(
        result.verified_proposal().is_none(),
        "an unverified brief is never promoted"
    );
}

#[test]
fn result_serializes_to_read_only_json() {
    let brief = read_brief("generative-accessibility-verified-brief-v1.json");
    let result =
        accessibility_intake(&brief, SolveBudget::default(), FIXED_NOW_MS).expect("intake runs");

    let json = result.to_json().expect("result serializes");
    assert!(json.contains(GENERATIVE_ACCESSIBILITY_SCHEMA_VERSION));
    assert!(json.contains("solverTrace") || json.contains("solver_trace"));
    assert!(json.contains("verified"));
}
