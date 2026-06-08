//! Campaign-Scale Generation v1 (#1649).
//!
//! Part of Content-at-Scale Generation and Curation v1 (#1648) under #1 Era G
//! Milestone 38. This module turns a single campaign brief — describing many
//! levels and a large card/relic pool across two genres — into a *set* of
//! proposals, one per item. It is the campaign-scale entry point to the
//! Milestone 30 generative front door; it does not introduce a new generator.
//!
//! Boundary: campaign-scale generation is proposal-only. Each item is produced
//! by the existing front door — grid-puzzle via [`crate::generative_intake::intake_brief`]
//! and deck-roguelike via [`crate::generative_intake::intake_deck_roguelike_brief`]
//! — so every emitted [`GenerativeProposal`] wraps the existing
//! [`crate::MutationProposal`] model, is unverified and pending, and is never
//! promoted here. There is no per-game escape hatch: an unsupported genre or a
//! malformed item fails the whole campaign closed. Promotion past the engine
//! room (gates + solver + curation) remains out of scope; a campaign set is a
//! batch of proposals awaiting the existing review/apply/trust-gradient path.

use crate::generative_intake::{
    intake_brief, intake_deck_roguelike_brief, DeckRoguelikeBrief, GenerativeBrief,
    GenerativeProposal, DECK_ROGUELIKE_GAME_CLASS, GRID_PUZZLE_GAME_CLASS,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Schema version for the campaign-scale brief and the generated proposal set.
pub const CAMPAIGN_SCALE_SCHEMA_VERSION: &str = "ouroforge.campaign-scale.v1";
/// Identifier recorded as the generator of a campaign-scale proposal set. The
/// individual proposals keep the front-door generator id; this records that the
/// *set* was assembled by the campaign-scale extension.
pub const CAMPAIGN_SCALE_GENERATOR: &str = "campaign-scale-generation-v1";

/// A campaign brief: campaign metadata plus the per-item briefs for each genre.
/// The two genre lists are explicit (rather than a tagged union) so the author
/// states genre coverage directly and each list is validated by its own genre's
/// front-door intake.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CampaignBrief {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    pub title: String,
    /// Natural-language description of the campaign. Preserved for provenance.
    pub description: String,
    /// Grid-puzzle level briefs (Milestone 30 / Grid-Puzzle Game Class v1).
    #[serde(rename = "gridPuzzles", default)]
    pub grid_puzzles: Vec<GenerativeBrief>,
    /// Deck-roguelike encounter briefs (Deck-Roguelike Game Class v1, #1601).
    #[serde(rename = "deckRoguelikes", default)]
    pub deck_roguelikes: Vec<DeckRoguelikeBrief>,
}

/// A campaign-scale proposal set: the campaign envelope plus every generated
/// proposal. This wraps — it does not modify — the existing proposal model.
/// Read-only audit metadata; it confers no apply or promotion authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CampaignProposalSet {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "campaignId")]
    pub campaign_id: String,
    pub generator: String,
    /// The set of generated proposals, one per campaign item.
    pub proposals: Vec<GenerativeProposal>,
    /// The distinct game classes covered by this set, sorted for determinism.
    pub genres: Vec<String>,
    /// Always true: campaign-scale generation emits proposals only.
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
}

impl CampaignBrief {
    /// Parse a campaign brief from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> Result<Self> {
        let brief: CampaignBrief = serde_json::from_str(text)
            .map_err(|err| anyhow!("campaign brief is not valid JSON: {err}"))?;
        Ok(brief)
    }

    /// Validate the campaign envelope, failing closed on any problem. Per-item
    /// structural acceptance is enforced by [`generate_campaign`] via each
    /// genre's front-door intake.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CAMPAIGN_SCALE_SCHEMA_VERSION {
            return Err(anyhow!(
                "campaign brief schemaVersion must be \"{CAMPAIGN_SCALE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("campaign brief campaignId", &self.campaign_id)?;
        crate::require_text("campaign brief title", &self.title)?;
        crate::require_text("campaign brief description", &self.description)?;
        if self.grid_puzzles.is_empty() && self.deck_roguelikes.is_empty() {
            return Err(anyhow!(
                "campaign brief must contain at least one item (gridPuzzles or deckRoguelikes)"
            ));
        }
        Ok(())
    }

    /// Total number of items declared across all genres.
    pub fn item_count(&self) -> usize {
        self.grid_puzzles.len() + self.deck_roguelikes.len()
    }
}

