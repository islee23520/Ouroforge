//! Narrative/Theme-Arc Candidate Generation v1 (#1864).
//!
//! This module generates bounded narrative/theme-wit/flavor candidate sets as
//! proposal-only data. It extends the Milestone 39 narrative system contract by
//! requiring a canonical narrative surface, and reuses the Milestone 30
//! generative intake front door for each candidate payload. It is not a new
//! narrative engine or writer: every output wraps an unverified
//! [`crate::generative_intake::GenerativeProposal`] and never performs, grants,
//! or records a trusted write.

use crate::export_hash::sha256_hex;
use crate::generative_intake::{
    intake_brief, intake_deck_roguelike_brief, DeckRoguelikeBrief, GenerativeBrief,
    GenerativeProposal, DECK_ROGUELIKE_GAME_CLASS, GRID_PUZZLE_GAME_CLASS,
};
use crate::narrative_system::{NARRATIVE_SYSTEM_BOUNDARY, NARRATIVE_SYSTEM_SCHEMA_VERSION};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const NARRATIVE_CANDIDATE_SCHEMA_VERSION: &str = "ouroforge.narrative-candidate.v1";
pub const NARRATIVE_CANDIDATE_GENERATOR: &str = "narrative-candidate-generation-v1";
pub const NARRATIVE_TONE_HUMAN_BOUNDARY: &str =
    "tone/soul/fun/quality are human judgments; generation emits proposals only";

const ALLOWED_CANDIDATE_CLASSES: &[&str] = &[
    "theme-arc-beat",
    "dialogue-variant",
    "event-hook",
    "flavor-text",
    "store-copy-draft",
    "onboarding-text",
    "moment-wit",
    "tone-note",
];

