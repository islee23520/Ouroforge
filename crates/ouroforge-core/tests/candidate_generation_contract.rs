//! Contract test for N-Variant Candidate Generation v1 (#1852).
//!
//! Candidate generation reuses the Milestone 30 front-door proposal path. It
//! emits N untrusted proposals for human curation, rejects malformed briefs, and
//! never performs or authorizes a trusted write.

use std::path::PathBuf;

use ouroforge_core::candidate_generation::{
    generate_candidates, CandidateGenerationBrief, CANDIDATE_GENERATION_GENERATOR,
    CANDIDATE_GENERATION_SCHEMA_VERSION,
};
use ouroforge_core::generative_intake::{DECK_ROGUELIKE_GAME_CLASS, GRID_PUZZLE_GAME_CLASS};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_786_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_text(name: &str) -> String {
    std::fs::read_to_string(repo_root().join("examples/curation-cockpit-v1").join(name))
        .unwrap_or_else(|_| panic!("fixture exists: {name}"))
}

fn read_brief(name: &str) -> CandidateGenerationBrief {
    CandidateGenerationBrief::from_json_str(&fixture_text(name)).expect("fixture parses")
}

#[test]
fn n_variant_generation_reuses_front_door_and_produces_proposals() {
    let brief = read_brief("candidate-generation-brief-v1.json");
    let set = generate_candidates(&brief, FIXED_NOW_MS).expect("candidate set is generated");

    set.validate().expect("candidate set validates");
    assert_eq!(set.schema_version, CANDIDATE_GENERATION_SCHEMA_VERSION);
    assert_eq!(set.generator, CANDIDATE_GENERATION_GENERATOR);
    assert_eq!(set.candidate_set_id, brief.candidate_set_id);
    assert_eq!(set.requested_count, 4);
    assert_eq!(set.candidates.len(), 4);

    let kinds: Vec<_> = set.candidates.iter().map(|c| c.kind.as_str()).collect();
    assert_eq!(kinds, ["card", "tuning", "flavor", "store-copy"]);

    let game_classes: Vec<_> = set
        .candidates
        .iter()
        .map(|candidate| candidate.proposal.provenance.game_class.as_str())
        .collect();
    assert!(game_classes.contains(&DECK_ROGUELIKE_GAME_CLASS));
    assert!(game_classes.contains(&GRID_PUZZLE_GAME_CLASS));

    for candidate in &set.candidates {
        assert!(candidate.proposal_only);
        candidate
            .proposal
            .validate()
            .expect("wrapped Milestone 30 proposal validates");
        assert_eq!(candidate.proposal.proposal.created_at_unix_ms, FIXED_NOW_MS);
    }
}

#[test]
fn malformed_brief_is_rejected_fail_closed() {
    let brief = read_brief("candidate-generation-brief-invalid.json");
    let error = generate_candidates(&brief, FIXED_NOW_MS)
        .expect_err("mismatched requestedCount must fail closed");
    assert!(
        error.to_string().contains("requestedCount"),
        "unexpected error: {error}"
    );
}

#[test]
fn proposals_are_untrusted_and_never_trusted_writes() {
    let brief = read_brief("candidate-generation-brief-v1.json");
    let set = generate_candidates(&brief, FIXED_NOW_MS).expect("candidate set is generated");

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
fn candidate_set_serializes_as_fixture_scoped_read_model() {
    let brief = read_brief("candidate-generation-brief-v1.json");
    let set = generate_candidates(&brief, FIXED_NOW_MS).expect("candidate set is generated");
    let value: Value = serde_json::to_value(&set).expect("candidate set serializes");

    assert_eq!(value["schemaVersion"], CANDIDATE_GENERATION_SCHEMA_VERSION);
    assert_eq!(value["proposalOnly"], true);
    assert_eq!(value["candidates"].as_array().unwrap().len(), 4);
    assert_eq!(value["candidates"][0]["proposalOnly"], true);
}

#[test]
fn docs_record_candidate_generation_boundaries() {
    let doc = std::fs::read_to_string(repo_root().join("docs/curation-cockpit-v1.md"))
        .expect("curation cockpit scope doc exists");
    assert!(doc.contains("Milestone 30"));
    assert!(doc.contains("proposal-only") || doc.contains("proposals only"));
    assert!(doc.contains("Human selection provenance"));
}
