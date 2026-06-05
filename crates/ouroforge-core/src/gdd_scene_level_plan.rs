use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_SCENE_LEVEL_PLAN_SCHEMA_VERSION: &str = "gdd-scene-level-plan-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddSceneLevelPlanStatus {
    Ready,
    Partial,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddSceneLevelCapabilityStatus {
    Supported,
    Unsupported,
    Deferred,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddSceneLevelTargetKind {
    Scene,
    Tilemap,
    LevelIntent,
    SceneGenerationPlan,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddSceneLevelPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: GddSceneLevelPlanStatus,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "scaffoldPlanRef")]
    pub scaffold_plan_ref: String,
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    #[serde(rename = "mechanicsMappingIds")]
    pub mechanics_mapping_ids: Vec<String>,
    #[serde(rename = "levelIntents")]
    pub level_intents: Vec<GddLevelIntentLink>,
    #[serde(rename = "sceneGenerationPlans")]
    pub scene_generation_plans: Vec<GddSceneGenerationPlanLink>,
    #[serde(rename = "objectivePlacements")]
    pub objective_placements: Vec<GddObjectivePlacementPlan>,
    #[serde(rename = "progressionPlan")]
    pub progression_plan: Vec<GddProgressionStep>,
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<GddSceneLevelTargetRef>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<GddSceneLevelExpectedEvidence>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddTraceLink {
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    #[serde(rename = "mechanicsMappingId")]
    pub mechanics_mapping_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddLevelIntentLink {
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "levelIntentRef")]
    pub level_intent_ref: String,
    #[serde(rename = "traceLinks")]
    pub trace_links: Vec<GddTraceLink>,
    #[serde(rename = "capabilityStatus")]
    pub capability_status: GddSceneLevelCapabilityStatus,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddSceneGenerationPlanLink {
    #[serde(rename = "scenePlanId")]
    pub scene_plan_id: String,
    #[serde(rename = "sceneGenerationPlanRef")]
    pub scene_generation_plan_ref: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "traceLinks")]
    pub trace_links: Vec<GddTraceLink>,
    #[serde(rename = "capabilityStatus")]
    pub capability_status: GddSceneLevelCapabilityStatus,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddObjectivePlacementPlan {
    #[serde(rename = "placementId")]
    pub placement_id: String,
    #[serde(rename = "objectiveId")]
    pub objective_id: String,
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    #[serde(rename = "mechanicsMappingId")]
    pub mechanics_mapping_id: String,
    #[serde(rename = "targetSceneRef")]
    pub target_scene_ref: String,
    #[serde(rename = "targetTilemapRef")]
    pub target_tilemap_ref: Option<String>,
    #[serde(rename = "proofExpectationRef")]
    pub proof_expectation_ref: String,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddProgressionStep {
    #[serde(rename = "stepId")]
    pub step_id: String,
    #[serde(rename = "afterObjectiveRefs")]
    pub after_objective_refs: Vec<String>,
    #[serde(rename = "beforeObjectiveRefs")]
    pub before_objective_refs: Vec<String>,
    pub rationale: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddSceneLevelTargetRef {
    #[serde(rename = "targetRef")]
    pub target_ref: String,
    pub kind: GddSceneLevelTargetKind,
    #[serde(rename = "expectedHash")]
    pub expected_hash: Option<String>,
    #[serde(rename = "staleTarget")]
    pub stale_target: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddSceneLevelExpectedEvidence {
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "pathHint")]
    pub path_hint: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddSceneLevelPlanReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "requirementCount")]
    pub requirement_count: usize,
    #[serde(rename = "mechanicsMappingCount")]
    pub mechanics_mapping_count: usize,
    #[serde(rename = "levelIntentCount")]
    pub level_intent_count: usize,
    #[serde(rename = "scenePlanCount")]
    pub scene_plan_count: usize,
    #[serde(rename = "objectivePlacementCount")]
    pub objective_placement_count: usize,
    #[serde(rename = "targetCount")]
    pub target_count: usize,
    #[serde(rename = "blockedCount")]
    pub blocked_count: usize,
    #[serde(rename = "targetKindCounts")]
    pub target_kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddSceneLevelPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse GDD Scene/Level Plan JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddSceneLevelPlanReadModel {
        let mut target_kind_counts = BTreeMap::new();
        for target in &self.target_refs {
            *target_kind_counts
                .entry(target_kind_label(&target.kind).to_string())
                .or_insert(0) += 1;
        }
        GddSceneLevelPlanReadModel {
            schema_version: self.schema_version.clone(),
            plan_id: self.plan_id.clone(),
            status: status_label(&self.status).to_string(),
            requirement_count: self.requirement_ids.len(),
            mechanics_mapping_count: self.mechanics_mapping_ids.len(),
            level_intent_count: self.level_intents.len(),
            scene_plan_count: self.scene_generation_plans.len(),
            objective_placement_count: self.objective_placements.len(),
            target_count: self.target_refs.len(),
            blocked_count: self.blocked_count(),
            target_kind_counts,
            validation_summary: vec![
                "every scene/level plan element links to GDD requirement ids and mechanics mapping ids".to_string(),
                "existing level-intent-v1 and scene-generation-plan-v1 contracts are referenced, not replaced".to_string(),
                "unsupported capabilities, contradictions, stale targets, unsafe refs, and missing proof expectations fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "non-mutating read model with no direct scene, tilemap, draft, or trusted apply writes".to_string(),
                "GDD, requirements, mechanics mapping, feasibility, scaffold, plans, drafts, review, apply, run evidence, and journal artifacts remain separate".to_string(),
                "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD scene/level plan read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_SCENE_LEVEL_PLAN_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD scene/level plan schemaVersion must be {GDD_SCENE_LEVEL_PLAN_SCHEMA_VERSION}"
            ));
        }
        require_id("GDD scene/level plan planId", &self.plan_id)?;
        require_ref(
            "GDD scene/level plan feasibilityGateRef",
            &self.feasibility_gate_ref,
        )?;
        require_ref(
            "GDD scene/level plan scaffoldPlanRef",
            &self.scaffold_plan_ref,
        )?;
        validate_id_list(
            "GDD scene/level plan requirementIds",
            &self.requirement_ids,
            true,
        )?;
        validate_id_list(
            "GDD scene/level plan mechanicsMappingIds",
            &self.mechanics_mapping_ids,
            true,
        )?;
        for (field, len) in [
            ("levelIntents", self.level_intents.len()),
            ("sceneGenerationPlans", self.scene_generation_plans.len()),
            ("objectivePlacements", self.objective_placements.len()),
            ("progressionPlan", self.progression_plan.len()),
            ("targetRefs", self.target_refs.len()),
            ("expectedEvidence", self.expected_evidence.len()),
        ] {
            require_nonempty(&format!("GDD scene/level plan {field}"), len)?;
        }
        if self.objective_placements.len() > 12 || self.target_refs.len() > 16 {
            return Err(anyhow!("GDD scene/level plan is overbroad for v1"));
        }
        let reqs: BTreeSet<&str> = self.requirement_ids.iter().map(String::as_str).collect();
        let maps: BTreeSet<&str> = self
            .mechanics_mapping_ids
            .iter()
            .map(String::as_str)
            .collect();
        let mut intents = BTreeSet::new();
        for intent in &self.level_intents {
            intent.validate(&reqs, &maps)?;
            if !intents.insert(intent.intent_id.as_str()) {
                return Err(anyhow!(
                    "GDD scene/level plan intentId `{}` is duplicated",
                    intent.intent_id
                ));
            }
        }
        let mut scene_plan_ids = BTreeSet::new();
        for plan in &self.scene_generation_plans {
            plan.validate(&reqs, &maps, &intents)?;
            if !scene_plan_ids.insert(plan.scene_plan_id.as_str()) {
                return Err(anyhow!(
                    "GDD scene/level plan scenePlanId `{}` is duplicated",
                    plan.scene_plan_id
                ));
            }
        }
        let mut objectives = BTreeSet::new();
        for placement in &self.objective_placements {
            placement.validate(&reqs, &maps)?;
            if !objectives.insert(placement.objective_id.as_str()) {
                return Err(anyhow!(
                    "GDD scene/level plan objectiveId `{}` has contradictory duplicate placements",
                    placement.objective_id
                ));
            }
        }
        for step in &self.progression_plan {
            step.validate(&objectives)?;
        }
        let mut targets = BTreeSet::new();
        for target in &self.target_refs {
            target.validate()?;
            if !targets.insert(target.target_ref.as_str()) {
                return Err(anyhow!(
                    "GDD scene/level plan targetRef `{}` is duplicated",
                    target.target_ref
                ));
            }
        }
        let mut evidence = BTreeSet::new();
        for item in &self.expected_evidence {
            item.validate()?;
            if !evidence.insert(item.evidence_id.as_str()) {
                return Err(anyhow!(
                    "GDD scene/level plan evidenceId `{}` is duplicated",
                    item.evidence_id
                ));
            }
        }
        validate_text_list(
            "GDD scene/level plan blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let stale = self.target_refs.iter().any(|target| target.stale_target);
        let blocked = self.blocked_count() > 0;
        match self.status {
            GddSceneLevelPlanStatus::Ready if stale || blocked => Err(anyhow!("ready GDD scene/level plan must not include stale targets or blocked/unsupported elements"))?,
            GddSceneLevelPlanStatus::Partial if !blocked => Err(anyhow!("partial GDD scene/level plan requires visible missing requirements or blocked reasons"))?,
            GddSceneLevelPlanStatus::Blocked if !blocked => Err(anyhow!("blocked GDD scene/level plan requires visible blocked reasons"))?,
            GddSceneLevelPlanStatus::Stale if !stale => Err(anyhow!("stale GDD scene/level plan requires at least one staleTarget"))?,
            _ => {}
        }
        require_text("GDD scene/level plan boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "non-mutating",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no direct scene or tilemap writes",
            "no autonomous unrestricted game creation",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD scene/level plan boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self
                .level_intents
                .iter()
                .filter(|x| {
                    x.capability_status != GddSceneLevelCapabilityStatus::Supported
                        || !x.blocked_reasons.is_empty()
                })
                .count()
            + self
                .scene_generation_plans
                .iter()
                .filter(|x| {
                    x.capability_status != GddSceneLevelCapabilityStatus::Supported
                        || !x.blocked_reasons.is_empty()
                })
                .count()
            + self
                .objective_placements
                .iter()
                .filter(|x| !x.blocked_reasons.is_empty())
                .count()
            + self
                .target_refs
                .iter()
                .filter(|x| x.stale_target || !x.blocked_reasons.is_empty())
                .count()
    }
}

