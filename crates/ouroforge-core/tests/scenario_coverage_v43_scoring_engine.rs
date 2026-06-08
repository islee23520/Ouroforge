//! Scenario Coverage v43 — Scoring-Engine Regression Suite (#1803).
//!
//! Locks Multiplicative Scoring-Engine v1 behavior with state/shape regressions
//! only: modifier model, deterministic resolution order, composition replay,
//! fixture-scoped demo shape, and substrate determinism backward compatibility.
//! No timing, network, live browser, trusted write, auto-merge, or automated
//! fun/quality assertions are introduced.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use ouroforge_core::{
    analyze_card_roguelite_score_composition, resolve_card_roguelite_score_resolution,
    resolve_card_roguelite_state, validate_card_roguelite_config, CardRogueliteConfig,
    CardRogueliteModifierEffectOperation, CardRogueliteModifierEffectScope,
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
    serde_json::from_value(read_json(relative)).expect("scoring config parses")
}

fn assert_repo_ref(relative: &str) {
    assert!(
        repo_root().join(relative).is_file(),
        "stale fixture ref: {relative}"
    );
}

const MATRIX: &str = "examples/scoring-engine-v1/scenario-coverage-v43/matrix.fixture.json";
const DOC: &str = "docs/scenario-coverage-v43.md";

#[test]
fn v43_matrix_enumerates_scoring_engine_regressions() {
    let matrix = read_json(MATRIX);
    assert_eq!(matrix["schemaVersion"], "scenario-coverage-v43");
    assert_eq!(matrix["issue"], 1803);
    assert_eq!(matrix["fixtureScoped"], true);

    let boundary = matrix["boundary"]
        .as_str()
        .expect("boundary")
        .to_ascii_lowercase();
    for required in [
        "rust/local owns scoring-engine regression coverage",
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
        scenarios.len() >= 8,
        "v43 enumerates all regression surfaces"
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
    for system in [
        "modifier-model",
        "resolution",
        "composition",
        "demo",
        "backcompat",
        "governance",
    ] {
        assert!(systems.contains(system), "missing coverage for {system}");
    }
    assert!(ids.contains("v43-backcompat-substrate-determinism-golden"));
}

#[test]
fn v43_modifier_model_regressions_are_locked() {
    let config = read_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/modifier-effects.valid.json",
    );
    validate_card_roguelite_config(&config).expect("modifier fixture validates");
    let tuned = config.modifiers.get("tuned").expect("tuned modifier");
    let effect = tuned.effect.as_ref().expect("readable effect");
    assert_eq!(effect.text, "add +3 before multipliers");
    assert_eq!(effect.scope, CardRogueliteModifierEffectScope::Card);
    assert_eq!(
        effect.operation,
        CardRogueliteModifierEffectOperation::Additive
    );

    let first = resolve_card_roguelite_state(&config).expect("state resolves");
    let second = resolve_card_roguelite_state(&config).expect("state replays");
    assert_eq!(first.score, 16);
    assert_eq!(first.digest, second.digest);

    let malformed = read_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/modifier-effects.malformed.json",
    );
    let error = validate_card_roguelite_config(&malformed)
        .expect_err("malformed effect fails closed")
        .to_string();
    assert!(error.contains("one readable ASCII line"));
    assert!(resolve_card_roguelite_state(&malformed).is_err());
}

#[test]
fn v43_resolution_order_and_determinism_are_locked() {
    let parity = read_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/resolution.parity.json",
    );
    validate_card_roguelite_config(&parity).expect("parity fixture validates");
    let first = resolve_card_roguelite_score_resolution(&parity).expect("resolution resolves");
    let second = resolve_card_roguelite_score_resolution(&parity).expect("resolution replays");
    let state = resolve_card_roguelite_state(&parity).expect("state resolves");
    assert_eq!(first, second);
    assert_eq!(first.total_score, 16);
    assert_eq!(first.total_score, state.score);
    assert_eq!(first.digest.value.len(), 16);

    let ordered =
        read_config("examples/card-roguelite-substrate-v1/scoring-engine-v1/resolution.order.json");
    let resolution = resolve_card_roguelite_score_resolution(&ordered).expect("ordered resolves");
    let trace = resolution.card_scores.first().expect("card trace");
    let step_ids = trace
        .steps
        .iter()
        .map(|step| step.modifier_id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(step_ids, ["a-add", "z-double"]);
    assert_eq!(trace.final_score, 30);
    assert_eq!(resolution.total_score, 30);
}

