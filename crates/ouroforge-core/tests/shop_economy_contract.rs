//! Shop economy contract (#1807).
//!
//! Locks buy/sell/reroll/remove as deterministic Rust-local substrate economy
//! behavior. This is fixture-scoped mechanical evidence only: no browser/Studio
//! trusted writes, no parallel engine, and no automated fun or release claim.

use std::path::PathBuf;

use ouroforge_core::{
    resolve_card_roguelite_shop_economy, resolve_card_roguelite_state, CardRogueliteConfig,
    CardRogueliteShopCommand, CARD_ROGUELITE_SHOP_ECONOMY_SCHEMA_VERSION,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("crate lives under crates/ouroforge-core")
        .to_path_buf()
}

fn read_json(relative: &str) -> Value {
    let path = repo_root().join(relative);
    serde_json::from_str(&std::fs::read_to_string(&path).expect("fixture is readable"))
        .expect("fixture parses as json")
}

fn read_config(relative: &str) -> CardRogueliteConfig {
    serde_json::from_value(read_json(relative)).expect("config parses")
}

#[test]
fn purchase_and_sell_integrity_is_checked() {
    let config = read_config("examples/run-shop-v1/fixtures/shop.buy-sell.json");
    let state = resolve_card_roguelite_state(&config).expect("state resolves");
    let offer = state.shop_offers[0].clone();
    let sell_index = state.deck.len();

    let report = resolve_card_roguelite_shop_economy(
        &config,
        &[
            CardRogueliteShopCommand::Buy { offer_index: 0 },
            CardRogueliteShopCommand::Sell {
                deck_index: sell_index,
            },
        ],
    )
    .expect("shop commands resolve");

    assert_eq!(
        report.schema_version,
        CARD_ROGUELITE_SHOP_ECONOMY_SCHEMA_VERSION
    );
    assert_eq!(report.transactions.len(), 2);
    assert_eq!(report.transactions[0].acquired_card_id, Some(offer.card_id));
    assert_eq!(report.transactions[0].gold_after, state.gold - offer.price);
    assert_eq!(report.transactions[0].deck_size_after, state.deck.len() + 1);
    assert_eq!(
        report.transactions[1].removed_card_id,
        report.transactions[0].acquired_card_id
    );
    assert_eq!(
        report.final_deck, state.deck,
        "selling the bought card restores deck shape"
    );
    assert!(report.final_gold > report.transactions[0].gold_after);
    assert!(
        report.final_gold <= state.gold,
        "sell value never fabricates profit over buy price"
    );
    assert!(report
        .read_only_inspection
        .disallowed_actions
        .contains(&"trusted writes".to_string()));
}

#[test]
fn reroll_is_seed_deterministic_and_records_per_run_variance() {
    let config = read_config("examples/run-shop-v1/fixtures/shop.reroll.json");
    let first = resolve_card_roguelite_shop_economy(&config, &[CardRogueliteShopCommand::Reroll])
        .expect("first reroll resolves");
    let second = resolve_card_roguelite_shop_economy(&config, &[CardRogueliteShopCommand::Reroll])
        .expect("second reroll resolves");
    let mut other_seed = config.clone();
    other_seed.seed += 1;
    let divergent =
        resolve_card_roguelite_shop_economy(&other_seed, &[CardRogueliteShopCommand::Reroll])
            .expect("different seed reroll resolves");

    assert_eq!(first.digest, second.digest);
    assert_eq!(first.final_offers, second.final_offers);
    assert_ne!(first.starting_offers, first.final_offers);
    assert_ne!(first.final_offers, divergent.final_offers);
    assert_eq!(first.reroll_count, 1);
    assert_eq!(first.seed_algorithm, "mulberry32");
}

#[test]
fn removal_is_a_probability_lever_over_the_seeded_deck() {
    let config = read_config("examples/run-shop-v1/fixtures/shop.remove.json");
    let state = resolve_card_roguelite_state(&config).expect("state resolves");
    let wound_index = state
        .deck
        .iter()
        .position(|card| card == "wound")
        .expect("fixture has one clutter card after shuffle");

    let report = resolve_card_roguelite_shop_economy(
        &config,
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
}

#[test]
fn insufficient_gold_fails_closed_before_mutating_report() {
    let mut config = read_config("examples/run-shop-v1/fixtures/shop.remove.json");
    config.shop.base_gold = 0;
    config.run.ante_steps[0].reward_gold = 0;
    let state = resolve_card_roguelite_state(&config).expect("state resolves with zero gold");
    let deck_index = state
        .deck
        .iter()
        .position(|card| card == "wound")
        .expect("fixture has clutter card");

    let error = resolve_card_roguelite_shop_economy(
        &config,
        &[CardRogueliteShopCommand::Remove { deck_index }],
    )
    .expect_err("insufficient gold fails closed");

    assert!(error.to_string().contains("insufficient gold to remove"));
}
