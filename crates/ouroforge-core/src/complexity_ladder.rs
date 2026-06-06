//! Complexity Ladder Model and Capability Gates v1 (#1494).
//!
//! This is a local read model over loop-produced evidence. It does not execute
//! demos, evaluate runs, or expand the runtime engine.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const COMPLEXITY_LADDER_SCHEMA_VERSION: &str = "complexity-ladder-gates-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComplexityLadder {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "ladderId")]
    pub ladder_id: String,
    pub rungs: Vec<ComplexityRung>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComplexityRung {
    pub order: u32,
    #[serde(rename = "rungId")]
    pub rung_id: String,
    #[serde(rename = "gameClass")]
    pub game_class: String,
    #[serde(rename = "requiredCapabilities")]
    pub required_capabilities: Vec<String>,
    #[serde(rename = "capabilityGate")]
    pub capability_gate: CapabilityGateEvidence,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CapabilityGateEvidence {
    #[serde(rename = "claimedStatus")]
    pub claimed_status: ComplexityRungStatus,
    #[serde(rename = "loopProducedDemo", default)]
    pub loop_produced_demo: bool,
    #[serde(rename = "demoRef", default, skip_serializing_if = "Option::is_none")]
    pub demo_ref: Option<String>,
    #[serde(rename = "demoRefState", default)]
    pub demo_ref_state: EvidenceRefState,
    #[serde(rename = "fourGate", default, skip_serializing_if = "Option::is_none")]
    pub four_gate: Option<FourGateEvidence>,
    #[serde(
        rename = "loopCoverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub loop_coverage: Option<LoopCoverageEvidence>,
}

impl Default for CapabilityGateEvidence {
    fn default() -> Self {
        Self {
            claimed_status: ComplexityRungStatus::InsufficientEvidence,
            loop_produced_demo: false,
            demo_ref: None,
            demo_ref_state: EvidenceRefState::Missing,
            four_gate: None,
            loop_coverage: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FourGateEvidence {
    #[serde(rename = "verdictRef")]
    pub verdict_ref: String,
    #[serde(rename = "verdictRefState", default)]
    pub verdict_ref_state: EvidenceRefState,
    pub mechanical: GateEvidenceStatus,
    pub runtime: GateEvidenceStatus,
    pub visual: GateEvidenceStatus,
    pub semantic: GateEvidenceStatus,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LoopCoverageEvidence {
    #[serde(rename = "verdictRef")]
    pub verdict_ref: String,
    #[serde(rename = "verdictRefState", default)]
    pub verdict_ref_state: EvidenceRefState,
    pub status: GateEvidenceStatus,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ComplexityRungStatus {
    Satisfied,
    Unsatisfied,
    InsufficientEvidence,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceRefState {
    Current,
    #[default]
    Missing,
    StaleRef,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GateEvidenceStatus {
    Pass,
    Fail,
    Missing,
    StaleRef,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComplexityLadderEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "ladderId")]
    pub ladder_id: String,
    pub rungs: Vec<ComplexityRungEvaluation>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComplexityRungEvaluation {
    pub order: u32,
    #[serde(rename = "rungId")]
    pub rung_id: String,
    pub status: ComplexityRungStatus,
    pub reasons: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

impl ComplexityLadder {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let ladder: Self =
            serde_json::from_str(input).context("failed to parse Complexity Ladder JSON")?;
        ladder.validate()?;
        Ok(ladder)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("failed to serialize complexity ladder")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != COMPLEXITY_LADDER_SCHEMA_VERSION {
            return Err(anyhow!(
                "complexity ladder schemaVersion must be {COMPLEXITY_LADDER_SCHEMA_VERSION}"
            ));
        }
        require_id("complexity ladder ladderId", &self.ladder_id)?;
        require_text("complexity ladder boundary", &self.boundary)?;
        for required in ["Rust/local", "read", "browser surfaces are read-only"] {
            if !self.boundary.contains(required) {
                return Err(anyhow!(
                    "complexity ladder boundary must state `{required}`"
                ));
            }
        }
        if self.rungs.is_empty() {
            return Err(anyhow!("complexity ladder requires at least one rung"));
        }

        let mut seen_orders = BTreeSet::new();
        let mut seen_rungs = BTreeSet::new();
        for (expected_order, rung) in (1..).zip(self.rungs.iter()) {
            rung.validate()?;
            if !seen_orders.insert(rung.order) {
                return Err(anyhow!(
                    "complexity ladder duplicate rung order `{}`",
                    rung.order
                ));
            }
            if !seen_rungs.insert(rung.rung_id.clone()) {
                return Err(anyhow!(
                    "complexity ladder duplicate rungId `{}`",
                    rung.rung_id
                ));
            }
            if rung.order != expected_order {
                return Err(anyhow!(
                    "complexity ladder rungs must be ordered contiguously from 1"
                ));
            }
        }
        Ok(())
    }
}

impl ComplexityRung {
    fn validate(&self) -> Result<()> {
        require_id("complexity ladder rungId", &self.rung_id)?;
        require_text("complexity ladder gameClass", &self.game_class)?;
        if self.required_capabilities.is_empty() {
            return Err(anyhow!(
                "complexity ladder rung `{}` requires capabilities",
                self.rung_id
            ));
        }
        let mut capabilities = BTreeSet::new();
        for capability in &self.required_capabilities {
            require_id("complexity ladder required capability", capability)?;
            if !capabilities.insert(capability) {
                return Err(anyhow!(
                    "complexity ladder rung `{}` duplicate capability `{}`",
                    self.rung_id,
                    capability
                ));
            }
        }
        self.capability_gate.validate(&self.rung_id)
    }
}

impl CapabilityGateEvidence {
    fn validate(&self, rung_id: &str) -> Result<()> {
        if let Some(demo_ref) = &self.demo_ref {
            require_ref("complexity ladder demoRef", demo_ref)?;
        }
        if let Some(four_gate) = &self.four_gate {
            four_gate.validate(rung_id)?;
        }
        if let Some(loop_coverage) = &self.loop_coverage {
            loop_coverage.validate(rung_id)?;
        }
        Ok(())
    }
}

impl FourGateEvidence {
    fn validate(&self, _rung_id: &str) -> Result<()> {
        require_ref("complexity ladder fourGate.verdictRef", &self.verdict_ref)
    }

    fn is_pass(&self) -> bool {
        self.verdict_ref_state == EvidenceRefState::Current
            && self.mechanical == GateEvidenceStatus::Pass
            && self.runtime == GateEvidenceStatus::Pass
            && self.visual == GateEvidenceStatus::Pass
            && self.semantic == GateEvidenceStatus::Pass
    }

    fn has_stale_ref(&self) -> bool {
        self.verdict_ref_state == EvidenceRefState::StaleRef
            || self.mechanical == GateEvidenceStatus::StaleRef
            || self.runtime == GateEvidenceStatus::StaleRef
            || self.visual == GateEvidenceStatus::StaleRef
            || self.semantic == GateEvidenceStatus::StaleRef
    }
}

impl LoopCoverageEvidence {
    fn validate(&self, _rung_id: &str) -> Result<()> {
        require_ref(
            "complexity ladder loopCoverage.verdictRef",
            &self.verdict_ref,
        )
    }

    fn is_pass(&self) -> bool {
        self.verdict_ref_state == EvidenceRefState::Current
            && self.status == GateEvidenceStatus::Pass
    }

    fn has_stale_ref(&self) -> bool {
        self.verdict_ref_state == EvidenceRefState::StaleRef
            || self.status == GateEvidenceStatus::StaleRef
    }
}

pub fn evaluate_complexity_ladder(ladder: &ComplexityLadder) -> Result<ComplexityLadderEvaluation> {
    ladder.validate()?;

    let mut rungs = Vec::new();
    let mut previous_satisfied = true;
    for rung in &ladder.rungs {
        let evaluation = evaluate_rung(rung)?;
        if rung.capability_gate.claimed_status == ComplexityRungStatus::Satisfied
            && !previous_satisfied
        {
            return Err(anyhow!(
                "out-of-order complexity rung claim `{}`: prior rung is not satisfied",
                rung.rung_id
            ));
        }
        previous_satisfied = evaluation.status == ComplexityRungStatus::Satisfied;
        rungs.push(evaluation);
    }

    Ok(ComplexityLadderEvaluation {
        schema_version: COMPLEXITY_LADDER_SCHEMA_VERSION.to_string(),
        ladder_id: ladder.ladder_id.clone(),
        rungs,
    })
}

fn evaluate_rung(rung: &ComplexityRung) -> Result<ComplexityRungEvaluation> {
    let gate = &rung.capability_gate;
    let mut reasons = Vec::new();
    let mut evidence_refs = Vec::new();

    if gate.demo_ref_state == EvidenceRefState::StaleRef {
        return Err(anyhow!(
            "stale-ref: complexity rung `{}` demoRef is stale",
            rung.rung_id
        ));
    }
    if let Some(four_gate) = &gate.four_gate {
        if four_gate.has_stale_ref() {
            return Err(anyhow!(
                "stale-ref: complexity rung `{}` four-gate verdict is stale",
                rung.rung_id
            ));
        }
        evidence_refs.push(four_gate.verdict_ref.clone());
    }
    if let Some(loop_coverage) = &gate.loop_coverage {
        if loop_coverage.has_stale_ref() {
            return Err(anyhow!(
                "stale-ref: complexity rung `{}` loop-coverage verdict is stale",
                rung.rung_id
            ));
        }
        evidence_refs.push(loop_coverage.verdict_ref.clone());
    }
    if let Some(demo_ref) = &gate.demo_ref {
        evidence_refs.push(demo_ref.clone());
    }

    if gate.claimed_status == ComplexityRungStatus::InsufficientEvidence
        && gate.demo_ref.is_none()
        && gate.four_gate.is_none()
        && gate.loop_coverage.is_none()
    {
        reasons.push("rung evidence is missing".to_string());
        return Ok(rung_evaluation(
            rung,
            ComplexityRungStatus::InsufficientEvidence,
            reasons,
            evidence_refs,
        ));
    }

    if !gate.loop_produced_demo || gate.demo_ref.is_none() {
        reasons.push("loop-produced demo evidence is required".to_string());
    }
    if gate.demo_ref.is_some() && gate.demo_ref_state != EvidenceRefState::Current {
        reasons.push("demoRef evidence must be current".to_string());
    }
    let Some(four_gate) = &gate.four_gate else {
        reasons.push("four-gate evidence is required".to_string());
        return Ok(rung_evaluation(
            rung,
            ComplexityRungStatus::InsufficientEvidence,
            reasons,
            evidence_refs,
        ));
    };
    let Some(loop_coverage) = &gate.loop_coverage else {
        reasons.push("loop-coverage verdict is required".to_string());
        return Ok(rung_evaluation(
            rung,
            ComplexityRungStatus::InsufficientEvidence,
            reasons,
            evidence_refs,
        ));
    };

    if !four_gate.is_pass() {
        reasons.push("four-gate evidence is not passing".to_string());
    }
    if !loop_coverage.is_pass() {
        reasons.push("loop-coverage verdict is not passing".to_string());
    }

    let status = if reasons.is_empty() {
        ComplexityRungStatus::Satisfied
    } else if four_gate.is_pass() && loop_coverage.is_pass() {
        ComplexityRungStatus::InsufficientEvidence
    } else {
        ComplexityRungStatus::Unsatisfied
    };

    Ok(rung_evaluation(rung, status, reasons, evidence_refs))
}

fn rung_evaluation(
    rung: &ComplexityRung,
    status: ComplexityRungStatus,
    reasons: Vec<String>,
    evidence_refs: Vec<String>,
) -> ComplexityRungEvaluation {
    ComplexityRungEvaluation {
        order: rung.order,
        rung_id: rung.rung_id.clone(),
        status,
        reasons,
        evidence_refs,
    }
}

fn require_id(field: &str, value: &str) -> Result<()> {
    let valid = !value.is_empty()
        && value.len() <= 128
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'));
    if !valid {
        return Err(anyhow!("{field} must be a stable non-empty id"));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 1024 {
        return Err(anyhow!("{field} must be non-empty text up to 1024 bytes"));
    }
    Ok(())
}

fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/')
        || value.contains("..")
        || value.starts_with("runs/")
        || value.starts_with("target/")
        || value.starts_with(".omx/")
    {
        return Err(anyhow!("{field} must be a safe tracked evidence ref"));
    }
    Ok(())
}