#[test]
fn v43_composition_and_demo_replay_are_locked() {
    let config = read_config(
        "examples/card-roguelite-substrate-v1/scoring-engine-v1/composition.degenerate.json",
    );
    let first = analyze_card_roguelite_score_composition(&config).expect("composition resolves");
    let second = analyze_card_roguelite_score_composition(&config).expect("composition replays");
    let finding = first.findings.first().expect("finding");
    assert_eq!(first, second);
    assert_eq!(first.total_score, 90);
    assert_eq!(finding.modifier_ids, ["tuned", "overdrive", "reactor-loop"]);
    assert_eq!(finding.base_score, 10);
    assert_eq!(finding.final_score, 90);
    assert_eq!(finding.power_delta, 80);
    assert_eq!(finding.multiplicative_count, 2);
    assert!(finding.degenerate);

    let demo = read_json("examples/scoring-engine-v1/demo/demo.manifest.json");
    assert_eq!(demo["schemaVersion"], "ouroforge.scoring-engine-demo.v1");
    assert_eq!(demo["issue"], "1802");
    for config_ref in [
        demo["readableComposedScore"]["config"]
            .as_str()
            .expect("readable config"),
        demo["degenerateCombo"]["config"]
            .as_str()
            .expect("degenerate config"),
    ] {
        assert_repo_ref(config_ref);
    }
    assert_eq!(demo["degenerateCombo"]["expectedTotalScore"], 90);
    assert_eq!(demo["degenerateCombo"]["expectedDegenerate"], true);
}

#[test]
fn v43_substrate_backcompat_golden_remains_valid() {
    let golden = read_json(
        "examples/scoring-engine-v1/scenario-coverage-v43/substrate-determinism.golden.json",
    );
    assert_eq!(
        golden["schemaVersion"],
        "scoring-engine-v43-substrate-backcompat-golden.v1"
    );
    assert_eq!(golden["fixtureScoped"], true);
    let config_ref = golden["configRef"].as_str().expect("config ref");
    assert_repo_ref(config_ref);

    let config = read_config(config_ref);
    let first = resolve_card_roguelite_state(&config).expect("substrate resolves");
    let second = resolve_card_roguelite_state(&config).expect("substrate replays");
    assert_eq!(first, second);
    assert_eq!(
        first.config_id,
        golden["expectedConfigId"].as_str().unwrap()
    );
    assert_eq!(first.variant, golden["expectedVariant"].as_str().unwrap());
    assert_eq!(first.seed, golden["expectedSeed"].as_u64().unwrap() as u32);
    assert_eq!(
        first.score,
        golden["expectedScore"].as_i64().unwrap() as i32
    );
    assert_eq!(
        first.shop_offers.len(),
        golden["expectedShopOfferCount"].as_u64().unwrap() as usize
    );
    assert_eq!(
        first.digest.value,
        golden["expectedDigest"].as_str().unwrap()
    );
    assert_eq!(first.digest.value.len(), 16);
    let prefix = golden["expectedDeckPrefix"]
        .as_array()
        .expect("deck prefix")
        .iter()
        .map(|entry| entry.as_str().expect("deck card"))
        .collect::<Vec<_>>();
    assert_eq!(
        first
            .deck
            .iter()
            .take(prefix.len())
            .map(String::as_str)
            .collect::<Vec<_>>(),
        prefix
    );
}

#[test]
fn v43_doc_preserves_state_shape_wording_and_governance() {
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
        "production-ready claim",
        "godot replacement/parity claim",
        "quality verdict",
        "automated fun score",
        "generated runs",
        "remain open governance anchors",
        "cargo test -p ouroforge-core --test scenario_coverage_v43_scoring_engine --jobs 2",
    ] {
        assert!(lowered.contains(required), "doc missing {required}");
    }
}
