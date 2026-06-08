//! Scenario Coverage v44 — Run and Shop Regression Suite (#1809).
//!
//! Locks Escalating Run Structure and Shop Economy v1 behavior with
//! state/shape regressions only: bounded run escalation, terminal win/loss,
//! shop buy/sell/reroll/remove, generated-state wording, and substrate
//! run/economy backward compatibility. No timing, network, live browser,
//! trusted write, auto-merge, or automated fun/quality assertions are introduced.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::{
    resolve_card_roguelite_run_ante, resolve_card_roguelite_shop_economy,
    resolve_card_roguelite_state, validate_card_roguelite_config, CardRogueliteConfig,
    CardRogueliteShopCommand, CardRogueliteStatus,
};
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

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("parse {relative}: {error}"))
}

fn read_config(relative: &str) -> CardRogueliteConfig {
    serde_json::from_value(read_json(relative)).expect("run/shop config parses")
}

fn assert_repo_ref(relative: &str) {
    assert!(
        repo_root().join(relative).is_file(),
        "stale fixture ref: {relative}"
    );
}

const MATRIX: &str = "examples/run-shop-v1/scenario-coverage-v44/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v44.md";

#[test]
fn v44_matrix_enumerates_run_shop_regressions() {
    let matrix = read_json(MATRIX);
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v44");
    assert_eq!(matrix["issue"], 1809);
    assert_eq!(matrix["fixtureScoped"], true);

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "rust/local owns run and shop regression coverage",
        "browser/studio surfaces are read-only",
        "state/shape assertions only",
        "no timing",
        "no network",
        "no live browser",
        "trusted writes",
        "auto-apply",
        "auto-merge",
        "automated fun score",
        "quality verdict",
        "#1 and #23 remain open",
    ] {
        assert!(boundary.contains(required), "missing boundary: {required}");
    }

    let scenarios = matrix["scenarios"].as_array().expect("scenarios array");
    assert!(
        scenarios.len() >= 9,
        "v44 enumerates all regression surfaces"
    );
    let mut ids = BTreeSet::new();
    let mut systems = BTreeSet::new();
    for scenario in scenarios {
        let id = scenario["id"].as_str().expect("scenario id");
        assert!(ids.insert(id.to_string()), "duplicate scenario id {id}");
        systems.insert(scenario["system"].as_str().expect("system").to_string());
        assert!(scenario["kind"].is_string(), "{id} has kind");
        assert_repo_ref(scenario["fixtureRef"].as_str().expect("fixture ref"));
        assert!(scenario["expect"].is_string(), "{id} has expectation");
    }
    for system in ["run", "shop", "backcompat", "governance"] {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    for id in [
        "v44-run-escalation-quota-curve",
        "v44-run-terminal-win",
        "v44-run-terminal-loss",
        "v44-shop-buy-integrity",
        "v44-shop-sell-integrity",
        "v44-shop-reroll-determinism",
        "v44-shop-remove-probability-lever",
        "v44-backcompat-substrate-run-economy-golden",
    ] {
        assert!(ids.contains(id), "missing scenario {id}");
    }
}

#[test]
fn v44_run_escalation_win_and_loss_states_are_locked() {
    let win = read_config("examples/run-shop-v1/fixtures/escalating.win.json");
    validate_card_roguelite_config(&win).expect("win fixture validates");
    let first = resolve_card_roguelite_run_ante(&win).expect("winning run resolves");
    let second = resolve_card_roguelite_run_ante(&win).expect("winning run replays");

    assert_eq!(first, second, "same seed and config replay exactly");
    assert!(first.bounded);
    assert_eq!(first.max_ante, 3);
    assert_eq!(first.terminal_status, CardRogueliteStatus::Won);
    assert_eq!(first.total_score, 84);
    assert_eq!(first.final_gold, 47);
    assert_eq!(
        first
            .rounds
            .iter()
            .map(|round| round.quota)
            .collect::<Vec<_>>(),
        [40, 64, 84]
    );
    assert!(first.rounds.iter().all(|round| round.passed));
    assert!(!first.budget_exhausted);
    assert_eq!(first.seed_algorithm, "mulberry32");
    assert_eq!(first.digest.value.len(), 16);

    let loss = read_config("examples/run-shop-v1/fixtures/escalating.loss.json");
    validate_card_roguelite_config(&loss).expect("loss fixture validates");
    let report = resolve_card_roguelite_run_ante(&loss).expect("losing run resolves");
    assert_eq!(report.terminal_status, CardRogueliteStatus::Lost);
    assert_eq!(report.total_score, 32);
    assert_eq!(report.rounds.len(), 2);
    assert!(report.rounds[0].passed);
    assert!(!report.rounds[1].passed);
    assert!(report.budget_exhausted);
    assert_eq!(report.final_gold, 13);
}

