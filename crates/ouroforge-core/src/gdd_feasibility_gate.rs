use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_FEASIBILITY_GATE_SCHEMA_VERSION: &str = "gdd-feasibility-gate-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddFeasibilityState {
    Pass,
    Fail,
    Defer,
    Downgrade,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddFeasibilityRiskKind {
    UnsupportedMechanic,
    ScopeTooLarge,
    UnclearAcceptance,
    AssetSource,
    MissingScenario,
    MissingPrerequisite,
    BoundaryDrift,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeScopeLimit {
    #[serde(rename = "maxScenes")]
    pub max_scenes: u32,
    #[serde(rename = "maxLevels")]
    pub max_levels: u32,
    #[serde(rename = "maxEntities")]
    pub max_entities: u32,
    #[serde(rename = "maxAssets")]
    pub max_assets: u32,
    #[serde(rename = "maxMechanics")]
    pub max_mechanics: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddFeasibilityGateArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "gateId")]
    pub gate_id: String,
    pub state: GddFeasibilityState,
    #[serde(rename = "mechanicsMappingRef")]
    pub mechanics_mapping_ref: String,
    #[serde(rename = "targetPrototypeSize")]
    pub target_prototype_size: GddPrototypeScopeLimit,
    #[serde(rename = "supportedMechanics")]
    pub supported_mechanics: Vec<String>,
    #[serde(rename = "requiredPriorMilestones")]
    pub required_prior_milestones: Vec<GddPriorMilestoneRef>,
    #[serde(rename = "acceptanceCriteriaRefs")]
    pub acceptance_criteria_refs: Vec<String>,
    #[serde(rename = "scenarioPlanRefs")]
    pub scenario_plan_refs: Vec<String>,
    #[serde(rename = "riskFlags")]
    pub risk_flags: Vec<GddFeasibilityRiskFlag>,
    #[serde(rename = "knownGaps")]
    pub known_gaps: Vec<String>,
    #[serde(
        rename = "sliceRecommendation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slice_recommendation: Option<GddFeasibilitySliceRecommendation>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPriorMilestoneRef {
    pub id: String,
    #[serde(rename = "docRef")]
    pub doc_ref: String,
    pub satisfied: bool,
    #[serde(
        rename = "blockedReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddFeasibilityRiskFlag {
    pub id: String,
    pub kind: GddFeasibilityRiskKind,
    pub severity: String,
    pub summary: String,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    #[serde(
        rename = "blockedReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddFeasibilitySliceRecommendation {
    pub id: String,
    pub action: String,
    pub summary: String,
    #[serde(rename = "includedRequirementRefs")]
    pub included_requirement_refs: Vec<String>,
    #[serde(rename = "deferredRequirementRefs")]
    pub deferred_requirement_refs: Vec<String>,
    #[serde(rename = "reviewRequired")]
    pub review_required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddFeasibilityGateReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "gateId")]
    pub gate_id: String,
    pub state: String,
    #[serde(rename = "riskCount")]
    pub risk_count: usize,
    #[serde(rename = "blockedRiskCount")]
    pub blocked_risk_count: usize,
    #[serde(rename = "knownGapCount")]
    pub known_gap_count: usize,
    #[serde(rename = "supportedMechanicCount")]
    pub supported_mechanic_count: usize,
    #[serde(rename = "riskKindCounts")]
    pub risk_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddFeasibilityGateArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse GDD Feasibility Gate JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddFeasibilityGateReadModel {
        let mut risk_kind_counts = BTreeMap::new();
        let mut blocked_risk_count = 0;
        for risk in &self.risk_flags {
            *risk_kind_counts
                .entry(risk_kind_label(&risk.kind).to_string())
                .or_insert(0) += 1;
            if risk.blocked_reason.is_some() {
                blocked_risk_count += 1;
            }
        }
        GddFeasibilityGateReadModel {
            schema_version: self.schema_version.clone(),
            gate_id: self.gate_id.clone(),
            state: state_label(&self.state).to_string(),
            risk_count: self.risk_flags.len(),
            blocked_risk_count,
            known_gap_count: self.known_gaps.len(),
            supported_mechanic_count: self.supported_mechanics.len(),
            risk_kind_counts,
            validation_summary: vec![
                "feasibility decisions are tied to mechanics mapping and scope limits".to_string(),
                "prototype planning starts only after pass or accepted bounded slice".to_string(),
                "unsupported or overbroad work remains visible as risk, defer, downgrade, fail, or blocked state".to_string(),
                "display-only read model; no generation or apply authority".to_string(),
            ],
            compatibility_notes: vec![
                "keeps GDD, requirements, mechanics mapping, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts separate".to_string(),
                "browser/dashboard/Studio consumers stay read-only or draft-only".to_string(),
                "no source/script mutation, asset generation, command bridge, auto-apply, auto-merge, or engine replacement claim".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD feasibility gate read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_FEASIBILITY_GATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD feasibility gate schemaVersion must be {GDD_FEASIBILITY_GATE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("GDD feasibility gate gateId", &self.gate_id)?;
        require_local_ref(
            "GDD feasibility gate mechanicsMappingRef",
            &self.mechanics_mapping_ref,
        )?;
        self.target_prototype_size.validate()?;
        validate_local_id_list(
            "GDD feasibility gate supportedMechanics",
            &self.supported_mechanics,
        )?;
        require_nonempty(
            "GDD feasibility gate requiredPriorMilestones",
            self.required_prior_milestones.len(),
        )?;
        for milestone in &self.required_prior_milestones {
            milestone.validate()?;
        }
        validate_local_ref_list(
            "GDD feasibility gate acceptanceCriteriaRefs",
            &self.acceptance_criteria_refs,
        )?;
        validate_local_ref_list(
            "GDD feasibility gate scenarioPlanRefs",
            &self.scenario_plan_refs,
        )?;
        let mut risk_ids = BTreeSet::new();
        for risk in &self.risk_flags {
            risk.validate()?;
            if !risk_ids.insert(risk.id.clone()) {
                return Err(anyhow!(
                    "GDD feasibility gate riskFlags.id `{}` is duplicated",
                    risk.id
                ));
            }
        }
        validate_string_list("GDD feasibility gate knownGaps", &self.known_gaps)?;
        if let Some(slice) = &self.slice_recommendation {
            slice.validate()?;
        }

        for milestone in &self.required_prior_milestones {
            if !milestone.satisfied && milestone.blocked_reason.is_none() {
                return Err(anyhow!(
                    "GDD feasibility gate unsatisfied milestone `{}` must include blockedReason",
                    milestone.id
                ));
            }
        }
        if self.acceptance_criteria_refs.is_empty() {
            require_risk_kind(
                self,
                GddFeasibilityRiskKind::UnclearAcceptance,
                "missing acceptance criteria",
            )?;
        }
        if self.scenario_plan_refs.is_empty() {
            require_risk_kind(
                self,
                GddFeasibilityRiskKind::MissingScenario,
                "missing scenario plan",
            )?;
        }
        if self.supported_mechanics.is_empty() {
            require_risk_kind(
                self,
                GddFeasibilityRiskKind::UnsupportedMechanic,
                "missing supported mechanics",
            )?;
        }
        if self.target_prototype_size.is_overlarge() {
            require_risk_kind(
                self,
                GddFeasibilityRiskKind::ScopeTooLarge,
                "overlarge scope",
            )?;
        }
        for risk in &self.risk_flags {
            if matches!(
                risk.kind,
                GddFeasibilityRiskKind::AssetSource
                    | GddFeasibilityRiskKind::UnsupportedMechanic
                    | GddFeasibilityRiskKind::MissingPrerequisite
            ) && risk.blocked_reason.is_none()
            {
                return Err(anyhow!(
                    "GDD feasibility gate risk `{}` must include blockedReason",
                    risk.id
                ));
            }
        }
        match self.state {
            GddFeasibilityState::Pass => {
                if !self.risk_flags.is_empty()
                    || self.slice_recommendation.is_some()
                    || self.target_prototype_size.is_overlarge()
                {
                    return Err(anyhow!(
                        "pass GDD feasibility gate must have no risks, overlarge scope, or slice recommendation"
                    ));
                }
                require_nonempty(
                    "pass GDD feasibility gate supportedMechanics",
                    self.supported_mechanics.len(),
                )?;
                require_nonempty(
                    "pass GDD feasibility gate acceptanceCriteriaRefs",
                    self.acceptance_criteria_refs.len(),
                )?;
                require_nonempty(
                    "pass GDD feasibility gate scenarioPlanRefs",
                    self.scenario_plan_refs.len(),
                )?;
                if self.required_prior_milestones.iter().any(|m| !m.satisfied) {
                    return Err(anyhow!(
                        "pass GDD feasibility gate requires all prior milestones satisfied"
                    ));
                }
            }
            GddFeasibilityState::Fail | GddFeasibilityState::Blocked => {
                require_nonempty(
                    "failed/blocked GDD feasibility gate riskFlags",
                    self.risk_flags.len(),
                )?;
                if !self
                    .risk_flags
                    .iter()
                    .any(|risk| risk.blocked_reason.is_some())
                {
                    return Err(anyhow!(
                        "failed/blocked GDD feasibility gate requires blocked risks"
                    ));
                }
            }
            GddFeasibilityState::Defer | GddFeasibilityState::Downgrade => {
                if self.slice_recommendation.is_none() {
                    return Err(anyhow!(
                        "defer/downgrade GDD feasibility gate requires sliceRecommendation"
                    ));
                }
                require_nonempty(
                    "defer/downgrade GDD feasibility gate riskFlags",
                    self.risk_flags.len(),
                )?;
            }
        }
        require_text("GDD feasibility gate boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "prototype planning starts only after feasibility passes or accepted bounded slice",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no autonomous unrestricted game creation",
            "not generation authority",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD feasibility gate boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl GddPrototypeScopeLimit {
    fn validate(&self) -> Result<()> {
        if self.max_scenes == 0
            || self.max_levels == 0
            || self.max_entities == 0
            || self.max_assets == 0
            || self.max_mechanics == 0
        {
            return Err(anyhow!(
                "GDD feasibility gate targetPrototypeSize limits must be positive"
            ));
        }
        Ok(())
    }

    fn is_overlarge(&self) -> bool {
        self.max_scenes > 3
            || self.max_levels > 3
            || self.max_entities > 64
            || self.max_assets > 32
            || self.max_mechanics > 6
    }
}

impl GddPriorMilestoneRef {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD feasibility gate requiredPriorMilestones.id", &self.id)?;
        require_local_ref(
            "GDD feasibility gate requiredPriorMilestones.docRef",
            &self.doc_ref,
        )?;
        if let Some(reason) = &self.blocked_reason {
            require_text(
                "GDD feasibility gate requiredPriorMilestones.blockedReason",
                reason,
            )?;
        }
        Ok(())
    }
}

impl GddFeasibilityRiskFlag {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD feasibility gate riskFlags.id", &self.id)?;
        require_text("GDD feasibility gate riskFlags.severity", &self.severity)?;
        let severity = self.severity.to_ascii_lowercase();
        if !["low", "medium", "high", "blocking"].contains(&severity.as_str()) {
            return Err(anyhow!(
                "GDD feasibility gate riskFlags.severity must be low, medium, high, or blocking"
            ));
        }
        require_text("GDD feasibility gate riskFlags.summary", &self.summary)?;
        if let Some(reason) = &self.blocked_reason {
            require_text("GDD feasibility gate riskFlags.blockedReason", reason)?;
        }
        require_local_ref(
            "GDD feasibility gate riskFlags.evidenceRef",
            &self.evidence_ref,
        )?;
        Ok(())
    }
}

impl GddFeasibilitySliceRecommendation {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD feasibility gate sliceRecommendation.id", &self.id)?;
        require_text(
            "GDD feasibility gate sliceRecommendation.action",
            &self.action,
        )?;
        let action = self.action.to_ascii_lowercase();
        if !(action.contains("slice")
            || action.contains("downgrade")
            || action.contains("defer")
            || action.contains("block"))
        {
            return Err(anyhow!(
                "GDD feasibility gate sliceRecommendation.action must slice, downgrade, defer, or block"
            ));
        }
        require_text(
            "GDD feasibility gate sliceRecommendation.summary",
            &self.summary,
        )?;
        validate_local_id_list(
            "GDD feasibility gate sliceRecommendation.includedRequirementRefs",
            &self.included_requirement_refs,
        )?;
        validate_local_id_list(
            "GDD feasibility gate sliceRecommendation.deferredRequirementRefs",
            &self.deferred_requirement_refs,
        )?;
        if !self.review_required {
            return Err(anyhow!(
                "GDD feasibility gate sliceRecommendation requires reviewRequired"
            ));
        }
        if self.included_requirement_refs.is_empty() && self.deferred_requirement_refs.is_empty() {
            return Err(anyhow!(
                "GDD feasibility gate sliceRecommendation must include or defer requirement refs"
            ));
        }
        Ok(())
    }
}

fn require_risk_kind(
    artifact: &GddFeasibilityGateArtifact,
    kind: GddFeasibilityRiskKind,
    label: &str,
) -> Result<()> {
    if !artifact.risk_flags.iter().any(|risk| risk.kind == kind) {
        return Err(anyhow!("GDD feasibility gate must flag {label}"));
    }
    Ok(())
}

fn state_label(state: &GddFeasibilityState) -> &'static str {
    match state {
        GddFeasibilityState::Pass => "pass",
        GddFeasibilityState::Fail => "fail",
        GddFeasibilityState::Defer => "defer",
        GddFeasibilityState::Downgrade => "downgrade",
        GddFeasibilityState::Blocked => "blocked",
    }
}

fn risk_kind_label(kind: &GddFeasibilityRiskKind) -> &'static str {
    match kind {
        GddFeasibilityRiskKind::UnsupportedMechanic => "unsupported-mechanic",
        GddFeasibilityRiskKind::ScopeTooLarge => "scope-too-large",
        GddFeasibilityRiskKind::UnclearAcceptance => "unclear-acceptance",
        GddFeasibilityRiskKind::AssetSource => "asset-source",
        GddFeasibilityRiskKind::MissingScenario => "missing-scenario",
        GddFeasibilityRiskKind::MissingPrerequisite => "missing-prerequisite",
        GddFeasibilityRiskKind::BoundaryDrift => "boundary-drift",
    }
}

fn validate_string_list(field: &str, values: &[String]) -> Result<()> {
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn validate_local_id_list(field: &str, values: &[String]) -> Result<()> {
    for value in values {
        require_local_id(field, value)?;
    }
    Ok(())
}

fn validate_local_ref_list(field: &str, values: &[String]) -> Result<()> {
    for value in values {
        require_local_ref(field, value)?;
    }
    Ok(())
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
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
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!(
            "{field} must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/"))
    {
        return Err(anyhow!("{field} must use examples/, docs/, or seeds/ refs"));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic import",
        "command bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
        "autonomous unrestricted game creation",
        "native export",
        "plugin runtime",
        "asset generation",
        "full game",
        "commercial readiness",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD/feasibility authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 6] = ["no ", "not ", "without ", "avoid ", "forbid ", "forbidden "];
    let hay = value;
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed), while a single leading negation still covers a
    // list such as `no auto-apply or self-approval`.
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
        let preceding = &hay[clause_start..idx];
        let negated = NEGATIONS.iter().any(|n| preceding.contains(n));
        if !negated {
            return true;
        }
        search_start = idx + phrase.len();
    }
    false
}
