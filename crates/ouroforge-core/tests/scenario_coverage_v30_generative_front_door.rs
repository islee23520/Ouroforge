//! Scenario Coverage v30 — Generative Front Door (#1597).

use ouroforge_core::generative_accessibility::{accessibility_intake, AccessibilityOutcome};
use ouroforge_core::generative_intake::{intake_brief, GenerativeBrief};
use ouroforge_core::generative_promotion_guard::{evaluate_promotion, PromotionGuardOutcome};
use ouroforge_core::puzzle_solver::SolveBudget;
use ouroforge_core::trust_gradient_audit::{
    AutoApplyAuditLog, TRUST_GRADIENT_AUDIT_SCHEMA_VERSION,
};
use ouroforge_evaluator::design_integrity_gate::DesignIntegrityGateState;
use serde_json::Value;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn read_json(rel: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(root().join(rel)).expect("read")).expect("json")
}
fn read_brief(rel: &str) -> GenerativeBrief {
    GenerativeBrief::from_json_str(&std::fs::read_to_string(root().join(rel)).expect("read"))
        .expect("brief")
}
const MATRIX: &str = "examples/generative-front-door/scenario-coverage-v30/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v30.md";
const NOW: u128 = 1_725_000_000_000;

#[test]
fn v30_matrix_pinned() {
    let m = read_json(MATRIX);
    assert_eq!(
        m["schemaVersion"],
        "scenario-coverage-v30-generative-front-door-matrix-v1"
    );
    assert_eq!(m["issue"], 1597);
}

#[test]
fn v30_intake_valid_and_invalid() {
    let m = read_json(MATRIX);
    let valid = read_brief(m["intake"]["valid"].as_str().unwrap());
    intake_brief(&valid, NOW).expect("valid intake");
    let invalid = read_brief(m["intake"]["invalid"].as_str().unwrap());
    assert!(intake_brief(&invalid, NOW).is_err());
}

#[test]
fn v30_promotion_and_accessibility_fixtures_exist() {
    let m = read_json(MATRIX);
    for section in ["promotion", "accessibility"] {
        for (_k, v) in m[section].as_object().unwrap() {
            let p = root().join(v.as_str().unwrap());
            assert!(p.is_file(), "fixture {}", p.display());
        }
    }
}

#[test]
fn v30_promotion_guard_exercises_pass_gate_fail_and_oversolution() {
    let m = read_json(MATRIX);
    let pass = intake_brief(&read_brief(m["promotion"]["pass"].as_str().unwrap()), NOW)
        .expect("pass intake");
    let pass_verdict = evaluate_promotion(&pass, SolveBudget::default()).expect("pass guard");
    assert!(pass_verdict.is_promotable());
    assert_eq!(pass_verdict.gate_state(), DesignIntegrityGateState::Pass);
    assert!(matches!(
        pass_verdict.outcome,
        PromotionGuardOutcome::Promotable { .. }
    ));
    assert_eq!(pass_verdict.promotable_proposal().unwrap(), &pass.proposal);

    let gate_fail = intake_brief(
        &read_brief(m["promotion"]["gateFail"].as_str().unwrap()),
        NOW,
    )
    .expect("gate-fail intake");
    let gate_fail_verdict =
        evaluate_promotion(&gate_fail, SolveBudget::default()).expect("gate-fail guard");
    assert!(!gate_fail_verdict.is_promotable());
    assert_eq!(
        gate_fail_verdict.gate_state(),
        DesignIntegrityGateState::IntentUnsatisfied
    );
    match &gate_fail_verdict.outcome {
        PromotionGuardOutcome::Blocked {
            reason,
            evidence_refs,
        } => {
            assert!(reason.contains("intent-unsatisfied"));
            assert!(evidence_refs.iter().any(|r| r.contains("captured-intent")));
        }
        other => panic!("expected gate-fail block, got {other:?}"),
    }

    let over = intake_brief(
        &read_brief(m["promotion"]["oversolution"].as_str().unwrap()),
        NOW,
    )
    .expect("over-solution intake");
    let over_verdict =
        evaluate_promotion(&over, SolveBudget::default()).expect("over-solution guard");
    assert!(!over_verdict.is_promotable());
    assert_eq!(
        over_verdict.gate_state(),
        DesignIntegrityGateState::OverSolution
    );
    match &over_verdict.outcome {
        PromotionGuardOutcome::Blocked {
            reason,
            evidence_refs,
        } => {
            assert!(reason.contains("over-solution"));
            assert!(evidence_refs.iter().any(|r| r.contains("counterexample")));
        }
        other => panic!("expected over-solution block, got {other:?}"),
    }
}

#[test]
fn v30_accessibility_exercises_verified_and_unverified() {
    let m = read_json(MATRIX);
    let verified = accessibility_intake(
        &read_brief(m["accessibility"]["verified"].as_str().unwrap()),
        SolveBudget::default(),
        NOW,
    )
    .expect("verified accessibility");
    assert!(verified.is_verified());
    assert!(verified.verified_proposal().is_some());
    assert!(verified.solver_trace.solvable);
    assert!(!verified.solver_trace.over_solution);
    assert!(matches!(
        verified.outcome,
        AccessibilityOutcome::Verified { .. }
    ));

    let unverified = accessibility_intake(
        &read_brief(m["accessibility"]["unverified"].as_str().unwrap()),
        SolveBudget::default(),
        NOW,
    )
    .expect("unverified accessibility");
    assert!(!unverified.is_verified());
    assert!(unverified.verified_proposal().is_none());
    assert!(unverified.solver_trace.solvable);
    assert!(unverified.solver_trace.over_solution);
    assert!(matches!(
        unverified.outcome,
        AccessibilityOutcome::Unverified { .. }
    ));
}

#[test]
fn v30_backward_compat_exercises_trust_gradient_doc_and_audit_shape() {
    let m = read_json(MATRIX);
    let doc = std::fs::read_to_string(
        root().join(m["backwardCompat"]["trustGradientDoc"].as_str().unwrap()),
    )
    .expect("trust-gradient doc");
    assert!(doc.contains("no auto-apply"));
    assert!(doc.contains("review-gated"));
    assert!(doc.contains("read-only"));
    assert!(doc.contains("backward-compatible"));

    let audit_text = std::fs::read_to_string(
        root().join(
            m["backwardCompat"]["trustGradientAuditFixture"]
                .as_str()
                .unwrap(),
        ),
    )
    .expect("trust-gradient audit");
    let audit = AutoApplyAuditLog::from_json_str(&audit_text).expect("audit fixture validates");
    assert_eq!(audit.schema_version, TRUST_GRADIENT_AUDIT_SCHEMA_VERSION);
    assert_eq!(audit.entries.len(), 2);
    assert!(!audit.is_autonomy_halted());
}

#[test]
fn v30_doc_governance() {
    let doc = std::fs::read_to_string(root().join(DOC)).expect("doc");
    assert!(doc.contains("#1") && doc.contains("#23") && doc.contains("fixture-scoped"));
}
