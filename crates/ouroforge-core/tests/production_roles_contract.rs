//! Contract test for Role Agent Model and Artifact Ownership v1 (#1675).
//!
//! Validates ownership assignment, unauthorized-write rejection (fail-closed),
//! and the observability records derived from the model.

use ouroforge_core::production_roles::{
    ProductionRoleOwnershipModel, PRODUCTION_ROLES_SCHEMA_VERSION,
};
use std::{fs, path::PathBuf};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/production-roles-v1")
        .join(name)
}

fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture(name)).expect(name)
}

fn valid_model() -> ProductionRoleOwnershipModel {
    ProductionRoleOwnershipModel::from_json_str(&read_fixture("model.valid.fixture.json"))
        .expect("valid fixture must parse")
}

#[test]
fn valid_model_parses_and_pins_schema() {
    let model = valid_model();
    assert_eq!(model.schema_version, PRODUCTION_ROLES_SCHEMA_VERSION);
    assert_eq!(model.schema_version, "production-roles-v1");
}

#[test]
fn ownership_assignment_is_recorded_per_role() {
    let model = valid_model();
    let read = model.read_model();
    // Single owner per class; designer owns two classes.
    assert_eq!(read.ownership_count, 4);
    assert_eq!(
        read.ownership_by_role["designer"],
        vec!["design-brief", "requirement-extraction"]
    );
    assert_eq!(
        read.ownership_by_role["level-designer"],
        vec!["scene-draft"]
    );
    assert_eq!(
        read.ownership_by_role["asset-import-planner"],
        vec!["asset-proposal"]
    );
    // The model never claims a trusted-write authority.
    assert_eq!(model.owner_of("design-brief"), Some("designer"));
    assert_eq!(model.owner_of("audio-cue"), None);
}

#[test]
fn unauthorized_writes_are_rejected_fail_closed() {
    let model = valid_model();
    let read = model.read_model();
    // attempt-001 authorized (owning role proposal); the other three rejected.
    assert_eq!(read.authorized_count, 1);
    assert_eq!(read.rejected_count, 3);

    let reason_for = |id: &str| {
        read.observations
            .iter()
            .find(|o| o.attempt_id == id)
            .unwrap_or_else(|| panic!("missing {id}"))
            .clone()
    };

    let authorized = reason_for("attempt-001");
    assert_eq!(authorized.outcome, "authorized");
    assert!(authorized.reason.is_empty());

    // Non-owning actor on an owned class.
    let non_owner = reason_for("attempt-002");
    assert_eq!(non_owner.outcome, "rejected");
    assert!(non_owner.reason.contains("is not the owning role"));

    // Direct trusted write is never authorized, even by the owner.
    let trusted = reason_for("attempt-003");
    assert_eq!(trusted.outcome, "rejected");
    assert!(trusted
        .reason
        .contains("direct trusted write is never authorized"));

    // Unowned artifact class fails closed.
    let unowned = reason_for("attempt-004");
    assert_eq!(unowned.outcome, "rejected");
    assert!(unowned.reason.contains("fail closed"));
}

#[test]
fn observability_records_are_complete_and_deterministic() {
    let model = valid_model();
    let read = model.read_model();
    // Every attempt produces an observability record.
    assert_eq!(read.attempt_count, model.attempts.len());
    assert_eq!(read.observations.len(), model.attempts.len());
    // Rejections are the fail-closed subset.
    assert_eq!(
        read.rejections.len(),
        read.observations
            .iter()
            .filter(|o| o.outcome == "rejected")
            .count()
    );
    // Observations are sorted (order-independent), so re-serializing is stable.
    let again = model.read_model();
    assert_eq!(
        model.read_model_json().unwrap(),
        serde_json::to_string_pretty(&again).unwrap()
    );
    // Read-only governance surface and conservative boundary.
    assert!(model.dashboard_compat.read_only);
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("no new orchestration engine")));
    assert!(read
        .compatibility_notes
        .iter()
        .any(|note| note.contains("review/apply/trust-gradient")));
}

#[test]
fn duplicate_ownership_conflict_fails_closed() {
    let err = ProductionRoleOwnershipModel::from_json_str(&read_fixture(
        "model.duplicate-owner.invalid.fixture.json",
    ))
    .expect_err("duplicate ownership must fail closed");
    assert!(
        err.to_string().contains("single owning role"),
        "unexpected error: {err:#}"
    );
}

#[test]
fn declared_outcome_mismatch_fails_closed() {
    let err = ProductionRoleOwnershipModel::from_json_str(&read_fixture(
        "model.outcome-mismatch.invalid.fixture.json",
    ))
    .expect_err("declared/computed outcome mismatch must fail closed");
    assert!(
        err.to_string().contains("does not match computed outcome"),
        "unexpected error: {err:#}"
    );
}
