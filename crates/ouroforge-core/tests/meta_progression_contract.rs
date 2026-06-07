//! Contract tests for Meta-Progression and Unlocks v1 (#1657).
//!
//! Covers the three required behaviors: progression accrual, unlock gating, and
//! save/restore parity, plus the fail-closed negative cases that keep the data
//! system deterministic and trusted-only.

use ouroforge_core::meta_progression::{
    MetaProgressionDefinition, MetaProgressionState, RunOutcome, META_PROGRESSION_BOUNDARY,
    META_PROGRESSION_SCHEMA_VERSION,
};

fn valid_definition_json() -> &'static str {
    include_str!("../../../examples/meta-progression-v1/valid/meta-progression.definition.json")
}

fn definition() -> MetaProgressionDefinition {
    MetaProgressionDefinition::from_json_str(valid_definition_json())
        .expect("valid meta-progression definition parses")
}

fn run(run_id: &str, deltas: &[(&str, u64)]) -> RunOutcome {
    RunOutcome {
        run_id: run_id.to_string(),
        deltas: deltas.iter().map(|(k, v)| ((*k).to_string(), *v)).collect(),
    }
}

#[test]
fn valid_definition_loads_and_initial_state_is_zeroed() {
    let def = definition();
    assert_eq!(def.profile_id, "demo-profile");
    assert_eq!(def.counters.len(), 3);

    let state = def.initial_state();
    assert_eq!(state.schema_version, META_PROGRESSION_SCHEMA_VERSION);
    assert_eq!(state.boundary, META_PROGRESSION_BOUNDARY);
    assert_eq!(state.counters.get("runsCompleted"), Some(&0));
    assert_eq!(state.counters.get("coinsEarned"), Some(&0));
    assert_eq!(state.counters.get("bossesDefeated"), Some(&0));
    assert!(state.applied_runs.is_empty());
    // Only the zero-threshold unlock is satisfied at the start.
    assert!(state.is_unlocked("starter-kit"));
    assert!(!state.is_unlocked("second-character"));
    assert!(!state.is_unlocked("gold-shop"));
}

#[test]
fn progression_accrues_across_runs_deterministically() {
    let def = definition();
    let outcomes = [
        run("run-1", &[("runsCompleted", 1), ("coinsEarned", 40)]),
        run("run-2", &[("runsCompleted", 1), ("coinsEarned", 35)]),
        run(
            "run-3",
            &[
                ("runsCompleted", 1),
                ("coinsEarned", 50),
                ("bossesDefeated", 1),
            ],
        ),
    ];

    let state = def
        .apply_run_outcomes(&def.initial_state(), &outcomes)
        .expect("outcomes apply");

    assert_eq!(state.counters.get("runsCompleted"), Some(&3));
    assert_eq!(state.counters.get("coinsEarned"), Some(&125));
    assert_eq!(state.counters.get("bossesDefeated"), Some(&1));
    assert_eq!(state.applied_runs, vec!["run-1", "run-2", "run-3"]);

    // Replaying the same outcomes from the same start reproduces the same state.
    let replay = def
        .apply_run_outcomes(&def.initial_state(), &outcomes)
        .expect("replay applies");
    assert_eq!(state, replay);
}

#[test]
fn unlocks_gate_on_thresholds_and_are_monotonic() {
    let def = definition();
    let mut state = def.initial_state();

    // Below all thresholds (except the zero one): nothing new unlocks.
    state = def
        .apply_run_outcome(
            &state,
            &run("r1", &[("runsCompleted", 1), ("coinsEarned", 40)]),
        )
        .expect("apply r1");
    assert!(!state.is_unlocked("second-character"));
    assert!(!state.is_unlocked("gold-shop"));

    // Crossing the coins threshold unlocks the gold shop.
    state = def
        .apply_run_outcome(&state, &run("r2", &[("coinsEarned", 60)]))
        .expect("apply r2");
    assert!(state.is_unlocked("gold-shop"));

    // Crossing the runs threshold unlocks the second character.
    state = def
        .apply_run_outcome(&state, &run("r3", &[("runsCompleted", 2)]))
        .expect("apply r3");
    assert!(state.is_unlocked("second-character"));

    // Further accrual never removes an unlock (monotonic).
    let before = state.unlocked.clone();
    state = def
        .apply_run_outcome(&state, &run("r4", &[("runsCompleted", 10)]))
        .expect("apply r4");
    assert!(before.iter().all(|u| state.is_unlocked(u)));
    assert!(state.is_unlocked("gold-shop"));
    assert!(state.is_unlocked("second-character"));
}

