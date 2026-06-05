mod dashboard;
mod journal;
mod rules;
mod validation;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub use dashboard::{read_dashboard_behavior_evidence, RunDashboardBehaviorEvidence};
pub use journal::render_behavior_evidence_journal_section;
pub use validation::{inspect_behavior_evidence_bundle, validate_behavior_evidence_bundle};

pub const BEHAVIOR_EVIDENCE_BUNDLE_SCHEMA_VERSION: &str = "behavior-evidence-bundle-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorEvidenceBundleStatus {
    Complete,
    Partial,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorEvidenceRef {
    pub kind: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorObservedFailure {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    pub summary: String,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: BehaviorEvidenceRef,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorNextStepHypothesis {
    pub id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorEvidenceBundleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub status: BehaviorEvidenceBundleStatus,
    #[serde(rename = "behaviorDefinitionRefs")]
    pub behavior_definition_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "runtimeEventRefs")]
    pub runtime_event_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "scenarioOutcomeRefs")]
    pub scenario_outcome_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "draftRefs")]
    pub draft_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "reviewDecisionRefs")]
    pub review_decision_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "applyTransactionRefs")]
    pub apply_transaction_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "rollbackMetadataRefs")]
    pub rollback_metadata_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "rerunComparisonRefs", default)]
    pub rerun_comparison_refs: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "observedFailures", default)]
    pub observed_failures: Vec<BehaviorObservedFailure>,
    #[serde(rename = "nextStepHypotheses", default)]
    pub next_step_hypotheses: Vec<BehaviorNextStepHypothesis>,
    #[serde(rename = "linkedEvidence")]
    pub linked_evidence: Vec<BehaviorEvidenceRef>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorEvidenceBundleValidation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub status: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "lifecycleRefCount")]
    pub lifecycle_ref_count: usize,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub guardrails: Vec<String>,
}

impl BehaviorEvidenceBundleArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: BehaviorEvidenceBundleArtifact =
            serde_json::from_str(input).context("failed to parse behavior evidence bundle JSON")?;
        validate_behavior_evidence_bundle(&artifact)?;
        Ok(artifact)
    }

    pub fn inspect(&self) -> BehaviorEvidenceBundleValidation {
        inspect_behavior_evidence_bundle(self)
    }
}

impl BehaviorEvidenceBundleValidation {
    pub fn is_blocked(&self) -> bool {
        self.status == "blocked"
    }
}
