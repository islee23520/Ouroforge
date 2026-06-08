//! N-Variant Candidate Generation v1 (#1852).
//!
//! Part of Candidate Generation and Curation Cockpit v1 (#1851) under #1 Era J
//! Milestone 57. This module turns a bounded curation brief into N proposal
//! variants (cards, tuning, flavor, store copy) by reusing the Milestone 30
//! generative front door. It is not a new writer: every emitted candidate wraps
//! the existing [`crate::generative_intake::GenerativeProposal`] / [`crate::MutationProposal`]
//! model, remains unverified and pending, and is never trusted or applied here.

use crate::generative_intake::{
    intake_brief, intake_deck_roguelike_brief, DeckRoguelikeBrief, GenerativeBrief,
    GenerativeProposal, DECK_ROGUELIKE_GAME_CLASS, GRID_PUZZLE_GAME_CLASS,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Schema version for candidate-generation curation briefs and proposal sets.
pub const CANDIDATE_GENERATION_SCHEMA_VERSION: &str = "ouroforge.candidate-generation.v1";
/// Identifier recorded as the generator of the candidate set. Individual
/// proposals still record the underlying Milestone 30 front-door generator.
pub const CANDIDATE_GENERATION_GENERATOR: &str = "candidate-generation-v1";

/// Candidate material classes allowed by the curation cockpit contract. These
/// are labels over proposal payloads, not separate generation engines.
const ALLOWED_VARIANT_KINDS: &[&str] = &["card", "tuning", "flavor", "store-copy"];

/// A curation brief asking for N candidate variants. Each variant carries exactly
/// one Milestone 30-compatible front-door brief: grid-puzzle or deck-roguelike.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CandidateGenerationBrief {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "candidateSetId")]
    pub candidate_set_id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "requestedCount")]
    pub requested_count: usize,
    pub variants: Vec<CandidateVariantBrief>,
}

/// One requested candidate variant. `kind` records whether the proposal is for
/// cards, tuning, flavor, or store copy. The actual proposed artifact is still
/// produced by an existing front-door intake path.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CandidateVariantBrief {
    #[serde(rename = "variantId")]
    pub variant_id: String,
    pub kind: String,
    pub summary: String,
    #[serde(rename = "gameClass")]
    pub game_class: String,
    #[serde(rename = "gridPuzzle", default)]
    pub grid_puzzle: Option<GenerativeBrief>,
    #[serde(rename = "deckRoguelike", default)]
    pub deck_roguelike: Option<DeckRoguelikeBrief>,
}

/// Generated candidate-set envelope. The set is proposal-only evidence for a
/// human curation decision; it grants no apply, promotion, or trusted-write
/// authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CandidateProposalSet {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "candidateSetId")]
    pub candidate_set_id: String,
    pub generator: String,
    #[serde(rename = "requestedCount")]
    pub requested_count: usize,
    pub candidates: Vec<CandidateProposal>,
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
}

/// One generated candidate proposal plus curation metadata. The wrapped proposal
/// is the existing Milestone 30 proposal model.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CandidateProposal {
    #[serde(rename = "variantId")]
    pub variant_id: String,
    pub kind: String,
    pub summary: String,
    pub proposal: GenerativeProposal,
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
}

impl CandidateGenerationBrief {
    /// Parse a candidate-generation brief from JSON, failing closed on malformed
    /// JSON or unknown fields.
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("candidate generation brief is not valid JSON: {err}"))
    }

    /// Validate the curation envelope and per-variant routing constraints. Deep
    /// structural validation of each proposal payload is delegated to the
    /// existing front-door intake functions in [`generate_candidates`].
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CANDIDATE_GENERATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "candidate generation brief schemaVersion must be \"{CANDIDATE_GENERATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "candidate generation brief candidateSetId",
            &self.candidate_set_id,
        )?;
        crate::require_text("candidate generation brief title", &self.title)?;
        crate::require_text("candidate generation brief description", &self.description)?;
        if self.requested_count == 0 {
            return Err(anyhow!(
                "candidate generation brief requestedCount must be greater than zero"
            ));
        }
        if self.variants.len() != self.requested_count {
            return Err(anyhow!(
                "candidate generation brief requestedCount must equal variants length"
            ));
        }
        let mut ids = BTreeSet::new();
        for variant in &self.variants {
            variant.validate()?;
            if !ids.insert(variant.variant_id.clone()) {
                return Err(anyhow!(
                    "candidate generation brief contains duplicate variantId: {}",
                    variant.variant_id
                ));
            }
        }
        Ok(())
    }
}