impl GddLevelIntentLink {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD scene/level plan levelIntents.intentId",
            &self.intent_id,
        )?;
        require_ref(
            "GDD scene/level plan levelIntents.levelIntentRef",
            &self.level_intent_ref,
        )?;
        validate_trace_links(
            "GDD scene/level plan levelIntents.traceLinks",
            &self.trace_links,
            reqs,
            maps,
        )?;
        validate_capability(
            "GDD scene/level plan levelIntents",
            &self.capability_status,
            &self.blocked_reasons,
        )
    }
}
impl GddSceneGenerationPlanLink {
    fn validate(
        &self,
        reqs: &BTreeSet<&str>,
        maps: &BTreeSet<&str>,
        intents: &BTreeSet<&str>,
    ) -> Result<()> {
        require_id(
            "GDD scene/level plan sceneGenerationPlans.scenePlanId",
            &self.scene_plan_id,
        )?;
        require_ref(
            "GDD scene/level plan sceneGenerationPlans.sceneGenerationPlanRef",
            &self.scene_generation_plan_ref,
        )?;
        require_id(
            "GDD scene/level plan sceneGenerationPlans.intentId",
            &self.intent_id,
        )?;
        if !intents.contains(self.intent_id.as_str()) {
            return Err(anyhow!("GDD scene/level plan sceneGenerationPlans.intentId `{}` must reference a levelIntents intentId", self.intent_id));
        }
        validate_trace_links(
            "GDD scene/level plan sceneGenerationPlans.traceLinks",
            &self.trace_links,
            reqs,
            maps,
        )?;
        validate_capability(
            "GDD scene/level plan sceneGenerationPlans",
            &self.capability_status,
            &self.blocked_reasons,
        )
    }
}
impl GddObjectivePlacementPlan {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD scene/level plan objectivePlacements.placementId",
            &self.placement_id,
        )?;
        require_id(
            "GDD scene/level plan objectivePlacements.objectiveId",
            &self.objective_id,
        )?;
        require_member(
            "GDD scene/level plan objectivePlacements.requirementId",
            &self.requirement_id,
            reqs,
        )?;
        require_member(
            "GDD scene/level plan objectivePlacements.mechanicsMappingId",
            &self.mechanics_mapping_id,
            maps,
        )?;
        require_ref(
            "GDD scene/level plan objectivePlacements.targetSceneRef",
            &self.target_scene_ref,
        )?;
        if let Some(ref value) = self.target_tilemap_ref {
            require_ref(
                "GDD scene/level plan objectivePlacements.targetTilemapRef",
                value,
            )?;
        }
        require_ref(
            "GDD scene/level plan objectivePlacements.proofExpectationRef",
            &self.proof_expectation_ref,
        )?;
        if !self.proof_expectation_ref.contains("scenario")
            && !self.proof_expectation_ref.contains("evidence")
        {
            return Err(anyhow!("GDD scene/level plan objective placement `{}` requires objective proof expectation refs", self.placement_id));
        }
        validate_text_list(
            "GDD scene/level plan objectivePlacements.blockedReasons",
            &self.blocked_reasons,
            false,
        )
    }
}
impl GddProgressionStep {
    fn validate(&self, objectives: &BTreeSet<&str>) -> Result<()> {
        require_id("GDD scene/level plan progressionPlan.stepId", &self.step_id)?;
        validate_id_list(
            "GDD scene/level plan progressionPlan.afterObjectiveRefs",
            &self.after_objective_refs,
            false,
        )?;
        validate_id_list(
            "GDD scene/level plan progressionPlan.beforeObjectiveRefs",
            &self.before_objective_refs,
            false,
        )?;
        require_text(
            "GDD scene/level plan progressionPlan.rationale",
            &self.rationale,
        )?;
        for objective in self
            .after_objective_refs
            .iter()
            .chain(self.before_objective_refs.iter())
        {
            require_member(
                "GDD scene/level plan progressionPlan objective ref",
                objective,
                objectives,
            )?;
        }
        for objective in &self.after_objective_refs {
            if self.before_objective_refs.contains(objective) {
                return Err(anyhow!("GDD scene/level plan progression step `{}` has contradictory level goals for objective `{}`", self.step_id, objective));
            }
        }
        Ok(())
    }
}
impl GddSceneLevelTargetRef {
    fn validate(&self) -> Result<()> {
        require_ref(
            "GDD scene/level plan targetRefs.targetRef",
            &self.target_ref,
        )?;
        if let Some(ref value) = self.expected_hash {
            validate_hash("GDD scene/level plan targetRefs.expectedHash", value)?;
        }
        validate_text_list(
            "GDD scene/level plan targetRefs.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.stale_target && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD scene/level plan stale target `{}` must include blockedReasons",
                self.target_ref
            ));
        }
        Ok(())
    }
}
impl GddSceneLevelExpectedEvidence {
    fn validate(&self) -> Result<()> {
        require_id(
            "GDD scene/level plan expectedEvidence.evidenceId",
            &self.evidence_id,
        )?;
        require_ref(
            "GDD scene/level plan expectedEvidence.pathHint",
            &self.path_hint,
        )?;
        if !self.path_hint.contains("evidence") && !self.path_hint.contains("scenario") {
            return Err(anyhow!(
                "GDD scene/level plan expectedEvidence must point to scenario/evidence refs"
            ));
        }
        require_text(
            "GDD scene/level plan expectedEvidence.description",
            &self.description,
        )
    }
}

