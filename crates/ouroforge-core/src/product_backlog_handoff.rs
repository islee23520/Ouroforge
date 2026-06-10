//! Product backlog handoff for unresolved live failures (#2383 / M127.3).
//!
//! Backlog entries use #2350 category/severity ids verbatim and connect live
//! failure classifications plus evolve journals to visible triage state. A
//! product-observed closure claim is blocked while any blocking item remains
//! untriaged.

use crate::evolve_iteration_journal::{EvolveIterationJournal, EvolveIterationOutcome};
use crate::live_failure_classifier::{LiveFailureClassification, LiveFailureSignal};
use crate::product_gap_taxonomy::{
    default_owner_for_category, validate_product_gap_category, validate_product_gap_severity,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION: &str = "product-backlog-handoff-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProductBacklogStatus {
    Untriaged,
    Triaged,
    Deferred,
    Accepted,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProductClosureClassification {
    ContractComplete,
    ProductObservedComplete,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductBacklogEntry {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub category: String,
    pub severity: String,
    pub owner: String,
    #[serde(rename = "ownerMilestoneOrIssue")]
    pub owner_milestone_or_issue: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "observedBehavior")]
    pub observed_behavior: String,
    #[serde(rename = "expectedBehavior")]
    pub expected_behavior: String,
    #[serde(rename = "productImpact")]
    pub product_impact: String,
    #[serde(rename = "recommendedBacklogAction")]
    pub recommended_backlog_action: String,
    #[serde(rename = "nextAction")]
    pub next_action: String,
    pub status: ProductBacklogStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductBacklogHandoff {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "handoffId")]
    pub handoff_id: String,
    pub entries: Vec<ProductBacklogEntry>,
    #[serde(rename = "classificationRefs")]
    pub classification_refs: Vec<String>,
    #[serde(rename = "journalRefs")]
    pub journal_refs: Vec<String>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductBacklogClosureEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "handoffId")]
    pub handoff_id: String,
    #[serde(rename = "requestedClosure")]
    pub requested_closure: ProductClosureClassification,
    #[serde(rename = "closureAllowed")]
    pub closure_allowed: bool,
    #[serde(rename = "blockingItems")]
    pub blocking_items: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl ProductBacklogHandoff {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse product backlog handoff JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION {
            return Err(anyhow!(
                "product backlog handoff schemaVersion must be {PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION}"
            ));
        }
        require_id("product backlog handoff handoffId", &self.handoff_id)?;
        let mut ids = BTreeSet::new();
        for entry in &self.entries {
            entry.validate()?;
            if !ids.insert(entry.item_id.as_str()) {
                return Err(anyhow!(
                    "product backlog handoff duplicate itemId `{}`",
                    entry.item_id
                ));
            }
        }
        validate_refs(
            "product backlog handoff classificationRefs",
            &self.classification_refs,
            false,
        )?;
        validate_refs(
            "product backlog handoff journalRefs",
            &self.journal_refs,
            false,
        )?;
        validate_texts("product backlog handoff guardrails", &self.guardrails, true)?;
        Ok(())
    }

    pub fn from_classification_and_journal(
        handoff_id: &str,
        classification_ref: &str,
        classification: &LiveFailureClassification,
        journal_ref: &str,
        journal: &EvolveIterationJournal,
    ) -> Result<Self> {
        let mut entries = Vec::new();
        for signal in &classification.signals {
            entries.push(entry_from_signal(signal));
        }
        for entry in &journal.entries {
            if matches!(
                entry.outcome,
                EvolveIterationOutcome::Regressed | EvolveIterationOutcome::Inconclusive
            ) {
                entries.push(ProductBacklogEntry {
                    item_id: format!("backlog-{}", entry.iteration_id),
                    category: "qa_evaluator_depth".to_string(),
                    severity: if entry.outcome == EvolveIterationOutcome::Regressed { "blocking" } else { "major" }.to_string(),
                    owner: "qa".to_string(),
                    owner_milestone_or_issue: "#2383".to_string(),
                    evidence_refs: entry.evidence_refs.clone(),
                    observed_behavior: format!("iteration {} ended as {:?}", entry.iteration_id, entry.outcome),
                    expected_behavior: "evolve iterations must leave visible evidence and routed next action".to_string(),
                    product_impact: "unresolved loop result blocks honest product-observed closure until triaged".to_string(),
                    recommended_backlog_action: entry.next_action.clone(),
                    next_action: entry.next_action.clone(),
                    status: ProductBacklogStatus::Untriaged,
                });
            }
        }
        let handoff = Self {
            schema_version: PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION.to_string(),
            handoff_id: handoff_id.to_string(),
            entries,
            classification_refs: vec![classification_ref.to_string()],
            journal_refs: vec![journal_ref.to_string()],
            guardrails: vec![
                "product-observed closure cannot hide untriaged blocking backlog items".to_string(),
                "backlog entries reuse docs/product-gap-taxonomy.json categories and severities"
                    .to_string(),
            ],
        };
        handoff.validate()?;
        Ok(handoff)
    }

    pub fn evaluate_closure(
        &self,
        requested_closure: ProductClosureClassification,
    ) -> ProductBacklogClosureEvaluation {
        let mut blocking_items = Vec::new();
        if requested_closure == ProductClosureClassification::ProductObservedComplete {
            for entry in &self.entries {
                if entry.severity == "blocking" && entry.status == ProductBacklogStatus::Untriaged {
                    blocking_items.push(entry.item_id.clone());
                }
            }
        }
        ProductBacklogClosureEvaluation {
            schema_version: PRODUCT_BACKLOG_HANDOFF_SCHEMA_VERSION.to_string(),
            handoff_id: self.handoff_id.clone(),
            requested_closure,
            closure_allowed: blocking_items.is_empty(),
            blocking_items,
            forbidden_actions: vec![
                "product_observed_close_with_untriaged_blockers".to_string(),
                "hide_backlog_gap".to_string(),
                "auto_close_issue".to_string(),
            ],
        }
    }
}