const FORBIDDEN_AUTOMATED_CLAIMS: &[&str] = &[
    "tone match",
    "soul match",
    "proves fun",
    "proven fun",
    "fun score",
    "narratively good",
    "production-ready",
    "shippable",
    "marketable",
    "godot replacement",
    "godot parity",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeCandidateBrief {
    pub schema_version: String,
    pub candidate_set_id: String,
    pub title: String,
    pub phase_shift: String,
    pub narrative_arc: String,
    #[serde(default)]
    pub theme_goals: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
    pub candidate_count: usize,
    pub narrative_surface: NarrativeCandidateSurface,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub candidates: Vec<NarrativeCandidateRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeCandidateSurface {
    pub schema_version: String,
    pub story_id: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeCandidateRequest {
    pub candidate_id: String,
    pub class: String,
    pub human_intent: String,
    pub compatibility_notes: String,
    pub game_class: String,
    #[serde(default)]
    pub grid_puzzle: Option<GenerativeBrief>,
    #[serde(default)]
    pub deck_roguelike: Option<DeckRoguelikeBrief>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeCandidateSet {
    pub schema_version: String,
    pub candidate_set_id: String,
    pub generator: String,
    pub story_id: String,
    pub phase_shift: String,
    pub candidate_count: usize,
    pub source_refs: Vec<String>,
    pub candidates: Vec<NarrativeCandidate>,
    pub proposal_only: bool,
    pub human_tone_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeCandidate {
    pub candidate_id: String,
    pub class: String,
    pub human_intent: String,
    pub compatibility_notes: String,
    pub source_brief_ref: String,
    pub payload_hash: String,
    pub proposal: GenerativeProposal,
    pub proposal_only: bool,
    pub human_tone_boundary: String,
}

impl NarrativeCandidateBrief {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("narrative candidate brief is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_CANDIDATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "narrative candidate brief schemaVersion must be \"{NARRATIVE_CANDIDATE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "narrative candidate brief candidateSetId",
            &self.candidate_set_id,
        )?;
        crate::require_text("narrative candidate brief title", &self.title)?;
        require_human_owned_text("narrative candidate brief phaseShift", &self.phase_shift)?;
        require_human_owned_text(
            "narrative candidate brief narrativeArc",
            &self.narrative_arc,
        )?;
        if self.theme_goals.is_empty() {
            return Err(anyhow!(
                "narrative candidate brief themeGoals must not be empty"
            ));
        }
        for goal in &self.theme_goals {
            require_human_owned_text("narrative candidate brief themeGoals", goal)?;
        }
        for constraint in &self.constraints {
            require_human_owned_text("narrative candidate brief constraints", constraint)?;
        }
        if self.candidate_count == 0 {
            return Err(anyhow!(
                "narrative candidate brief candidateCount must be greater than zero"
            ));
        }
        if self.candidate_count != self.candidates.len() {
            return Err(anyhow!(
                "narrative candidate brief candidateCount must equal candidates length"
            ));
        }
        self.narrative_surface.validate()?;
        for source_ref in &self.source_refs {
            validate_source_ref(source_ref)?;
        }
        let mut ids = BTreeSet::new();
        for candidate in &self.candidates {
            candidate.validate()?;
            if !ids.insert(candidate.candidate_id.clone()) {
                return Err(anyhow!(
                    "narrative candidate brief contains duplicate candidateId: {}",
                    candidate.candidate_id
                ));
            }
        }
        Ok(())
    }
}

impl NarrativeCandidateSurface {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_SYSTEM_SCHEMA_VERSION {
            return Err(anyhow!(
                "narrative candidate surface schemaVersion must match Milestone 39 narrative system"
            ));
        }
        crate::require_text("narrative candidate surface storyId", &self.story_id)?;
        if self.boundary != NARRATIVE_SYSTEM_BOUNDARY {
            return Err(anyhow!(
                "narrative candidate surface boundary must be the Milestone 39 read-only/proposal-only boundary"
            ));
        }
        Ok(())
    }
}

impl NarrativeCandidateRequest {
    pub fn validate(&self) -> Result<()> {
        crate::require_text(
            "narrative candidate request candidateId",
            &self.candidate_id,
        )?;
        crate::require_text("narrative candidate request class", &self.class)?;
        require_human_owned_text(
            "narrative candidate request humanIntent",
            &self.human_intent,
        )?;
        require_human_owned_text(
            "narrative candidate request compatibilityNotes",
            &self.compatibility_notes,
        )?;
        if !ALLOWED_CANDIDATE_CLASSES.contains(&self.class.as_str()) {
            return Err(anyhow!(
                "narrative candidate request class \"{}\" is unsupported; expected one of {:?}",
                self.class,
                ALLOWED_CANDIDATE_CLASSES
            ));
        }
        let has_grid = self.grid_puzzle.is_some();
        let has_deck = self.deck_roguelike.is_some();
        if has_grid == has_deck {
            return Err(anyhow!(
                "narrative candidate request must declare exactly one Milestone 30 front-door brief"
            ));
        }
        match self.game_class.as_str() {
            GRID_PUZZLE_GAME_CLASS if has_grid => Ok(()),
            DECK_ROGUELIKE_GAME_CLASS if has_deck => Ok(()),
            GRID_PUZZLE_GAME_CLASS | DECK_ROGUELIKE_GAME_CLASS => Err(anyhow!(
                "narrative candidate request gameClass does not match declared front-door brief"
            )),
            other => Err(anyhow!(
                "narrative candidate request gameClass \"{other}\" is unsupported"
            )),
        }
    }
}

impl NarrativeCandidateSet {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_CANDIDATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "narrative candidate set schemaVersion must be \"{NARRATIVE_CANDIDATE_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "narrative candidate set candidateSetId",
            &self.candidate_set_id,
        )?;
        if self.generator != NARRATIVE_CANDIDATE_GENERATOR {
            return Err(anyhow!(
                "narrative candidate set generator must be \"{NARRATIVE_CANDIDATE_GENERATOR}\""
            ));
        }
        crate::require_text("narrative candidate set storyId", &self.story_id)?;
        require_human_owned_text("narrative candidate set phaseShift", &self.phase_shift)?;
        if !self.proposal_only {
            return Err(anyhow!(
                "narrative candidate set proposalOnly must be true: generation emits proposals only"
            ));
        }
        if self.candidate_count == 0 || self.candidate_count != self.candidates.len() {
            return Err(anyhow!(
                "narrative candidate set candidateCount must equal candidates length"
            ));
        }
        if self.human_tone_boundary != NARRATIVE_TONE_HUMAN_BOUNDARY {
            return Err(anyhow!(
                "narrative candidate set must preserve the human tone/soul boundary"
            ));
        }
        for source_ref in &self.source_refs {
            validate_source_ref(source_ref)?;
        }
        let mut candidate_ids = BTreeSet::new();
        let mut proposal_ids = BTreeSet::new();
        for candidate in &self.candidates {
            candidate.validate()?;
            if !candidate_ids.insert(candidate.candidate_id.clone()) {
                return Err(anyhow!(
                    "narrative candidate set contains duplicate candidateId: {}",
                    candidate.candidate_id
                ));
            }
            if !proposal_ids.insert(candidate.proposal.proposal.id.clone()) {
                return Err(anyhow!(
                    "narrative candidate set contains duplicate proposal id: {}",
                    candidate.proposal.proposal.id
                ));
            }
        }
        Ok(())
    }
}

impl NarrativeCandidate {
    pub fn validate(&self) -> Result<()> {
        crate::require_text("narrative candidate candidateId", &self.candidate_id)?;
        crate::require_text("narrative candidate class", &self.class)?;
        require_human_owned_text("narrative candidate humanIntent", &self.human_intent)?;
        require_human_owned_text(
            "narrative candidate compatibilityNotes",
            &self.compatibility_notes,
        )?;
        crate::require_text("narrative candidate sourceBriefRef", &self.source_brief_ref)?;
        if !ALLOWED_CANDIDATE_CLASSES.contains(&self.class.as_str()) {
            return Err(anyhow!("narrative candidate class is unsupported"));
        }
        if self.payload_hash.len() != 64
            || !self.payload_hash.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(anyhow!(
                "narrative candidate payloadHash must be a 64-character hex digest"
            ));
        }
        if !self.proposal_only || self.human_tone_boundary != NARRATIVE_TONE_HUMAN_BOUNDARY {
            return Err(anyhow!(
                "narrative candidate must remain proposal-only with the human tone/soul boundary"
            ));
        }
        self.proposal.validate()?;
        if self.source_brief_ref != self.proposal.provenance.brief_id {
            return Err(anyhow!(
                "narrative candidate sourceBriefRef must match wrapped proposal provenance briefId"
            ));
        }
        if self.payload_hash != sha256_hex(self.proposal.proposal.to.as_bytes()) {
            return Err(anyhow!(
                "narrative candidate payloadHash must match wrapped proposal payload"
            ));
        }
        if self.proposal.proposal.status != "proposed"
            || self.proposal.proposal.verdict_status != "pending"
            || self.proposal.proposal.confidence != "unverified"
            || !self.proposal.provenance.proposal_only
        {
            return Err(anyhow!(
                "narrative candidate proposal must remain unverified, pending, and proposal-only"
            ));
        }
        Ok(())
    }
}

