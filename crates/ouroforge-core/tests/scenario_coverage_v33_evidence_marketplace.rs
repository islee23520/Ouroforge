//! Scenario Coverage v33 — Evidence-Native Marketplace Regression Suite (#1616).
//!
//! Locks Evidence-Native Marketplace v1 behavior: local registry publish/consume
//! validation, verify-on-consume, tamper detection, provenance lineage, and the
//! backward-compatibility guarantee that the Milestone 25 provenance bundle
//! remains valid standalone. State/shape assertions only — no flaky or
//! timing-based checks.

use anyhow::{anyhow, bail, Result};
use ouroforge_core::evidence_marketplace_registry::{LocalAssetRegistry, MarketplaceAsset};
use ouroforge_core::export_hash::sha256_prefixed;
use ouroforge_core::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("core crate has workspace root")
        .to_path_buf()
}

fn fixture_root() -> PathBuf {
    repo_root().join("examples/evidence-marketplace-v1/scenario-coverage-v33")
}

fn read_text(relative: &str) -> String {
    let path = fixture_root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {path:?}: {error}"))
}

fn read_json(relative: &str) -> Value {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("parse {relative}: {error}"))
}

/// Raw deserialize without the validation gate, so negative cases exercise the
/// publish-side fail-closed checks rather than failing at parse time.
fn raw_asset(relative: &str) -> MarketplaceAsset {
    serde_json::from_str(&read_text(relative))
        .unwrap_or_else(|error| panic!("deserialize {relative}: {error}"))
}

fn verify_on_consume(registry: &LocalAssetRegistry, asset_id: &str, root: &Path) -> Result<()> {
    let asset = registry.consume(asset_id)?;
    let template_bytes = fs::read(root.join(&asset.template_ref))?;
    let actual_digest = sha256_prefixed(&template_bytes);
    match asset.template_digest.as_deref() {
        Some(recorded) if recorded == actual_digest => {}
        Some(_) => bail!("tampered asset: template digest mismatch"),
        None => bail!("asset has no template digest to verify against"),
    }
    let bundle = asset
        .provenance
        .as_ref()
        .ok_or_else(|| anyhow!("asset has no provenance lineage"))?;
    let evaluation = bundle.evaluate_with_root(root);
    if evaluation.computed_status != ProvenanceBundleStatus::Complete
        || !evaluation.status_consistent
    {
        bail!(
            "provenance evidence does not verify: {:?}",
            evaluation.issues
        );
    }
    Ok(())
}

#[test]
fn v33_registry_validation_cases_are_enumerated() {
    let matrix = read_json("matrix.fixture.json");
    assert_eq!(
        matrix["schemaVersion"],
        "scenario-coverage-v33-evidence-marketplace-matrix-v1"
    );
    assert_eq!(matrix["issue"], 1616);

    for case in matrix["registryCases"].as_array().expect("registry cases") {
        let fixture = case["fixture"].as_str().expect("fixture path");
        let outcome = case["expectedOutcome"].as_str().expect("expected outcome");
        let mut registry = LocalAssetRegistry::new();
        match outcome {
            "published" => {
                let receipt = registry
                    .publish(raw_asset(fixture))
                    .unwrap_or_else(|error| panic!("{fixture} should publish: {error}"));
                if case["expectsReplayProof"].as_bool().unwrap_or(false) {
                    assert!(receipt.has_replay_proof, "{fixture}");
                }
                if let Some(expected) = case["expectedProvenanceStatus"].as_str() {
                    let want = match expected {
                        "complete" => ProvenanceBundleStatus::Complete,
                        other => panic!("unexpected provenance status {other}"),
                    };
                    assert_eq!(receipt.provenance_status, want, "{fixture}");
                }
                assert_eq!(registry.len(), 1, "{fixture}");
            }
            "rejected" => {
                let error = registry
                    .publish(raw_asset(fixture))
                    .expect_err(&format!("{fixture} must be rejected"));
                let reason = case["rejectReason"].as_str().expect("reject reason");
                assert!(
                    error.to_string().contains(reason),
                    "{fixture}: error `{error}` should contain `{reason}`"
                );
                assert!(registry.is_empty(), "{fixture}: rejected asset not stored");
            }
            other => panic!("unexpected registry outcome {other}"),
        }
    }
}

