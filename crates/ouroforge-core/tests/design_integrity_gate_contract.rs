//! Contract test for Design-Integrity Gate v1 (#1583).
//!
//! Part of Puzzle Solver and Over-Solution Detection v1 (#1579) under #1 Era F
//! Milestone 28. These tests machine-check the design-integrity gate contract: a
//! clean level (intent satisfied, no over-solution) passes; an over-solution
//! fails; an unsatisfiable intent fails; an exhausted/inconclusive search fails
//! closed (never a silent pass); malformed evidence fails closed; and the gate
//! composes into the existing four-gate `declared-gate-and` aggregation as one
//! more declared category — not a parallel evaluator.
//!
//! The gate lives in `ouroforge-evaluator` (it reuses the evaluator gate
//! vocabulary and aggregation); `ouroforge-core` depends on the evaluator, so the
//! contract test exercises it here through the public evaluator API.

use std::path::PathBuf;

use ouroforge_evaluator::design_integrity_gate::{
    compose_design_integrity_into_categories, design_integrity_gate_category,
    evaluate_design_integrity_check, evaluate_design_integrity_gate, DesignIntegrityCheck,
    DesignIntegrityGateState,
};
use ouroforge_evaluator::{evaluation_gate_categories, VisualGateState, VisualGateVerdict};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_check(name: &str) -> DesignIntegrityCheck {
    let path: PathBuf = repo_root().join("examples/design-integrity").join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    DesignIntegrityCheck::from_json_str(&text).expect("fixture check parses")
}

#[test]
fn clean_level_passes_the_gate() {
    let check = read_check("pass.json");
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::Pass);
    assert!(verdict.state.is_pass());
    assert_eq!(verdict.level_id, "grid-puzzle-scene-v1");
    assert_eq!(verdict.evidence_refs.len(), 2);
}

#[test]
fn over_solution_fails_the_gate() {
    let check = read_check("over-solution.json");
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::OverSolution);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("over-solution"));
    // The verdict links to the replayable counterexample evidence.
    assert!(verdict
        .evidence_refs
        .iter()
        .any(|r| r.contains("counterexample")));
}

#[test]
fn unsatisfiable_intent_fails() {
    let mut check = read_check("pass.json");
    check.intent_satisfied = false;
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::IntentUnsatisfied);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("win state"));
}

#[test]
fn exhausted_search_is_inconclusive_not_a_silent_pass() {
    // A clean check whose bounded search was exhausted before fully exploring the
    // shorter-solution space cannot conclude absence — it fails closed.
    let mut check = read_check("pass.json");
    check.search_exhausted = true;
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::Inconclusive);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("inconclusive"));
}

#[test]
fn malformed_evidence_fails_closed() {
    let check = read_check("malformed-evidence.json");
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::MalformedEvidence);
    assert!(!verdict.state.is_pass());
    assert!(verdict.reason.contains("evidence references"));
}

#[test]
fn wrong_schema_version_fails_closed() {
    let mut check = read_check("pass.json");
    check.schema_version = "ouroforge.design-integrity-gate.v0".to_string();
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::MalformedEvidence);
}

#[test]
fn missing_captured_intent_fails_closed() {
    // A check with no captured intended solution (intendedLength 0) is malformed.
    let mut check = read_check("pass.json");
    check.intended_length = 0;
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::MalformedEvidence);
    assert!(verdict.reason.contains("intended solution"));
}

#[test]
fn intent_failure_precedes_over_solution() {
    // A check failing multiple dimensions reports the highest-precedence failure:
    // an unsatisfiable intent before an over-solution.
    let mut check = read_check("over-solution.json");
    check.intent_satisfied = false;
    let verdict = evaluate_design_integrity_check(&check);
    assert_eq!(verdict.state, DesignIntegrityGateState::IntentUnsatisfied);
}

#[test]
fn gate_category_aggregates_pass_and_fail() {
    let pass = evaluate_design_integrity_gate(&[read_check("pass.json")]);
    let category = design_integrity_gate_category(&pass).expect("declared gate has a category");
    assert_eq!(category["declared"], true);
    assert_eq!(category["status"], "pass");
    assert_eq!(category["resultCount"], 1);
    assert_eq!(category["failureCount"], 0);

    let mixed = evaluate_design_integrity_gate(&[
        read_check("pass.json"),
        read_check("over-solution.json"),
    ]);
    let category = design_integrity_gate_category(&mixed).expect("declared gate has a category");
    assert_eq!(category["status"], "fail");
    assert_eq!(category["resultCount"], 2);
    assert_eq!(category["failureCount"], 1);
}

#[test]
fn undeclared_gate_is_neutral() {
    // No design-integrity checks: the gate is undeclared and contributes no
    // category, preserving undeclaredGatePolicy: neutral.
    assert!(design_integrity_gate_category(&[]).is_none());
    let mut categories = serde_json::json!({ "aggregation": { "operator": "declared-gate-and" } });
    let added = compose_design_integrity_into_categories(&mut categories, &[]);
    assert!(!added);
    assert!(categories.get("designIntegrity").is_none());
}

#[test]
fn composes_into_the_existing_four_gate_aggregation() {
    // Build a real four-gate categories object with a passing visual gate, then
    // compose the design-integrity gate into the same declared-gate-and
    // aggregation.
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
    assert!(categories.get("designIntegrity").is_none());

    let verdicts = evaluate_design_integrity_gate(&[read_check("pass.json")]);
    let added = compose_design_integrity_into_categories(&mut categories, &verdicts);
    assert!(added);
    // The design-integrity gate is now one more declared category ANDed with the
    // four.
    assert_eq!(categories["designIntegrity"]["declared"], true);
    assert_eq!(categories["designIntegrity"]["status"], "pass");
    assert_eq!(categories["visual"]["status"], "pass");
    assert_eq!(categories["aggregation"]["operator"], "declared-gate-and");
}

#[test]
fn over_solution_fails_the_composed_aggregation() {
    let mut categories = serde_json::json!({ "aggregation": { "operator": "declared-gate-and", "undeclaredGatePolicy": "neutral" } });
    let verdicts = evaluate_design_integrity_gate(&[read_check("over-solution.json")]);
    assert!(compose_design_integrity_into_categories(
        &mut categories,
        &verdicts
    ));
    assert_eq!(categories["designIntegrity"]["status"], "fail");
    assert_eq!(categories["designIntegrity"]["failureCount"], 1);
}

#[test]
fn docs_record_the_design_integrity_gate_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/puzzle-solver-oversolution-v1.md"))
        .expect("puzzle solver design doc exists");
    assert!(
        doc.contains("#1583"),
        "doc records the design-integrity gate issue (#1583)"
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
