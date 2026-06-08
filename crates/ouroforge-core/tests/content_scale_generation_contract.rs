//! Contract test for Campaign-Scale Generation v1 (#1649).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. These tests machine-check the campaign-scale contract: a
//! well-formed campaign brief produces a *set* of validated proposals across
//! both supported genres (grid-puzzle + deck-roguelike); a malformed item fails
//! the whole campaign closed (no per-game escape hatch); and the deck-roguelike
//! genre extension of the Milestone 30 front door produces a proposal-only
//! artifact that is never promoted.
//!
//! Boundary checks assert the proposal-only model: every generated proposal is
//! proposed/pending/unverified, provenance records proposal-only, and campaign
//! generation performs no trusted write. The campaign-scale extension reuses the
//! existing proposal model and the Milestone 30 intake path; it is not a new
//! generator.

use std::path::PathBuf;

use ouroforge_core::content_scale_generation::{
    generate_campaign, CampaignBrief, CAMPAIGN_SCALE_GENERATOR, CAMPAIGN_SCALE_SCHEMA_VERSION,
};
use ouroforge_core::generative_intake::{
    intake_deck_roguelike_brief, DeckRoguelikeBrief, DECK_ROGUELIKE_GAME_CLASS,
    DECK_ROGUELIKE_SCHEMA_VERSION, GENERATIVE_INTAKE_GENERATOR, GENERATIVE_INTAKE_SOURCE,
    GRID_PUZZLE_GAME_CLASS,
};
use serde_json::Value;

const FIXED_NOW_MS: u128 = 1_725_000_000_000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_campaign(name: &str) -> CampaignBrief {
    let path: PathBuf = repo_root()
        .join("examples/generative-front-door")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("fixture exists: {name}"));
    CampaignBrief::from_json_str(&text).expect("fixture campaign brief parses")
}

#[test]
fn campaign_generation_produces_a_proposal_set() {
    let brief = read_campaign("campaign-scale-brief-v1.json");
    let set = generate_campaign(&brief, FIXED_NOW_MS).expect("well-formed campaign is accepted");

    set.validate().expect("campaign proposal set validates");
    assert_eq!(set.schema_version, CAMPAIGN_SCALE_SCHEMA_VERSION);
    assert_eq!(set.generator, CAMPAIGN_SCALE_GENERATOR);
    assert!(set.proposal_only);
    // A campaign produces many proposals — one per declared item.
    assert_eq!(set.proposals.len(), brief.item_count());
    assert!(
        set.proposals.len() >= 2,
        "campaign scale must produce a set of more than one proposal"
    );
    // Proposal ids are unique across the set.
    let mut ids: Vec<&str> = set
        .proposals
        .iter()
        .map(|g| g.proposal.id.as_str())
        .collect();
    let total = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), total, "campaign proposal ids must be unique");
}

#[test]
fn campaign_generation_covers_both_genres() {
    let brief = read_campaign("campaign-scale-brief-v1.json");
    let set = generate_campaign(&brief, FIXED_NOW_MS).expect("well-formed campaign is accepted");

    assert!(
        set.covers_both_genres(),
        "campaign set must cover puzzle + roguelike, got {:?}",
        set.genres
    );
    let has_grid = set
        .proposals
        .iter()
        .any(|g| g.provenance.game_class == GRID_PUZZLE_GAME_CLASS);
    let has_deck = set
        .proposals
        .iter()
        .any(|g| g.provenance.game_class == DECK_ROGUELIKE_GAME_CLASS);
    assert!(has_grid, "campaign set must contain a grid-puzzle proposal");
    assert!(
        has_deck,
        "campaign set must contain a deck-roguelike proposal"
    );
}

#[test]
fn campaign_proposals_are_proposal_only_and_not_promoted() {
    let brief = read_campaign("campaign-scale-brief-v1.json");
    let set = generate_campaign(&brief, FIXED_NOW_MS).expect("well-formed campaign is accepted");

    for generative in &set.proposals {
        // A freshly generated proposal has not passed the engine room: it is
        // proposed/pending, not applied/promoted/approved.
        assert_eq!(generative.proposal.status, "proposed");
        assert_eq!(generative.proposal.verdict_status, "pending");
        assert_eq!(generative.proposal.confidence, "unverified");
        assert!(generative.provenance.proposal_only);
        assert_eq!(generative.provenance.generator, GENERATIVE_INTAKE_GENERATOR);
        assert_eq!(generative.provenance.source, GENERATIVE_INTAKE_SOURCE);
        assert_eq!(generative.proposal.created_at_unix_ms, FIXED_NOW_MS);
    }
}

