//! Contract test for Narrative/Theme-Arc Candidate Generation v1 (#1864).
//!
//! Narrative candidates extend the Milestone 39 narrative surface and reuse the
//! Milestone 30 proposal front door. They remain proposal-only, reject malformed
//! prompts, and never grant trusted-write or automated tone/soul authority.

use std::path::PathBuf;

use ouroforge_core::generative_intake::{DECK_ROGUELIKE_GAME_CLASS, GRID_PUZZLE_GAME_CLASS};
use ouroforge_core::narrative_candidate::{
    generate_narrative_candidates, NarrativeCandidateBrief, NarrativeCandidateSet,
    NARRATIVE_CANDIDATE_GENERATOR, NARRATIVE_CANDIDATE_SCHEMA_VERSION,
    NARRATIVE_TONE_HUMAN_BOUNDARY,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_786_400_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(repo_root().join("examples/narrative-assist-v1").join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn read_brief(name: &str) -> NarrativeCandidateBrief {
    NarrativeCandidateBrief::from_json_str(&fixture_text(name)).expect("fixture parses")
}

#[test]
fn narrative_candidates_reuse_milestone_39_and_30_as_proposals() {
    let brief = read_brief("narrative-candidate-brief-v1.json");
    let set =
        generate_narrative_candidates(&brief, FIXED_NOW_MS).expect("narrative candidates generate");

    set.validate().expect("candidate set validates");
    assert_eq!(set.schema_version, NARRATIVE_CANDIDATE_SCHEMA_VERSION);
    assert_eq!(set.generator, NARRATIVE_CANDIDATE_GENERATOR);
    assert_eq!(set.candidate_set_id, brief.candidate_set_id);
    assert_eq!(set.story_id, "demo-story");
    assert_eq!(set.candidate_count, 3);
    assert_eq!(set.candidates.len(), 3);
    assert_eq!(set.human_tone_boundary, NARRATIVE_TONE_HUMAN_BOUNDARY);

    let classes: Vec<_> = set.candidates.iter().map(|c| c.class.as_str()).collect();
    assert_eq!(
        classes,
        ["theme-arc-beat", "dialogue-variant", "flavor-text"]
    );

    let game_classes: Vec<_> = set
        .candidates
        .iter()
        .map(|candidate| candidate.proposal.provenance.game_class.as_str())
        .collect();
    assert!(game_classes.contains(&GRID_PUZZLE_GAME_CLASS));
    assert!(game_classes.contains(&DECK_ROGUELIKE_GAME_CLASS));

    for candidate in &set.candidates {
        assert!(candidate.proposal_only);
        assert_eq!(candidate.human_tone_boundary, NARRATIVE_TONE_HUMAN_BOUNDARY);
        assert_eq!(
            candidate.source_brief_ref,
            candidate.proposal.provenance.brief_id
        );
        assert_eq!(candidate.payload_hash.len(), 64);
        candidate
            .proposal
            .validate()
            .expect("wrapped Milestone 30 proposal validates");
        assert_eq!(candidate.proposal.proposal.created_at_unix_ms, FIXED_NOW_MS);
    }
}

#[test]
fn malformed_prompt_is_rejected_fail_closed() {
    let brief = read_brief("narrative-candidate-malformed-v1.json");
    let error = generate_narrative_candidates(&brief, FIXED_NOW_MS)
        .expect_err("mismatched candidateCount must fail closed");
    assert!(
        error.to_string().contains("candidateCount"),
        "unexpected error: {error}"
    );
}

#[test]
fn proposal_only_drift_is_rejected_and_never_trusted_write() {
    let drift: NarrativeCandidateSet =
        serde_json::from_str(&fixture_text("narrative-candidate-proposal-only-v1.json"))
            .expect("proposal-only drift fixture parses");
    let err = drift
        .validate()
        .expect_err("proposalOnly=false must fail closed");
    assert!(err.to_string().contains("proposalOnly"));

    let brief = read_brief("narrative-candidate-brief-v1.json");
    let set =
        generate_narrative_candidates(&brief, FIXED_NOW_MS).expect("narrative candidates generate");
    assert!(set.proposal_only);
    for candidate in &set.candidates {
        let proposal = &candidate.proposal.proposal;
        assert_eq!(proposal.status, "proposed");
        assert_eq!(proposal.verdict_status, "pending");
        assert_eq!(proposal.confidence, "unverified");
        assert!(candidate.proposal.provenance.proposal_only);
        assert!(proposal.path.ends_with(".json"));
        assert!(
            !proposal.path.starts_with("crates/")
                && !proposal.path.starts_with("docs/")
                && !proposal.path.starts_with("examples/")
        );
    }
}

#[test]
fn candidate_set_serializes_as_read_only_fixture_scoped_evidence() {
    let brief = read_brief("narrative-candidate-brief-v1.json");
    let set =
        generate_narrative_candidates(&brief, FIXED_NOW_MS).expect("narrative candidates generate");
    let value: Value = serde_json::to_value(&set).expect("candidate set serializes");

    assert_eq!(value["schemaVersion"], NARRATIVE_CANDIDATE_SCHEMA_VERSION);
    assert_eq!(value["proposalOnly"], true);
    assert_eq!(value["humanToneBoundary"], NARRATIVE_TONE_HUMAN_BOUNDARY);
    assert_eq!(value["candidates"].as_array().unwrap().len(), 3);
    assert_eq!(value["candidates"][0]["proposalOnly"], true);
}

#[test]
fn automated_tone_or_quality_claims_are_rejected() {
    let mut brief = read_brief("narrative-candidate-brief-v1.json");
    brief.candidates[0].compatibility_notes =
        "This machine proves fun and production-ready tone.".to_string();
    let err = generate_narrative_candidates(&brief, FIXED_NOW_MS)
        .expect_err("automated tone/quality claims must fail closed");
    assert!(err.to_string().contains("human judgments"));
}

#[test]
fn docs_record_narrative_candidate_boundaries() {
    let doc = std::fs::read_to_string(repo_root().join("docs/narrative-assist-v1.md"))
        .expect("narrative assist scope doc exists");
    assert!(doc.contains("Narrative/theme candidate generation contract"));
    assert!(doc.contains("Milestone 39"));
    assert!(doc.contains("Milestone 30"));
    assert!(doc.contains("proposal-only") || doc.contains("proposals only"));
    assert!(doc.contains("Tone/soul is a human decision boundary"));
}
