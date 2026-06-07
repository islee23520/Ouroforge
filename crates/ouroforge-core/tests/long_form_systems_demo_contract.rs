//! Long-Form Game Systems Demo v1 (#1662) — a deterministic smoke test that
//! composes the milestone's systems (meta-progression, economy, save/profile +
//! run-history, UI/UX flow, and the optional narrative system) into one longer
//! slice and asserts both the composed system behavior and the Milestone 24 rung
//! linkage (four-gate + loop-coverage evidence) recorded in the ladder fixture.
//!
//! Deterministic and fixture-scoped: no network, no live browser. See
//! `docs/long-form-systems-v1-demo.md`.

use std::collections::BTreeMap;

use ouroforge_core::complexity_ladder::{
    evaluate_complexity_ladder, ComplexityLadder, ComplexityRungStatus,
};
use ouroforge_core::economy_system::{
    EconomyDefinition, EconomyTransaction, EconomyTransactionKind,
};
use ouroforge_core::meta_progression::{MetaProgressionDefinition, RunOutcome};
use ouroforge_core::narrative_system::NarrativeDefinition;
use ouroforge_core::save_profile_scale::{RunHistoryEntry, SaveStore};
use ouroforge_core::uiux_flow::UiuxFlowContract;

fn meta_def() -> MetaProgressionDefinition {
    MetaProgressionDefinition::from_json_str(include_str!(
        "../../../examples/long-form-systems-v1/demo/meta.definition.json"
    ))
    .expect("meta definition")
}

fn economy_def() -> EconomyDefinition {
    EconomyDefinition::from_json_str(include_str!(
        "../../../examples/long-form-systems-v1/demo/economy.definition.json"
    ))
    .expect("economy definition")
}

fn uiux_contract() -> UiuxFlowContract {
    UiuxFlowContract::from_json_str(include_str!(
        "../../../examples/long-form-systems-v1/demo/uiux-flow.json"
    ))
    .expect("uiux flow contract")
}

fn narrative_def() -> NarrativeDefinition {
    NarrativeDefinition::from_json_str(include_str!(
        "../../../examples/long-form-systems-v1/demo/narrative.definition.json"
    ))
    .expect("narrative definition")
}