#[test]
fn v44_shop_buy_sell_reroll_and_remove_shapes_are_locked() {
    let buy_sell = read_config("examples/run-shop-v1/fixtures/shop.buy-sell.json");
    validate_card_roguelite_config(&buy_sell).expect("buy/sell fixture validates");
    let state = resolve_card_roguelite_state(&buy_sell).expect("buy/sell state resolves");
    let offer = state.shop_offers[0].clone();
    let report = resolve_card_roguelite_shop_economy(
        &buy_sell,
        &[
            CardRogueliteShopCommand::Buy { offer_index: 0 },
            CardRogueliteShopCommand::Sell {
                deck_index: state.deck.len(),
            },
        ],
    )
    .expect("buy/sell resolves");
    assert_eq!(report.transactions.len(), 2);
    assert_eq!(report.transactions[0].acquired_card_id, Some(offer.card_id));
    assert_eq!(report.transactions[0].gold_after, state.gold - offer.price);
    assert_eq!(report.transactions[0].deck_size_after, state.deck.len() + 1);
    assert_eq!(
        report.transactions[1].removed_card_id,
        report.transactions[0].acquired_card_id
    );
    assert_eq!(report.final_deck, state.deck);
    assert!(report.final_gold > report.transactions[0].gold_after);
    assert!(report.final_gold <= state.gold);

    let reroll = read_config("examples/run-shop-v1/fixtures/shop.reroll.json");
    validate_card_roguelite_config(&reroll).expect("reroll fixture validates");
    let first = resolve_card_roguelite_shop_economy(&reroll, &[CardRogueliteShopCommand::Reroll])
        .expect("reroll resolves");
    let second = resolve_card_roguelite_shop_economy(&reroll, &[CardRogueliteShopCommand::Reroll])
        .expect("reroll replays");
    let mut other_seed = reroll.clone();
    other_seed.seed += 1;
    let divergent =
        resolve_card_roguelite_shop_economy(&other_seed, &[CardRogueliteShopCommand::Reroll])
            .expect("different seed resolves");
    assert_eq!(first.digest, second.digest);
    assert_ne!(first.starting_offers, first.final_offers);
    assert_ne!(first.final_offers, divergent.final_offers);
    assert_eq!(first.reroll_count, 1);
    assert_eq!(first.seed_algorithm, "mulberry32");

    let remove = read_config("examples/run-shop-v1/fixtures/shop.remove.json");
    validate_card_roguelite_config(&remove).expect("remove fixture validates");
    let state = resolve_card_roguelite_state(&remove).expect("remove state resolves");
    let wound_index = state
        .deck
        .iter()
        .position(|card| card == "wound")
        .expect("fixture has wound");
    let report = resolve_card_roguelite_shop_economy(
        &remove,
        &[CardRogueliteShopCommand::Remove {
            deck_index: wound_index,
        }],
    )
    .expect("remove resolves");
    assert!(report.removal_lever_used);
    assert_eq!(
        report.transactions[0].removed_card_id,
        Some("wound".to_string())
    );
    assert_eq!(report.final_deck.len(), report.starting_deck.len() - 1);
    assert!(!report.final_deck.iter().any(|card| card == "wound"));
    assert_eq!(
        report.transactions[0].gold_before - report.transactions[0].gold_after,
        3
    );
    assert!(report
        .read_only_inspection
        .disallowed_actions
        .contains(&"trusted writes".to_string()));
}

