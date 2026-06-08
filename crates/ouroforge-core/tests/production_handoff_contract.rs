//! Contract test for Handoff Artifacts and Conflict Resolution v1 (#1676).
//!
//! Validates clean handoff, conflicting edits resolved deterministically, and
//! stale refs, plus the fail-closed declared/computed cross-checks.

use ouroforge_core::production_handoff::{
    ProductionHandoffLedger, PRODUCTION_HANDOFF_SCHEMA_VERSION,
};
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/production-handoff-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

fn valid_ledger() -> ProductionHandoffLedger {
    ProductionHandoffLedger::from_json_str(&read_fixture("ledger.valid.fixture.json"))
        .expect("valid fixture must parse")
}

#[test]
fn valid_ledger_parses_and_pins_schema() {
    let ledger = valid_ledger();
    assert_eq!(ledger.schema_version, PRODUCTION_HANDOFF_SCHEMA_VERSION);
    assert_eq!(ledger.schema_version, "production-handoff-v1");
}

#[test]
fn clean_handoff_is_accepted() {
    let read = valid_ledger().read_model();
    assert_eq!(read.clean_count, 1);
    assert_eq!(read.accepted_count, 1);
    let clean = read
        .observations
        .iter()
        .find(|o| o.handoff_id == "handoff-001")
        .expect("handoff-001");
    assert_eq!(clean.status, "clean");
    assert_eq!(clean.resolution, "accepted");
    assert!(clean.conflicts_with.is_empty());
    assert!(clean.reason.contains("route through review/apply"));
}

#[test]
fn conflicting_edits_resolve_deterministically() {
    let ledger = valid_ledger();
    let read = ledger.read_model();
    // handoff-002 and handoff-003 edit scene-draft from the same base.
    assert_eq!(read.conflict_count, 2);
    assert_eq!(read.blocked_count, 2);

    let h2 = read
        .observations
        .iter()
        .find(|o| o.handoff_id == "handoff-002")
        .expect("handoff-002");
    let h3 = read
        .observations
        .iter()
        .find(|o| o.handoff_id == "handoff-003")
        .expect("handoff-003");
    // Symmetric, deterministic conflict sets — never auto-merged.
    assert_eq!(h2.conflicts_with, vec!["handoff-003".to_string()]);
    assert_eq!(h3.conflicts_with, vec!["handoff-002".to_string()]);
    assert_eq!(h2.resolution, "blocked");
    assert!(h2.reason.contains("never auto-merged"));

    // Re-serialization is stable regardless of input order.
    let mut reordered = ledger.clone();
    reordered.handoffs.reverse();
    assert_eq!(
        ledger.read_model_json().unwrap(),
        reordered.read_model_json().unwrap()
    );
}

#[test]
fn stale_refs_are_handled_needs_fix() {
    let read = valid_ledger().read_model();
    assert_eq!(read.stale_count, 1);
    assert_eq!(read.needs_fix_count, 1);
    let stale = read
        .observations
        .iter()
        .find(|o| o.handoff_id == "handoff-004")
        .expect("handoff-004");
    assert_eq!(stale.status, "stale");
    assert_eq!(stale.resolution, "needs-fix");
    // The blocked + needs-fix set is the unresolved (fail-closed) evidence.
    assert_eq!(
        read.unresolved.len(),
        read.blocked_count + read.needs_fix_count
    );
    assert!(read
        .unresolved
        .iter()
        .any(|o| o.handoff_id == "handoff-004"));
}

#[test]
fn declared_status_mismatch_fails_closed() {
    let err = ProductionHandoffLedger::from_json_str(&read_fixture(
        "ledger.status-mismatch.invalid.fixture.json",
    ))
    .expect_err("status mismatch must fail closed");
    assert!(
        err.to_string().contains("does not match computed status"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn read_model_reuses_review_apply_and_stays_read_only() {
    let ledger = valid_ledger();
    assert!(ledger.dashboard_compat.read_only);
    let read = ledger.read_model();
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("no new writer")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("review/apply/trust-gradient")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("no auto-merge")));
}
