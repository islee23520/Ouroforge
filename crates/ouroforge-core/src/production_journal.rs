//! Dogfood production journal schema (#2388 / M129.1).
//!
//! This is an extension of the M127 evolve iteration journal family, not a
//! second parallel journal system. Entries keep production decisions, proposals,
//! applied diffs, evidence refs, playtest notes, human fun/feel decisions, and
//! unresolved gaps audit-friendly and append-only.

use crate::evolve_iteration_journal::EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCTION_JOURNAL_SCHEMA_VERSION: &str = "production-journal-v1";
pub const PRODUCTION_JOURNAL_FAMILY: &str = EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProductionJournalEntryKind {
    Decision,
    AgentProposal,
    AppliedDiff,
    EvidenceRef,
    PlaytestNote,
    HumanFunFeelDecision,
    UnresolvedGap,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionJournalEntry {
    #[serde(rename = "entryId")]
    pub entry_id: String,
    pub kind: ProductionJournalEntryKind,
    pub summary: String,
    pub rationale: String,
    #[serde(rename = "issueRefs")]
    pub issue_refs: Vec<String>,
    #[serde(rename = "prRefs")]
    pub pr_refs: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(
        rename = "proposalRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proposal_ref: Option<String>,
    #[serde(rename = "reviewRef", default, skip_serializing_if = "Option::is_none")]
    pub review_ref: Option<String>,
    #[serde(
        rename = "appliedDiffRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub applied_diff_ref: Option<String>,
    #[serde(rename = "humanDecision", default)]
    pub human_decision: bool,
    #[serde(rename = "unresolvedGap", default)]
    pub unresolved_gap: bool,
    #[serde(rename = "nextAction")]
    pub next_action: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionJournal {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "schemaFamily")]
    pub schema_family: String,
    #[serde(rename = "journalId")]
    pub journal_id: String,
    #[serde(rename = "sourceEvolveJournalRef")]
    pub source_evolve_journal_ref: String,
    pub entries: Vec<ProductionJournalEntry>,
    pub guardrails: Vec<String>,
}

impl ProductionJournal {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let journal: Self = serde_json::from_str(input).context("parse production journal JSON")?;
        journal.validate()?;
        Ok(journal)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_JOURNAL_SCHEMA_VERSION {
            return Err(anyhow!(
                "production journal schemaVersion must be {PRODUCTION_JOURNAL_SCHEMA_VERSION}"
            ));
        }
        if self.schema_family != PRODUCTION_JOURNAL_FAMILY {
            return Err(anyhow!(
                "production journal must reuse M127 schema family {PRODUCTION_JOURNAL_FAMILY}"
            ));
        }
        require_id("production journal journalId", &self.journal_id)?;
        require_ref(
            "production journal sourceEvolveJournalRef",
            &self.source_evolve_journal_ref,
        )?;
        if self.entries.is_empty() {
            return Err(anyhow!("production journal entries must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !ids.insert(entry.entry_id.as_str()) {
                return Err(anyhow!(
                    "production journal duplicate entryId `{}`",
                    entry.entry_id
                ));
            }
        }
        if !self
            .guardrails
            .iter()
            .any(|guardrail| guardrail.contains("no second journal"))
        {
            return Err(anyhow!(
                "production journal guardrails must forbid a second journal family"
            ));
        }
        Ok(())
    }

    pub fn validate_is_append_of(&self, previous: &Self) -> Result<()> {
        if self.journal_id != previous.journal_id || self.schema_family != previous.schema_family {
            return Err(anyhow!(
                "production journal identity/family changed; not an append"
            ));
        }
        if self.entries.len() < previous.entries.len() {
            return Err(anyhow!("production journal lost entries"));
        }
        for (index, prior) in previous.entries.iter().enumerate() {
            if &self.entries[index] != prior {
                return Err(anyhow!(
                    "production journal rewrote entry {index}; history is append-only"
                ));
            }
        }
        Ok(())
    }
}

impl ProductionJournalEntry {
    pub fn validate(&self) -> Result<()> {
        require_id("production journal entry entryId", &self.entry_id)?;
        for (field, value) in [
            ("summary", &self.summary),
            ("rationale", &self.rationale),
            ("nextAction", &self.next_action),
        ] {
            require_text(&format!("production journal entry {field}"), value)?;
        }
        validate_refs("production journal entry issueRefs", &self.issue_refs, true)?;
        validate_refs(
            "production journal entry evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        validate_refs("production journal entry prRefs", &self.pr_refs, false)?;
        for (field, value) in [
            ("proposalRef", &self.proposal_ref),
            ("reviewRef", &self.review_ref),
            ("appliedDiffRef", &self.applied_diff_ref),
        ] {
            if let Some(reference) = value {
                require_ref(&format!("production journal entry {field}"), reference)?;
            }
        }
        if self.applied_diff_ref.is_some() && self.review_ref.is_none() {
            return Err(anyhow!(
                "production journal appliedDiffRef requires reviewRef"
            ));
        }
        if matches!(self.kind, ProductionJournalEntryKind::HumanFunFeelDecision)
            && !self.human_decision
        {
            return Err(anyhow!(
                "human fun/feel journal entries must be marked humanDecision"
            ));
        }
        if matches!(self.kind, ProductionJournalEntryKind::UnresolvedGap) && !self.unresolved_gap {
            return Err(anyhow!(
                "unresolved gap journal entries must be marked unresolvedGap"
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
