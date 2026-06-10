//! Live failure classification for product-observed bundles (#2381 / M127.1).
//!
//! This classifier is evidence-only: it maps planted/live bundle signals to
//! actionable categories, severity, and next-owner hints. It reuses #2358
//! runtime diagnostics (`BehaviorRuntimeDiagnostic`) as diagnostic input and
//! validates category/severity ids against the #2350 taxonomy JSON instead of
//! declaring a second taxonomy.

use crate::behavior_runtime::BehaviorRuntimeDiagnostic;
use crate::product_gap_taxonomy::{
    default_owner_for_category, validate_product_gap_category, validate_product_gap_severity,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION: &str = "live-failure-classifier-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum LiveFailureClass {
    ConsoleRuntime,
    GameplayObjective,
    VisualReadability,
    InputControl,
    Authoring,
    EvidenceMissing,
    FlakeInconclusive,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LiveFailureClassifierStatus {
    Classified,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LiveFailureSignal {
    #[serde(rename = "signalId")]
    pub signal_id: String,
    pub class: LiveFailureClass,
    /// Must be one of docs/product-gap-taxonomy.json categoryEnum ids.
    pub category: String,
    /// Must be one of docs/product-gap-taxonomy.json severityEnum ids.
    pub severity: String,
    #[serde(rename = "nextOwner")]
    pub next_owner: String,
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
    #[serde(
        rename = "runtimeDiagnostics",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub runtime_diagnostics: Vec<BehaviorRuntimeDiagnostic>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LiveFailureClassification {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "classificationId")]
    pub classification_id: String,
    #[serde(rename = "bundleRef")]
    pub bundle_ref: String,
    #[serde(rename = "requiredArtifactRefs")]
    pub required_artifact_refs: Vec<String>,
    #[serde(rename = "missingArtifactRefs", default)]
    pub missing_artifact_refs: Vec<String>,
    pub signals: Vec<LiveFailureSignal>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LiveFailureClassificationReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "classificationId")]
    pub classification_id: String,
    pub status: LiveFailureClassifierStatus,
    #[serde(rename = "signalCount")]
    pub signal_count: usize,
    #[serde(rename = "missingArtifactCount")]
    pub missing_artifact_count: usize,
    #[serde(rename = "classSummary")]
    pub class_summary: Vec<String>,
    #[serde(rename = "nextOwnerHints")]
    pub next_owner_hints: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl LiveFailureClassification {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse live failure classification JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn computed_status(&self) -> LiveFailureClassifierStatus {
        if self.blocked_reasons.is_empty() && self.missing_artifact_refs.is_empty() {
            LiveFailureClassifierStatus::Classified
        } else {
            LiveFailureClassifierStatus::Blocked
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION {
            return Err(anyhow!(
                "live failure classifier schemaVersion must be {LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION}"
            ));
        }
        require_id(
            "live failure classifier classificationId",
            &self.classification_id,
        )?;
        require_ref("live failure classifier bundleRef", &self.bundle_ref)?;
        validate_refs(
            "live failure classifier requiredArtifactRefs",
            &self.required_artifact_refs,
            true,
        )?;
        validate_refs(
            "live failure classifier missingArtifactRefs",
            &self.missing_artifact_refs,
            false,
        )?;
        let required: BTreeSet<&str> = self
            .required_artifact_refs
            .iter()
            .map(String::as_str)
            .collect();
        for missing in &self.missing_artifact_refs {
            if !required.contains(missing.as_str()) {
                return Err(anyhow!(
                    "live failure classifier missingArtifactRefs must reference requiredArtifactRefs"
                ));
            }
        }
        if self.signals.is_empty() {
            return Err(anyhow!("live failure classifier signals must not be empty"));
        }
        let mut ids = BTreeSet::new();
        let mut has_evidence_missing = false;
        for signal in &self.signals {
            signal.validate()?;
            if !ids.insert(signal.signal_id.as_str()) {
                return Err(anyhow!(
                    "live failure classifier duplicate signalId `{}`",
                    signal.signal_id
                ));
            }
            has_evidence_missing |= signal.class == LiveFailureClass::EvidenceMissing;
        }
        if !self.missing_artifact_refs.is_empty() && !has_evidence_missing {
            return Err(anyhow!(
                "missing required artifacts must produce an evidence-missing classification"
            ));
        }
        validate_texts(
            "live failure classifier blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        require_text("live failure classifier boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "no automatic fix",
            "evidence-based",
            "backlog routing",
            "read-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "live failure classifier boundary must state {required}"
                ));
            }
        }
        Ok(())
    }

    pub fn read_model(&self) -> LiveFailureClassificationReadModel {
        let mut classes = BTreeSet::new();
        let mut owners = BTreeSet::new();
        let mut blocked = self.blocked_reasons.clone();
        if !self.missing_artifact_refs.is_empty() {
            blocked.push(format!(
                "missing required artifacts: {}",
                self.missing_artifact_refs.join(",")
            ));
        }
        for signal in &self.signals {
            classes.insert(format!("{:?}:{}", signal.class, signal.severity));
            owners.insert(format!("{}:{}", signal.category, signal.next_owner));
        }
        LiveFailureClassificationReadModel {
            schema_version: LIVE_FAILURE_CLASSIFIER_SCHEMA_VERSION.to_string(),
            classification_id: self.classification_id.clone(),
            status: self.computed_status(),
            signal_count: self.signals.len(),
            missing_artifact_count: self.missing_artifact_refs.len(),
            class_summary: classes.into_iter().collect(),
            next_owner_hints: owners.into_iter().collect(),
            blocked_reasons: blocked,
            forbidden_actions: vec![
                "auto_fix".to_string(),
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "hide_missing_evidence".to_string(),
            ],
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize live failure classification read model JSON")
    }
}

impl LiveFailureSignal {
    fn validate(&self) -> Result<()> {
        require_id("live failure signal signalId", &self.signal_id)?;
        validate_product_gap_category("live failure signal category", &self.category)?;
        validate_product_gap_severity("live failure signal severity", &self.severity)?;
        require_text("live failure signal nextOwner", &self.next_owner)?;
        let expected_owner = default_owner_for_category(&self.category)?;
        if self.next_owner != expected_owner {
            return Err(anyhow!(
                "live failure signal nextOwner `{}` does not match #2350 default owner `{expected_owner}` for category `{}`",
                self.next_owner,
                self.category
            ));
        }
        validate_refs(
            "live failure signal evidenceRefs",
            &self.evidence_refs,
            true,
        )?;
        for (field, value) in [
            ("observedBehavior", &self.observed_behavior),
            ("expectedBehavior", &self.expected_behavior),
            ("productImpact", &self.product_impact),
            ("recommendedBacklogAction", &self.recommended_backlog_action),
        ] {
            require_text(&format!("live failure signal {field}"), value)?;
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