impl ProductBacklogEntry {
    fn validate(&self) -> Result<()> {
        require_id("product backlog entry itemId", &self.item_id)?;
        validate_product_gap_category("product backlog entry category", &self.category)?;
        validate_product_gap_severity("product backlog entry severity", &self.severity)?;
        require_text("product backlog entry owner", &self.owner)?;
        let expected_owner = default_owner_for_category(&self.category)?;
        if self.owner != expected_owner {
            return Err(anyhow!(
                "product backlog entry owner `{}` does not match #2350 default owner `{expected_owner}` for category `{}`",
                self.owner,
                self.category
            ));
        }
        validate_refs(
            "product backlog entry evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        for (field, value) in [
            ("ownerMilestoneOrIssue", &self.owner_milestone_or_issue),
            ("observedBehavior", &self.observed_behavior),
            ("expectedBehavior", &self.expected_behavior),
            ("productImpact", &self.product_impact),
            ("recommendedBacklogAction", &self.recommended_backlog_action),
            ("nextAction", &self.next_action),
        ] {
            require_text(&format!("product backlog entry {field}"), value)?;
        }
        Ok(())
    }
}

fn entry_from_signal(signal: &LiveFailureSignal) -> ProductBacklogEntry {
    ProductBacklogEntry {
        item_id: format!("backlog-{}", signal.signal_id),
        category: signal.category.clone(),
        severity: signal.severity.clone(),
        owner: signal.next_owner.clone(),
        owner_milestone_or_issue: "#2383".to_string(),
        evidence_refs: signal.evidence_refs.clone(),
        observed_behavior: signal.observed_behavior.clone(),
        expected_behavior: signal.expected_behavior.clone(),
        product_impact: signal.product_impact.clone(),
        recommended_backlog_action: signal.recommended_backlog_action.clone(),
        next_action: signal.recommended_backlog_action.clone(),
        status: ProductBacklogStatus::Untriaged,
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
