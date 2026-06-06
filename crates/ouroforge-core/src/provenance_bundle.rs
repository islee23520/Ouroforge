//! Provenance Bundle Model v1 (#1500).
//!
//! A read-only audit/replay surface that composes existing provenance,
//! evidence, rollback, review, regression, and promotion records by reference.
//! This module records and validates references only; it does not create a
//! parallel provenance engine and does not execute or apply anything.

use crate::export_hash::sha256_hex;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

pub const PROVENANCE_BUNDLE_SCHEMA_VERSION: &str = "provenance-bundle-v1";

const REQUIRED_LINK_KINDS: &[ProvenanceBundleLinkKind] = &[
    ProvenanceBundleLinkKind::IntentDesignBrief,
    ProvenanceBundleLinkKind::GeneratedEditedArtifact,
    ProvenanceBundleLinkKind::ValidationResult,
    ProvenanceBundleLinkKind::RuntimeObservation,
    ProvenanceBundleLinkKind::EvaluatorVerdict,
    ProvenanceBundleLinkKind::RegressionComparison,
    ProvenanceBundleLinkKind::JournalReviewDecision,
    ProvenanceBundleLinkKind::PromotionRollbackRecord,
];

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProvenanceBundleStatus {
    Complete,
    Incomplete,
    Dangling,
    Stale,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ProvenanceBundleLinkKind {
    IntentDesignBrief,
    GeneratedEditedArtifact,
    ValidationResult,
    RuntimeObservation,
    EvaluatorVerdict,
    RegressionComparison,
    JournalReviewDecision,
    PromotionRollbackRecord,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBundleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "changeId")]
    pub change_id: String,
    pub status: ProvenanceBundleStatus,
    #[serde(rename = "chainLinks")]
    pub chain_links: Vec<ProvenanceBundleLink>,
    #[serde(rename = "generatedState")]
    pub generated_state: ProvenanceBundleGeneratedState,
    #[serde(default, rename = "incompleteReasons")]
    pub incomplete_reasons: Vec<String>,
    #[serde(default, rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    #[serde(
        default,
        rename = "replayInputs",
        skip_serializing_if = "Option::is_none"
    )]
    pub replay_inputs: Option<ProvenanceBundleReplayInputs>,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBundleLink {
    pub kind: ProvenanceBundleLinkKind,
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(
        default,
        rename = "expectedDigest",
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_digest: Option<String>,
    #[serde(default)]
    pub stale: bool,
    #[serde(
        default,
        rename = "staleReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub stale_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBundleGeneratedState {
    pub generated: bool,
    pub tracked: bool,
    #[serde(rename = "fixtureScoped")]
    pub fixture_scoped: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBundleReplayInputs {
    #[serde(rename = "runRef")]
    pub run_ref: String,
    #[serde(rename = "expectedVerdictRef")]
    pub expected_verdict_ref: String,
    #[serde(rename = "deterministicInputs")]
    pub deterministic_inputs: bool,
    #[serde(default, rename = "deterministicMetadataRefs")]
    pub deterministic_metadata_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProvenanceBundleEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    #[serde(rename = "computedStatus")]
    pub computed_status: ProvenanceBundleStatus,
    #[serde(rename = "statusConsistent")]
    pub status_consistent: bool,
    #[serde(rename = "linkStates")]
    pub link_states: BTreeMap<String, String>,
    pub issues: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
}

impl ProvenanceBundleArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse provenance bundle JSON")?;
        artifact.validate_shape()?;
        Ok(artifact)
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != PROVENANCE_BUNDLE_SCHEMA_VERSION {
            return Err(anyhow!(
                "provenance bundle schemaVersion must be {PROVENANCE_BUNDLE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("provenance bundle bundleId", &self.bundle_id)?;
        require_local_id("provenance bundle changeId", &self.change_id)?;
        if self.chain_links.is_empty() {
            return Err(anyhow!("provenance bundle chainLinks must not be empty"));
        }
        let mut seen = BTreeSet::new();
        for link in &self.chain_links {
            link.validate()?;
            if !seen.insert(link.kind) {
                return Err(anyhow!(
                    "provenance bundle duplicates chain link kind `{}`",
                    link_kind_label(link.kind)
                ));
            }
        }
        self.generated_state.validate()?;
        validate_text_list(
            "provenance bundle incompleteReasons",
            &self.incomplete_reasons,
            false,
        )?;
        validate_text_list(
            "provenance bundle compatibilityNotes",
            &self.compatibility_notes,
            false,
        )?;
        if let Some(replay_inputs) = &self.replay_inputs {
            replay_inputs.validate()?;
        }
        require_text("provenance bundle boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in ["audit", "replay", "read-only"] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "provenance bundle boundary must state `{required}`"
                ));
            }
        }
        validate_text_list("provenance bundle guardrails", &self.guardrails, true)?;
        Ok(())
    }

    pub fn evaluate_with_root(&self, root: &Path) -> ProvenanceBundleEvaluation {
        let mut issues = Vec::new();
        let mut link_states = BTreeMap::new();
        let present: BTreeSet<_> = self.chain_links.iter().map(|link| link.kind).collect();

        for required in REQUIRED_LINK_KINDS {
            if !present.contains(required) {
                issues.push(format!(
                    "missing chain link: {}",
                    link_kind_label(*required)
                ));
                link_states.insert(
                    link_kind_label(*required).to_string(),
                    "missing".to_string(),
                );
            }
        }

        for link in &self.chain_links {
            let label = link_kind_label(link.kind).to_string();
            if link.stale {
                issues.push(format!(
                    "stale ref: {} -> {}",
                    link_kind_label(link.kind),
                    link.reference
                ));
                link_states.insert(label, "stale".to_string());
                continue;
            }

            match resolve_local_ref(root, &link.reference) {
                Ok(path) if path.exists() => {
                    if let Some(expected) = &link.expected_digest {
                        match sha256_file_digest(&path) {
                            Ok(actual) if &actual == expected => {
                                link_states.insert(label, "present".to_string());
                            }
                            Ok(actual) => {
                                issues.push(format!(
                                    "stale ref: {} expected digest `{}` but found `{}`",
                                    link.reference, expected, actual
                                ));
                                link_states.insert(label, "stale".to_string());
                            }
                            Err(error) => {
                                issues.push(format!(
                                    "dangling reference: {} could not be read: {error}",
                                    link.reference
                                ));
                                link_states.insert(label, "dangling".to_string());
                            }
                        }
                    } else {
                        link_states.insert(label, "present".to_string());
                    }
                }
                Ok(_) => {
                    issues.push(format!("dangling reference: {}", link.reference));
                    link_states.insert(label, "dangling".to_string());
                }
                Err(error) => {
                    issues.push(format!("dangling reference: {} ({error})", link.reference));
                    link_states.insert(label, "dangling".to_string());
                }
            }
        }

        if self.status == ProvenanceBundleStatus::Incomplete && self.incomplete_reasons.is_empty() {
            issues.push("incomplete bundle state lacks explicit incompleteReasons".to_string());
        }
        for reason in &self.incomplete_reasons {
            issues.push(format!("incomplete bundle state: {reason}"));
        }

        let computed_status = if issues
            .iter()
            .any(|issue| issue.contains("dangling reference"))
        {
            ProvenanceBundleStatus::Dangling
        } else if issues.iter().any(|issue| issue.contains("stale ref")) {
            ProvenanceBundleStatus::Stale
        } else if issues.is_empty() {
            ProvenanceBundleStatus::Complete
        } else {
            ProvenanceBundleStatus::Incomplete
        };

        let status_consistent = computed_status == self.status;
        if !status_consistent {
            issues.push(format!(
                "incomplete bundle state: declared `{}` but evidence implies `{}`",
                status_label(self.status),
                status_label(computed_status)
            ));
        }

        ProvenanceBundleEvaluation {
            schema_version: PROVENANCE_BUNDLE_SCHEMA_VERSION.to_string(),
            bundle_id: self.bundle_id.clone(),
            computed_status,
            status_consistent,
            link_states,
            issues,
            compatibility_notes: self.compatibility_notes.clone(),
            allowed_actions: vec![
                "inspect_bundle".to_string(),
                "audit_references".to_string(),
                "replay_validation_locally".to_string(),
            ],
            forbidden_actions: vec![
                "apply_patch".to_string(),
                "merge_branch".to_string(),
                "promote_without_review".to_string(),
                "execute_command".to_string(),
            ],
        }
    }

    pub fn evaluation_json_with_root(&self, root: &Path) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate_with_root(root))
            .context("failed to serialize provenance bundle evaluation JSON")
    }
}

