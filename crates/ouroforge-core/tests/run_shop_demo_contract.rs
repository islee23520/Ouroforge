//! Run and Shop Demo v1 smoke (#1808).
//!
//! Fixture-scoped deterministic evidence only: no network, live browser,
//! trusted writes, parallel engine, or subjective fun/release claims.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    resolve_card_roguelite_run_ante, resolve_card_roguelite_shop_economy,
    resolve_card_roguelite_state, CardRogueliteConfig, CardRogueliteShopCommand,
    CardRogueliteStatus,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DemoManifest {
    schema_version: String,
    demo_id: String,
    issue: u32,
    builds_against: BuildsAgainst,
    execution: DemoExecution,
    escalating_run: EscalatingRun,
    shop_lever: ShopLever,
    governance: DemoGovernance,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BuildsAgainst {
    deckbuilder_tree_ref: String,
    substrate_ref: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DemoExecution {
    mode: String,
    network: String,
    live_browser: String,
    browser_studio_mode: String,
    trusted_writes: String,
    fun_verdict: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EscalatingRun {
    config_ref: String,
    expected_status: String,
    expected_quota_curve: Vec<i32>,
    expected_total_score: i32,
    expected_final_gold: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ShopLever {
    config_ref: String,
    steps: Vec<ShopDemoStep>,
    expected_reroll_count: u32,
    expected_removal_lever_used: bool,
    expected_removed_card_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ShopDemoStep {
    Reroll,
    RemoveCardId { card_id: String },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DemoGovernance {
    anchors_remain_open: Vec<u32>,
    claim_boundary: String,
    generated_state_policy: String,
}

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn demo_path(relative: &str) -> PathBuf {
    workspace_path("examples/run-shop-v1/demo").join(relative)
}

fn read_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> T {
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

fn config_from_demo_ref(config_ref: &str) -> CardRogueliteConfig {
    read_json(demo_path(config_ref))
}

#[test]
fn run_shop_demo_manifest_documents_deterministic_boundaries() {
    let manifest: DemoManifest = read_json(demo_path("run-shop-demo.manifest.json"));
    let doc = std::fs::read_to_string(workspace_path("docs/run-shop-v1-demo.md"))
        .expect("demo doc exists");

    assert_eq!(manifest.schema_version, "ouroforge.run-shop-demo.v1");
    assert_eq!(manifest.demo_id, "run-shop-v1-demo");
    assert_eq!(manifest.issue, 1808);
    assert!(workspace_path(&manifest.builds_against.deckbuilder_tree_ref).exists());
    assert!(workspace_path(&manifest.builds_against.substrate_ref).exists());
    assert_eq!(manifest.execution.mode, "rust-local-fixture-smoke");
    assert_eq!(manifest.execution.network, "disabled");
    assert_eq!(manifest.execution.live_browser, "not-required");
    assert_eq!(manifest.execution.trusted_writes, "none");
    assert_eq!(manifest.execution.fun_verdict, "human-era-j-only");
    assert!(manifest.execution.browser_studio_mode.contains("read-only"));
    assert_eq!(manifest.governance.anchors_remain_open, vec![1, 23]);
    assert!(manifest
        .governance
        .claim_boundary
        .contains("mechanical deterministic"));
    assert!(manifest
        .governance
        .generated_state_policy
        .contains("fixture-scoped"));

    for required in [
        "no network",
        "no live browser",
        "read-only inspection",
        "does not assert subjective fun",
        "#1 and #23 remain governance anchors",
    ] {
        assert!(doc.contains(required), "doc must mention {required}");
    }
}

#[test]
fn run_shop_demo_escalates_to_documented_terminal_run() {
    let manifest: DemoManifest = read_json(demo_path("run-shop-demo.manifest.json"));
    let config = config_from_demo_ref(&manifest.escalating_run.config_ref);
    let report = resolve_card_roguelite_run_ante(&config).expect("run resolves");

    assert_eq!(manifest.escalating_run.expected_status, "won");
    assert_eq!(report.terminal_status, CardRogueliteStatus::Won);
    assert_eq!(
        report
            .rounds
            .iter()
            .map(|round| round.quota)
            .collect::<Vec<_>>(),
        manifest.escalating_run.expected_quota_curve
    );
    assert_eq!(
        report.total_score,
        manifest.escalating_run.expected_total_score
    );
    assert_eq!(
        report.final_gold,
        manifest.escalating_run.expected_final_gold
    );
    assert!(report.rounds.iter().all(|round| round.passed));
    assert!(report.bounded);
}

#[test]
fn run_shop_demo_reroll_and_removal_turn_bad_luck_into_a_choice() {
    let manifest: DemoManifest = read_json(demo_path("run-shop-demo.manifest.json"));
    let config = config_from_demo_ref(&manifest.shop_lever.config_ref);
    let state = resolve_card_roguelite_state(&config).expect("state resolves");
    let mut commands = Vec::new();

    for step in &manifest.shop_lever.steps {
        match step {
            ShopDemoStep::Reroll => commands.push(CardRogueliteShopCommand::Reroll),
            ShopDemoStep::RemoveCardId { card_id } => {
                let deck_index = state
                    .deck
                    .iter()
                    .position(|candidate| candidate == card_id)
                    .expect("documented removable card appears in seeded deck");
                commands.push(CardRogueliteShopCommand::Remove { deck_index });
            }
        }
    }

    let report = resolve_card_roguelite_shop_economy(&config, &commands).expect("shop resolves");

    assert_eq!(
        report.reroll_count,
        manifest.shop_lever.expected_reroll_count
    );
    assert_eq!(
        report.removal_lever_used,
        manifest.shop_lever.expected_removal_lever_used
    );
    assert_ne!(
        report.starting_offers, report.final_offers,
        "reroll changes the draft"
    );
    assert_eq!(
        report
            .transactions
            .last()
            .and_then(|tx| tx.removed_card_id.clone()),
        Some(manifest.shop_lever.expected_removed_card_id.clone())
    );
    assert!(!report
        .final_deck
        .iter()
        .any(|card| card == &manifest.shop_lever.expected_removed_card_id));
    assert!(report
        .read_only_inspection
        .disallowed_actions
        .contains(&"trusted writes".to_string()));
}
