//! Human-Curated Narrative Integration v1 (#1865).
//!
//! This module records a human-selected narrative candidate and prepares a
//! read-only integration provenance record. It reuses the Narrative/Theme-Arc
//! Candidate Generation v1 output plus the same read-only curation boundary
//! pattern as Milestone 57. It does not write trusted title state, apply a
//! proposal, approve its own output, or bypass review/apply/trust-gradient.

use crate::curation_surface::CURATION_READ_ONLY_BOUNDARY;
use crate::narrative_candidate::{
    NarrativeCandidate, NarrativeCandidateSet, NARRATIVE_TONE_HUMAN_BOUNDARY,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const NARRATIVE_INTEGRATION_SCHEMA_VERSION: &str = "ouroforge.narrative-integration.v1";
pub const NARRATIVE_INTEGRATION_BOUNDARY: &str = "read-only-narrative-integration-provenance";
pub const NARRATIVE_INTEGRATION_STATUS_READY: &str = "ready-for-review-apply";

const ALLOWED_DECISIONS: &[&str] = &[
    "selected",
    "rejected",
    "deferred",
    "needs-rework",
    "superseded",
    "no-selection",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeIntegrationSelectionRecord {
    pub schema_version: String,
    pub selection_id: String,
    pub candidate_set_id: String,
    pub target_title_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_candidate_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_proposal_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_payload_hash: Option<String>,
    pub decision: String,
    pub human_actor: String,
    pub rationale: String,
    pub recorded_at_unix_ms: u128,
    pub surface_boundary: String,
    pub trusted_write_requested: bool,
    pub apply_authority: bool,
    pub review_apply_required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeIntegrationRecord {
    pub schema_version: String,
    pub integration_id: String,
    pub target_title_id: String,
    pub candidate_set_id: String,
    pub selected_candidate_id: String,
    pub selected_proposal_id: String,
    pub selected_payload_hash: String,
    pub selected_proposal_path: String,
    pub selection_id: String,
    pub human_actor: String,
    pub rationale: String,
    pub status: String,
    pub review_apply_required: bool,
    pub surface_boundary: String,
    pub curation_boundary: String,
    pub trusted_write_authority: bool,
    pub human_tone_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NarrativeIntegrationReadModel {
    pub schema_version: String,
    pub target_title_id: String,
    pub candidate_set_id: String,
    pub selected_candidate_id: String,
    pub status: String,
    pub review_apply_required: bool,
    pub surface_boundary: String,
    pub trusted_write_authority: bool,
    pub allowed_actions: Vec<String>,
    pub human_tone_boundary: String,
}

impl NarrativeIntegrationSelectionRecord {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text).map_err(|err| {
            anyhow!("narrative integration selection record is not valid JSON: {err}")
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_INTEGRATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "narrative integration selection schemaVersion must be \"{NARRATIVE_INTEGRATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "narrative integration selection selectionId",
            &self.selection_id,
        )?;
        crate::require_text(
            "narrative integration selection candidateSetId",
            &self.candidate_set_id,
        )?;
        crate::require_text(
            "narrative integration selection targetTitleId",
            &self.target_title_id,
        )?;
        crate::require_text("narrative integration selection decision", &self.decision)?;
        if !ALLOWED_DECISIONS.contains(&self.decision.as_str()) {
            return Err(anyhow!(
                "narrative integration selection decision \"{}\" is unsupported",
                self.decision
            ));
        }
        crate::require_text(
            "narrative integration selection humanActor",
            &self.human_actor,
        )?;
        crate::require_text("narrative integration selection rationale", &self.rationale)?;
        if self.recorded_at_unix_ms == 0 {
            return Err(anyhow!(
                "narrative integration selection recordedAtUnixMs must be greater than zero"
            ));
        }
        if self.surface_boundary != NARRATIVE_INTEGRATION_BOUNDARY {
            return Err(anyhow!(
                "narrative integration selection surfaceBoundary must be \"{NARRATIVE_INTEGRATION_BOUNDARY}\""
            ));
        }
        if self.trusted_write_requested || self.apply_authority || !self.review_apply_required {
            return Err(anyhow!(
                "narrative integration selection is read-only provenance and cannot request trusted write/apply authority"
            ));
        }
        if self.decision == "selected" {
            require_optional_text(
                "narrative integration selection selectedCandidateId",
                &self.selected_candidate_id,
            )?;
            require_optional_text(
                "narrative integration selection selectedProposalId",
                &self.selected_proposal_id,
            )?;
            let digest = require_optional_text(
                "narrative integration selection selectedPayloadHash",
                &self.selected_payload_hash,
            )?;
            validate_digest(
                "narrative integration selection selectedPayloadHash",
                digest,
            )?;
        }
        Ok(())
    }
}

impl NarrativeIntegrationRecord {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("narrative integration record is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_INTEGRATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "narrative integration record schemaVersion must be \"{NARRATIVE_INTEGRATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "narrative integration record integrationId",
            &self.integration_id,
        )?;
        crate::require_text(
            "narrative integration record targetTitleId",
            &self.target_title_id,
        )?;
        crate::require_text(
            "narrative integration record candidateSetId",
            &self.candidate_set_id,
        )?;
        crate::require_text(
            "narrative integration record selectedCandidateId",
            &self.selected_candidate_id,
        )?;
        crate::require_text(
            "narrative integration record selectedProposalId",
            &self.selected_proposal_id,
        )?;
        validate_digest(
            "narrative integration record selectedPayloadHash",
            &self.selected_payload_hash,
        )?;
        crate::require_text(
            "narrative integration record selectedProposalPath",
            &self.selected_proposal_path,
        )?;
        if self.selected_proposal_path.starts_with("crates/")
            || self.selected_proposal_path.starts_with("docs/")
            || self.selected_proposal_path.starts_with("examples/")
            || self.selected_proposal_path.starts_with('/')
            || self.selected_proposal_path.contains("..")
        {
            return Err(anyhow!(
                "narrative integration record selectedProposalPath must not be a trusted source write target"
            ));
        }
        crate::require_text(
            "narrative integration record selectionId",
            &self.selection_id,
        )?;
        crate::require_text("narrative integration record humanActor", &self.human_actor)?;
        crate::require_text("narrative integration record rationale", &self.rationale)?;
        if self.status != NARRATIVE_INTEGRATION_STATUS_READY {
            return Err(anyhow!(
                "narrative integration record status must be \"{NARRATIVE_INTEGRATION_STATUS_READY}\""
            ));
        }
        if !self.review_apply_required
            || self.surface_boundary != NARRATIVE_INTEGRATION_BOUNDARY
            || self.curation_boundary != CURATION_READ_ONLY_BOUNDARY
            || self.trusted_write_authority
            || self.human_tone_boundary != NARRATIVE_TONE_HUMAN_BOUNDARY
        {
            return Err(anyhow!(
                "narrative integration record must stay read-only, review/apply-gated, and human-tone bounded"
            ));
        }
        Ok(())
    }

    pub fn read_model(&self) -> Result<NarrativeIntegrationReadModel> {
        self.validate()?;
        let model = NarrativeIntegrationReadModel {
            schema_version: self.schema_version.clone(),
            target_title_id: self.target_title_id.clone(),
            candidate_set_id: self.candidate_set_id.clone(),
            selected_candidate_id: self.selected_candidate_id.clone(),
            status: self.status.clone(),
            review_apply_required: self.review_apply_required,
            surface_boundary: self.surface_boundary.clone(),
            trusted_write_authority: self.trusted_write_authority,
            allowed_actions: vec![
                "inspect-selected-candidate".to_string(),
                "route-through-review-apply".to_string(),
            ],
            human_tone_boundary: self.human_tone_boundary.clone(),
        };
        model.validate()?;
        Ok(model)
    }
}

impl NarrativeIntegrationReadModel {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != NARRATIVE_INTEGRATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "narrative integration read model schemaVersion must be \"{NARRATIVE_INTEGRATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text(
            "narrative integration read model targetTitleId",
            &self.target_title_id,
        )?;
        crate::require_text(
            "narrative integration read model candidateSetId",
            &self.candidate_set_id,
        )?;
        crate::require_text(
            "narrative integration read model selectedCandidateId",
            &self.selected_candidate_id,
        )?;
        if self.status != NARRATIVE_INTEGRATION_STATUS_READY
            || !self.review_apply_required
            || self.surface_boundary != NARRATIVE_INTEGRATION_BOUNDARY
            || self.trusted_write_authority
            || self.human_tone_boundary != NARRATIVE_TONE_HUMAN_BOUNDARY
        {
            return Err(anyhow!(
                "narrative integration read model must remain read-only and review/apply-gated"
            ));
        }
        let actions: BTreeSet<_> = self.allowed_actions.iter().map(String::as_str).collect();
        let expected: BTreeSet<_> = ["inspect-selected-candidate", "route-through-review-apply"]
            .into_iter()
            .collect();
        if actions != expected {
            return Err(anyhow!(
                "narrative integration read model allowedActions must be read-only/review-apply only"
            ));
        }
        Ok(())
    }
}