#[test]
fn v44_substrate_run_economy_backcompat_golden_remains_valid() {
    let golden =
        read_json("examples/run-shop-v1/scenario-coverage-v44/substrate-run-economy.golden.json");
    assert_eq!(
        golden["schemaVersion"],
        "run-shop-v44-substrate-run-economy-backcompat-golden.v1"
    );
    assert_eq!(golden["fixtureScoped"], true);
    let config_ref = golden["configRef"].as_str().expect("config ref");
    assert_repo_ref(config_ref);

    let config = read_config(config_ref);
    validate_card_roguelite_config(&config).expect("classic substrate validates");
    let state = resolve_card_roguelite_state(&config).expect("classic state resolves");
    let run = resolve_card_roguelite_run_ante(&config).expect("classic run resolves");
    let shop = resolve_card_roguelite_shop_economy(&config, &[CardRogueliteShopCommand::Reroll])
        .expect("classic shop reroll resolves");

    assert_eq!(
        state.config_id,
        golden["expectedConfigId"].as_str().unwrap()
    );
    assert_eq!(state.variant, golden["expectedVariant"].as_str().unwrap());
    assert_eq!(state.seed, golden["expectedSeed"].as_u64().unwrap() as u32);
    assert_eq!(
        state.digest.value,
        golden["expectedStateDigest"].as_str().unwrap()
    );
    assert_eq!(run.terminal_status, CardRogueliteStatus::Won);
    assert_eq!(golden["expectedRunStatus"], "won");
    assert_eq!(
        run.total_score,
        golden["expectedRunTotalScore"].as_i64().unwrap() as i32
    );
    assert_eq!(
        run.final_gold,
        golden["expectedRunFinalGold"].as_i64().unwrap() as i32
    );
    assert_eq!(
        run.digest.value,
        golden["expectedRunDigest"].as_str().unwrap()
    );
    assert_eq!(
        shop.starting_gold,
        golden["expectedShopStartingGold"].as_i64().unwrap() as i32
    );
    assert_eq!(
        shop.final_gold,
        golden["expectedShopFinalGoldAfterReroll"].as_i64().unwrap() as i32
    );
    assert_eq!(
        shop.reroll_count,
        golden["expectedShopRerollCount"].as_u64().unwrap() as u32
    );
    assert_eq!(
        shop.digest.value,
        golden["expectedShopDigestAfterReroll"].as_str().unwrap()
    );

    let starting_offer_ids = golden["expectedStartingOfferIds"]
        .as_array()
        .expect("starting offers")
        .iter()
        .map(|entry| entry.as_str().expect("offer id"))
        .collect::<Vec<_>>();
    assert_eq!(
        shop.starting_offers
            .iter()
            .map(|offer| offer.card_id.as_str())
            .collect::<Vec<_>>(),
        starting_offer_ids
    );
    let final_offer_ids = golden["expectedFinalOfferIdsAfterReroll"]
        .as_array()
        .expect("final offers")
        .iter()
        .map(|entry| entry.as_str().expect("offer id"))
        .collect::<Vec<_>>();
    assert_eq!(
        shop.final_offers
            .iter()
            .map(|offer| offer.card_id.as_str())
            .collect::<Vec<_>>(),
        final_offer_ids
    );
}

#[test]
fn v44_doc_preserves_state_shape_wording_and_governance() {
    let doc = read_text(DOC);
    assert!(doc.contains("#1") && doc.contains("#23"));
    let lowered = doc.to_ascii_lowercase();
    for required in [
        "state/shape regressions only",
        "fixture-scoped",
        "no timing",
        "no network",
        "no live browser",
        "read-only",
        "no auto-merge",
        "no auto-apply",
        "production-ready engine",
        "godot replacement/parity",
        "automated fun score",
        "generated runs",
        "remain open governance anchors",
        "cargo test -p ouroforge-core --test scenario_coverage_v44_run_shop --jobs 2",
    ] {
        assert!(lowered.contains(required), "doc missing {required}");
    }
}
