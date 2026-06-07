//! Scenario Coverage v37 — Long-Form Game Systems Regression Suite (#1663).
//!
//! Locks Long-Form Game Systems v1 behavior: meta-progression/unlocks (#1657),
//! economy/currency (#1658), save/profile + run-history at scale (#1659), UI/UX
//! flow + accessibility (#1660), and the optional narrative/dialogue/event system
//! (#1661), plus the backward-compatibility guarantee that an existing single-run
//! save/restore remains valid. State/shape assertions only — no flaky or
//! timing-based checks — so a breaking change fails CI.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use ouroforge_core::economy_system::{
    EconomyDefinition, EconomyTransaction, EconomyTransactionKind,
};
use ouroforge_core::meta_progression::{MetaProgressionDefinition, RunOutcome};
use ouroforge_core::narrative_system::NarrativeDefinition;
use ouroforge_core::save_profile_scale::{RunHistoryEntry, SaveStore};
use ouroforge_core::uiux_flow::UiuxFlowContract;
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn read_text(relative: &str) -> String {
    let path = repo_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn demo(relative: &str) -> String {
    read_text(&format!("examples/long-form-systems-v1/demo/{relative}"))
}

fn coverage(relative: &str) -> String {
    read_text(&format!(
        "examples/long-form-systems-v1/scenario-coverage-v37/{relative}"
    ))
}

fn matrix() -> Value {
    serde_json::from_str(&coverage("matrix.fixture.json")).expect("matrix parses")
}

fn run_outcome(run_id: &str, deltas: &[(&str, u64)]) -> RunOutcome {
    RunOutcome {
        run_id: run_id.to_string(),
        deltas: deltas
            .iter()
            .map(|(k, v)| ((*k).to_string(), *v))
            .collect::<BTreeMap<_, _>>(),
    }
}

fn tx(
    tx_id: &str,
    kind: EconomyTransactionKind,
    currency: &str,
    amount: u64,
) -> EconomyTransaction {
    EconomyTransaction {
        tx_id: tx_id.to_string(),
        kind,
        currency: currency.to_string(),
        amount,
    }
}

fn entry(run_id: &str, digest: &str) -> RunHistoryEntry {
    RunHistoryEntry {
        run_id: run_id.to_string(),
        recorded_at: "2026-06-07T00:00:00Z".to_string(),
        replay_digest: digest.to_string(),
    }
}

#[test]
fn v37_matrix_is_enumerated() {
    let matrix = matrix();
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v37");
    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 12,
        "v37 enumerates the milestone systems"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(
            scenario["system"]
                .as_str()
                .expect("scenario system")
                .to_string(),
        );
        assert!(scenario["kind"].is_string());
        assert!(scenario["expect"].is_string());
    }
    for system in [
        "meta-progression",
        "economy",
        "save-profile",
        "uiux-flow",
        "narrative",
    ] {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("backcompat-single-run-save-restore"));
}

#[test]
fn v37_meta_progression_accrual_and_fail_closed() {
    let def = MetaProgressionDefinition::from_json_str(&demo("meta.definition.json"))
        .expect("meta definition");
    let state = def
        .apply_run_outcomes(
            &def.initial_state(),
            &[
                run_outcome("r1", &[("runsCompleted", 1), ("coinsEarned", 60)]),
                run_outcome("r2", &[("runsCompleted", 2), ("coinsEarned", 60)]),
            ],
        )
        .expect("apply");
    assert_eq!(state.counters.get("runsCompleted"), Some(&3));
    assert!(state.is_unlocked("second-character"));
    assert!(state.is_unlocked("gold-shop"));
    // Replay determinism.
    let replay = def
        .apply_run_outcomes(
            &def.initial_state(),
            &[
                run_outcome("r1", &[("runsCompleted", 1), ("coinsEarned", 60)]),
                run_outcome("r2", &[("runsCompleted", 2), ("coinsEarned", 60)]),
            ],
        )
        .expect("replay");
    assert_eq!(state, replay);

    // Fail-closed: an unlock not justified by the counters is rejected on restore.
    let mut tampered = def.initial_state();
    tampered.unlocked.insert("gold-shop".to_string());
    assert!(def.validate_state(&tampered).is_err());
}

#[test]
fn v37_economy_earn_spend_and_non_negative() {
    let def = EconomyDefinition::from_json_str(&demo("economy.definition.json"))
        .expect("economy definition");
    let state = def
        .apply_transactions(
            &def.initial_state(),
            &[
                tx("e1", EconomyTransactionKind::Earn, "coins", 100),
                tx("s1", EconomyTransactionKind::Spend, "coins", 40),
            ],
        )
        .expect("apply");
    assert_eq!(state.balance("coins"), 60);
    // Fail-closed non-negative invariant.
    assert!(def
        .apply_transaction(
            &state,
            &tx("s2", EconomyTransactionKind::Spend, "coins", 61)
        )
        .is_err());
}