impl CandidateVariantBrief {
    pub fn validate(&self) -> Result<()> {
        crate::require_text("candidate variant variantId", &self.variant_id)?;
        crate::require_text("candidate variant kind", &self.kind)?;
        crate::require_text("candidate variant summary", &self.summary)?;
        if !ALLOWED_VARIANT_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!(
                "candidate variant kind \"{}\" is unsupported; expected one of {:?}",
                self.kind,
                ALLOWED_VARIANT_KINDS
            ));
        }
        let has_grid = self.grid_puzzle.is_some();
        let has_deck = self.deck_roguelike.is_some();
        if has_grid == has_deck {
            return Err(anyhow!(
                "candidate variant must declare exactly one front-door brief"
            ));
        }
        match self.game_class.as_str() {
            GRID_PUZZLE_GAME_CLASS if has_grid => Ok(()),
            DECK_ROGUELIKE_GAME_CLASS if has_deck => Ok(()),
            GRID_PUZZLE_GAME_CLASS | DECK_ROGUELIKE_GAME_CLASS => Err(anyhow!(
                "candidate variant gameClass does not match declared front-door brief"
            )),
            other => Err(anyhow!(
                "candidate variant gameClass \"{other}\" is unsupported"
            )),
        }
    }
}

impl CandidateProposalSet {
    /// Validate the generated set, failing closed on malformed envelopes,
    /// duplicate candidate/proposal ids, or non-proposal-only drift.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CANDIDATE_GENERATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "candidate proposal set schemaVersion must be \"{CANDIDATE_GENERATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "candidate proposal set candidateSetId",
            &self.candidate_set_id,
        )?;
        if self.generator != CANDIDATE_GENERATION_GENERATOR {
            return Err(anyhow!(
                "candidate proposal set generator must be \"{CANDIDATE_GENERATION_GENERATOR}\""
            ));
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "candidate proposal set proposalOnly must be true: generation emits proposals only"
            ));
        }
        if self.requested_count == 0 || self.candidates.len() != self.requested_count {
            return Err(anyhow!(
                "candidate proposal set requestedCount must equal candidates length"
            ));
        }
        let mut variant_ids = BTreeSet::new();
        let mut proposal_ids = BTreeSet::new();
        for candidate in &self.candidates {
            candidate.validate()?;
            if !variant_ids.insert(candidate.variant_id.clone()) {
                return Err(anyhow!(
                    "candidate proposal set contains duplicate variantId: {}",
                    candidate.variant_id
                ));
            }
            if !proposal_ids.insert(candidate.proposal.proposal.id.clone()) {
                return Err(anyhow!(
                    "candidate proposal set contains duplicate proposal id: {}",
                    candidate.proposal.proposal.id
                ));
            }
        }
        Ok(())
    }
}

impl CandidateProposal {
    pub fn validate(&self) -> Result<()> {
        crate::require_text("candidate proposal variantId", &self.variant_id)?;
        crate::require_text("candidate proposal kind", &self.kind)?;
        crate::require_text("candidate proposal summary", &self.summary)?;
        if !ALLOWED_VARIANT_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!("candidate proposal kind is unsupported"));
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "candidate proposal proposalOnly must be true: generation emits proposals only"
            ));
        }
        self.proposal.validate()?;
        if self.proposal.proposal.status != "proposed"
            || self.proposal.proposal.verdict_status != "pending"
            || self.proposal.proposal.confidence != "unverified"
            || !self.proposal.provenance.proposal_only
        {
            return Err(anyhow!(
                "candidate proposal must remain unverified, pending, and proposal-only"
            ));
        }
        Ok(())
    }
}

/// Generate N candidate proposals by routing each variant through the existing
/// Milestone 30 front-door path. `now_unix_ms` is supplied by the caller for
/// deterministic tests. This function never reads the clock, filesystem, or
/// network, and performs no trusted write.
pub fn generate_candidates(
    brief: &CandidateGenerationBrief,
    now_unix_ms: u128,
) -> Result<CandidateProposalSet> {
    brief.validate()?;

    let mut candidates = Vec::with_capacity(brief.requested_count);
    for variant in &brief.variants {
        let proposal = match variant.game_class.as_str() {
            GRID_PUZZLE_GAME_CLASS => intake_brief(
                variant
                    .grid_puzzle
                    .as_ref()
                    .ok_or_else(|| anyhow!("candidate variant missing gridPuzzle brief"))?,
                now_unix_ms,
            )
            .map_err(|err| anyhow!("candidate variant \"{}\": {err}", variant.variant_id))?,
            DECK_ROGUELIKE_GAME_CLASS => intake_deck_roguelike_brief(
                variant
                    .deck_roguelike
                    .as_ref()
                    .ok_or_else(|| anyhow!("candidate variant missing deckRoguelike brief"))?,
                now_unix_ms,
            )
            .map_err(|err| anyhow!("candidate variant \"{}\": {err}", variant.variant_id))?,
            other => {
                return Err(anyhow!(
                    "candidate variant gameClass \"{other}\" is unsupported"
                ))
            }
        };
        candidates.push(CandidateProposal {
            variant_id: variant.variant_id.clone(),
            kind: variant.kind.clone(),
            summary: variant.summary.clone(),
            proposal,
            proposal_only: true,
        });
    }

    let set = CandidateProposalSet {
        schema_version: CANDIDATE_GENERATION_SCHEMA_VERSION.to_string(),
        candidate_set_id: brief.candidate_set_id.clone(),
        generator: CANDIDATE_GENERATION_GENERATOR.to_string(),
        requested_count: brief.requested_count,
        candidates,
        proposal_only: true,
    };
    set.validate()?;
    Ok(set)
}