impl CampaignProposalSet {
    /// Validate the whole set, failing closed: the envelope, every wrapped
    /// proposal, the proposal-only flag, and the recorded genre coverage.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CAMPAIGN_SCALE_SCHEMA_VERSION {
            return Err(anyhow!(
                "campaign proposal set schemaVersion must be \"{CAMPAIGN_SCALE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("campaign proposal set campaignId", &self.campaign_id)?;
        if self.generator != CAMPAIGN_SCALE_GENERATOR {
            return Err(anyhow!(
                "campaign proposal set generator must be \"{CAMPAIGN_SCALE_GENERATOR}\""
            ));
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "campaign proposal set proposalOnly must be true: generation emits proposals only"
            ));
        }
        if self.proposals.is_empty() {
            return Err(anyhow!(
                "campaign proposal set must contain at least one proposal"
            ));
        }
        let mut ids = BTreeSet::new();
        let mut observed: BTreeSet<String> = BTreeSet::new();
        for generative in &self.proposals {
            generative.validate()?;
            if !ids.insert(generative.proposal.id.clone()) {
                return Err(anyhow!(
                    "campaign proposal set contains a duplicate proposal id: {}",
                    generative.proposal.id
                ));
            }
            observed.insert(generative.provenance.game_class.clone());
        }
        let recorded: BTreeSet<String> = self.genres.iter().cloned().collect();
        if recorded != observed {
            return Err(anyhow!(
                "campaign proposal set genres must match the genres of its proposals"
            ));
        }
        Ok(())
    }

    /// True iff this set covers both supported genres (puzzle + roguelike).
    pub fn covers_both_genres(&self) -> bool {
        self.genres.iter().any(|g| g == GRID_PUZZLE_GAME_CLASS)
            && self.genres.iter().any(|g| g == DECK_ROGUELIKE_GAME_CLASS)
    }
}

/// Generate a campaign-scale set of proposals from a campaign brief. Each item
/// is routed through its genre's existing front-door intake; a single malformed
/// item fails the whole campaign closed (no per-game escape hatch). `now_unix_ms`
/// is supplied by the caller so the result is deterministic and testable; this
/// function never reads the clock, the filesystem, the network, or performs any
/// trusted write.
pub fn generate_campaign(brief: &CampaignBrief, now_unix_ms: u128) -> Result<CampaignProposalSet> {
    brief.validate()?;

    let mut proposals = Vec::with_capacity(brief.item_count());
    let mut genres: BTreeSet<String> = BTreeSet::new();

    for item in &brief.grid_puzzles {
        let generative = intake_brief(item, now_unix_ms)
            .map_err(|err| anyhow!("campaign grid-puzzle item \"{}\": {err}", item.brief_id))?;
        genres.insert(generative.provenance.game_class.clone());
        proposals.push(generative);
    }
    for item in &brief.deck_roguelikes {
        let generative = intake_deck_roguelike_brief(item, now_unix_ms)
            .map_err(|err| anyhow!("campaign deck-roguelike item \"{}\": {err}", item.brief_id))?;
        genres.insert(generative.provenance.game_class.clone());
        proposals.push(generative);
    }

    let set = CampaignProposalSet {
        schema_version: CAMPAIGN_SCALE_SCHEMA_VERSION.to_string(),
        campaign_id: brief.campaign_id.clone(),
        generator: CAMPAIGN_SCALE_GENERATOR.to_string(),
        proposals,
        genres: genres.into_iter().collect(),
        proposal_only: true,
    };
    set.validate()?;
    Ok(set)
}
