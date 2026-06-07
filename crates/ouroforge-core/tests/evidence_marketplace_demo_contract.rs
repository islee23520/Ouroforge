//! Demo smoke test for Evidence-Native Marketplace Demo v1 (#1615).
//!
//! Deterministic, fixture-scoped, no network or live browser. Demonstrates
//! publish -> consume -> verify (the asset's proof re-runs locally to confirm
//! it works) and that a tampered asset is rejected on local verification.

use anyhow::{anyhow, bail, Result};
use ouroforge_core::evidence_marketplace_registry::{LocalAssetRegistry, MarketplaceAsset};
use ouroforge_core::export_hash::sha256_prefixed;
use ouroforge_core::provenance_bundle::ProvenanceBundleStatus;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn demo_root() -> PathBuf {
    repo_root().join("examples/evidence-marketplace-v1/demo")
}

fn read_asset(name: &str) -> String {
    let path = demo_root().join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

/// Verify-on-consume: re-run the asset's proof against local evidence. The
/// recorded template digest must match the on-disk template (tamper check) and
/// the bound Milestone 25 provenance bundle must resolve to a complete chain.
fn verify_on_consume(registry: &LocalAssetRegistry, asset_id: &str, root: &Path) -> Result<()> {
    let asset = registry.consume(asset_id)?;

    let template_bytes = fs::read(root.join(&asset.template_ref))?;
    let actual_digest = sha256_prefixed(&template_bytes);
    match asset.template_digest.as_deref() {
        Some(recorded) if recorded == actual_digest => {}
        Some(recorded) => bail!(
            "tampered asset: template digest mismatch (recorded {recorded}, actual {actual_digest})"
        ),
        None => bail!("asset has no template digest to verify against"),
    }

    let bundle = asset
        .provenance
        .as_ref()
        .ok_or_else(|| anyhow!("asset has no provenance lineage"))?;
    let evaluation = bundle.evaluate_with_root(root);
    if evaluation.computed_status != ProvenanceBundleStatus::Complete || !evaluation.status_consistent
    {
        bail!(
            "provenance evidence does not verify: status {:?}, issues {:?}",
            evaluation.computed_status,
            evaluation.issues
        );
    }
    Ok(())
}

#[test]
fn demo_publishes_consumes_and_verifies_a_valid_asset() {
    let asset = MarketplaceAsset::from_json_str(&read_asset("asset.valid.fixture.json"))
        .expect("valid asset parses and validates");
    let asset_id = asset.asset_id.clone();

    let mut registry = LocalAssetRegistry::new();
    let receipt = registry.publish(asset).expect("valid asset publishes locally");
    assert!(receipt.has_replay_proof);
    assert_eq!(receipt.provenance_status, ProvenanceBundleStatus::Complete);

    verify_on_consume(&registry, &asset_id, &demo_root())
        .expect("valid asset verifies on consume: its proof re-runs and reproduces");
}

#[test]
fn demo_rejects_a_tampered_asset_on_verification() {
    let tampered = MarketplaceAsset::from_json_str(&read_asset("asset.tampered.fixture.json"))
        .expect("tampered asset is still structurally well-formed");
    let asset_id = tampered.asset_id.clone();

    // A tampered asset can still be recorded (shape is intact); the tamper is
    // caught when its proof re-runs on consume.
    let mut registry = LocalAssetRegistry::new();
    registry
        .publish(tampered)
        .expect("structurally valid asset is recorded");

    let error = verify_on_consume(&registry, &asset_id, &demo_root())
        .expect_err("tampered asset must be rejected on verification");
    assert!(
        error.to_string().contains("tampered asset"),
        "rejection should name the tamper: {error}"
    );
}

#[test]
fn demo_evidence_resolves_and_provenance_is_complete() {
    let asset = MarketplaceAsset::from_json_str(&read_asset("asset.valid.fixture.json"))
        .expect("valid asset");
    let bundle = asset.provenance.expect("valid asset carries provenance");
    let evaluation = bundle.evaluate_with_root(&demo_root());
    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Complete);
    assert!(evaluation.issues.is_empty(), "{:?}", evaluation.issues);
}

#[test]
fn demo_doc_and_fixtures_preserve_boundaries_and_governance_audits() {
    let doc = fs::read_to_string(repo_root().join("docs/evidence-marketplace-v1-demo.md"))
        .expect("read demo doc");
    let valid = read_asset("asset.valid.fixture.json");
    let tampered = read_asset("asset.tampered.fixture.json");
    let all = format!("{doc}\n{valid}\n{tampered}");

    assert!(all.contains("#1613"));
    assert!(all.contains("#1508"));
    assert!(all.contains("Layer-3"));
    assert!(all.contains("read-only"));
    assert!(all.contains("tamper") || all.contains("tampered"));
    assert!(all.contains("backward-compatible"));
    assert!(all.contains("Generated state remains untracked unless explicitly fixture-scoped"));
    assert!(all.contains("#1 remains open"));
    assert!(all.contains("#23 remains open"));

    let lower = all.to_ascii_lowercase();
    for forbidden in [
        "closes #1",
        "closed #1",
        "closes #23",
        "closed #23",
        "auto-merge enabled",
        "auto-approval enabled",
        "production-ready",
        "production grade",
        "quality guarantee",
        "godot replacement",
        "godot parity",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
