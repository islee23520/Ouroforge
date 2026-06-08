//! Read-Only Curation Surface v1 (#1853).
//!
//! Part of Candidate Generation and Curation Cockpit v1 (#1851) under #1 Era J
//! Milestone 57. This module models the Rust/local provenance record behind a
//! read-only curation surface: it can validate that a human selected a generated
//! candidate, and it can replay that selection against the candidate set. It does
//! not apply, promote, or write trusted source/project state.

use crate::candidate_generation::{CandidateProposal, CandidateProposalSet};
use crate::export_hash::sha256_hex;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Schema version for human curation selection provenance.
pub const CURATION_SELECTION_SCHEMA_VERSION: &str = "ouroforge.curation-selection.v1";
/// Required boundary label for curation records. It is intentionally explicit so
/// fixtures/read models can fail closed if a surface drifts into command/write
/// authority.
pub const CURATION_READ_ONLY_BOUNDARY: &str = "read-only-provenance";

const ALLOWED_DECISIONS: &[&str] = &[
    "selected",
    "rejected",
    "deferred",
    "needs-rework",
    "superseded",
    "no-selection",
];

/// Human selection provenance for one candidate set. This is evidence about a
/// human curation decision; it confers no apply or promotion authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CurationSelectionRecord {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "selectionId")]
    pub selection_id: String,
    #[serde(rename = "candidateSetId")]
    pub candidate_set_id: String,
    #[serde(rename = "selectedVariantId")]
    pub selected_variant_id: Option<String>,
    #[serde(rename = "selectedProposalId")]
    pub selected_proposal_id: Option<String>,
    #[serde(rename = "selectedPayloadDigest")]
    pub selected_payload_digest: Option<String>,
    pub decision: String,
    #[serde(rename = "humanActor")]
    pub human_actor: String,
    pub rationale: String,
    #[serde(rename = "recordedAtUnixMs")]
    pub recorded_at_unix_ms: u128,
    #[serde(rename = "surfaceBoundary")]
    pub surface_boundary: String,
    #[serde(rename = "trustedWriteRequested")]
    pub trusted_write_requested: bool,
    #[serde(rename = "applyAuthority")]
    pub apply_authority: bool,
}

/// Read-only curation surface projection for dashboard/cockpit consumers.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CurationReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "candidateSetId")]
    pub candidate_set_id: String,
    #[serde(rename = "candidateCount")]
    pub candidate_count: usize,
    pub candidates: Vec<CurationCandidateSummary>,
    pub selections: Vec<CurationSelectionRecord>,
    #[serde(rename = "surfaceBoundary")]
    pub surface_boundary: String,
    #[serde(rename = "trustedWriteAuthority")]
    pub trusted_write_authority: bool,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
}

/// Display summary for one candidate. Proposal payload remains in the candidate
/// set; the surface displays ids/digests/status for read-only inspection.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CurationCandidateSummary {
    #[serde(rename = "variantId")]
    pub variant_id: String,
    pub kind: String,
    pub summary: String,
    #[serde(rename = "proposalId")]
    pub proposal_id: String,
    #[serde(rename = "payloadDigest")]
    pub payload_digest: String,
    pub status: String,
    #[serde(rename = "verdictStatus")]
    pub verdict_status: String,
}

impl CurationSelectionRecord {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("curation selection record is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CURATION_SELECTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "curation selection schemaVersion must be \"{CURATION_SELECTION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("curation selection selectionId", &self.selection_id)?;
        crate::require_text("curation selection candidateSetId", &self.candidate_set_id)?;
        crate::require_text("curation selection decision", &self.decision)?;
        if !ALLOWED_DECISIONS.contains(&self.decision.as_str()) {
            return Err(anyhow!(
                "curation selection decision \"{}\" is unsupported",
                self.decision
            ));
        }
        crate::require_text("curation selection humanActor", &self.human_actor)?;
        crate::require_text("curation selection rationale", &self.rationale)?;
        if self.recorded_at_unix_ms == 0 {
            return Err(anyhow!(
                "curation selection recordedAtUnixMs must be greater than zero"
            ));
        }
        if self.surface_boundary != CURATION_READ_ONLY_BOUNDARY {
            return Err(anyhow!(
                "curation selection surfaceBoundary must be \"{CURATION_READ_ONLY_BOUNDARY}\""
            ));
        }
        if self.trusted_write_requested || self.apply_authority {
            return Err(anyhow!(
                "curation selection is read-only provenance and cannot request trusted write/apply authority"
            ));
        }
        if self.decision == "selected" {
            require_optional_text(
                "curation selection selectedVariantId",
                &self.selected_variant_id,
            )?;
            require_optional_text(
                "curation selection selectedProposalId",
                &self.selected_proposal_id,
            )?;
            let digest = require_optional_text(
                "curation selection selectedPayloadDigest",
                &self.selected_payload_digest,
            )?;
            validate_digest("curation selection selectedPayloadDigest", digest)?;
        }
        Ok(())
    }
}

impl CurationReadModel {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CURATION_SELECTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "curation read model schemaVersion must be \"{CURATION_SELECTION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("curation read model candidateSetId", &self.candidate_set_id)?;
        if self.candidate_count == 0 || self.candidate_count != self.candidates.len() {
            return Err(anyhow!(
                "curation read model candidateCount must equal candidates length"
            ));
        }
        if self.surface_boundary != CURATION_READ_ONLY_BOUNDARY || self.trusted_write_authority {
            return Err(anyhow!(
                "curation read model must remain read-only and without trusted write authority"
            ));
        }
        let actions: BTreeSet<_> = self.allowed_actions.iter().map(String::as_str).collect();
        let expected: BTreeSet<_> = ["inspect-candidates", "record-selection-provenance"]
            .into_iter()
            .collect();
        if actions != expected {
            return Err(anyhow!(
                "curation read model allowedActions must be read-only/provenance-only"
            ));
        }
        for candidate in &self.candidates {
            candidate.validate()?;
        }
        for selection in &self.selections {
            selection.validate()?;
        }
        Ok(())
    }
}