fn validate_trace_links(
    field: &str,
    links: &[GddTraceLink],
    reqs: &BTreeSet<&str>,
    maps: &BTreeSet<&str>,
) -> Result<()> {
    require_nonempty(field, links.len())?;
    for link in links {
        require_member(
            &format!("{field}.requirementId"),
            &link.requirement_id,
            reqs,
        )?;
        require_member(
            &format!("{field}.mechanicsMappingId"),
            &link.mechanics_mapping_id,
            maps,
        )?;
    }
    Ok(())
}
fn validate_capability(
    field: &str,
    status: &GddSceneLevelCapabilityStatus,
    blockers: &[String],
) -> Result<()> {
    validate_text_list(&format!("{field}.blockedReasons"), blockers, false)?;
    if *status != GddSceneLevelCapabilityStatus::Supported && blockers.is_empty() {
        return Err(anyhow!(
            "{field} with unsupported/deferred level design capabilities requires blockedReasons"
        ));
    }
    Ok(())
}
fn status_label(status: &GddSceneLevelPlanStatus) -> &'static str {
    match status {
        GddSceneLevelPlanStatus::Ready => "ready",
        GddSceneLevelPlanStatus::Partial => "partial",
        GddSceneLevelPlanStatus::Blocked => "blocked",
        GddSceneLevelPlanStatus::Stale => "stale",
    }
}
fn target_kind_label(kind: &GddSceneLevelTargetKind) -> &'static str {
    match kind {
        GddSceneLevelTargetKind::Scene => "scene",
        GddSceneLevelTargetKind::Tilemap => "tilemap",
        GddSceneLevelTargetKind::LevelIntent => "level-intent",
        GddSceneLevelTargetKind::SceneGenerationPlan => "scene-generation-plan",
    }
}
fn validate_id_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    let mut seen = BTreeSet::new();
    for value in values {
        require_id(field, value)?;
        if !seen.insert(value.as_str()) {
            return Err(anyhow!("{field} contains duplicate id `{value}`"));
        }
    }
    Ok(())
}
fn validate_text_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}
fn require_member(field: &str, value: &str, allowed: &BTreeSet<&str>) -> Result<()> {
    require_id(field, value)?;
    if !allowed.contains(value) {
        return Err(anyhow!(
            "{field} `{value}` is missing from declared GDD requirements or mechanics mapping ids"
        ));
    }
    Ok(())
}
fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
fn require_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty()
        || value.len() > 96
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!("{field} must be a bounded local id"));
    }
    Ok(())
}
fn require_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} contains forbidden traversal and must stay inside local fixture/reference roots"));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/")
        || value.starts_with("evidence/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, runs/, or evidence/ refs"
        ));
    }
    Ok(())
}
fn validate_hash(field: &str, value: &str) -> Result<()> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(anyhow!("{field} must use sha256:<64 hex> form"));
    };
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must use sha256:<64 hex> form"));
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
        "dynamic code loading",
        "dynamic import",
        "command bridge",
        "local server bridge",
        "browser trusted write",
        "auto-merge",
        "auto-apply",
        "self-approval",
        "godot replacement",
        "production-ready",
        "shipped-game",
        "commercial readiness",
        "hosted/cloud",
        "native export",
        "plugin runtime",
        "asset generation",
        "autonomous unrestricted game creation",
        "direct scene write",
        "direct tilemap write",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD scene/level authority text `{forbidden}`"
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
    const CONTRASTS: [&str; 6] = [
        " but ",
        " however ",
        " yet ",
        " whereas ",
        " nevertheless ",
        " though ",
    ];
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let mut clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
        // A contrastive conjunction ends the preceding negation's scope so a
        // negated mention cannot whitelist a later positive mention in the same
        // sentence (e.g. `no auto-fix, but auto-fix enabled` fails closed),
        // while simple comma/or lists such as `no auto-apply or self-approval`
        // stay negated.
        if let Some(reset) = CONTRASTS
            .iter()
            .filter_map(|c| {
                hay[clause_start..idx]
                    .rfind(c)
                    .map(|p| clause_start + p + c.len())
            })
            .max()
        {
            clause_start = reset;
        }
        let preceding = &hay[clause_start..idx];
        let negated = NEGATIONS.iter().any(|n| preceding.contains(n));
        if !negated {
            return true;
        }
        search_start = idx + phrase.len();
    }
    false
}