impl ProvenanceBundleReplayInputs {
    fn validate(&self) -> Result<()> {
        require_local_ref("provenance bundle replayInputs.runRef", &self.run_ref)?;
        require_local_ref(
            "provenance bundle replayInputs.expectedVerdictRef",
            &self.expected_verdict_ref,
        )?;
        for reference in &self.deterministic_metadata_refs {
            require_local_ref(
                "provenance bundle replayInputs.deterministicMetadataRefs",
                reference,
            )?;
        }
        Ok(())
    }
}

impl ProvenanceBundleLink {
    fn validate(&self) -> Result<()> {
        require_local_ref("provenance bundle ref", &self.reference)?;
        require_local_id("provenance bundle artifactId", &self.artifact_id)?;
        if let Some(expected) = &self.expected_digest {
            require_sha256_digest("provenance bundle expectedDigest", expected)?;
        }
        if self.stale {
            require_text(
                "provenance bundle staleReason",
                self.stale_reason.as_deref().unwrap_or_default(),
            )?;
        }
        Ok(())
    }
}

impl ProvenanceBundleGeneratedState {
    fn validate(&self) -> Result<()> {
        if self.generated && self.tracked && !self.fixture_scoped {
            return Err(anyhow!(
                "generated provenance bundles must be untracked unless fixture-scoped"
            ));
        }
        Ok(())
    }
}