#[test]
fn v33_verify_on_consume_and_tamper_detection() {
    let matrix = read_json("matrix.fixture.json");
    for case in matrix["verifyCases"].as_array().expect("verify cases") {
        let fixture = case["fixture"].as_str().expect("fixture path");
        let expected = case["expectedVerify"].as_str().expect("expected verify");
        let asset = raw_asset(fixture);
        let asset_id = asset.asset_id.clone();
        let mut registry = LocalAssetRegistry::new();
        registry
            .publish(asset)
            .unwrap_or_else(|error| panic!("{fixture} should record: {error}"));

        let result = verify_on_consume(&registry, &asset_id, &fixture_root());
        match expected {
            "verified" => result.unwrap_or_else(|error| panic!("{fixture} should verify: {error}")),
            "rejected" => {
                let error = result.expect_err(&format!("{fixture} must be rejected"));
                let reason = case["rejectReason"].as_str().expect("reject reason");
                assert!(
                    error.to_string().contains(reason),
                    "{fixture}: error `{error}` should contain `{reason}`"
                );
            }
            other => panic!("unexpected verify outcome {other}"),
        }
    }
}

#[test]
fn v33_provenance_lineage_resolves_complete() {
    let matrix = read_json("matrix.fixture.json");
    for case in matrix["provenanceLineageCases"]
        .as_array()
        .expect("lineage cases")
    {
        let fixture = case["fixture"].as_str().expect("fixture path");
        let asset = MarketplaceAsset::from_json_str(&read_text(fixture))
            .unwrap_or_else(|error| panic!("{fixture} should validate: {error}"));
        let bundle = asset.provenance.expect("asset carries provenance");
        let evaluation = bundle.evaluate_with_root(&fixture_root());
        assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Complete);
        assert!(evaluation.status_consistent, "{evaluation:#?}");
        assert!(evaluation.issues.is_empty(), "{:?}", evaluation.issues);
    }
}

#[test]
fn v33_milestone25_provenance_bundle_remains_valid_standalone() {
    let matrix = read_json("matrix.fixture.json");
    let compat = &matrix["backwardCompatibility"];
    let fixture = compat["fixture"].as_str().expect("compat fixture");

    // Parses and validates purely as a Milestone 25 provenance bundle, with no
    // marketplace wrapper — the registry is additive over the existing bundle.
    let bundle = ProvenanceBundleArtifact::from_json_str(&read_text(fixture))
        .expect("standalone provenance bundle remains valid");
    let evaluation = bundle.evaluate_with_root(&fixture_root());
    assert_eq!(evaluation.computed_status, ProvenanceBundleStatus::Complete);
    assert!(evaluation.status_consistent, "{evaluation:#?}");
    assert!(evaluation.issues.is_empty(), "{:?}", evaluation.issues);

    // The marketplace asset schema must not leak into the standalone bundle.
    let raw = read_json(fixture);
    assert_eq!(raw["schemaVersion"], "provenance-bundle-v1");
    assert!(raw.get("templateRef").is_none());
    assert!(raw.get("replayProof").is_none());
}

#[test]
fn v33_docs_and_fixtures_preserve_generated_state_wording_and_governance() {
    let docs = fs::read_to_string(repo_root().join("docs/scenario-coverage-v33.md")).expect("docs");
    let mut all = docs.clone();
    for relative in [
        "matrix.fixture.json",
        "assets/valid.json",
        "assets/proof-less.json",
        "assets/provenance-gap.json",
        "assets/tampered.json",
        "compatibility/provenance-bundle-standalone.golden.json",
    ] {
        all.push('\n');
        all.push_str(&read_text(relative));
    }

    assert!(all.contains("#1612"));
    assert!(all.contains("#1613"));
    assert!(all.contains("#1508"));
    assert!(all.contains("Layer-3"));
    assert!(all.contains("Generated state remains untracked unless explicitly fixture-scoped"));
    assert!(all.contains("backward-compatible"));
    assert!(all.contains("#1 remains open"));
    assert!(all.contains("#23 remains open"));

    for asset_fixture in [
        "assets/valid.json",
        "assets/proof-less.json",
        "assets/provenance-gap.json",
        "assets/tampered.json",
    ] {
        let asset = read_json(asset_fixture);
        assert_eq!(asset["generatedState"]["fixtureScoped"], true);
        assert_eq!(asset["generatedState"]["tracked"], true);
    }

    let lower = all.to_ascii_lowercase();
    for forbidden in [
        "closes #1",
        "closed #1",
        "closes #23",
        "closed #23",
        "auto-merge enabled",
        "auto-approval enabled",
        "auto-promote enabled",
        "production-ready",
        "production grade",
        "quality guarantee",
        "godot replacement",
        "godot parity",
    ] {
        assert!(!lower.contains(forbidden), "forbidden wording: {forbidden}");
    }
}
