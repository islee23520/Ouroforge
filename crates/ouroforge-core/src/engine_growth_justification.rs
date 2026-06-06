//! Engine-Growth Demand Justification Gate v1 (#1495).
//!
//! This module records an auditable governance signal for proposed or added
//! engine capabilities. It does not mutate source and does not auto-block code;
//! it only evaluates whether each capability cites a current rung gate and
//! satisfied prerequisites.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const ENGINE_GROWTH_JUSTIFICATION_SCHEMA_VERSION: &str = "engine-growth-justification-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EngineGrowthCapabilityState {
    Proposed,
    Added,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EngineGrowthEvaluationStatus {
    Justified,
    Unjustified,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EngineGrowthFindingKind {
    MissingRungGate,
    StaleRungGateRef,
    UnsatisfiedPrerequisite,
    StalePrerequisiteRef,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EngineGrowthFindingSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EngineGrowthFindingStatus {
    Open,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EngineGrowthJustificationArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "knownRungGateRefs")]
    pub known_rung_gate_refs: Vec<String>,
    #[serde(rename = "knownPrerequisiteRefs", default)]
    pub known_prerequisite_refs: Vec<String>,
    #[serde(rename = "satisfiedPrerequisiteRefs", default)]
    pub satisfied_prerequisite_refs: Vec<String>,
    pub capabilities: Vec<EngineGrowthCapabilityJustification>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EngineGrowthCapabilityJustification {
    #[serde(rename = "capabilityId")]
    pub capability_id: String,
    pub state: EngineGrowthCapabilityState,
    pub summary: String,
    #[serde(
        rename = "rungGateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rung_gate_ref: Option<String>,
    #[serde(rename = "prerequisiteRefs", default)]
    pub prerequisite_refs: Vec<String>,
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EngineGrowthEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub status: EngineGrowthEvaluationStatus,
    pub capabilities: Vec<EngineGrowthCapabilityEvaluation>,
    pub findings: Vec<EngineGrowthFinding>,
    #[serde(rename = "autoBlock")]
    pub auto_block: bool,
    #[serde(rename = "governanceNote")]
    pub governance_note: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EngineGrowthCapabilityEvaluation {
    #[serde(rename = "capabilityId")]
    pub capability_id: String,
    pub state: EngineGrowthCapabilityState,
    pub status: EngineGrowthEvaluationStatus,
    #[serde(
        rename = "rungGateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rung_gate_ref: Option<String>,
    #[serde(rename = "prerequisiteRefs")]
    pub prerequisite_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EngineGrowthFinding {
    pub kind: EngineGrowthFindingKind,
    pub severity: EngineGrowthFindingSeverity,
    pub status: EngineGrowthFindingStatus,
    #[serde(rename = "capabilityId")]
    pub capability_id: String,
    #[serde(
        rename = "rungGateRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rung_gate_ref: Option<String>,
    #[serde(
        rename = "affectedRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub affected_ref: Option<String>,
    pub reason: String,
}

impl EngineGrowthJustificationArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse engine growth justification JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ENGINE_GROWTH_JUSTIFICATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "engine growth justification schemaVersion must be {ENGINE_GROWTH_JUSTIFICATION_SCHEMA_VERSION}"
            ));
        }
        require_ref("engine growth justification artifactId", &self.artifact_id)?;
        require_nonempty(
            "engine growth justification knownRungGateRefs",
            self.known_rung_gate_refs.len(),
        )?;
        validate_ref_list(
            "engine growth justification knownRungGateRefs",
            &self.known_rung_gate_refs,
        )?;
        validate_ref_list(
            "engine growth justification knownPrerequisiteRefs",
            &self.known_prerequisite_refs,
        )?;
        validate_ref_list(
            "engine growth justification satisfiedPrerequisiteRefs",
            &self.satisfied_prerequisite_refs,
        )?;
        require_nonempty(
            "engine growth justification capabilities",
            self.capabilities.len(),
        )?;
        for capability in &self.capabilities {
            capability.validate()?;
        }
        Ok(())
    }

    pub fn evaluate(&self) -> EngineGrowthEvaluation {
        let known_rung_gates: BTreeSet<&str> = self
            .known_rung_gate_refs
            .iter()
            .map(String::as_str)
            .collect();
        let known_prerequisites: BTreeSet<&str> = self
            .known_prerequisite_refs
            .iter()
            .map(String::as_str)
            .collect();
        let satisfied_prerequisites: BTreeSet<&str> = self
            .satisfied_prerequisite_refs
            .iter()
            .map(String::as_str)
            .collect();

        let mut findings = Vec::new();
        let mut capabilities = Vec::new();

        for capability in &self.capabilities {
            let start_findings = findings.len();

            match capability.rung_gate_ref.as_deref() {
                Some(rung_gate_ref) if known_rung_gates.contains(rung_gate_ref) => {}
                Some(rung_gate_ref) => findings.push(EngineGrowthFinding {
                    kind: EngineGrowthFindingKind::StaleRungGateRef,
                    severity: EngineGrowthFindingSeverity::Error,
                    status: EngineGrowthFindingStatus::Open,
                    capability_id: capability.capability_id.clone(),
                    rung_gate_ref: Some(rung_gate_ref.to_string()),
                    affected_ref: Some(rung_gate_ref.to_string()),
                    reason: format!(
                        "capability `{}` cites stale rung gate ref `{rung_gate_ref}`",
                        capability.capability_id
                    ),
                }),
                None => findings.push(EngineGrowthFinding {
                    kind: EngineGrowthFindingKind::MissingRungGate,
                    severity: EngineGrowthFindingSeverity::Warning,
                    status: EngineGrowthFindingStatus::Open,
                    capability_id: capability.capability_id.clone(),
                    rung_gate_ref: None,
                    affected_ref: None,
                    reason: format!(
                        "capability `{}` must cite the rung gate that demands engine growth",
                        capability.capability_id
                    ),
                }),
            }

            for prerequisite_ref in &capability.prerequisite_refs {
                if !known_prerequisites.contains(prerequisite_ref.as_str()) {
                    findings.push(EngineGrowthFinding {
                        kind: EngineGrowthFindingKind::StalePrerequisiteRef,
                        severity: EngineGrowthFindingSeverity::Error,
                        status: EngineGrowthFindingStatus::Open,
                        capability_id: capability.capability_id.clone(),
                        rung_gate_ref: capability.rung_gate_ref.clone(),
                        affected_ref: Some(prerequisite_ref.clone()),
                        reason: format!(
                            "capability `{}` cites stale prerequisite ref `{prerequisite_ref}`",
                            capability.capability_id
                        ),
                    });
                } else if !satisfied_prerequisites.contains(prerequisite_ref.as_str()) {
                    findings.push(EngineGrowthFinding {
                        kind: EngineGrowthFindingKind::UnsatisfiedPrerequisite,
                        severity: EngineGrowthFindingSeverity::Warning,
                        status: EngineGrowthFindingStatus::Open,
                        capability_id: capability.capability_id.clone(),
                        rung_gate_ref: capability.rung_gate_ref.clone(),
                        affected_ref: Some(prerequisite_ref.clone()),
                        reason: format!(
                            "capability `{}` requires unsatisfied prerequisite `{prerequisite_ref}`",
                            capability.capability_id
                        ),
                    });
                }
            }

            let status = if findings.len() == start_findings {
                EngineGrowthEvaluationStatus::Justified
            } else {
                EngineGrowthEvaluationStatus::Unjustified
            };

            capabilities.push(EngineGrowthCapabilityEvaluation {
                capability_id: capability.capability_id.clone(),
                state: capability.state,
                status,
                rung_gate_ref: capability.rung_gate_ref.clone(),
                prerequisite_refs: capability.prerequisite_refs.clone(),
            });
        }

        let status = if findings.is_empty() {
            EngineGrowthEvaluationStatus::Justified
        } else {
            EngineGrowthEvaluationStatus::Unjustified
        };

        EngineGrowthEvaluation {
            schema_version: ENGINE_GROWTH_JUSTIFICATION_SCHEMA_VERSION.to_string(),
            artifact_id: self.artifact_id.clone(),
            status,
            capabilities,
            findings,
            auto_block: false,
            governance_note:
                "unjustified engine growth is an auditable governance signal only; this evaluator does not auto-block code or mutate source".to_string(),
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize engine growth justification evaluation JSON")
    }
}

