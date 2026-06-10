//! Production usability gate contract for M130 (#2391-#2394).
//!
//! The gate is an evidence index: it validates that the final product-observed
//! claim keeps each M130 issue ordered, evidence-linked, and honest about any
//! manual gaps. It does not execute Studio, write trusted files, publish, or
//! replace generated browser evidence.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCTION_USABILITY_GATE_SCHEMA_VERSION: &str = "production-usability-gate-v1";
pub const PRODUCTION_USABILITY_SCENARIO_COVERAGE: &str = "scenario-coverage-v111";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProductionUsabilityVerdict {
    ProductObservedComplete,
    ProductObservedFail,
    RegressionRecorded,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProductionUsabilityPhaseKind {
    NewGameWorkflow,
    StudioEditRerun,
    LocalPackageExport,
    PostmortemGovernance,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonVerdict {
    Improvement,
    Regression,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ManualGapLedgerEntry {
    #[serde(rename = "gapId")]
    pub gap_id: String,
    #[serde(rename = "ownerIssue")]
    pub owner_issue: String,
    pub summary: String,
    #[serde(rename = "followUp")]
    pub follow_up: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionUsabilityPhase {
    pub issue: u64,
    pub kind: ProductionUsabilityPhaseKind,
    pub verdict: ProductionUsabilityVerdict,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "screenshotRefs", default)]
    pub screenshot_refs: Vec<String>,
    #[serde(rename = "workflowTranscriptRefs", default)]
    pub workflow_transcript_refs: Vec<String>,
    #[serde(rename = "manualGaps", default)]
    pub manual_gaps: Vec<ManualGapLedgerEntry>,
    #[serde(
        rename = "comparisonVerdict",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub comparison_verdict: Option<ComparisonVerdict>,
    #[serde(rename = "packageRefs", default)]
    pub package_refs: Vec<String>,
    #[serde(rename = "governanceRefs", default)]
    pub governance_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProductionUsabilityGate {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "gateId")]
    pub gate_id: String,
    #[serde(rename = "scenarioCoverageSuite")]
    pub scenario_coverage_suite: String,
    #[serde(rename = "targetClassification")]
    pub target_classification: ProductionUsabilityVerdict,
    pub phases: Vec<ProductionUsabilityPhase>,
    #[serde(rename = "generatedStateRoots")]
    pub generated_state_roots: Vec<String>,
    #[serde(rename = "anchorsRemainOpen")]
    pub anchors_remain_open: Vec<u64>,
    pub guardrails: Vec<String>,
}

impl ProductionUsabilityGate {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCTION_USABILITY_GATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "production usability gate schemaVersion must be {PRODUCTION_USABILITY_GATE_SCHEMA_VERSION}"
            ));
        }
        require_id("production usability gate gateId", &self.gate_id)?;
        if self.scenario_coverage_suite != PRODUCTION_USABILITY_SCENARIO_COVERAGE {
            return Err(anyhow!(
                "production usability gate must declare {PRODUCTION_USABILITY_SCENARIO_COVERAGE}"
            ));
        }
        if self.target_classification != ProductionUsabilityVerdict::ProductObservedComplete {
            return Err(anyhow!(
                "M130 target classification must be product-observed-complete or record issue-level failure before closure"
            ));
        }
        let expected_issues = [2391, 2392, 2393, 2394];
        if self.phases.len() != expected_issues.len() {
            return Err(anyhow!(
                "production usability gate must include #2391-#2394 phases"
            ));
        }
        for (phase, expected_issue) in self.phases.iter().zip(expected_issues) {
            if phase.issue != expected_issue {
                return Err(anyhow!(
                    "production usability gate phase order must be #2391 -> #2392 -> #2393 -> #2394"
                ));
            }
            phase.validate()?;
            if phase.verdict != ProductionUsabilityVerdict::ProductObservedComplete {
                return Err(anyhow!(
                    "product-observed-complete gate cannot include non-complete phase #{}",
                    phase.issue
                ));
            }
        }
        if self.anchors_remain_open != vec![1, 23] {
            return Err(anyhow!(
                "production usability gate must preserve #1 and #23 as open anchors"
            ));
        }
        require_generated_root(&self.generated_state_roots, "runs")?;
        require_generated_root(&self.generated_state_roots, "screenshots")?;
        require_generated_root(&self.generated_state_roots, "browser-profiles")?;
        require_generated_root(&self.generated_state_roots, "dist")?;
        if !self
            .guardrails
            .iter()
            .any(|g| g.contains("no new distribution"))
        {
            return Err(anyhow!(
                "production usability gate guardrails must forbid new distribution scope"
            ));
        }
        if !self.guardrails.iter().any(|g| g.contains("no marketing")) {
            return Err(anyhow!(
                "production usability gate guardrails must forbid marketing overclaims"
            ));
        }
        Ok(())
    }
}

