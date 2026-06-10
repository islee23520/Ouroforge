//! Agent/human production decision log integration (#2389 / M129.2).
//!
//! Integrates proposal workbench decisions and human creative rationale into the
//! production journal without granting agent self-approval.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCTION_DECISION_LOG_SCHEMA_VERSION: &str = "production-decision-log-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProductionDecisionOutcome {
    Accepted,
    Rejected,
    Deferred,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecisionRecord {
    #[serde(rename = "decisionId")]
    pub decision_id: String,
    #[serde(rename = "journalEntryRef")]
    pub journal_entry_ref: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    pub outcome: ProductionDecisionOutcome,
    #[serde(rename = "agentSuggested")]
    pub agent_suggested: bool,
    #[serde(rename = "humanReviewer")]
    pub human_reviewer: String,
    #[serde(rename = "humanRationale")]
    pub human_rationale: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "selfApproved", default)]
    pub self_approved: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecisionLogReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "logId")]
    pub log_id: String,
    #[serde(rename = "decisionCount")]
    pub decision_count: usize,
    #[serde(rename = "acceptedCount")]
    pub accepted_count: usize,
    #[serde(rename = "rejectedCount")]
    pub rejected_count: usize,
    #[serde(rename = "deferredCount")]
    pub deferred_count: usize,
    #[serde(rename = "humanReviewerCount")]
    pub human_reviewer_count: usize,
    #[serde(rename = "inspectionRows")]
    pub inspection_rows: Vec<ProductionDecisionInspectionRow>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecisionInspectionRow {
    #[serde(rename = "decisionId")]
    pub decision_id: String,
    pub outcome: ProductionDecisionOutcome,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "journalEntryRef")]
    pub journal_entry_ref: String,
    #[serde(rename = "humanReviewer")]
    pub human_reviewer: String,
    #[serde(rename = "evidenceCount")]
    pub evidence_count: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionDecisionLog {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "logId")]
    pub log_id: String,
    #[serde(rename = "productionJournalRef")]
    pub production_journal_ref: String,
    pub decisions: Vec<ProductionDecisionRecord>,
}

impl ProductionDecisionLog {
    pub fn read_model(&self) -> ProductionDecisionLogReadModel {
        let reviewers: BTreeSet<_> = self
            .decisions
            .iter()
            .map(|decision| decision.human_reviewer.clone())
            .collect();
        ProductionDecisionLogReadModel {
            schema_version: PRODUCTION_DECISION_LOG_SCHEMA_VERSION.to_string(),
            log_id: self.log_id.clone(),
            decision_count: self.decisions.len(),
            accepted_count: self
                .decisions
                .iter()
                .filter(|d| d.outcome == ProductionDecisionOutcome::Accepted)
                .count(),
            rejected_count: self
                .decisions
                .iter()
                .filter(|d| d.outcome == ProductionDecisionOutcome::Rejected)
                .count(),
            deferred_count: self
                .decisions
                .iter()
                .filter(|d| d.outcome == ProductionDecisionOutcome::Deferred)
                .count(),
            human_reviewer_count: reviewers.len(),
            inspection_rows: self
                .decisions
                .iter()
                .map(|decision| ProductionDecisionInspectionRow {
                    decision_id: decision.decision_id.clone(),
                    outcome: decision.outcome.clone(),
                    proposal_ref: decision.proposal_ref.clone(),
                    journal_entry_ref: decision.journal_entry_ref.clone(),
                    human_reviewer: decision.human_reviewer.clone(),
                    evidence_count: decision.evidence_refs.len(),
                })
                .collect(),
            forbidden_actions: vec![
                "agent_self_approval".to_string(),
                "auto_apply".to_string(),
                "hide_rejected_proposal".to_string(),
            ],
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_DECISION_LOG_SCHEMA_VERSION {
            return Err(anyhow!("production decision log schemaVersion must be {PRODUCTION_DECISION_LOG_SCHEMA_VERSION}"));
        }
        require_id("production decision log logId", &self.log_id)?;
        require_ref(
            "production decision log productionJournalRef",
            &self.production_journal_ref,
        )?;
        if self.decisions.is_empty() {
            return Err(anyhow!(
                "production decision log decisions must not be empty"
            ));
        }
        let mut ids = BTreeSet::new();
        for decision in &self.decisions {
            decision.validate()?;
            if !ids.insert(decision.decision_id.as_str()) {
                return Err(anyhow!(
                    "production decision log duplicate decisionId `{}`",
                    decision.decision_id
                ));
            }
        }
        if !self
            .decisions
            .iter()
            .any(|decision| decision.outcome == ProductionDecisionOutcome::Accepted)
        {
            return Err(anyhow!(
                "production decision log must trace at least one accepted proposal"
            ));
        }
        if !self
            .decisions
            .iter()
            .any(|decision| decision.outcome == ProductionDecisionOutcome::Rejected)
        {
            return Err(anyhow!(
                "production decision log must trace at least one rejected proposal"
            ));
        }
        Ok(())
    }
}

impl ProductionDecisionRecord {
    pub fn validate(&self) -> Result<()> {
        require_id("production decision decisionId", &self.decision_id)?;
        require_ref(
            "production decision journalEntryRef",
            &self.journal_entry_ref,
        )?;
        require_ref("production decision proposalRef", &self.proposal_ref)?;
        require_text("production decision humanReviewer", &self.human_reviewer)?;
        require_text("production decision humanRationale", &self.human_rationale)?;
        validate_refs(
            "production decision evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        if self.self_approved {
            return Err(anyhow!("production decision cannot be agent self-approved"));
        }
        if !self.agent_suggested {
            return Err(anyhow!(
                "production decision must identify agent-side proposal provenance"
            ));
        }
        Ok(())
    }
}

fn validate_refs(field: &str, refs: &[String], require_nonempty: bool) -> Result<()> {
    if require_nonempty && refs.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for reference in refs {
        require_ref(field, reference)?;
    }
    Ok(())
}
fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} contains an unsafe ref"));
    }
    Ok(())
}
fn require_id(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must be a local id"));
    }
    Ok(())
}
fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
