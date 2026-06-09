use ouroforge_evaluator::human_constraint_gate::{
    compose_human_constraints_into_categories, evaluate_human_constraint_gate,
    human_constraint_gate_category, CandidateConstraintEvidence, HumanConstraintGateInput,
    HumanConstraintGateState, HumanConstraintKind, HumanConstraintRecord, HumanConstraintStatus,
    HUMAN_CONSTRAINT_GATE_BOUNDARY, HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION,
};
use serde_json::json;

fn candidate() -> CandidateConstraintEvidence {
    CandidateConstraintEvidence {
        candidate_id: "candidate-m78-001".to_string(),
        target_ref: "runs/m78/candidates/card.json".to_string(),
        mechanics: vec!["dash".to_string(), "burn".to_string()],
        style: "pixel-art".to_string(),
        budget: 8,
        evidence_refs: vec!["runs/m78/evidence/candidate.json".to_string()],
    }
}

fn constraint(kind: HumanConstraintKind) -> HumanConstraintRecord {
    let (forbidden_mechanic, required_style, budget_cap) = match kind {
        HumanConstraintKind::ForbiddenMechanic => (Some("dash".to_string()), None, None),
        HumanConstraintKind::RequiredStyle => (None, Some("pixel-art".to_string()), None),
        HumanConstraintKind::BudgetCap => (None, None, Some(10)),
    };
    HumanConstraintRecord {
        constraint_id: format!("constraint-{kind:?}").to_ascii_lowercase(),
        kind,
        status: HumanConstraintStatus::Active,
        author: "human:local-designer".to_string(),
        author_provenance_ref: "runs/m78/provenance/human.json".to_string(),
        target_ref: "runs/m78/candidates/card.json".to_string(),
        target_base_ref: "hash:before-m78".to_string(),
        normalized_constraint_ref: "runs/m78/constraints/normalized.json".to_string(),
        review_apply_ref: "runs/m78/review/decision.json".to_string(),
        evaluator_evidence_ref: "runs/m78/evaluator/human-constraint.json".to_string(),
        evidence_refs: vec!["runs/m78/evidence/constraint.json".to_string()],
        forbidden_mechanic,
        required_style,
        budget_cap,
        intervention_as_evidence: true,
        read_gated_write: true,
        raw_bypass_requested: false,
        direct_artifact_write: false,
        studio_trusted_write_authority: false,
        human_required_for_autonomous_loop: false,
        cli_fallback_supported: true,
    }
}

fn input(constraints: Vec<HumanConstraintRecord>) -> HumanConstraintGateInput {
    HumanConstraintGateInput {
        schema_version: HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION.to_string(),
        gate_id: "m78-human-constraint-gate".to_string(),
        candidate: candidate(),
        constraints,
        boundary: HUMAN_CONSTRAINT_GATE_BOUNDARY.to_string(),
    }
}

#[test]
fn violating_forbidden_mechanic_blocks_with_evidence() {
    let verdicts = evaluate_human_constraint_gate(&input(vec![constraint(
        HumanConstraintKind::ForbiddenMechanic,
    )]));

    assert_eq!(verdicts.len(), 1);
    assert_eq!(verdicts[0].state, HumanConstraintGateState::Violation);
    assert!(verdicts[0].reason.contains("forbidden mechanic dash"));
    assert!(verdicts[0]
        .evidence_refs
        .contains(&"runs/m78/evaluator/human-constraint.json".to_string()));

    let category = human_constraint_gate_category(&verdicts).expect("declared gate");
    assert_eq!(category["status"], "fail");
    assert_eq!(category["failureCount"], 1);
}

#[test]
fn style_and_budget_constraints_pass_and_compose_with_existing_gate_categories() {
    let verdicts = evaluate_human_constraint_gate(&input(vec![
        constraint(HumanConstraintKind::RequiredStyle),
        constraint(HumanConstraintKind::BudgetCap),
    ]));
    assert!(verdicts.iter().all(|verdict| verdict.state.is_pass()));

    let mut categories = json!({
        "operator": "declared-gate-and",
        "undeclaredGatePolicy": "neutral",
        "scenario": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0},
        "visual": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0},
        "semantic": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0},
        "reviewApply": {"declared": true, "status": "pass", "resultCount": 1, "failureCount": 0}
    });
    assert!(compose_human_constraints_into_categories(
        &mut categories,
        &verdicts
    ));
    assert_eq!(categories["operator"], "declared-gate-and");
    assert_eq!(categories["humanConstraints"]["status"], "pass");
    assert_eq!(categories["humanConstraints"]["failureCount"], 0);
}

#[test]
fn budget_cap_violation_blocks_candidate() {
    let mut cap = constraint(HumanConstraintKind::BudgetCap);
    cap.budget_cap = Some(7);
    let verdicts = evaluate_human_constraint_gate(&input(vec![cap]));
    assert_eq!(verdicts[0].state, HumanConstraintGateState::Violation);
    assert!(verdicts[0].reason.contains("budget 8 exceeds cap 7"));
}

#[test]
fn malformed_or_bypass_constraints_fail_closed_before_gate_pass() {
    let mut raw = constraint(HumanConstraintKind::RequiredStyle);
    raw.required_style = Some("raw_apply_bypass".to_string());
    let verdicts = evaluate_human_constraint_gate(&input(vec![raw]));
    assert_eq!(
        verdicts[0].state,
        HumanConstraintGateState::MalformedConstraint
    );
    assert!(verdicts[0].reason.contains("raw bypass"));

    let mut direct = constraint(HumanConstraintKind::BudgetCap);
    direct.direct_artifact_write = true;
    let verdicts = evaluate_human_constraint_gate(&input(vec![direct]));
    assert_eq!(
        verdicts[0].state,
        HumanConstraintGateState::MalformedConstraint
    );
    assert!(verdicts[0].reason.contains("direct writes"));
}

#[test]
fn inactive_constraints_are_neutral_but_blocked_or_stale_constraints_fail_closed() {
    let mut inactive = constraint(HumanConstraintKind::ForbiddenMechanic);
    inactive.status = HumanConstraintStatus::Inactive;
    let verdicts = evaluate_human_constraint_gate(&input(vec![inactive]));
    assert_eq!(verdicts[0].state, HumanConstraintGateState::Pass);

    let mut blocked = constraint(HumanConstraintKind::RequiredStyle);
    blocked.status = HumanConstraintStatus::Blocked;
    let verdicts = evaluate_human_constraint_gate(&input(vec![blocked]));
    assert_eq!(
        verdicts[0].state,
        HumanConstraintGateState::BlockedConstraint
    );

    let mut stale = constraint(HumanConstraintKind::BudgetCap);
    stale.status = HumanConstraintStatus::Stale;
    let verdicts = evaluate_human_constraint_gate(&input(vec![stale]));
    assert_eq!(verdicts[0].state, HumanConstraintGateState::StaleConstraint);
}

#[test]
fn boundary_preserves_agent_first_two_plane_and_local_first_contract() {
    for token in [
        "human constraints as first-class gates",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no raw bypass",
        "local-first CLI fallback",
        "loop completes without human",
        "#1 and #23 remain open",
    ] {
        assert!(HUMAN_CONSTRAINT_GATE_BOUNDARY.contains(token));
    }
}
