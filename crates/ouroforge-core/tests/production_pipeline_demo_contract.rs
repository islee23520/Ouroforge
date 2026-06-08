//! Deterministic demo smoke test for Multi-Agent Production Pipeline Demo v1 (#1679).
//!
//! Part of Multi-Agent Production Pipeline v1 (#1674) under #1 Era H Milestone 42.
//! This composes the already-merged role-agent/ownership model (#1675), handoff
//! artifacts and conflict resolution (#1676), and reviewer/critic promotion
//! gates (#1678) into one end-to-end walkthrough and asserts the documented
//! behavior: multiple role agents collaborate on one project with handoffs, and
//! a reviewer/critic gate blocks an unreviewed promotion that proceeds only after
//! an independent reviewer and critic both approve. It asserts behavior and gate
//! states, never subjective quality, and runs with no network and no live
//! browser.

use ouroforge_core::production_handoff::ProductionHandoffLedger;
use ouroforge_core::production_review_gates::ProductionReviewGateLedger;
use ouroforge_core::production_roles::ProductionRoleOwnershipModel;
use std::{fs, path::PathBuf};

fn demo_fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/production-pipeline-v1/demo")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

#[test]
fn demo_roles_record_ownership_and_reject_unauthorized_writes() {
    let model = ProductionRoleOwnershipModel::from_json_str(&demo_fixture("roles.fixture.json"))
        .expect("demo roles fixture must validate");
    let read = model.read_model();

    // Each role owns exactly one artifact class.
    assert_eq!(read.ownership_by_role["designer"], vec!["design-brief"]);
    assert_eq!(
        read.ownership_by_role["level-designer"],
        vec!["scene-draft"]
    );
    assert_eq!(
        read.ownership_by_role["asset-import-planner"],
        vec!["asset-proposal"]
    );

    // The owning-role proposal is authorized; the non-owner proposal and the
    // direct trusted write are both rejected fail-closed.
    assert_eq!(read.authorized_count, 1);
    assert_eq!(read.rejected_count, 2);
    assert!(read
        .rejections
        .iter()
        .any(|r| r.reason.contains("is not the owning role")));
    assert!(read.rejections.iter().any(|r| r
        .reason
        .contains("direct trusted write is never authorized")));
}

#[test]
fn demo_handoffs_are_clean_and_accepted() {
    let ledger = ProductionHandoffLedger::from_json_str(&demo_fixture("handoffs.fixture.json"))
        .expect("demo handoffs fixture must validate");
    let read = ledger.read_model();

    assert_eq!(read.handoff_count, 3);
    assert_eq!(read.clean_count, 3);
    assert_eq!(read.accepted_count, 3);
    assert!(
        read.unresolved.is_empty(),
        "no unresolved conflicts in the demo"
    );

    // The collaboration chain is present.
    let pair = |id: &str| {
        let o = read
            .observations
            .iter()
            .find(|o| o.handoff_id == id)
            .unwrap_or_else(|| panic!("missing {id}"));
        (o.from_role.clone(), o.to_role.clone())
    };
    assert_eq!(
        pair("demo-handoff-001"),
        ("designer".to_string(), "level-designer".to_string())
    );
    assert_eq!(
        pair("demo-handoff-002"),
        ("level-designer".to_string(), "reviewer".to_string())
    );
}

#[test]
fn demo_unreviewed_promotion_is_blocked_until_review_passes() {
    let before =
        ProductionReviewGateLedger::from_json_str(&demo_fixture("review-gate.before.fixture.json"))
            .expect("demo before-gate fixture must validate");
    let after =
        ProductionReviewGateLedger::from_json_str(&demo_fixture("review-gate.after.fixture.json"))
            .expect("demo after-gate fixture must validate");

    let before_read = before.read_model();
    let after_read = after.read_model();

    // The same gate id progresses from blocked (unreviewed) to promote-allowed.
    let before_gate = before_read
        .audit
        .iter()
        .find(|r| r.gate_id == "demo-gate-scene-draft")
        .expect("before gate");
    let after_gate = after_read
        .audit
        .iter()
        .find(|r| r.gate_id == "demo-gate-scene-draft")
        .expect("after gate");

    assert_eq!(before_gate.outcome, "blocked");
    assert!(before_gate.reason.contains("blocked until reviewed"));
    assert_eq!(before_read.promote_allowed_count, 0);

    assert_eq!(after_gate.outcome, "promote-allowed");
    assert_eq!(after_read.promote_allowed_count, 1);
    // Even when cleared, promotion only routes through review/apply.
    assert!(after_gate.reason.contains("never auto-applied"));
    // Independent actors throughout (no self-approval, distinct reviewer/critic).
    assert_ne!(after_gate.implementer_role, after_gate.reviewer_role);
    assert_ne!(after_gate.reviewer_role, after_gate.critic_role);
}

#[test]
fn demo_is_deterministic_and_read_only() {
    // Re-loading and re-serializing each fixture is stable (no clocks, no RNG).
    for name in [
        "roles.fixture.json",
        "handoffs.fixture.json",
        "review-gate.before.fixture.json",
        "review-gate.after.fixture.json",
    ] {
        let raw = demo_fixture(name);
        if name.starts_with("roles") {
            let a = ProductionRoleOwnershipModel::from_json_str(&raw).unwrap();
            assert!(a.dashboard_compat.read_only);
            assert_eq!(a.read_model_json().unwrap(), a.read_model_json().unwrap());
        } else if name.starts_with("handoffs") {
            let a = ProductionHandoffLedger::from_json_str(&raw).unwrap();
            assert!(a.dashboard_compat.read_only);
            assert_eq!(a.read_model_json().unwrap(), a.read_model_json().unwrap());
        } else {
            let a = ProductionReviewGateLedger::from_json_str(&raw).unwrap();
            assert!(a.dashboard_compat.read_only);
            assert_eq!(a.read_model_json().unwrap(), a.read_model_json().unwrap());
        }
    }
}