#[test]
fn campaign_generation_is_deterministic() {
    let brief = read_campaign("campaign-scale-brief-v1.json");
    let first = generate_campaign(&brief, FIXED_NOW_MS).expect("accepted");
    let second = generate_campaign(&brief, FIXED_NOW_MS).expect("accepted");
    assert_eq!(first, second);
}

#[test]
fn malformed_campaign_item_is_rejected_fail_closed() {
    let brief = read_campaign("campaign-scale-brief-invalid.json");
    let error = generate_campaign(&brief, FIXED_NOW_MS)
        .expect_err("a campaign with a malformed item must be rejected fail-closed");
    assert!(
        error.to_string().contains("undeclared card"),
        "unexpected error: {error}"
    );
}

#[test]
fn empty_campaign_is_rejected() {
    let mut brief = read_campaign("campaign-scale-brief-v1.json");
    brief.grid_puzzles.clear();
    brief.deck_roguelikes.clear();
    let error = generate_campaign(&brief, FIXED_NOW_MS)
        .expect_err("a campaign with no items must be rejected fail-closed");
    assert!(
        error.to_string().contains("at least one item"),
        "unexpected error: {error}"
    );
}

#[test]
fn deck_roguelike_intake_extends_the_front_door() {
    // The deck-roguelike genre extension produces a valid proposal whose `to`
    // payload is the assembled deck-roguelike artifact, reusing the existing
    // proposal model and provenance.
    let brief = read_campaign("campaign-scale-brief-v1.json");
    let deck_brief: &DeckRoguelikeBrief = brief
        .deck_roguelikes
        .first()
        .expect("fixture has a deck-roguelike item");
    let generative =
        intake_deck_roguelike_brief(deck_brief, FIXED_NOW_MS).expect("deck brief is accepted");

    generative.validate().expect("deck proposal validates");
    assert_eq!(generative.proposal.target, DECK_ROGUELIKE_GAME_CLASS);
    assert_eq!(generative.provenance.game_class, DECK_ROGUELIKE_GAME_CLASS);

    let artifact: Value = serde_json::from_str(&generative.proposal.to)
        .expect("proposal carries a deck-roguelike artifact");
    assert_eq!(artifact["schemaVersion"], DECK_ROGUELIKE_SCHEMA_VERSION);
    assert_eq!(artifact["id"], deck_brief.run_id);
    assert_eq!(
        artifact["deck"],
        serde_json::to_value(&deck_brief.deck).unwrap()
    );

    // Provenance links the proposal back to the exact brief that produced it.
    assert_eq!(generative.provenance.brief_id, deck_brief.brief_id);
    assert_eq!(
        generative.provenance.brief_digest,
        deck_brief.digest().expect("brief digest")
    );
}

#[test]
fn deck_roguelike_intake_rejects_undeclared_relic() {
    // The deck-roguelike validator fails closed when relics reference an
    // undeclared relic vocabulary entry.
    let brief = read_campaign("campaign-scale-brief-v1.json");
    let mut deck_brief = brief
        .deck_roguelikes
        .first()
        .expect("fixture has a deck-roguelike item")
        .clone();
    deck_brief.relics = vec!["ghost-relic".to_string()];
    let error = intake_deck_roguelike_brief(&deck_brief, FIXED_NOW_MS)
        .expect_err("undeclared relic must be rejected fail-closed");
    assert!(
        error.to_string().contains("undeclared relic"),
        "unexpected error: {error}"
    );
}

#[test]
fn docs_record_the_campaign_scale_contract() {
    let doc = std::fs::read_to_string(repo_root().join("docs/content-scale-generation-v1.md"))
        .expect("content-scale generation doc exists");
    assert!(
        doc.contains("#1649"),
        "Content-Scale Generation v1 doc records this issue (#1649)"
    );
    assert!(
        doc.contains("proposals only") || doc.contains("proposal-only"),
        "doc records the proposals-only contract"
    );
}
