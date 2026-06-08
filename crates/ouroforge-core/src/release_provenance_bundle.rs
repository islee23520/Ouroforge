//! Per-Release Provenance Bundle v1 (#1691, #1 Era H Milestone 44).
//!
//! This module extends Milestone 25 by composition. A release candidate records
//! references to existing per-change provenance bundles plus content, asset, QA,
//! compliance, and release-candidate evidence. It validates and summarizes those
//! references; it does not create a parallel provenance engine, fabricate a
//! chain, execute replay commands, apply changes, or release anything.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path};

use crate::provenance_bundle::{ProvenanceBundleArtifact, ProvenanceBundleStatus};

pub const RELEASE_PROVENANCE_BUNDLE_SCHEMA_VERSION: &str = "release-provenance-bundle-v1";

const BOUNDARY: &str = "per-release provenance by composition over Milestone 25 bundles: Rust/local read-only audit and replay evidence, browser and Studio read-only, no parallel provenance engine, no auto-merge, no release authority, no quality/fun or production-ready claim";

const REQUIRED_LINKS: &[ReleaseProvenanceLinkKind] = &[
    ReleaseProvenanceLinkKind::Intent,
    ReleaseProvenanceLinkKind::Content,
    ReleaseProvenanceLinkKind::Assets,
    ReleaseProvenanceLinkKind::Qa,
    ReleaseProvenanceLinkKind::PerChangeProvenance,
    ReleaseProvenanceLinkKind::Compliance,
    ReleaseProvenanceLinkKind::ReleaseCandidate,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseProvenanceStatus {
    Complete,
    Incomplete,
    Dangling,
    Stale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseProvenanceLinkKind {
    Intent,
    Content,
    Assets,
    Qa,
    PerChangeProvenance,
    Compliance,
    ReleaseCandidate,
}

impl ReleaseProvenanceLinkKind {
    fn label(self) -> &'static str {
        match self {
            Self::Intent => "intent",
            Self::Content => "content",
            Self::Assets => "assets",
            Self::Qa => "qa",
            Self::PerChangeProvenance => "per-change-provenance",
            Self::Compliance => "compliance",
            Self::ReleaseCandidate => "release-candidate",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseProvenanceBundle {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "releaseId")]
    pub release_id: String,
    pub status: ReleaseProvenanceStatus,
    #[serde(rename = "chainLinks")]
    pub chain_links: Vec<ReleaseProvenanceLink>,
    #[serde(default, rename = "incompleteReasons")]
    pub incomplete_reasons: Vec<String>,
    #[serde(rename = "replayability")]
    pub replayability: ReleaseReplayability,
    #[serde(rename = "generatedState")]
    pub generated_state: ReleaseGeneratedState,
    pub boundary: String,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseProvenanceLink {
    pub kind: ReleaseProvenanceLinkKind,
    #[serde(rename = "ref")]
    pub reference: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(default)]
    pub stale: bool,
    #[serde(
        default,
        rename = "staleReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub stale_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseReplayability {
    #[serde(rename = "deterministicInputs")]
    pub deterministic_inputs: bool,
    #[serde(default, rename = "perChangeBundleRefs")]
    pub per_change_bundle_refs: Vec<String>,
    #[serde(default, rename = "expectedReplayRefs")]
    pub expected_replay_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseGeneratedState {
    pub generated: bool,
    pub tracked: bool,
    #[serde(rename = "fixtureScoped")]
    pub fixture_scoped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseProvenanceEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "releaseId")]
    pub release_id: String,
    #[serde(rename = "computedStatus")]
    pub computed_status: ReleaseProvenanceStatus,
    #[serde(rename = "statusConsistent")]
    pub status_consistent: bool,
    #[serde(rename = "linkStates")]
    pub link_states: BTreeMap<String, String>,
    #[serde(rename = "perChangeBundleStates")]
    pub per_change_bundle_states: BTreeMap<String, String>,
    #[serde(rename = "replayable")]
    pub replayable: bool,
    pub issues: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
    pub boundary: String,
}

impl ReleaseProvenanceBundle {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let bundle: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse release provenance bundle: {err}"))?;
        bundle.validate_shape()?;
        Ok(bundle)
    }

    pub fn validate_shape(&self) -> Result<()> {
        if self.schema_version != RELEASE_PROVENANCE_BUNDLE_SCHEMA_VERSION {
            return Err(anyhow!(
                "unexpected schema version: {}",
                self.schema_version
            ));
        }
        require_id("releaseId", &self.release_id)?;
        if self.chain_links.is_empty() {
            return Err(anyhow!("release provenance chainLinks must not be empty"));
        }
        for link in &self.chain_links {
            link.validate()?;
        }
        validate_texts("incompleteReasons", &self.incomplete_reasons, false)?;
        self.replayability.validate()?;
        self.generated_state.validate()?;
        validate_texts("guardrails", &self.guardrails, true)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "composition",
            "milestone 25",
            "read-only",
            "no parallel provenance engine",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "release provenance boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    pub fn evaluate_with_root(&self, root: &Path) -> ReleaseProvenanceEvaluation {
        let mut issues = Vec::new();
        let mut link_states = BTreeMap::new();
        let mut per_change_bundle_states = BTreeMap::new();
        let mut has_incomplete = false;
        let mut has_dangling = false;
        let mut has_stale = false;

        for required in REQUIRED_LINKS {
            if !self.chain_links.iter().any(|link| link.kind == *required) {
                has_incomplete = true;
                let label = required.label().to_string();
                link_states.insert(label.clone(), "missing".to_string());
                issues.push(format!("missing release chain link: {label}"));
            }
        }

        for link in &self.chain_links {
            let label = link.kind.label().to_string();
            match resolve_ref(root, &link.reference) {
                Ok(path) if path.is_file() => {
                    if link.stale {
                        has_stale = true;
                        issues.push(format!(
                            "stale release ref: {} -> {}",
                            label, link.reference
                        ));
                        link_states.insert(label, "stale".to_string());
                    } else {
                        link_states.insert(label.clone(), "present".to_string());
                    }
                    if link.kind == ReleaseProvenanceLinkKind::PerChangeProvenance {
                        match fs::read_to_string(&path)
                            .with_context(|| format!("failed to read {}", path.display()))
                            .and_then(|text| ProvenanceBundleArtifact::from_json_str(&text))
                        {
                            Ok(bundle) => {
                                let eval = bundle.evaluate_with_root(root);
                                per_change_bundle_states.insert(
                                    link.artifact_id.clone(),
                                    format!("{:?}", eval.computed_status).to_ascii_lowercase(),
                                );
                                match eval.computed_status {
                                    ProvenanceBundleStatus::Complete => {}
                                    ProvenanceBundleStatus::Incomplete => {
                                        has_incomplete = true;
                                        issues.push(format!(
                                            "per-change provenance bundle incomplete: {}",
                                            link.artifact_id
                                        ));
                                    }
                                    ProvenanceBundleStatus::Dangling => {
                                        has_dangling = true;
                                        issues.push(format!(
                                            "per-change provenance bundle dangling: {}",
                                            link.artifact_id
                                        ));
                                    }
                                    ProvenanceBundleStatus::Stale => {
                                        has_stale = true;
                                        issues.push(format!(
                                            "per-change provenance bundle stale: {}",
                                            link.artifact_id
                                        ));
                                    }
                                }
                            }
                            Err(error) => {
                                has_dangling = true;
                                per_change_bundle_states
                                    .insert(link.artifact_id.clone(), "dangling".to_string());
                                issues.push(format!(
                                    "per-change provenance bundle could not be parsed: {}: {error}",
                                    link.reference
                                ));
                            }
                        }
                    }
                }
                Ok(_) => {
                    has_dangling = true;
                    link_states.insert(label.clone(), "dangling".to_string());
                    issues.push(format!(
                        "dangling release ref: {} -> {}",
                        label, link.reference
                    ));
                }
                Err(error) => {
                    has_dangling = true;
                    link_states.insert(label.clone(), "dangling".to_string());
                    issues.push(format!("unsafe release ref: {}: {error}", link.reference));
                }
            }
        }

        if self.replayability.per_change_bundle_refs.is_empty()
            || !self.replayability.deterministic_inputs
        {
            has_incomplete = true;
            issues.push(
                "release replayability requires deterministic inputs and per-change bundle refs"
                    .to_string(),
            );
        }
        for reference in self
            .replayability
            .per_change_bundle_refs
            .iter()
            .chain(self.replayability.expected_replay_refs.iter())
        {
            match resolve_ref(root, reference) {
                Ok(path) if path.is_file() => {}
                Ok(_) => {
                    has_dangling = true;
                    issues.push(format!("missing replayability ref: {reference}"));
                }
                Err(error) => {
                    has_dangling = true;
                    issues.push(format!("unsafe replayability ref: {reference}: {error}"));
                }
            }
        }

        let computed_status = if has_dangling {
            ReleaseProvenanceStatus::Dangling
        } else if has_stale {
            ReleaseProvenanceStatus::Stale
        } else if has_incomplete {
            ReleaseProvenanceStatus::Incomplete
        } else {
            ReleaseProvenanceStatus::Complete
        };
        let replayable = computed_status == ReleaseProvenanceStatus::Complete
            && self.replayability.deterministic_inputs
            && !self.replayability.per_change_bundle_refs.is_empty();

        ReleaseProvenanceEvaluation {
            schema_version: RELEASE_PROVENANCE_BUNDLE_SCHEMA_VERSION.to_string(),
            release_id: self.release_id.clone(),
            computed_status,
            status_consistent: computed_status == self.status,
            link_states,
            per_change_bundle_states,
            replayable,
            issues,
            allowed_actions: vec![
                "inspect_release_provenance".to_string(),
                "replay_referenced_bundles_locally".to_string(),
            ],
            forbidden_actions: vec![
                "fabricate_missing_chain".to_string(),
                "apply_patch".to_string(),
                "auto_merge".to_string(),
                "release_without_human_go_no_go".to_string(),
            ],
            boundary: BOUNDARY.to_string(),
        }
    }
}

