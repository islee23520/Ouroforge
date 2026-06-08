//! Card-Roguelite Substrate v1 deterministic demo smoke (#1795).
//!
//! The demo reuses fixture-scoped deck-roguelike parity and engine-builder
//! deckbuilder configs to prove both variants run over one Rust/local substrate
//! without network, a live browser, trusted writes, or subjective fun claims.

use std::path::{Path, PathBuf};

use ouroforge_core::{
    card_roguelite_probe_state, resolve_card_roguelite_state, validate_card_roguelite_config,
    CardRogueliteConfig,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DemoManifest {
    schema_version: String,
    demo_id: String,
    substrate: String,
    execution: DemoExecution,
    runs: Vec<DemoRun>,
    governance: DemoGovernance,
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
struct DemoRun {
    id: String,
    config_ref: String,
    variant: String,
    seed: u32,
    parity_ref: Option<String>,
    distinct_from: Option<String>,
    #[serde(default)]
    expected_first_deck_cards: Vec<String>,
    expected_offer_count: usize,
    expected_score_at_least: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DemoGovernance {
    anchors_remain_open: Vec<u32>,
    issue: u32,
    claim_boundary: String,
}

fn workspace_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative)
}

fn demo_path(relative: &str) -> PathBuf {
    workspace_path("examples/card-roguelite-substrate-v1/demo").join(relative)
}

fn read_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> T {
    let text = std::fs::read_to_string(path).expect("fixture exists");
    serde_json::from_str(&text).expect("fixture parses")
}

fn config_from_demo_ref(config_ref: &str) -> CardRogueliteConfig {
    read_json(demo_path(config_ref))
}

#[test]
fn card_roguelite_substrate_demo_manifest_documents_safe_fixture_boundaries() {
    let manifest: DemoManifest = read_json(demo_path("substrate-demo.manifest.json"));
    let doc = std::fs::read_to_string(workspace_path("docs/card-roguelite-substrate-v1-demo.md"))
        .expect("demo doc exists");

    assert_eq!(
        manifest.schema_version,
        "ouroforge.card-roguelite-substrate-demo.v1"
    );
    assert_eq!(manifest.demo_id, "card-roguelite-substrate-v1-demo");
    assert_eq!(manifest.substrate, "card-roguelite-substrate-v1");
    assert_eq!(manifest.execution.mode, "rust-local-fixture-smoke");
    assert_eq!(manifest.execution.network, "disabled");
    assert_eq!(manifest.execution.live_browser, "not-required");
    assert_eq!(manifest.execution.trusted_writes, "none");
    assert_eq!(manifest.execution.fun_verdict, "human-era-j-only");
    assert!(manifest.execution.browser_studio_mode.contains("read-only"));
    assert_eq!(manifest.governance.anchors_remain_open, vec![1, 23]);
    assert_eq!(manifest.governance.issue, 1795);
    assert!(manifest
        .governance
        .claim_boundary
        .contains("mechanical deterministic"));

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
fn card_roguelite_substrate_demo_runs_both_configs_deterministically() {
    let manifest: DemoManifest = read_json(demo_path("substrate-demo.manifest.json"));
    let mut resolved = Vec::new();

    for run in &manifest.runs {
        let config = config_from_demo_ref(&run.config_ref);
        validate_card_roguelite_config(&config).expect("demo config validates");
        assert_eq!(config.variant, run.variant);
        assert_eq!(config.seed, run.seed);

        let first = resolve_card_roguelite_state(&config).expect("demo config resolves");
        let second =
            resolve_card_roguelite_state(&config).expect("same demo config resolves again");
        let probe = card_roguelite_probe_state(&config).expect("probe resolves");

        assert_eq!(first, second, "{} must be deterministic", run.id);
        assert_eq!(
            probe.substrate_state, first,
            "{} is probe-observable",
            run.id
        );
        assert_eq!(first.shop_offers.len(), run.expected_offer_count);
        assert!(
            first.score >= run.expected_score_at_least,
            "{} score {} must meet mechanical smoke floor {}",
            run.id,
            first.score,
            run.expected_score_at_least
        );
        assert!(first
            .read_only_inspection
            .disallowed_actions
            .iter()
            .any(|action| action == "trusted writes"));
        assert!(first
            .read_only_inspection
            .disallowed_actions
            .iter()
            .any(|action| action == "automated fun verdict"));

        if !run.expected_first_deck_cards.is_empty() {
            assert_eq!(
                &first.deck[..run.expected_first_deck_cards.len()],
                run.expected_first_deck_cards.as_slice()
            );
        }

        resolved.push((run.id.clone(), config, first.digest.value));
    }

    let deck = resolved
        .iter()
        .find(|(id, _, _)| id == "deck-roguelike-parity")
        .expect("deck run exists");
    let engine = resolved
        .iter()
        .find(|(id, _, _)| id == "engine-builder-deckbuilder")
        .expect("engine run exists");
    assert_ne!(deck.1.variant, engine.1.variant);
    assert_ne!(deck.1.cards, engine.1.cards);
    assert_ne!(
        deck.2, engine.2,
        "distinct configs produce distinct digests"
    );
}

#[test]
fn card_roguelite_substrate_demo_preserves_deck_roguelike_golden_parity() {
    let manifest: DemoManifest = read_json(demo_path("substrate-demo.manifest.json"));
    let deck_run = manifest
        .runs
        .iter()
        .find(|run| run.id == "deck-roguelike-parity")
        .expect("deck parity run exists");
    let parity_ref = deck_run.parity_ref.as_ref().expect("parity ref declared");

    let demo_config = config_from_demo_ref(&deck_run.config_ref);
    let golden_config = config_from_demo_ref(parity_ref);
    assert_eq!(
        demo_config, golden_config,
        "demo keeps golden config parity"
    );

    let engine_run = manifest
        .runs
        .iter()
        .find(|run| run.id == "engine-builder-deckbuilder")
        .expect("engine run exists");
    assert_eq!(
        engine_run.distinct_from.as_deref(),
        Some("deck-roguelike-parity"),
        "engine-builder run must declare distinctness from the parity run"
    );
}
