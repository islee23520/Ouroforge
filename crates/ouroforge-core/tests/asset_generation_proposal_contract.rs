//! Contract test for Asset Generation Proposal Model v1 (#1635).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! These tests machine-check the asset generation contract: a well-formed,
//! licensed brief produces a validated asset proposal carrying license/
//! provenance; a missing/unrecognized license is rejected fail-closed; a
//! malformed (out-of-bounds) asset is rejected fail-closed; and the provenance
//! links the proposal back to the exact brief that produced it.
//!
//! Boundary checks assert the proposal-only model: the proposal is never
//! promoted (it has not passed the asset-QA gate, #1636), provenance records
//! proposal-only, and generation performs no trusted write.

use std::path::PathBuf;

use ouroforge_core::asset_generation_proposal::{
    generate_asset_proposal, AssetGenerationBrief, ASSET_ARTIFACT_SCHEMA_VERSION,
    ASSET_GENERATION_GENERATOR, ASSET_GENERATION_SOURCE,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_brief(name: &str) -> AssetGenerationBrief {
    let path: PathBuf = repo_root().join("examples/asset-generation").join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    AssetGenerationBrief::from_json_str(&text).expect("fixture brief parses")
}

#[test]
fn well_formed_licensed_brief_produces_a_valid_asset_proposal() {
    let brief = read_brief("asset-generation-brief-v1.json");
    let generated =
        generate_asset_proposal(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    generated.validate().expect("asset proposal validates");
    assert_eq!(generated.proposal.target, brief.asset_kind);
    assert_eq!(generated.proposal.created_at_unix_ms, FIXED_NOW_MS);
    assert_eq!(generated.proposal.path, "assets/sprite/hero-idle.png");

    // The `to` payload is the assembled asset descriptor carrying the license.
    let artifact: Value =
        serde_json::from_str(&generated.proposal.to).expect("proposal carries an asset descriptor");
    assert_eq!(artifact["schemaVersion"], ASSET_ARTIFACT_SCHEMA_VERSION);
    assert_eq!(artifact["id"], brief.asset_id);
    assert_eq!(artifact["kind"], "sprite");
    assert_eq!(artifact["width"], 32);
    assert_eq!(artifact["height"], 32);
    assert_eq!(artifact["license"]["license"], "CC-BY-4.0");

    // License/provenance is attached to the proposal provenance.
    assert_eq!(generated.provenance.license.license, "CC-BY-4.0");
    assert!(generated.provenance.license.attribution.is_some());
    assert!(generated.provenance.license.allowed_source);
}

#[test]
fn proposal_is_proposal_only_and_not_promoted() {
    let brief = read_brief("asset-generation-brief-v1.json");
    let generated =
        generate_asset_proposal(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    // A freshly generated proposal has not passed the asset-QA gate (#1636):
    // it is proposed/pending, not applied/promoted/approved.
    assert_eq!(generated.proposal.status, "proposed");
    assert_eq!(generated.proposal.verdict_status, "pending");
    assert_eq!(generated.proposal.confidence, "unverified");
    assert_eq!(generated.proposal.from, "(no prior asset)");

    // Provenance records the proposal-only boundary and the generator identity.
    assert!(generated.provenance.proposal_only);
    assert_eq!(generated.provenance.generator, ASSET_GENERATION_GENERATOR);
    assert_eq!(generated.provenance.source, ASSET_GENERATION_SOURCE);
}

#[test]
fn provenance_links_proposal_to_its_brief() {
    let brief = read_brief("asset-generation-brief-v1.json");
    let generated =
        generate_asset_proposal(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    assert_eq!(generated.provenance.brief_id, brief.brief_id);
    assert_eq!(
        generated.provenance.brief_digest,
        brief.digest().expect("brief digest")
    );
    assert!(generated.links_to(&brief).expect("linkage check"));

    // A different brief must not link to this proposal.
    let mut other = brief.clone();
    other.brief_id = "asset-brief-other-v1".to_string();
    assert!(!generated.links_to(&other).expect("linkage check"));
}

#[test]
fn generation_is_deterministic() {
    let brief = read_brief("asset-generation-brief-v1.json");
    let first = generate_asset_proposal(&brief, FIXED_NOW_MS).expect("accepted");
    let second = generate_asset_proposal(&brief, FIXED_NOW_MS).expect("accepted");
    assert_eq!(first, second);
}

#[test]
fn missing_license_is_rejected_fail_closed() {
    let brief = read_brief("asset-generation-brief-missing-license.json");
    let error = generate_asset_proposal(&brief, FIXED_NOW_MS)
        .expect_err("a brief with no license must be rejected fail-closed");
    assert!(
        error.to_string().contains("asset license"),
        "unexpected error: {error}"
    );
}

#[test]
fn unrecognized_license_is_rejected() {
    let mut brief = read_brief("asset-generation-brief-v1.json");
    brief.license.license = "All-Rights-Reserved".to_string();
    let error = generate_asset_proposal(&brief, FIXED_NOW_MS)
        .expect_err("an unrecognized license must be rejected fail-closed");
    assert!(
        error.to_string().contains("not a recognized license"),
        "unexpected error: {error}"
    );
}

#[test]
fn missing_required_attribution_is_rejected() {
    let mut brief = read_brief("asset-generation-brief-v1.json");
    // CC-BY-4.0 requires attribution; dropping it must fail closed.
    brief.license.attribution = None;
    let error = generate_asset_proposal(&brief, FIXED_NOW_MS)
        .expect_err("a CC-BY asset without attribution must be rejected");
    assert!(
        error
            .to_string()
            .contains("requires a non-empty attribution"),
        "unexpected error: {error}"
    );
}

#[test]
fn off_list_source_is_rejected() {
    let mut brief = read_brief("asset-generation-brief-v1.json");
    brief.license.allowed_source = false;
    let error = generate_asset_proposal(&brief, FIXED_NOW_MS)
        .expect_err("an off-list source must be rejected fail-closed");
    assert!(
        error.to_string().contains("allowed-sources list"),
        "unexpected error: {error}"
    );
}

#[test]
fn malformed_asset_is_rejected_fail_closed() {
    let brief = read_brief("asset-generation-brief-malformed.json");
    let error = generate_asset_proposal(&brief, FIXED_NOW_MS)
        .expect_err("an out-of-bounds asset must be rejected fail-closed");
    assert!(
        error.to_string().contains("height must be in"),
        "unexpected error: {error}"
    );
}

#[test]
fn unsupported_asset_kind_is_rejected() {
    let mut brief = read_brief("asset-generation-brief-v1.json");
    brief.asset_kind = "shader".to_string();
    let error = generate_asset_proposal(&brief, FIXED_NOW_MS)
        .expect_err("an unsupported asset kind must be rejected");
    assert!(
        error.to_string().contains("unsupported"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_asset_generation_contract() {
    // The asset generation/proposal contract is documented under the Asset
    // Generation and Asset-QA v1 design gate.
    let doc = std::fs::read_to_string(repo_root().join("docs/asset-pipeline-design.md"))
        .expect("asset pipeline design doc exists");
    assert!(
        doc.contains("#1635"),
        "design gate doc records the proposal-model follow-up (#1635)"
    );
    assert!(
        doc.contains("proposal-only") || doc.contains("proposals only"),
        "doc records the proposal-only contract"
    );
    assert!(
        doc.contains("license/provenance"),
        "doc records the license/provenance requirement"
    );
}