impl ProductionUsabilityPhase {
    pub fn validate(&self) -> Result<()> {
        if self.evidence_refs.is_empty() {
            return Err(anyhow!("M130 phase #{} must link evidence", self.issue));
        }
        validate_refs("M130 phase evidenceRefs", &self.evidence_refs, true)?;
        validate_refs("M130 phase screenshotRefs", &self.screenshot_refs, false)?;
        validate_refs(
            "M130 phase workflowTranscriptRefs",
            &self.workflow_transcript_refs,
            false,
        )?;
        validate_refs("M130 phase packageRefs", &self.package_refs, false)?;
        validate_refs("M130 phase governanceRefs", &self.governance_refs, false)?;
        match self.issue {
            2391 => {
                require_kind(self, ProductionUsabilityPhaseKind::NewGameWorkflow)?;
                if self.workflow_transcript_refs.is_empty() || self.screenshot_refs.is_empty() {
                    return Err(anyhow!(
                        "#2391 requires workflow transcript and screenshot refs"
                    ));
                }
                if self.manual_gaps.is_empty() {
                    return Err(anyhow!(
                        "#2391 must enumerate every manual step in the gap ledger"
                    ));
                }
            }
            2392 => {
                require_kind(self, ProductionUsabilityPhaseKind::StudioEditRerun)?;
                if self.comparison_verdict.is_none() {
                    return Err(anyhow!(
                        "#2392 requires an improvement or regression comparison verdict"
                    ));
                }
                if self.comparison_verdict == Some(ComparisonVerdict::Regression)
                    && self.verdict == ProductionUsabilityVerdict::ProductObservedComplete
                {
                    return Err(anyhow!(
                        "#2392 regression comparison cannot be product-observed-complete"
                    ));
                }
            }
            2393 => {
                require_kind(self, ProductionUsabilityPhaseKind::LocalPackageExport)?;
                if self.package_refs.is_empty() {
                    return Err(anyhow!("#2393 requires local package/provenance refs"));
                }
            }
            2394 => {
                require_kind(self, ProductionUsabilityPhaseKind::PostmortemGovernance)?;
                if self.governance_refs.is_empty() {
                    return Err(anyhow!("#2394 requires #1/roadmap/backlog governance refs"));
                }
            }
            other => return Err(anyhow!("unexpected M130 phase issue #{other}")),
        }
        let mut gaps = BTreeSet::new();
        for gap in &self.manual_gaps {
            gap.validate()?;
            if !gaps.insert(gap.gap_id.as_str()) {
                return Err(anyhow!("duplicate manual gap `{}`", gap.gap_id));
            }
        }
        Ok(())
    }
}

impl ManualGapLedgerEntry {
    pub fn validate(&self) -> Result<()> {
        require_id("manual gap gapId", &self.gap_id)?;
        require_ref("manual gap ownerIssue", &self.owner_issue)?;
        require_text("manual gap summary", &self.summary)?;
        require_text("manual gap followUp", &self.follow_up)?;
        Ok(())
    }
}

fn require_kind(
    phase: &ProductionUsabilityPhase,
    expected: ProductionUsabilityPhaseKind,
) -> Result<()> {
    if phase.kind != expected {
        return Err(anyhow!("M130 phase #{} has wrong kind", phase.issue));
    }
    Ok(())
}

fn require_generated_root(roots: &[String], expected: &str) -> Result<()> {
    if roots.iter().any(|root| root == expected) {
        Ok(())
    } else {
        Err(anyhow!("missing generated state root `{expected}`"))
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