impl CurationCandidateSummary {
    pub fn validate(&self) -> Result<()> {
        crate::require_text("curation candidate variantId", &self.variant_id)?;
        crate::require_text("curation candidate kind", &self.kind)?;
        crate::require_text("curation candidate summary", &self.summary)?;
        crate::require_text("curation candidate proposalId", &self.proposal_id)?;
        validate_digest("curation candidate payloadDigest", &self.payload_digest)?;
        if self.status != "proposed" || self.verdict_status != "pending" {
            return Err(anyhow!(
                "curation candidate summary must describe proposed/pending candidates only"
            ));
        }
        Ok(())
    }
}

/// Build a selection record from a human choice. This creates provenance data in
/// memory; callers decide where/how to persist fixture-scoped evidence. It never
/// applies the selected proposal.
pub fn record_human_selection(
    set: &CandidateProposalSet,
    selection_id: &str,
    selected_variant_id: &str,
    human_actor: &str,
    rationale: &str,
    recorded_at_unix_ms: u128,
) -> Result<CurationSelectionRecord> {
    set.validate()?;
    crate::require_text("curation selection selectedVariantId", selected_variant_id)?;
    let candidate = find_candidate(set, selected_variant_id)?;
    let record = CurationSelectionRecord {
        schema_version: CURATION_SELECTION_SCHEMA_VERSION.to_string(),
        selection_id: selection_id.to_string(),
        candidate_set_id: set.candidate_set_id.clone(),
        selected_variant_id: Some(candidate.variant_id.clone()),
        selected_proposal_id: Some(candidate.proposal.proposal.id.clone()),
        selected_payload_digest: Some(candidate_payload_digest(candidate)),
        decision: "selected".to_string(),
        human_actor: human_actor.to_string(),
        rationale: rationale.to_string(),
        recorded_at_unix_ms,
        surface_boundary: CURATION_READ_ONLY_BOUNDARY.to_string(),
        trusted_write_requested: false,
        apply_authority: false,
    };
    record.validate()?;
    Ok(record)
}

/// Validate and replay a selection record against a candidate set, returning the
/// selected candidate only when ids and payload digest still match.
pub fn replay_selection<'a>(
    set: &'a CandidateProposalSet,
    selection: &CurationSelectionRecord,
) -> Result<&'a CandidateProposal> {
    set.validate()?;
    selection.validate()?;
    if selection.candidate_set_id != set.candidate_set_id {
        return Err(anyhow!(
            "curation selection candidateSetId does not match set"
        ));
    }
    if selection.decision != "selected" {
        return Err(anyhow!(
            "curation selection replay requires a selected decision"
        ));
    }
    let variant_id = selection
        .selected_variant_id
        .as_deref()
        .ok_or_else(|| anyhow!("curation selection missing selectedVariantId"))?;
    let candidate = find_candidate(set, variant_id)?;
    let proposal_id = selection
        .selected_proposal_id
        .as_deref()
        .ok_or_else(|| anyhow!("curation selection missing selectedProposalId"))?;
    if proposal_id != candidate.proposal.proposal.id {
        return Err(anyhow!("curation selection selectedProposalId is stale"));
    }
    let digest = selection
        .selected_payload_digest
        .as_deref()
        .ok_or_else(|| anyhow!("curation selection missing selectedPayloadDigest"))?;
    if digest != candidate_payload_digest(candidate) {
        return Err(anyhow!("curation selection selectedPayloadDigest is stale"));
    }
    Ok(candidate)
}

/// Build the read-only dashboard/cockpit projection. Selection records are
/// validated/replayed so stale or unsafe records fail closed.
pub fn build_curation_read_model(
    set: &CandidateProposalSet,
    selections: &[CurationSelectionRecord],
) -> Result<CurationReadModel> {
    set.validate()?;
    for selection in selections {
        replay_selection(set, selection)?;
    }
    let model = CurationReadModel {
        schema_version: CURATION_SELECTION_SCHEMA_VERSION.to_string(),
        candidate_set_id: set.candidate_set_id.clone(),
        candidate_count: set.candidates.len(),
        candidates: set
            .candidates
            .iter()
            .map(|candidate| CurationCandidateSummary {
                variant_id: candidate.variant_id.clone(),
                kind: candidate.kind.clone(),
                summary: candidate.summary.clone(),
                proposal_id: candidate.proposal.proposal.id.clone(),
                payload_digest: candidate_payload_digest(candidate),
                status: candidate.proposal.proposal.status.clone(),
                verdict_status: candidate.proposal.proposal.verdict_status.clone(),
            })
            .collect(),
        selections: selections.to_vec(),
        surface_boundary: CURATION_READ_ONLY_BOUNDARY.to_string(),
        trusted_write_authority: false,
        allowed_actions: vec![
            "inspect-candidates".to_string(),
            "record-selection-provenance".to_string(),
        ],
    };
    model.validate()?;
    Ok(model)
}

fn find_candidate<'a>(
    set: &'a CandidateProposalSet,
    selected_variant_id: &str,
) -> Result<&'a CandidateProposal> {
    set.candidates
        .iter()
        .find(|candidate| candidate.variant_id == selected_variant_id)
        .ok_or_else(|| anyhow!("curation selection references unknown candidate variant"))
}

fn candidate_payload_digest(candidate: &CandidateProposal) -> String {
    sha256_hex(candidate.proposal.proposal.to.as_bytes())
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