impl ReleaseProvenanceLink {
    fn validate(&self) -> Result<()> {
        if !is_safe_ref(&self.reference) {
            return Err(anyhow!(
                "release provenance ref must be safe: {}",
                self.reference
            ));
        }
        require_id("artifactId", &self.artifact_id)?;
        if self.stale
            && self
                .stale_reason
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
        {
            return Err(anyhow!(
                "stale release provenance links require staleReason"
            ));
        }
        Ok(())
    }
}

impl ReleaseReplayability {
    fn validate(&self) -> Result<()> {
        for reference in self
            .per_change_bundle_refs
            .iter()
            .chain(self.expected_replay_refs.iter())
        {
            if !is_safe_ref(reference) {
                return Err(anyhow!(
                    "release replayability ref must be safe: {reference}"
                ));
            }
        }
        Ok(())
    }
}

impl ReleaseGeneratedState {
    fn validate(&self) -> Result<()> {
        if self.generated && self.tracked && !self.fixture_scoped {
            return Err(anyhow!(
                "generated release provenance may be tracked only when fixture-scoped"
            ));
        }
        Ok(())
    }
}

fn resolve_ref(root: &Path, reference: &str) -> Result<std::path::PathBuf> {
    if !is_safe_ref(reference) {
        return Err(anyhow!(
            "reference must stay inside the local evidence root"
        ));
    }
    Ok(root.join(reference))
}

fn is_safe_ref(reference: &str) -> bool {
    let trimmed = reference.trim();
    let path = Path::new(trimmed);
    !trimmed.is_empty()
        && !path.is_absolute()
        && !trimmed.contains('\\')
        && !trimmed.contains(';')
        && !trimmed.contains("&&")
        && !trimmed.contains('|')
        && !path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
}

fn require_id(field: &str, value: &str) -> Result<()> {
    let valid = !value.trim().is_empty()
        && value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if valid {
        Ok(())
    } else {
        Err(anyhow!("{field} must be a bounded local id"))
    }
}

fn validate_texts(field: &str, values: &[String], require_one: bool) -> Result<()> {
    if require_one && values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if values.iter().any(|value| value.trim().is_empty()) {
        return Err(anyhow!("{field} must not contain blank text"));
    }
    Ok(())
}
