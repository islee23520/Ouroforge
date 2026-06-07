//! Contract tests for the Local Verifiable-Asset Registry v1 (#1613).
//!
//! Validates that a template can be published and consumed locally, and that
//! proof-less or provenance-gapped assets are rejected fail-closed.

use ouroforge_core::evidence_marketplace_registry::{
    LocalAssetRegistry, MarketplaceAsset, EVIDENCE_MARKETPLACE_REGISTRY_SCHEMA_VERSION,
};
use ouroforge_core::provenance_bundle::ProvenanceBundleStatus;

fn fixture_text(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/evidence-marketplace-registry-v1613")
        .join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("fixture {} should exist: {error}", path.display()))
}

/// Raw deserialize without the validation gate, so negative cases can exercise
/// the publish-side fail-closed checks rather than failing at parse time.
fn raw_asset(name: &str) -> MarketplaceAsset {
    serde_json::from_str(&fixture_text(name)).expect("fixture is well-formed JSON")
}

#[test]
fn valid_asset_publishes_and_consumes_locally() {
    let asset = MarketplaceAsset::from_json_str(&fixture_text("valid.fixture.json"))
        .expect("valid asset passes validation");
    assert_eq!(
        asset.schema_version,
        EVIDENCE_MARKETPLACE_REGISTRY_SCHEMA_VERSION
    );

    let mut registry = LocalAssetRegistry::new();
    assert!(registry.is_empty());

    let receipt = registry.publish(asset).expect("valid asset publishes");
    assert_eq!(receipt.asset_id, "collect-and-exit-template-v1");
    assert!(receipt.has_replay_proof);
    assert_eq!(receipt.provenance_status, ProvenanceBundleStatus::Complete);
    assert!(receipt
        .forbidden_actions
        .contains(&"host_or_sell_remotely".to_string()));
    assert!(receipt
        .allowed_actions
        .contains(&"replay_proof_locally".to_string()));

    assert_eq!(registry.len(), 1);
    assert_eq!(
        registry.published_ids(),
        vec!["collect-and-exit-template-v1".to_string()]
    );

    let consumed = registry
        .consume("collect-and-exit-template-v1")
        .expect("published asset can be consumed and re-verified");
    assert_eq!(consumed.asset_id, "collect-and-exit-template-v1");
    assert_eq!(
        consumed.acceptance_suite_ref,
        "templates/collect-and-exit/acceptance-suite.json"
    );
}

#[test]
fn proof_less_asset_is_rejected_on_publish() {
    let asset = raw_asset("proof-less.fixture.json");
    assert!(
        asset.replay_proof.is_none(),
        "fixture omits the replay proof"
    );

    let mut registry = LocalAssetRegistry::new();
    let error = registry
        .publish(asset)
        .expect_err("proof-less asset must be rejected");
    assert!(
        error.to_string().contains("replay proof"),
        "error should explain the missing replay proof: {error}"
    );
    assert!(registry.is_empty(), "rejected asset is not stored");
}

#[test]
fn provenance_gapped_asset_is_rejected_on_publish() {
    let asset = raw_asset("provenance-gap.fixture.json");

    let mut registry = LocalAssetRegistry::new();
    let error = registry
        .publish(asset)
        .expect_err("provenance-gapped asset must be rejected");
    assert!(
        error.to_string().contains("provenance gap"),
        "error should explain the provenance gap: {error}"
    );
    assert!(registry.is_empty(), "rejected asset is not stored");
}

#[test]
fn from_json_str_rejects_proof_less_and_provenance_gap() {
    let proof_less = MarketplaceAsset::from_json_str(&fixture_text("proof-less.fixture.json"));
    assert!(proof_less.is_err(), "proof-less asset fails validation");

    let gapped = MarketplaceAsset::from_json_str(&fixture_text("provenance-gap.fixture.json"));
    assert!(gapped.is_err(), "provenance-gapped asset fails validation");
}

#[test]
fn consuming_unpublished_asset_fails_closed() {
    let registry = LocalAssetRegistry::new();
    let error = registry
        .consume("never-published")
        .expect_err("consuming an unpublished asset fails closed");
    assert!(error.to_string().contains("not published"));
}

#[test]
fn duplicate_publish_is_rejected() {
    let first =
        MarketplaceAsset::from_json_str(&fixture_text("valid.fixture.json")).expect("valid asset");
    let second =
        MarketplaceAsset::from_json_str(&fixture_text("valid.fixture.json")).expect("valid asset");

    let mut registry = LocalAssetRegistry::new();
    registry.publish(first).expect("first publish succeeds");
    let error = registry
        .publish(second)
        .expect_err("duplicate id is rejected");
    assert!(error.to_string().contains("already published"));
    assert_eq!(registry.len(), 1);
}