pub fn generate_narrative_candidates(
    brief: &NarrativeCandidateBrief,
    now_unix_ms: u128,
) -> Result<NarrativeCandidateSet> {
    brief.validate()?;

    let mut candidates = Vec::with_capacity(brief.candidate_count);
    for request in &brief.candidates {
        let proposal = match request.game_class.as_str() {
            GRID_PUZZLE_GAME_CLASS => intake_brief(
                request.grid_puzzle.as_ref().ok_or_else(|| {
                    anyhow!("narrative candidate request missing gridPuzzle brief")
                })?,
                now_unix_ms,
            )
            .map_err(|err| anyhow!("narrative candidate \"{}\": {err}", request.candidate_id))?,
            DECK_ROGUELIKE_GAME_CLASS => intake_deck_roguelike_brief(
                request.deck_roguelike.as_ref().ok_or_else(|| {
                    anyhow!("narrative candidate request missing deckRoguelike brief")
                })?,
                now_unix_ms,
            )
            .map_err(|err| anyhow!("narrative candidate \"{}\": {err}", request.candidate_id))?,
            other => {
                return Err(anyhow!(
                    "narrative candidate gameClass \"{other}\" is unsupported"
                ))
            }
        };

        candidates.push(NarrativeCandidate {
            candidate_id: request.candidate_id.clone(),
            class: request.class.clone(),
            human_intent: request.human_intent.clone(),
            compatibility_notes: request.compatibility_notes.clone(),
            source_brief_ref: proposal.provenance.brief_id.clone(),
            payload_hash: sha256_hex(proposal.proposal.to.as_bytes()),
            proposal,
            proposal_only: true,
            human_tone_boundary: NARRATIVE_TONE_HUMAN_BOUNDARY.to_string(),
        });
    }

    let set = NarrativeCandidateSet {
        schema_version: NARRATIVE_CANDIDATE_SCHEMA_VERSION.to_string(),
        candidate_set_id: brief.candidate_set_id.clone(),
        generator: NARRATIVE_CANDIDATE_GENERATOR.to_string(),
        story_id: brief.narrative_surface.story_id.clone(),
        phase_shift: brief.phase_shift.clone(),
        candidate_count: brief.candidate_count,
        source_refs: brief.source_refs.clone(),
        candidates,
        proposal_only: true,
        human_tone_boundary: NARRATIVE_TONE_HUMAN_BOUNDARY.to_string(),
    };
    set.validate()?;
    Ok(set)
}

fn require_human_owned_text(label: &str, text: &str) -> Result<()> {
    crate::require_text(label, text)?;
    let lower = text.to_ascii_lowercase();
    for claim in FORBIDDEN_AUTOMATED_CLAIMS {
        if lower.contains(claim) {
            return Err(anyhow!(
                "{label} contains automated tone/quality claim \"{claim}\"; tone/soul/fun remain human judgments"
            ));
        }
    }
    Ok(())
}

fn validate_source_ref(source_ref: &str) -> Result<()> {
    crate::require_text("narrative candidate sourceRef", source_ref)?;
    if source_ref.starts_with('/') || source_ref.contains("..") || source_ref.contains('\\') {
        return Err(anyhow!(
            "narrative candidate sourceRef must be a safe relative evidence/doc reference"
        ));
    }
    Ok(())
}
