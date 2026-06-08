//! Contract test for Reviewer/Critic Promotion Gates v1 (#1678).
//!
//! Validates blocked-until-reviewed, critic veto, and audit records, plus the
//! fail-closed declared/computed and separation-of-duties checks.

use ouroforge_core::production_review_gates::{
    ProductionReviewGateLedger, PRODUCTION_REVIEW_GATES_SCHEMA_VERSION,
};
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/production-review-gates-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

fn valid_ledger() -> ProductionReviewGateLedger {
    ProductionReviewGateLedger::from_json_str(&read_fixture("ledger.valid.fixture.json"))
        .expect("valid fixture must parse")
}

fn record<'a>(
    read: &'a ouroforge_core::production_review_gates::ProductionReviewGatesReadModel,
    id: &str,
) -> &'a ouroforge_core::production_review_gates::GateAuditRecord {
    read.audit
        .iter()
        .find(|r| r.gate_id == id)
        .unwrap_or_else(|| panic!("missing {id}"))
}

#[test]
fn valid_ledger_parses_and_pins_schema() {
    let ledger = valid_ledger();
    assert_eq!(
        ledger.schema_version,
        PRODUCTION_REVIEW_GATES_SCHEMA_VERSION
    );
    assert_eq!(ledger.schema_version, "production-review-gates-v1");
}

#[test]
fn promotion_is_blocked_until_reviewed() {
    let read = valid_ledger().read_model();
    // gate-001: reviewer pending -> blocked-until-reviewed.
    let g1 = record(&read, "gate-001");
    assert_eq!(g1.outcome, "blocked");
    assert!(g1.reason.contains("blocked until reviewed"));
    // gate-004: low risk + reviewer approve (critic pending) -> promote-allowed.
    let g4 = record(&read, "gate-004");
    assert_eq!(g4.outcome, "promote-allowed");
    assert!(g4.reason.contains("never auto-applied"));
}

#[test]
fn critic_veto_blocks_promotion() {
    let read = valid_ledger().read_model();
    // gate-002: reviewer approve but critic veto -> blocked.
    let g2 = record(&read, "gate-002");
    assert_eq!(g2.outcome, "blocked");
    assert!(g2.reason.contains("vetoed promotion"));
    assert_eq!(read.veto_count, 1);
}

#[test]
fn higher_risk_requires_stronger_review() {
    let read = valid_ledger().read_model();
    // gate-003: high risk, reviewer approve, critic pending -> still blocked.
    let g3 = record(&read, "gate-003");
    assert_eq!(g3.outcome, "blocked");
    assert!(g3.reason.contains("requires an explicit critic approval"));
    // gate-005: high risk, reviewer + critic approve -> promote-allowed.
    let g5 = record(&read, "gate-005");
    assert_eq!(g5.outcome, "promote-allowed");
}

#[test]
fn audit_records_cover_every_gate_and_are_deterministic() {
    let ledger = valid_ledger();
    let read = ledger.read_model();
    assert_eq!(read.gate_count, 5);
    assert_eq!(read.audit.len(), 5);
    assert_eq!(read.blocked_count, 3);
    assert_eq!(read.promote_allowed_count, 2);
    assert_eq!(read.blocked.len(), 3);
    // Each audit record carries the actor roles and decisions (the audit trail).
    for r in &read.audit {
        assert!(!r.reviewer_role.is_empty());
        assert!(!r.critic_role.is_empty());
        assert!(r.implementer_role != r.reviewer_role);
    }
    // Order-independent serialization.
    let mut reordered = ledger.clone();
    reordered.gates.reverse();
    assert_eq!(
        ledger.read_model_json().unwrap(),
        reordered.read_model_json().unwrap()
    );
    // Conservative, reuse-anchored compatibility notes.
    assert!(read
        .compatibility_notes
        .iter()
        .any(|n| n.contains("no reviewer bypass")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|n| n.contains("Milestone 22 trust gradient")));
}

#[test]
fn self_approval_fails_closed() {
    let err = ProductionReviewGateLedger::from_json_str(&read_fixture(
        "ledger.self-approval.invalid.fixture.json",
    ))
    .expect_err("self-approval must fail closed");
    assert!(
        err.to_string().contains("no self-approval"),
        "unexpected error: {err:#}"
    );
}
