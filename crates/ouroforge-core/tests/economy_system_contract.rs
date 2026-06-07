//! Contract tests for Economy and Currency v1 (#1658).
//!
//! Covers the three required behaviors: earn/spend integrity, the non-negative
//! invariant, and save/restore parity, plus the fail-closed negative cases that
//! keep the ledger deterministic and trusted-only.

use ouroforge_core::economy_system::{
    EconomyDefinition, EconomyState, EconomyTransaction, EconomyTransactionKind,
    ECONOMY_SYSTEM_BOUNDARY, ECONOMY_SYSTEM_SCHEMA_VERSION,
};

fn valid_definition_json() -> &'static str {
    include_str!("../../../examples/economy-system-v1/valid/economy.definition.json")
}

fn definition() -> EconomyDefinition {
    EconomyDefinition::from_json_str(valid_definition_json())
        .expect("valid economy definition parses")
}

fn earn(tx_id: &str, currency: &str, amount: u64) -> EconomyTransaction {
    EconomyTransaction {
        tx_id: tx_id.to_string(),
        kind: EconomyTransactionKind::Earn,
        currency: currency.to_string(),
        amount,
    }
}

fn spend(tx_id: &str, currency: &str, amount: u64) -> EconomyTransaction {
    EconomyTransaction {
        tx_id: tx_id.to_string(),
        kind: EconomyTransactionKind::Spend,
        currency: currency.to_string(),
        amount,
    }
}

#[test]
fn valid_definition_loads_and_initial_state_is_zeroed() {
    let def = definition();
    assert_eq!(def.profile_id, "demo-profile");
    assert_eq!(def.currencies.len(), 3);

    let state = def.initial_state();
    assert_eq!(state.schema_version, ECONOMY_SYSTEM_SCHEMA_VERSION);
    assert_eq!(state.boundary, ECONOMY_SYSTEM_BOUNDARY);
    assert_eq!(state.balance("coins"), 0);
    assert_eq!(state.balance("gems"), 0);
    assert_eq!(state.balance("energy"), 0);
    assert!(state.applied_tx.is_empty());
}

#[test]
fn earn_and_spend_integrity_is_deterministic() {
    let def = definition();
    let txs = [
        earn("t1", "coins", 100),
        spend("t2", "coins", 30),
        earn("t3", "gems", 5),
        spend("t4", "coins", 20),
    ];

    let state = def
        .apply_transactions(&def.initial_state(), &txs)
        .expect("transactions apply");

    assert_eq!(state.balance("coins"), 50);
    assert_eq!(state.balance("gems"), 5);
    assert_eq!(state.balance("energy"), 0);
    assert_eq!(state.applied_tx, vec!["t1", "t2", "t3", "t4"]);

    // Replaying the same transactions from the same start reproduces the state.
    let replay = def
        .apply_transactions(&def.initial_state(), &txs)
        .expect("replay applies");
    assert_eq!(state, replay);
}

#[test]
fn spend_beyond_balance_violates_non_negative_invariant_and_is_rejected() {
    let def = definition();
    let state = def
        .apply_transaction(&def.initial_state(), &earn("t1", "coins", 10))
        .expect("earn");
    let err = def
        .apply_transaction(&state, &spend("t2", "coins", 11))
        .expect_err("overspend must fail closed");
    assert!(err.to_string().contains("insufficient"));

    // The rejected transaction does not mutate the ledger.
    assert_eq!(state.balance("coins"), 10);
    assert!(!state.applied_tx.contains(&"t2".to_string()));
}

#[test]
fn spend_to_exactly_zero_is_allowed() {
    let def = definition();
    let state = def
        .apply_transactions(
            &def.initial_state(),
            &[earn("t1", "energy", 7), spend("t2", "energy", 7)],
        )
        .expect("apply");
    assert_eq!(state.balance("energy"), 0);
}

#[test]
fn save_restore_round_trips_unchanged() {
    let def = definition();
    let state = def
        .apply_transactions(
            &def.initial_state(),
            &[
                earn("a", "coins", 250),
                spend("b", "coins", 75),
                earn("c", "gems", 12),
            ],
        )
        .expect("apply");

    let json = state.to_json().expect("serialize");
    let restored = EconomyState::from_json_str(&json).expect("deserialize");

    assert_eq!(state, restored);
    def.validate_state(&restored)
        .expect("restored state is consistent");
    assert_eq!(restored.balance("coins"), 175);
    assert_eq!(restored.balance("gems"), 12);

    // Applying further transactions after restore continues deterministically.
    let continued = def
        .apply_transaction(&restored, &spend("d", "coins", 175))
        .expect("spend after restore");
    assert_eq!(continued.balance("coins"), 0);
}

