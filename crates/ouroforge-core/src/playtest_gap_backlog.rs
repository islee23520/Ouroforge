//! Playtest findings and gap backlog model (#2390 / M129.3).

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION: &str = "playtest-gap-backlog-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PlaytestFindingStatus {
    Open,
    Deferred,
    Resolved,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestFinding {
    #[serde(rename = "findingId")]
    pub finding_id: String,
    pub category: String,
    pub severity: String,
    pub status: PlaytestFindingStatus,
    pub observation: String,
    #[serde(
        rename = "humanFunFeelNote",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub human_fun_feel_note: Option<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "ownerIssue")]
    pub owner_issue: String,
    #[serde(rename = "nextAction")]
    pub next_action: String,
    #[serde(rename = "blocksProductObserved", default)]
    pub blocks_product_observed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestGapBacklog {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "backlogId")]
    pub backlog_id: String,
    #[serde(rename = "productionJournalRef")]
    pub production_journal_ref: String,
    pub findings: Vec<PlaytestFinding>,
}

impl PlaytestGapBacklog {
    pub fn validate(
        &self,
        allowed_categories: &BTreeSet<String>,
        allowed_severities: &BTreeSet<String>,
    ) -> Result<()> {
        if self.schema_version != PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION {
            return Err(anyhow!(
                "playtest gap backlog schemaVersion must be {PLAYTEST_GAP_BACKLOG_SCHEMA_VERSION}"
            ));
        }
        require_id("playtest gap backlog backlogId", &self.backlog_id)?;
        require_ref(
            "playtest gap backlog productionJournalRef",
            &self.production_journal_ref,
        )?;
        if self.findings.is_empty() {
            return Err(anyhow!("playtest gap backlog findings must not be empty"));
        }
        let mut ids = BTreeSet::new();
        for finding in &self.findings {
            finding.validate(allowed_categories, allowed_severities)?;
            if !ids.insert(finding.finding_id.as_str()) {
                return Err(anyhow!(
                    "playtest gap backlog duplicate findingId `{}`",
                    finding.finding_id
                ));
            }
        }
        Ok(())
    }
}

impl PlaytestFinding {
    pub fn validate(
        &self,
        allowed_categories: &BTreeSet<String>,
        allowed_severities: &BTreeSet<String>,
    ) -> Result<()> {
        require_id("playtest finding findingId", &self.finding_id)?;
        require_text("playtest finding observation", &self.observation)?;
        require_ref("playtest finding ownerIssue", &self.owner_issue)?;
        require_text("playtest finding nextAction", &self.next_action)?;
        validate_refs("playtest finding evidenceRefs", &self.evidence_refs, true)?;
        if !allowed_categories.contains(&self.category) {
            return Err(anyhow!(
                "playtest finding category `{}` is not in M117 taxonomy",
                self.category
            ));
        }
        if !allowed_severities.contains(&self.severity) {
            return Err(anyhow!(
                "playtest finding severity `{}` is not in M117 taxonomy",
                self.severity
            ));
        }
        if self.blocks_product_observed && self.status != PlaytestFindingStatus::Deferred {
            return Err(anyhow!("blocking playtest findings must be explicitly deferred before product-observed closure"));
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