pub fn record_human_narrative_selection(
    set: &NarrativeCandidateSet,
    selection_id: &str,
    target_title_id: &str,
    selected_candidate_id: &str,
    human_actor: &str,
    rationale: &str,
    recorded_at_unix_ms: u128,
) -> Result<NarrativeIntegrationSelectionRecord> {
    set.validate()?;
    crate::require_text(
        "narrative integration selectedCandidateId",
        selected_candidate_id,
    )?;
    let candidate = find_candidate(set, selected_candidate_id)?;
    let record = NarrativeIntegrationSelectionRecord {
        schema_version: NARRATIVE_INTEGRATION_SCHEMA_VERSION.to_string(),
        selection_id: selection_id.to_string(),
        candidate_set_id: set.candidate_set_id.clone(),
        target_title_id: target_title_id.to_string(),
        selected_candidate_id: Some(candidate.candidate_id.clone()),
        selected_proposal_id: Some(candidate.proposal.proposal.id.clone()),
        selected_payload_hash: Some(candidate.payload_hash.clone()),
        decision: "selected".to_string(),
        human_actor: human_actor.to_string(),
        rationale: rationale.to_string(),
        recorded_at_unix_ms,
        surface_boundary: NARRATIVE_INTEGRATION_BOUNDARY.to_string(),
        trusted_write_requested: false,
        apply_authority: false,
        review_apply_required: true,
    };
    record.validate()?;
    Ok(record)
}