#[test]
fn duplicate_transaction_id_is_rejected() {
    let def = definition();
    let state = def
        .apply_transaction(&def.initial_state(), &earn("dup", "coins", 1))
        .expect("first apply");
    let err = def
        .apply_transaction(&state, &earn("dup", "coins", 1))
        .expect_err("duplicate tx id must fail closed");
    assert!(err.to_string().contains("already applied"));
}

#[test]
fn undeclared_currency_is_rejected() {
    let def = definition();
    let err = def
        .apply_transaction(&def.initial_state(), &earn("x", "rubies", 1))
        .expect_err("undeclared currency must fail closed");
    assert!(err.to_string().contains("undeclared currency"));
}

#[test]
fn earn_overflow_is_rejected() {
    let def = definition();
    let state = def
        .apply_transaction(&def.initial_state(), &earn("max", "coins", u64::MAX))
        .expect("earn max");
    let err = def
        .apply_transaction(&state, &earn("more", "coins", 1))
        .expect_err("overflow must fail closed");
    assert!(err.to_string().contains("overflow"));
}

#[test]
fn wrong_schema_version_is_rejected() {
    let bad = r#"{
        "schemaVersion": "economy-system-v0",
        "profileId": "p",
        "currencies": ["coins"],
        "boundary": "x"
    }"#;
    let err =
        EconomyDefinition::from_json_str(bad).expect_err("wrong schema version must fail closed");
    assert!(err.to_string().contains("schema version"));
}

#[test]
fn duplicate_currency_definition_is_rejected() {
    let bad = format!(
        r#"{{
        "schemaVersion": "economy-system-v1",
        "profileId": "p",
        "currencies": ["coins", "coins"],
        "boundary": "{ECONOMY_SYSTEM_BOUNDARY}"
    }}"#
    );
    let err =
        EconomyDefinition::from_json_str(&bad).expect_err("duplicate currency must fail closed");
    assert!(err.to_string().contains("duplicate"));
}

#[test]
fn definition_with_wrong_boundary_is_rejected() {
    let bad = r#"{
        "schemaVersion": "economy-system-v1",
        "profileId": "p",
        "currencies": ["coins"],
        "boundary": "browser-can-write"
    }"#;
    let err =
        EconomyDefinition::from_json_str(bad).expect_err("non-canonical boundary must fail closed");
    assert!(err
        .to_string()
        .contains("canonical read-only/proposal-only"));
}

#[test]
fn state_with_wrong_boundary_is_rejected() {
    let def = definition();
    let mut state = def.initial_state();
    state.boundary = "browser-can-write".to_string();
    let err = def
        .validate_state(&state)
        .expect_err("non-canonical state boundary must fail closed");
    assert!(err
        .to_string()
        .contains("canonical read-only/proposal-only"));
}

#[test]
fn state_from_foreign_profile_is_rejected() {
    let def = definition();
    let mut state = def.initial_state();
    state.profile_id = "other-profile".to_string();
    let err = def
        .validate_state(&state)
        .expect_err("foreign profile state must fail closed");
    assert!(err.to_string().contains("does not match"));
}

#[test]
fn state_with_undeclared_currency_balance_is_rejected() {
    let def = definition();
    let mut state = def.initial_state();
    state.balances.insert("rubies".to_string(), 5);
    let err = def
        .validate_state(&state)
        .expect_err("undeclared currency balance must fail closed");
    assert!(err.to_string().contains("do not match declared currencies"));
}

#[test]
fn read_model_exposes_read_only_summary() {
    let def = definition();
    let state = def
        .apply_transaction(&def.initial_state(), &earn("t", "coins", 42))
        .expect("apply");
    let read = state.read_model();
    assert_eq!(read.profile_id, "demo-profile");
    assert_eq!(read.balances.get("coins"), Some(&42));
    assert_eq!(read.applied_tx_count, 1);
    assert_eq!(read.boundary, ECONOMY_SYSTEM_BOUNDARY);
}
