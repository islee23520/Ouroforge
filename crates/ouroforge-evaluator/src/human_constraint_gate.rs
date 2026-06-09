//! Human Constraints as First-Class Gates v1 (#2066).
//!
//! Human constraints are opt-in intervention evidence, not prompt-only policy or
//! trusted writes. The Rust data plane validates the constraint records and
//! compiles them into a declared evaluator gate category that composes with the
//! existing `declared-gate-and` aggregation. Studio may capture and route these
//! records later, but this module owns the data-plane gate semantics.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;

pub const HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION: &str = "ouroforge.human-constraint-gate.v1";
pub const HUMAN_CONSTRAINT_GATE_BOUNDARY: &str = "human constraints as first-class gates; intervention-as-evidence; read + gated-write; Rust = data plane; Elixir/OTP + Phoenix LiveView = control + presentation; review/apply, scene/source-apply, evaluator, evidence/provenance gates reused; no raw bypass; local-first CLI fallback; loop completes without human; #1 and #23 remain open";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum HumanConstraintKind {
    ForbiddenMechanic,
    RequiredStyle,
    BudgetCap,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum HumanConstraintStatus {
    Active,
    Inactive,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum HumanConstraintGateState {
    Pass,
    Violation,
    BlockedConstraint,
    StaleConstraint,
    MalformedConstraint,
    MissingEvidence,
}

impl HumanConstraintGateState {
    pub fn is_pass(self) -> bool {
        matches!(self, HumanConstraintGateState::Pass)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HumanConstraintRecord {
    pub constraint_id: String,
    pub kind: HumanConstraintKind,
    pub status: HumanConstraintStatus,
    pub author: String,
    pub author_provenance_ref: String,
    pub target_ref: String,
    pub target_base_ref: String,
    pub normalized_constraint_ref: String,
    pub review_apply_ref: String,
    pub evaluator_evidence_ref: String,
    pub evidence_refs: Vec<String>,
    pub forbidden_mechanic: Option<String>,
    pub required_style: Option<String>,
    pub budget_cap: Option<u64>,
    pub intervention_as_evidence: bool,
    pub read_gated_write: bool,
    pub raw_bypass_requested: bool,
    pub direct_artifact_write: bool,
    pub studio_trusted_write_authority: bool,
    pub human_required_for_autonomous_loop: bool,
    pub cli_fallback_supported: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CandidateConstraintEvidence {
    pub candidate_id: String,
    pub target_ref: String,
    pub mechanics: Vec<String>,
    pub style: String,
    pub budget: u64,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HumanConstraintGateInput {
    pub schema_version: String,
    pub gate_id: String,
    pub candidate: CandidateConstraintEvidence,
    pub constraints: Vec<HumanConstraintRecord>,
    pub boundary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HumanConstraintGateVerdict {
    pub constraint_id: String,
    pub kind: HumanConstraintKind,
    pub state: HumanConstraintGateState,
    pub reason: String,
    pub evidence_refs: Vec<String>,
}

impl HumanConstraintGateInput {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "human constraint gate schemaVersion must be {HUMAN_CONSTRAINT_GATE_SCHEMA_VERSION}"
            ));
        }
        require_token("gateId", &self.gate_id)?;
        require_boundary(&self.boundary)?;
        self.candidate.validate()?;
        if self.constraints.is_empty() {
            return Err(anyhow!(
                "human constraint gate requires at least one constraint"
            ));
        }
        let mut ids = BTreeSet::new();
        for constraint in &self.constraints {
            constraint.validate()?;
            if !ids.insert(constraint.constraint_id.as_str()) {
                return Err(anyhow!(
                    "duplicate human constraint id: {}",
                    constraint.constraint_id
                ));
            }
        }
        Ok(())
    }
}

impl CandidateConstraintEvidence {
    fn validate(&self) -> Result<()> {
        require_token("candidateId", &self.candidate_id)?;
        require_ref("targetRef", &self.target_ref)?;
        require_refs("candidate evidenceRefs", &self.evidence_refs)?;
        require_clean_text("style", &self.style)?;
        for mechanic in &self.mechanics {
            require_clean_text("mechanic", mechanic)?;
        }
        Ok(())
    }
}

impl HumanConstraintRecord {
    fn validate(&self) -> Result<()> {
        require_token("constraintId", &self.constraint_id)?;
        if self.author != "human" && !self.author.starts_with("human:") {
            return Err(anyhow!(
                "human constraint author must record author=human provenance"
            ));
        }
        require_ref("authorProvenanceRef", &self.author_provenance_ref)?;
        require_ref("targetRef", &self.target_ref)?;
        require_ref("targetBaseRef", &self.target_base_ref)?;
        require_ref("normalizedConstraintRef", &self.normalized_constraint_ref)?;
        require_ref("reviewApplyRef", &self.review_apply_ref)?;
        require_ref("evaluatorEvidenceRef", &self.evaluator_evidence_ref)?;
        require_refs("constraint evidenceRefs", &self.evidence_refs)?;
        if !self.intervention_as_evidence || !self.read_gated_write {
            return Err(anyhow!(
                "human constraints must be intervention-as-evidence and read + gated-write"
            ));
        }
        if self.raw_bypass_requested
            || self.direct_artifact_write
            || self.studio_trusted_write_authority
            || self.human_required_for_autonomous_loop
            || !self.cli_fallback_supported
        {
            return Err(anyhow!(
                "human constraint gate forbids raw bypass, direct writes, trusted Studio writes, mandatory humans, or broken CLI fallback"
            ));
        }
        match self.kind {
            HumanConstraintKind::ForbiddenMechanic => {
                require_clean_text(
                    "forbiddenMechanic",
                    self.forbidden_mechanic.as_deref().unwrap_or(""),
                )?;
                if self.required_style.is_some() || self.budget_cap.is_some() {
                    return Err(anyhow!(
                        "forbidden-mechanic constraints cannot carry style or budget payloads"
                    ));
                }
            }
            HumanConstraintKind::RequiredStyle => {
                require_clean_text(
                    "requiredStyle",
                    self.required_style.as_deref().unwrap_or(""),
                )?;
                if self.forbidden_mechanic.is_some() || self.budget_cap.is_some() {
                    return Err(anyhow!(
                        "required-style constraints cannot carry mechanic or budget payloads"
                    ));
                }
            }
            HumanConstraintKind::BudgetCap => {
                let Some(cap) = self.budget_cap else {
                    return Err(anyhow!("budget-cap constraints require budgetCap"));
                };
                if cap == 0 {
                    return Err(anyhow!("budgetCap must be positive"));
                }
                if self.forbidden_mechanic.is_some() || self.required_style.is_some() {
                    return Err(anyhow!(
                        "budget-cap constraints cannot carry mechanic or style payloads"
                    ));
                }
            }
        }
        Ok(())
    }
}

pub fn evaluate_human_constraint_gate(
    input: &HumanConstraintGateInput,
) -> Vec<HumanConstraintGateVerdict> {
    if let Err(err) = input.validate() {
        return vec![HumanConstraintGateVerdict {
            constraint_id: input.gate_id.clone(),
            kind: HumanConstraintKind::ForbiddenMechanic,
            state: HumanConstraintGateState::MalformedConstraint,
            reason: format!("human constraint gate input is invalid: {err:#}"),
            evidence_refs: input.candidate.evidence_refs.clone(),
        }];
    }

    input
        .constraints
        .iter()
        .map(|constraint| evaluate_constraint(constraint, &input.candidate))
        .collect()
}

pub fn human_constraint_gate_category(verdicts: &[HumanConstraintGateVerdict]) -> Option<Value> {
    if verdicts.is_empty() {
        return None;
    }
    let failed = verdicts.iter().filter(|v| !v.state.is_pass()).count();
    Some(json!({
        "declared": true,
        "status": if failed == 0 { "pass" } else { "fail" },
        "resultCount": verdicts.len(),
        "failureCount": failed
    }))
}

pub fn compose_human_constraints_into_categories(
    categories: &mut Value,
    verdicts: &[HumanConstraintGateVerdict],
) -> bool {
    let Some(category) = human_constraint_gate_category(verdicts) else {
        return false;
    };
    if let Some(object) = categories.as_object_mut() {
        object.insert("humanConstraints".to_string(), category);
        true
    } else {
        false
    }
}

fn evaluate_constraint(
    constraint: &HumanConstraintRecord,
    candidate: &CandidateConstraintEvidence,
) -> HumanConstraintGateVerdict {
    let mut evidence_refs = constraint.evidence_refs.clone();
    evidence_refs.push(constraint.evaluator_evidence_ref.clone());
    evidence_refs.extend(candidate.evidence_refs.clone());
    evidence_refs.sort();
    evidence_refs.dedup();

    let (state, reason) = match constraint.status {
        HumanConstraintStatus::Inactive => (
            HumanConstraintGateState::Pass,
            "inactive human constraint is recorded as evidence and neutral for this candidate"
                .to_string(),
        ),
        HumanConstraintStatus::Blocked => (
            HumanConstraintGateState::BlockedConstraint,
            "human constraint is blocked and cannot pass the evaluator gate".to_string(),
        ),
        HumanConstraintStatus::Stale => (
            HumanConstraintGateState::StaleConstraint,
            "human constraint evidence is stale and fails closed".to_string(),
        ),
        HumanConstraintStatus::Active => evaluate_active_constraint(constraint, candidate),
    };

    HumanConstraintGateVerdict {
        constraint_id: constraint.constraint_id.clone(),
        kind: constraint.kind,
        state,
        reason,
        evidence_refs,
    }
}

fn evaluate_active_constraint(
    constraint: &HumanConstraintRecord,
    candidate: &CandidateConstraintEvidence,
) -> (HumanConstraintGateState, String) {
    match constraint.kind {
        HumanConstraintKind::ForbiddenMechanic => {
            let forbidden = constraint.forbidden_mechanic.as_deref().unwrap_or_default();
            if candidate
                .mechanics
                .iter()
                .any(|mechanic| mechanic.eq_ignore_ascii_case(forbidden))
            {
                (
                    HumanConstraintGateState::Violation,
                    format!(
                        "candidate {} violates human constraint {}: forbidden mechanic {forbidden}",
                        candidate.candidate_id, constraint.constraint_id
                    ),
                )
            } else {
                (
                    HumanConstraintGateState::Pass,
                    format!("candidate does not use forbidden mechanic {forbidden}"),
                )
            }
        }
        HumanConstraintKind::RequiredStyle => {
            let required = constraint.required_style.as_deref().unwrap_or_default();
            if candidate.style.eq_ignore_ascii_case(required) {
                (
                    HumanConstraintGateState::Pass,
                    format!("candidate style matches required style {required}"),
                )
            } else {
                (
                    HumanConstraintGateState::Violation,
                    format!(
                        "candidate {} violates human constraint {}: required style {required}, observed {}",
                        candidate.candidate_id, constraint.constraint_id, candidate.style
                    ),
                )
            }
        }
        HumanConstraintKind::BudgetCap => {
            let cap = constraint.budget_cap.unwrap_or(0);
            if candidate.budget <= cap {
                (
                    HumanConstraintGateState::Pass,
                    format!("candidate budget {} is within cap {cap}", candidate.budget),
                )
            } else {
                (
                    HumanConstraintGateState::Violation,
                    format!(
                        "candidate {} violates human constraint {}: budget {} exceeds cap {cap}",
                        candidate.candidate_id, constraint.constraint_id, candidate.budget
                    ),
                )
            }
        }
    }
}

fn require_boundary(boundary: &str) -> Result<()> {
    for token in [
        "human constraints as first-class gates",
        "intervention-as-evidence",
        "read + gated-write",
        "Rust = data plane",
        "Elixir/OTP + Phoenix LiveView = control + presentation",
        "review/apply",
        "scene/source-apply",
        "evaluator",
        "evidence/provenance",
        "no raw bypass",
        "local-first CLI fallback",
        "loop completes without human",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(token) {
            return Err(anyhow!("human constraint boundary missing token: {token}"));
        }
    }
    Ok(())
}

fn require_token(label: &str, value: &str) -> Result<()> {
    require_clean_text(label, value)?;
    if value.contains('/') || value.contains('\\') || value.contains("..") {
        return Err(anyhow!("{label} must be a safe token"));
    }
    Ok(())
}

fn require_ref(label: &str, value: &str) -> Result<()> {
    require_clean_text(label, value)?;
    if value.starts_with('/') || value.contains('\\') || value.contains("..") {
        return Err(anyhow!("{label} must be a safe repo-relative ref"));
    }
    Ok(())
}

fn require_refs(label: &str, refs: &[String]) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    for reference in refs {
        require_ref(label, reference)?;
    }
    Ok(())
}

fn require_clean_text(label: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    let lower = trimmed.to_ascii_lowercase();
    if lower.contains("raw_write_bypass") || lower.contains("raw_apply_bypass") {
        return Err(anyhow!("{label} contains raw bypass language"));
    }
    Ok(())
}