fn ladder() -> ComplexityLadder {
    ComplexityLadder::from_json_str(include_str!(
        "../../../examples/long-form-systems-v1/demo/complexity-ladder.json"
    ))
    .expect("complexity ladder")
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

fn earn(tx: &str, currency: &str, amount: u64) -> EconomyTransaction {
    EconomyTransaction {
        tx_id: tx.to_string(),
        kind: EconomyTransactionKind::Earn,
        currency: currency.to_string(),
        amount,
    }
}

fn spend(tx: &str, currency: &str, amount: u64) -> EconomyTransaction {
    EconomyTransaction {
        tx_id: tx.to_string(),
        kind: EconomyTransactionKind::Spend,
        currency: currency.to_string(),
        amount,
    }
}

/// Run the composed longer slice across the systems for one profile and return
/// the observable end-state tuple used by the behavior assertions.
#[allow(clippy::type_complexity)]
fn run_demo_slice() -> (
    ouroforge_core::meta_progression::MetaProgressionState,
    ouroforge_core::economy_system::EconomyState,
    SaveStore,
    ouroforge_core::narrative_system::NarrativeState,
) {
    // 1. Meta-progression: three recorded run outcomes accrue counters.
    let meta = meta_def();
    let meta_state = meta
        .apply_run_outcomes(
            &meta.initial_state(),
            &[
                run_outcome("run-1", &[("runsCompleted", 1), ("coinsEarned", 40)]),
                run_outcome("run-2", &[("runsCompleted", 1), ("coinsEarned", 35)]),
                run_outcome(
                    "run-3",
                    &[
                        ("runsCompleted", 1),
                        ("coinsEarned", 50),
                        ("bossesDefeated", 1),
                    ],
                ),
            ],
        )
        .expect("meta run outcomes apply");

    // 2. Economy: the runs earn currency and a spend is integrity-checked.
    let economy = economy_def();
    let economy_state = economy
        .apply_transactions(
            &economy.initial_state(),
            &[
                earn("run-1-coins", "coins", 40),
                earn("run-2-coins", "coins", 35),
                earn("run-3-coins", "coins", 50),
                spend("buy-relic", "coins", 25),
            ],
        )
        .expect("economy transactions apply");

    // 3. Save/profile: each run is appended to the profile run-history.
    let mut save = SaveStore::new();
    for (i, run_id) in ["run-1", "run-2", "run-3"].iter().enumerate() {
        save.append_run(
            "demo-hero",
            RunHistoryEntry {
                run_id: (*run_id).to_string(),
                recorded_at: format!("2026-06-07T00:0{i}:00Z"),
                replay_digest: format!("demo-digest-{run_id}"),
            },
        )
        .expect("append run history");
    }
    save.verify_integrity().expect("save store verifies");

    // 5. Narrative: a short dialogue advances and its events fire.
    let narrative = narrative_def();
    let narrative_state = narrative
        .advance(&narrative.initial_state(), None)
        .and_then(|s| narrative.advance(&s, None))
        .expect("narrative advances");

    (meta_state, economy_state, save, narrative_state)
}

#[test]
fn demo_composes_systems_with_expected_behavior() {
    let (meta_state, economy_state, save, narrative_state) = run_demo_slice();

    // Meta-progression behavior: counters and threshold unlocks.
    assert_eq!(meta_state.counters.get("runsCompleted"), Some(&3));
    assert_eq!(meta_state.counters.get("coinsEarned"), Some(&125));
    assert!(meta_state.is_unlocked("starter-kit"));
    assert!(meta_state.is_unlocked("second-character"));
    assert!(meta_state.is_unlocked("gold-shop"));

    // Economy behavior: earn/spend integrity and balance.
    assert_eq!(economy_state.balance("coins"), 100);

    // Save behavior: the profile holds the run-history with verified integrity.
    let profile = save.profile("demo-hero").expect("profile exists");
    assert_eq!(profile.run_history.len(), 3);

    // UI/UX behavior: the flow contract validates and exposes its surfaces.
    let uiux = uiux_contract().read_model();
    assert_eq!(uiux.screen_count, 4);
    assert_eq!(uiux.accessibility_option_count, 2);

    // Narrative behavior: the dialogue ended and its events fired.
    assert!(narrative_state.is_ended());
    assert_eq!(narrative_state.flags.get("tutorialDone"), Some(&true));
    assert!(narrative_state.fired_events.contains("logTutorial"));
}

#[test]
fn demo_records_satisfied_rungs_with_four_gate_and_loop_coverage() {
    let ladder = ladder();
    let evaluation = evaluate_complexity_ladder(&ladder).expect("ladder evaluates");
    assert_eq!(evaluation.rungs.len(), 4);
    for rung in &evaluation.rungs {
        assert_eq!(
            rung.status,
            ComplexityRungStatus::Satisfied,
            "rung {} should be satisfied, reasons: {:?}",
            rung.rung_id,
            rung.reasons
        );
        // Each satisfied rung links a four-gate verdict and a loop-coverage verdict.
        assert!(
            rung.evidence_refs.len() >= 2,
            "rung {} should link four-gate + loop-coverage evidence",
            rung.rung_id
        );
    }
    // The rung linkage covers the milestone systems in order.
    let rung_ids: Vec<&str> = evaluation
        .rungs
        .iter()
        .map(|r| r.rung_id.as_str())
        .collect();
    assert_eq!(
        rung_ids,
        vec![
            "meta-progression-v1",
            "economy-system-v1",
            "save-profile-scale-v1",
            "uiux-flow-v1",
        ]
    );
}

#[test]
fn demo_slice_is_deterministic() {
    let first = run_demo_slice();
    let second = run_demo_slice();
    assert_eq!(first.0, second.0);
    assert_eq!(first.1, second.1);
    assert_eq!(first.2, second.2);
    assert_eq!(first.3, second.3);
}

#[test]
fn demo_fixtures_are_present_and_loadable() {
    // The referenced demo fixtures load and validate (fail-closed parsing).
    let _ = meta_def();
    let _ = economy_def();
    let _ = uiux_contract();
    let _ = narrative_def();
    let _ = ladder();
}