#[test]
fn save_restore_round_trips_unchanged() {
    let def = definition();
    let state = def
        .apply_run_outcomes(
            &def.initial_state(),
            &[
                run("a", &[("runsCompleted", 3), ("coinsEarned", 120)]),
                run("b", &[("bossesDefeated", 2)]),
            ],
        )
        .expect("apply");

    let json = state.to_json().expect("serialize");
    let restored = MetaProgressionState::from_json_str(&json).expect("deserialize");

    assert_eq!(state, restored);
    // A restored state validates against its definition and preserves unlocks.
    def.validate_state(&restored)
        .expect("restored state is consistent");
    assert!(restored.is_unlocked("second-character"));
    assert!(restored.is_unlocked("gold-shop"));
    assert!(restored.is_unlocked("true-ending"));

    // Applying further outcomes after restore continues deterministically.
    let continued = def
        .apply_run_outcome(&restored, &run("c", &[("coinsEarned", 5)]))
        .expect("apply after restore");
    assert_eq!(continued.counters.get("coinsEarned"), Some(&125));
}

#[test]
fn duplicate_run_id_is_rejected() {
    let def = definition();
    let state = def
        .apply_run_outcome(&def.initial_state(), &run("dup", &[("runsCompleted", 1)]))
        .expect("first apply");
    let err = def
        .apply_run_outcome(&state, &run("dup", &[("runsCompleted", 1)]))
        .expect_err("duplicate run id must fail closed");
    assert!(err.to_string().contains("already applied"));
}

#[test]
fn unknown_counter_is_rejected() {
    let def = definition();
    let err = def
        .apply_run_outcome(&def.initial_state(), &run("x", &[("notACounter", 1)]))
        .expect_err("undeclared counter must fail closed");
    assert!(err.to_string().contains("undeclared counter"));
}

#[test]
fn accrual_overflow_is_rejected() {
    let def = definition();
    let state = def
        .apply_run_outcome(
            &def.initial_state(),
            &run("max", &[("coinsEarned", u64::MAX)]),
        )
        .expect("apply max");
    let err = def
        .apply_run_outcome(&state, &run("more", &[("coinsEarned", 1)]))
        .expect_err("overflow must fail closed");
    assert!(err.to_string().contains("overflow"));
}

#[test]
fn wrong_schema_version_is_rejected() {
    let bad = r#"{
        "schemaVersion": "meta-progression-v0",
        "profileId": "p",
        "counters": ["a"],
        "unlocks": [],
        "boundary": "x"
    }"#;
    let err = MetaProgressionDefinition::from_json_str(bad)
        .expect_err("wrong schema version must fail closed");
    assert!(err.to_string().contains("schema version"));
}

#[test]
fn unlock_referencing_undeclared_counter_is_rejected() {
    let bad = r#"{
        "schemaVersion": "meta-progression-v1",
        "profileId": "p",
        "counters": ["a"],
        "unlocks": [{ "unlockId": "u", "counter": "missing", "threshold": 1 }],
        "boundary": "x"
    }"#;
    let err = MetaProgressionDefinition::from_json_str(bad)
        .expect_err("unlock over undeclared counter must fail closed");
    assert!(err.to_string().contains("undeclared counter"));
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
fn tampered_unlock_without_threshold_is_rejected() {
    let def = definition();
    // A stale/tampered save that claims a gated unlock while its counter is
    // still below the threshold must fail closed on restore.
    let mut state = def.initial_state();
    state.unlocked.insert("true-ending".to_string()); // requires bossesDefeated >= 2
    let err = def
        .validate_state(&state)
        .expect_err("unjustified unlock must fail closed");
    assert!(err.to_string().contains("not justified"));
}

#[test]
fn state_missing_required_unlock_is_rejected() {
    let def = definition();
    // Counters justify an unlock but the save omits it: also inconsistent.
    let mut state = def.initial_state();
    state.counters.insert("coinsEarned".to_string(), 100);
    let err = def
        .validate_state(&state)
        .expect_err("missing required unlock must fail closed");
    assert!(err.to_string().contains("missing unlock"));
}

#[test]
fn read_model_exposes_read_only_summary() {
    let def = definition();
    let state = def
        .apply_run_outcome(&def.initial_state(), &run("r", &[("coinsEarned", 100)]))
        .expect("apply");
    let read = state.read_model();
    assert_eq!(read.profile_id, "demo-profile");
    assert_eq!(read.unlock_count, read.unlocked.len());
    assert!(read.unlocked.iter().any(|u| u == "gold-shop"));
    assert_eq!(read.applied_run_count, 1);
    assert_eq!(read.boundary, META_PROGRESSION_BOUNDARY);
}