fn resolve_local_ref(root: &Path, reference: &str) -> Result<std::path::PathBuf> {
    let path = Path::new(reference);
    if path.is_absolute()
        || reference.contains('\\')
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(anyhow!(
            "reference must stay inside the local evidence root"
        ));
    }
    Ok(root.join(path))
}

fn sha256_file_digest(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(format!("sha256:{}", sha256_hex(&bytes)))
}

fn link_kind_label(kind: ProvenanceBundleLinkKind) -> &'static str {
    match kind {
        ProvenanceBundleLinkKind::IntentDesignBrief => "intent-design-brief",
        ProvenanceBundleLinkKind::GeneratedEditedArtifact => "generated-edited-artifact",
        ProvenanceBundleLinkKind::ValidationResult => "validation-result",
        ProvenanceBundleLinkKind::RuntimeObservation => "runtime-observation",
        ProvenanceBundleLinkKind::EvaluatorVerdict => "evaluator-verdict",
        ProvenanceBundleLinkKind::RegressionComparison => "regression-comparison",
        ProvenanceBundleLinkKind::JournalReviewDecision => "journal-review-decision",
        ProvenanceBundleLinkKind::PromotionRollbackRecord => "promotion-rollback-record",
    }
}

fn status_label(status: ProvenanceBundleStatus) -> &'static str {
    match status {
        ProvenanceBundleStatus::Complete => "complete",
        ProvenanceBundleStatus::Incomplete => "incomplete",
        ProvenanceBundleStatus::Dangling => "dangling",
        ProvenanceBundleStatus::Stale => "stale",
    }
}

fn validate_text_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required && values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 128
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, or dot"
        ));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    resolve_local_ref(Path::new("."), value).map(|_| ())
}

fn require_sha256_digest(field: &str, value: &str) -> Result<()> {
    let digest = value.strip_prefix("sha256:").unwrap_or_default();
    if digest.len() != 64 || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a sha256:<64 hex> digest"));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