#[test]
fn v37_save_isolation_scale_and_tamper() {
    let mut store = SaveStore::new();
    store.append_run("alice", entry("a1", "d1")).expect("a1");
    store.append_run("bob", entry("b1", "d2")).expect("b1");
    // Isolation.
    assert_eq!(store.profile("alice").unwrap().run_history.len(), 1);
    assert_eq!(store.profile("bob").unwrap().run_history.len(), 1);

    // Scale + integrity round-trip.
    let mut scaler = SaveStore::new();
    for i in 0..2000u32 {
        scaler
            .append_run(
                "scaler",
                entry(&format!("run-{i:05}"), &format!("dg-{i:05}")),
            )
            .expect("append");
    }
    let restored = SaveStore::from_json_str(&scaler.to_json().unwrap()).expect("round-trip");
    assert_eq!(restored.profile("scaler").unwrap().run_history.len(), 2000);

    // Fail-closed tamper.
    let mut tampered = scaler.clone();
    tampered.profiles.get_mut("scaler").unwrap().run_history[100].replay_digest =
        "tampered".to_string();
    assert!(tampered.verify_integrity().is_err());
}

#[test]
fn v37_uiux_flow_shape_and_fail_closed() {
    let flow = UiuxFlowContract::from_json_str(&demo("uiux-flow.json")).expect("uiux flow");
    let read = flow.read_model();
    assert_eq!(read.screen_count, 4);
    assert!(read.accessibility_option_count >= 1);

    // Fail-closed: an unreachable screen is rejected.
    let mut json: Value = serde_json::from_str(&demo("uiux-flow.json")).unwrap();
    json["screens"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({ "id": "orphan", "kind": "menu" }));
    assert!(UiuxFlowContract::from_json_str(&json.to_string()).is_err());
}

#[test]
fn v37_narrative_event_trigger_and_fail_closed() {
    let def = NarrativeDefinition::from_json_str(&demo("narrative.definition.json"))
        .expect("narrative definition");
    let ended = def
        .advance(&def.initial_state(), None)
        .and_then(|s| def.advance(&s, None))
        .expect("advance to end");
    assert!(ended.is_ended());
    assert_eq!(ended.flags.get("tutorialDone"), Some(&true));
    assert!(ended.fired_events.contains("logTutorial"));

    // Fail-closed: a restored state marking an event fired while its effect flag
    // is not set is rejected (the event would otherwise be skipped forever).
    let effectful = NarrativeDefinition::from_json_str(
        r#"{
            "schemaVersion": "narrative-system-v1",
            "storyId": "coverage-story",
            "flags": ["trigger", "effect"],
            "initialNode": "n1",
            "nodes": [{ "id": "n1", "setFlags": ["trigger"] }],
            "events": [
                { "id": "ev", "when": [{ "flag": "trigger", "equals": true }], "setFlags": ["effect"] }
            ],
            "boundary": "rust-trusted-state; browser/studio read-only; generation proposal-only via review/apply/trust-gradient"
        }"#,
    )
    .expect("effectful narrative definition");
    let mut bad = effectful.initial_state();
    bad.fired_events.insert("ev".to_string()); // marked fired, but "effect" is still false
    let err = effectful
        .validate_state(&bad)
        .expect_err("unjustified fired event must be rejected");
    assert!(err.to_string().contains("effect flag"));
}

#[test]
fn v37_backward_compatibility_single_run_save_restore() {
    // A prior single-profile save-profile-v0 document still migrates and verifies.
    let migrated =
        SaveStore::from_legacy_v0_json(&coverage("legacy-save.v0.json")).expect("legacy migrates");
    let profile = migrated.profile("legacy-solo").expect("legacy profile");
    assert_eq!(profile.run_history.len(), 1);
    assert_eq!(profile.run_history[0].run_id, "legacy-run-1");
    migrated.verify_integrity().expect("migrated verifies");

    // A single-run v1 store still round-trips unchanged.
    let mut store = SaveStore::new();
    store
        .append_run("solo", entry("only-run", "digest"))
        .expect("append");
    let restored = SaveStore::from_json_str(&store.to_json().unwrap()).expect("round-trip");
    assert_eq!(store, restored);
    assert_eq!(restored.profile("solo").unwrap().run_history.len(), 1);
}
