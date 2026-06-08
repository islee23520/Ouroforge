use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_root().join(relative)).expect(relative)
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read(relative)).expect(relative)
}

#[test]
fn fixture_declares_shop_and_run_map_ui_over_existing_deckbuilder_scene() {
    let scene = read_json("examples/game-runtime/deckbuilder-ui-scene-v1.json");
    assert_eq!(
        scene["deckbuilderUi"]["schemaVersion"],
        "ouroforge.deckbuilder-ui.v1"
    );

    let shop = &scene["deckbuilderUi"]["shop"];
    assert_eq!(shop["id"], "act1-shop");
    assert_eq!(shop["currency"], "gold");
    assert_eq!(shop["balance"], 45);
    let offers = shop["offers"].as_array().expect("shop offers");
    assert_eq!(offers.len(), 2);
    assert_eq!(offers[0]["id"], "offer-strike-plus");
    assert_eq!(offers[0]["available"], true);
    assert_eq!(offers[1]["id"], "offer-vigor-charm");
    assert_eq!(offers[1]["available"], false);
    assert_eq!(offers[1]["unavailableReason"], "insufficient gold");

    let run_map = &scene["deckbuilderUi"]["runMap"];
    assert_eq!(run_map["id"], "act1-run-map");
    assert_eq!(run_map["currentNodeId"], "start");
    let nodes = run_map["nodes"].as_array().expect("run-map nodes");
    let node_ids = nodes
        .iter()
        .map(|node| node["id"].as_str().expect("node id"))
        .collect::<Vec<_>>();
    assert_eq!(node_ids, ["start", "shop", "elite", "boss"]);
    let edges = run_map["edges"].as_array().expect("run-map edges");
    assert!(edges
        .iter()
        .any(|edge| edge["from"] == "start" && edge["to"] == "shop"));
    assert!(edges
        .iter()
        .any(|edge| edge["from"] == "start" && edge["to"] == "elite" && edge["blocked"] == true));
}

#[test]
fn runtime_sources_expose_shop_run_map_probe_methods_without_trusted_writes() {
    let runtime = read("examples/game-runtime/runtime.js");
    let module = read("examples/game-runtime/deckbuilder-ui.js");
    let combined = format!("{runtime}\n{module}");

    for required in [
        "selectShopOffer",
        "planRunMapNode",
        "deckbuilderUiSelectShopOffer",
        "deckbuilderUiPlanRunMapNode",
        "runtime.deckbuilder_ui.select_shop_offer",
        "runtime.deckbuilder_ui.plan_run_map_node",
        "shop-offer-selection",
        "run-map-path-plan",
        "trustedWrite: false",
        "draftOnly: true",
    ] {
        assert!(
            combined.contains(required),
            "missing shop/run-map surface: {required}"
        );
    }

    let lower = combined.to_ascii_lowercase();
    for required in [
        "read-only",
        "draft-only",
        "existing review/apply/trust-gradient path",
        "browser-runtime-deckbuilder-ui-probe",
    ] {
        assert!(
            lower.contains(required),
            "missing boundary phrase: {required}"
        );
    }
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "production-ready engine",
        "godot replacement enabled",
        "quality/fun guaranteed",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}

#[test]
fn deckbuilder_ui_docs_record_shop_run_map_boundaries_and_governance() {
    let docs = read("docs/deckbuilder-ui-v1.md");
    let lower = docs.to_ascii_lowercase();
    for required in [
        "issue: #1827",
        "shop and run-map ui implementation compatibility",
        "read-only/draft-only shop and run-map",
        "invalid node ids",
        "stale offers",
        "impossible paths",
        "generated runs/artifacts remain untracked unless explicitly fixture-scoped",
        "issues #1 and #23 remain open",
        "no production-ready",
    ] {
        assert!(lower.contains(required), "missing doc boundary: {required}");
    }
}
