//! Contract test for Audio Generation Proposal Model v1 (#1642).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! These tests machine-check the audio generation contract: a well-formed brief
//! produces a validated audio asset proposal with attached license and generation
//! provenance; a brief that omits the license or declares malformed audio is
//! rejected fail-closed; and the proposal routes through the existing
//! review/apply/trust-gradient path as manual-only (never auto-applied).
//!
//! Boundary checks assert the proposal-only model: the proposal is never promoted
//! (it has not cleared the audio-QA gate, #1643), provenance records
//! proposal-only and carries the license, and generation performs no trusted
//! write.

use std::path::PathBuf;

use ouroforge_core::audio_generation::{
    generate_audio, AudioGenerationBrief, AUDIO_ASSET_SCHEMA_VERSION, AUDIO_GENERATION_GENERATOR,
    AUDIO_GENERATION_SOURCE, AUDIO_GENERATION_TARGET,
};
use ouroforge_core::trust_gradient_risk_tier::{
    classify_mutation_risk_tier, AutoApplyEligibility, RiskTier,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_brief(name: &str) -> AudioGenerationBrief {
    let path: PathBuf = repo_root().join("examples/audio-generation").join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    AudioGenerationBrief::from_json_str(&text).expect("fixture brief parses")
}

#[test]
fn well_formed_brief_produces_a_valid_audio_proposal_with_license() {
    let brief = read_brief("audio-generation-brief-v1.json");
    let generated = generate_audio(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    generated.validate().expect("audio proposal validates");
    assert_eq!(generated.proposal.target, AUDIO_GENERATION_TARGET);
    assert_eq!(generated.proposal.created_at_unix_ms, FIXED_NOW_MS);
    assert_eq!(
        generated.proposal.path,
        "audio/coin-pickup-from-brief-v1.ogg"
    );

    // The `to` payload is the assembled audio asset descriptor, license embedded.
    let asset: Value =
        serde_json::from_str(&generated.proposal.to).expect("proposal carries an audio asset");
    assert_eq!(asset["schemaVersion"], AUDIO_ASSET_SCHEMA_VERSION);
    assert_eq!(asset["audioId"], brief.audio_id);
    assert_eq!(asset["kind"], "sfx");
    assert_eq!(asset["format"], "ogg");
    assert_eq!(asset["durationMs"], 450);
    assert_eq!(asset["channels"], 1);
    assert_eq!(asset["license"]["licenseId"], "CC0-1.0");

    // The provenance carries the mandatory license/credit.
    assert_eq!(generated.provenance.license.license_id, "CC0-1.0");
    assert_eq!(
        generated.provenance.license.holder,
        "Ouroforge local fixtures"
    );
}

#[test]
fn proposal_is_proposal_only_and_not_promoted() {
    let brief = read_brief("audio-generation-brief-v1.json");
    let generated = generate_audio(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    // A freshly generated proposal has not cleared the audio-QA gate (#1643):
    // it is proposed/pending, not applied/promoted/approved.
    assert_eq!(generated.proposal.status, "proposed");
    assert_eq!(generated.proposal.verdict_status, "pending");
    assert_eq!(generated.proposal.confidence, "unverified");

    // Provenance records the proposal-only boundary and the generator identity.
    assert!(generated.provenance.proposal_only);
    assert_eq!(generated.provenance.generator, AUDIO_GENERATION_GENERATOR);
    assert_eq!(generated.provenance.source, AUDIO_GENERATION_SOURCE);
    assert_eq!(generated.provenance.audio_kind, "sfx");
}

#[test]
fn proposal_routes_through_trust_gradient_as_manual_only() {
    let brief = read_brief("audio-generation-brief-v1.json");
    let generated = generate_audio(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    // Routing the proposal through the existing trust-gradient classifier must
    // never make a freshly generated, unverified audio asset auto-apply eligible.
    let descriptor = generated.to_risk_tier_descriptor();
    let classification =
        classify_mutation_risk_tier(&descriptor).expect("descriptor classifies cleanly");
    assert_eq!(classification.proposal_ref, generated.proposal.id);
    assert_eq!(
        classification.eligibility,
        AutoApplyEligibility::ManualOnly,
        "generated audio must route to manual-only review"
    );
    assert_ne!(classification.tier, RiskTier::Low);
}

#[test]
fn provenance_links_proposal_to_its_brief() {
    let brief = read_brief("audio-generation-brief-v1.json");
    let generated = generate_audio(&brief, FIXED_NOW_MS).expect("well-formed brief is accepted");

    assert_eq!(generated.provenance.brief_id, brief.brief_id);
    assert_eq!(
        generated.provenance.brief_digest,
        brief.digest().expect("brief digest")
    );
    assert!(generated.links_to(&brief).expect("linkage check"));

    // A different brief must not link to this proposal.
    let mut other = brief.clone();
    other.brief_id = "brief-other-v1".to_string();
    assert!(!generated.links_to(&other).expect("linkage check"));
}

#[test]
fn generation_is_deterministic() {
    let brief = read_brief("audio-generation-brief-v1.json");
    let first = generate_audio(&brief, FIXED_NOW_MS).expect("accepted");
    let second = generate_audio(&brief, FIXED_NOW_MS).expect("accepted");
    assert_eq!(first, second);
}

#[test]
fn missing_license_is_rejected_fail_closed() {
    let brief = read_brief("audio-generation-brief-missing-license.json");
    let error = generate_audio(&brief, FIXED_NOW_MS)
        .expect_err("a brief without a license must be rejected fail-closed");
    assert!(
        error.to_string().contains("requires a license"),
        "unexpected error: {error}"
    );
}

#[test]
fn malformed_audio_is_rejected_fail_closed() {
    let brief = read_brief("audio-generation-brief-malformed.json");
    let error = generate_audio(&brief, FIXED_NOW_MS)
        .expect_err("a brief with an unsupported format must be rejected fail-closed");
    assert!(
        error.to_string().contains("format \"flac\" is unsupported"),
        "unexpected error: {error}"
    );
}

#[test]
fn blank_license_field_is_rejected() {
    let mut brief = read_brief("audio-generation-brief-v1.json");
    if let Some(license) = brief.license.as_mut() {
        license.holder = "   ".to_string();
    }
    let error = generate_audio(&brief, FIXED_NOW_MS)
        .expect_err("a blank license credit must be rejected fail-closed");
    assert!(
        error
            .to_string()
            .contains("audio license holder is required"),
        "unexpected error: {error}"
    );
}

#[test]
fn out_of_range_channels_is_rejected() {
    let mut brief = read_brief("audio-generation-brief-v1.json");
    brief.channels = 8;
    let error = generate_audio(&brief, FIXED_NOW_MS)
        .expect_err("an out-of-range channel count must be rejected fail-closed");
    assert!(
        error.to_string().contains("channels 8 is out of range"),
        "unexpected error: {error}"
    );
}

#[test]
fn zero_duration_is_rejected() {
    let mut brief = read_brief("audio-generation-brief-v1.json");
    brief.duration_ms = 0;
    let error = generate_audio(&brief, FIXED_NOW_MS)
        .expect_err("a zero-duration audio brief must be rejected fail-closed");
    assert!(
        error.to_string().contains("durationMs 0 is out of range"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_audio_generation_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/audio-pipeline-v1.md"))
        .expect("audio pipeline doc exists");
    assert!(
        doc.contains("#1642"),
        "audio pipeline doc records the generation-model follow-up (#1642)"
    );
    assert!(
        doc.contains("audio-proposal contract"),
        "doc records the audio-proposal contract"
    );
}
