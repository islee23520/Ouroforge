//! Append-only evolve iteration journal (#2382 / M127.2).
//!
//! Records each live evolve iteration from hypothesis through evidence,
//! proposal, review, apply, rerun, comparison, result, and next action. Applied
//! changes cannot omit review/apply/evidence links, and history cannot be
//! rewritten.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION: &str = "evolve-iteration-journal-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EvolveIterationOutcome {
    Accepted,
    Rejected,
    Regressed,
    Inconclusive,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveIterationEntry {
    #[serde(rename = "iterationId")]
    pub iteration_id: String,
    pub hypothesis: String,
    #[serde(rename = "beforeBundleRef")]
    pub before_bundle_ref: String,
    #[serde(rename = "proposalRef")]
    pub proposal_ref: String,
    #[serde(rename = "reviewRef", default, skip_serializing_if = "Option::is_none")]
    pub review_ref: Option<String>,
    #[serde(rename = "applyRef", default, skip_serializing_if = "Option::is_none")]
    pub apply_ref: Option<String>,
    #[serde(
        rename = "afterBundleRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_bundle_ref: Option<String>,
    #[serde(
        rename = "comparisonRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comparison_ref: Option<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    pub outcome: EvolveIterationOutcome,
    pub decision: String,
    #[serde(rename = "nextAction")]
    pub next_action: String,
    #[serde(rename = "appliedChange")]
    pub applied_change: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveIterationJournal {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "journalId")]
    pub journal_id: String,
    pub entries: Vec<EvolveIterationEntry>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvolveIterationJournalReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "journalId")]
    pub journal_id: String,
    #[serde(rename = "entryCount")]
    pub entry_count: usize,
    #[serde(rename = "appliedCount")]
    pub applied_count: usize,
    #[serde(rename = "inconclusiveCount")]
    pub inconclusive_count: usize,
    #[serde(rename = "appendOnly")]
    pub append_only: bool,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl EvolveIterationJournal {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse evolve iteration journal JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION {
            return Err(anyhow!(
                "evolve iteration journal schemaVersion must be {EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION}"
            ));
        }
        require_id("evolve iteration journal journalId", &self.journal_id)?;
        let mut ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !ids.insert(entry.iteration_id.as_str()) {
                return Err(anyhow!(
                    "evolve iteration journal duplicate iterationId `{}`",
                    entry.iteration_id
                ));
            }
        }
        validate_texts(
            "evolve iteration journal guardrails",
            &self.guardrails,
            true,
        )?;
        Ok(())
    }

    pub fn append_entry(&self, entry: EvolveIterationEntry) -> Result<Self> {
        entry.validate()?;
        if self
            .entries
            .iter()
            .any(|prior| prior.iteration_id == entry.iteration_id)
        {
            return Err(anyhow!(
                "cannot append duplicate iterationId `{}` to evolve iteration journal",
                entry.iteration_id
            ));
        }
        let mut next = self.clone();
        next.entries.push(entry);
        Ok(next)
    }

    pub fn validate_is_append_of(&self, previous: &Self) -> Result<()> {
        if self.journal_id != previous.journal_id {
            return Err(anyhow!(
                "evolve iteration journal id changed; not an append"
            ));
        }
        if self.entries.len() < previous.entries.len() {
            return Err(anyhow!("evolve iteration journal lost entries"));
        }
        for (index, prior) in previous.entries.iter().enumerate() {
            if &self.entries[index] != prior {
                return Err(anyhow!(
                    "evolve iteration journal rewrote entry {index}; history is append-only"
                ));
            }
        }
        Ok(())
    }

    pub fn validate_references_exist(&self, known_refs: &BTreeSet<String>) -> Result<()> {
        for entry in &self.entries {
            for reference in entry.all_refs() {
                if !known_refs.contains(reference) {
                    return Err(anyhow!(
                        "evolve iteration journal reference `{reference}` is missing from known evidence set"
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn read_model(&self) -> EvolveIterationJournalReadModel {
        EvolveIterationJournalReadModel {
            schema_version: EVOLVE_ITERATION_JOURNAL_SCHEMA_VERSION.to_string(),
            journal_id: self.journal_id.clone(),
            entry_count: self.entries.len(),
            applied_count: self.entries.iter().filter(|e| e.applied_change).count(),
            inconclusive_count: self
                .entries
                .iter()
                .filter(|e| e.outcome == EvolveIterationOutcome::Inconclusive)
                .count(),
            append_only: true,
            forbidden_actions: vec![
                "rewrite_journal_history".to_string(),
                "hide_inconclusive_run".to_string(),
                "apply_without_review".to_string(),
            ],
        }
    }
}

impl EvolveIterationEntry {
    pub fn validate(&self) -> Result<()> {
        require_id("evolve iteration entry iterationId", &self.iteration_id)?;
        for (field, value) in [
            ("hypothesis", &self.hypothesis),
            ("beforeBundleRef", &self.before_bundle_ref),
            ("proposalRef", &self.proposal_ref),
            ("decision", &self.decision),
            ("nextAction", &self.next_action),
        ] {
            require_text(&format!("evolve iteration entry {field}"), value)?;
        }
        require_ref(
            "evolve iteration entry beforeBundleRef",
            &self.before_bundle_ref,
        )?;
        require_ref("evolve iteration entry proposalRef", &self.proposal_ref)?;
        validate_refs(
            "evolve iteration entry evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        for (field, value) in [
            ("reviewRef", &self.review_ref),
            ("applyRef", &self.apply_ref),
            ("afterBundleRef", &self.after_bundle_ref),
            ("comparisonRef", &self.comparison_ref),
        ] {
            if let Some(reference) = value {
                require_ref(&format!("evolve iteration entry {field}"), reference)?;
            }
        }
        if self.applied_change {
            for (field, value) in [
                ("reviewRef", &self.review_ref),
                ("applyRef", &self.apply_ref),
                ("afterBundleRef", &self.after_bundle_ref),
                ("comparisonRef", &self.comparison_ref),
            ] {
                if value.is_none() {
                    return Err(anyhow!("applied evolve iteration cannot omit {field}"));
                }
            }
        }
        Ok(())
    }

    fn all_refs(&self) -> Vec<&String> {
        let mut refs = vec![&self.before_bundle_ref, &self.proposal_ref];
        refs.extend(self.evidence_refs.iter());
        for value in [
            &self.review_ref,
            &self.apply_ref,
            &self.after_bundle_ref,
            &self.comparison_ref,
        ]
        .into_iter()
        .flatten()
        {
            refs.push(value);
        }
        refs
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

fn validate_texts(field: &str, values: &[String], require_nonempty: bool) -> Result<()> {
    if require_nonempty && values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        require_text(field, value)?;
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
    if value.contains('/') || value.contains('\\') || value.contains("..") {
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