impl EngineGrowthCapabilityJustification {
    fn validate(&self) -> Result<()> {
        require_ref(
            "engine growth justification capabilities.capabilityId",
            &self.capability_id,
        )?;
        require_text(
            "engine growth justification capabilities.summary",
            &self.summary,
        )?;
        if let Some(rung_gate_ref) = &self.rung_gate_ref {
            require_ref(
                "engine growth justification capabilities.rungGateRef",
                rung_gate_ref,
            )?;
        }
        validate_ref_list(
            "engine growth justification capabilities.prerequisiteRefs",
            &self.prerequisite_refs,
        )?;
        require_nonempty(
            "engine growth justification capabilities.evidenceRefs",
            self.evidence_refs.len(),
        )?;
        validate_ref_list(
            "engine growth justification capabilities.evidenceRefs",
            &self.evidence_refs,
        )?;
        Ok(())
    }
}

fn validate_ref_list(label: &str, values: &[String]) -> Result<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        require_ref(label, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{label} contains duplicate ref `{value}`"));
        }
    }
    Ok(())
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.chars().any(char::is_whitespace) {
        return Err(anyhow!("{label} must not contain whitespace"));
    }
    Ok(())
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    Ok(())
}

fn require_nonempty(label: &str, count: usize) -> Result<()> {
    if count == 0 {
        return Err(anyhow!("{label} must not be empty"));
    }
    Ok(())
}