pub fn replay_narrative_selection<'a>(
    set: &'a NarrativeCandidateSet,
    selection: &NarrativeIntegrationSelectionRecord,
) -> Result<&'a NarrativeCandidate> {
    set.validate()?;
    selection.validate()?;
    if selection.candidate_set_id != set.candidate_set_id {
        return Err(anyhow!(
            "narrative integration selection candidateSetId does not match set"
        ));
    }
    if selection.decision != "selected" {
        return Err(anyhow!(
            "narrative integration replay requires a selected decision"
        ));
    }
    let candidate_id = selection
        .selected_candidate_id
        .as_deref()
        .ok_or_else(|| anyhow!("narrative integration selection missing selectedCandidateId"))?;
    let candidate = find_candidate(set, candidate_id)?;
    let proposal_id = selection
        .selected_proposal_id
        .as_deref()
        .ok_or_else(|| anyhow!("narrative integration selection missing selectedProposalId"))?;
    if proposal_id != candidate.proposal.proposal.id {
        return Err(anyhow!(
            "narrative integration selection selectedProposalId is stale"
        ));
    }
    let payload_hash = selection
        .selected_payload_hash
        .as_deref()
        .ok_or_else(|| anyhow!("narrative integration selection missing selectedPayloadHash"))?;
    if payload_hash != candidate.payload_hash {
        return Err(anyhow!(
            "narrative integration selection selectedPayloadHash is stale"
        ));
    }
    Ok(candidate)
}

pub fn integrate_human_selected_candidate(
    set: &NarrativeCandidateSet,
    selection: &NarrativeIntegrationSelectionRecord,
    integration_id: &str,
) -> Result<NarrativeIntegrationRecord> {
    let candidate = replay_narrative_selection(set, selection)?;
    crate::require_text("narrative integration integrationId", integration_id)?;
    let record = NarrativeIntegrationRecord {
        schema_version: NARRATIVE_INTEGRATION_SCHEMA_VERSION.to_string(),
        integration_id: integration_id.to_string(),
        target_title_id: selection.target_title_id.clone(),
        candidate_set_id: set.candidate_set_id.clone(),
        selected_candidate_id: candidate.candidate_id.clone(),
        selected_proposal_id: candidate.proposal.proposal.id.clone(),
        selected_payload_hash: candidate.payload_hash.clone(),
        selected_proposal_path: candidate.proposal.proposal.path.clone(),
        selection_id: selection.selection_id.clone(),
        human_actor: selection.human_actor.clone(),
        rationale: selection.rationale.clone(),
        status: NARRATIVE_INTEGRATION_STATUS_READY.to_string(),
        review_apply_required: true,
        surface_boundary: NARRATIVE_INTEGRATION_BOUNDARY.to_string(),
        curation_boundary: CURATION_READ_ONLY_BOUNDARY.to_string(),
        trusted_write_authority: false,
        human_tone_boundary: NARRATIVE_TONE_HUMAN_BOUNDARY.to_string(),
    };
    record.validate()?;
    Ok(record)
}

fn find_candidate<'a>(
    set: &'a NarrativeCandidateSet,
    selected_candidate_id: &str,
) -> Result<&'a NarrativeCandidate> {
    set.candidates
        .iter()
        .find(|candidate| candidate.candidate_id == selected_candidate_id)
        .ok_or_else(|| anyhow!("narrative integration references unknown candidate"))
}

fn require_optional_text<'a>(label: &str, value: &'a Option<String>) -> Result<&'a str> {
    let text = value
        .as_deref()
        .ok_or_else(|| anyhow!("{label} is required"))?;
    crate::require_text(label, text)?;
    Ok(text)
}

fn validate_digest(label: &str, digest: &str) -> Result<()> {
    crate::require_text(label, digest)?;
    if digest.len() != 64 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!("{label} must be a 64-character hex digest"));
    }
    Ok(())
}
