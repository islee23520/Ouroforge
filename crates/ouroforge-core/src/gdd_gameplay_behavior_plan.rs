use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_GAMEPLAY_BEHAVIOR_PLAN_SCHEMA_VERSION: &str = "gdd-gameplay-behavior-plan-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddGameplayBehaviorPlanStatus {
    Ready,
    Partial,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddBehaviorCapabilityStatus {
    Supported,
    Unsupported,
    Deferred,
    ScriptNeeded,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddBehaviorTargetKind {
    BehaviorModel,
    EventSignal,
    StateMachine,
    AbilityAction,
    ScenarioPack,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddGameplayBehaviorPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: GddGameplayBehaviorPlanStatus,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "mechanicsMappingRef")]
    pub mechanics_mapping_ref: String,
    #[serde(rename = "requirementIds")]
    pub requirement_ids: Vec<String>,
    #[serde(rename = "mechanicsMappingIds")]
    pub mechanics_mapping_ids: Vec<String>,
    #[serde(rename = "behaviorModels")]
    pub behavior_models: Vec<GddBehaviorModelLink>,
    #[serde(rename = "eventSignalNeeds")]
    pub event_signal_needs: Vec<GddEventSignalNeed>,
    #[serde(rename = "stateAbilityPlans")]
    pub state_ability_plans: Vec<GddStateAbilityPlan>,
    #[serde(rename = "expectedFlagsEvents")]
    pub expected_flags_events: Vec<GddExpectedFlagEvent>,
    #[serde(rename = "scenarioNeeds")]
    pub scenario_needs: Vec<GddBehaviorScenarioNeed>,
    #[serde(rename = "unsupportedScriptNeeds")]
    pub unsupported_script_needs: Vec<GddUnsupportedScriptNeed>,
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<GddBehaviorTargetRef>,
    #[serde(rename = "expectedEvidence")]
    pub expected_evidence: Vec<GddBehaviorExpectedEvidence>,
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
pub struct GddBehaviorModelLink {
    #[serde(rename = "behaviorModelId")]
    pub behavior_model_id: String,
    #[serde(rename = "behaviorModelRef")]
    pub behavior_model_ref: String,
    #[serde(rename = "traceLinks")]
    pub trace_links: Vec<GddTraceLink>,
    #[serde(rename = "capabilityStatus")]
    pub capability_status: GddBehaviorCapabilityStatus,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddEventSignalNeed {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "signalRef")]
    pub signal_ref: String,
    #[serde(rename = "traceLinks")]
    pub trace_links: Vec<GddTraceLink>,
    #[serde(rename = "capabilityStatus")]
    pub capability_status: GddBehaviorCapabilityStatus,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddStateAbilityPlan {
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "stateMachineRef")]
    pub state_machine_ref: String,
    #[serde(rename = "abilityActionRef")]
    pub ability_action_ref: String,
    #[serde(rename = "traceLinks")]
    pub trace_links: Vec<GddTraceLink>,
    #[serde(rename = "capabilityStatus")]
    pub capability_status: GddBehaviorCapabilityStatus,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddExpectedFlagEvent {
    #[serde(rename = "expectationId")]
    pub expectation_id: String,
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    #[serde(rename = "mechanicsMappingId")]
    pub mechanics_mapping_id: String,
    #[serde(rename = "flagOrEvent")]
    pub flag_or_event: String,
    #[serde(rename = "proofExpectationRef")]
    pub proof_expectation_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddBehaviorScenarioNeed {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "scenarioRef")]
    pub scenario_ref: String,
    #[serde(rename = "traceLinks")]
    pub trace_links: Vec<GddTraceLink>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddUnsupportedScriptNeed {
    #[serde(rename = "scriptNeedId")]
    pub script_need_id: String,
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    #[serde(rename = "mechanicsMappingId")]
    pub mechanics_mapping_id: String,
    pub description: String,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddBehaviorTargetRef {
    #[serde(rename = "targetRef")]
    pub target_ref: String,
    pub kind: GddBehaviorTargetKind,
    #[serde(rename = "expectedHash")]
    pub expected_hash: Option<String>,
    #[serde(rename = "staleRef")]
    pub stale_ref: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddBehaviorExpectedEvidence {
    #[serde(rename = "evidenceId")]
    pub evidence_id: String,
    #[serde(rename = "pathHint")]
    pub path_hint: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddGameplayBehaviorPlanReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "behaviorModelCount")]
    pub behavior_model_count: usize,
    #[serde(rename = "eventSignalCount")]
    pub event_signal_count: usize,
    #[serde(rename = "stateAbilityCount")]
    pub state_ability_count: usize,
    #[serde(rename = "scenarioNeedCount")]
    pub scenario_need_count: usize,
    #[serde(rename = "unsupportedScriptNeedCount")]
    pub unsupported_script_need_count: usize,
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

impl GddGameplayBehaviorPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Gameplay Behavior Plan JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }
    pub fn read_model(&self) -> GddGameplayBehaviorPlanReadModel {
        let mut target_kind_counts = BTreeMap::new();
        for target in &self.target_refs {
            *target_kind_counts
                .entry(target_kind_label(&target.kind).to_string())
                .or_insert(0) += 1;
        }
        GddGameplayBehaviorPlanReadModel { schema_version: self.schema_version.clone(), plan_id: self.plan_id.clone(), status: status_label(&self.status).to_string(), behavior_model_count: self.behavior_models.len(), event_signal_count: self.event_signal_needs.len(), state_ability_count: self.state_ability_plans.len(), scenario_need_count: self.scenario_needs.len(), unsupported_script_need_count: self.unsupported_script_needs.len(), blocked_count: self.blocked_count(), target_kind_counts, validation_summary: vec!["behavior plan elements link to GDD requirement ids and mechanics mapping ids".to_string(), "structured gameplay behavior, event/signal, state-machine, and ability/action contracts are referenced, not replaced".to_string(), "unsupported script needs, stale refs, unsafe refs, contradictions, and missing proof expectations fail closed".to_string()], compatibility_notes: vec!["non-mutating read model with no arbitrary script generation or execution".to_string(), "GDD, requirements, mechanics mapping, behavior plans, drafts, review, apply, run evidence, and journal artifacts remain separate".to_string(), "Browser/dashboard/Studio consumers remain read-only or draft-only".to_string()], boundary: self.boundary.clone() }
    }
    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD gameplay behavior plan read model JSON")
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_GAMEPLAY_BEHAVIOR_PLAN_SCHEMA_VERSION {
            return Err(anyhow!("GDD gameplay behavior plan schemaVersion must be {GDD_GAMEPLAY_BEHAVIOR_PLAN_SCHEMA_VERSION}"));
        }
        require_id("GDD gameplay behavior plan planId", &self.plan_id)?;
        require_ref(
            "GDD gameplay behavior plan feasibilityGateRef",
            &self.feasibility_gate_ref,
        )?;
        require_ref(
            "GDD gameplay behavior plan mechanicsMappingRef",
            &self.mechanics_mapping_ref,
        )?;
        validate_id_list(
            "GDD gameplay behavior plan requirementIds",
            &self.requirement_ids,
            true,
        )?;
        validate_id_list(
            "GDD gameplay behavior plan mechanicsMappingIds",
            &self.mechanics_mapping_ids,
            true,
        )?;
        for (field, len) in [
            ("behaviorModels", self.behavior_models.len()),
            ("eventSignalNeeds", self.event_signal_needs.len()),
            ("stateAbilityPlans", self.state_ability_plans.len()),
            ("expectedFlagsEvents", self.expected_flags_events.len()),
            ("scenarioNeeds", self.scenario_needs.len()),
            ("targetRefs", self.target_refs.len()),
            ("expectedEvidence", self.expected_evidence.len()),
        ] {
            require_nonempty(&format!("GDD gameplay behavior plan {field}"), len)?;
        }
        if self.expected_flags_events.len() > 16 || self.target_refs.len() > 16 {
            return Err(anyhow!("GDD gameplay behavior plan is overbroad for v1"));
        }
        let reqs: BTreeSet<&str> = self.requirement_ids.iter().map(String::as_str).collect();
        let maps: BTreeSet<&str> = self
            .mechanics_mapping_ids
            .iter()
            .map(String::as_str)
            .collect();
        let mut ids = BTreeSet::new();
        for item in &self.behavior_models {
            item.validate(&reqs, &maps)?;
            unique(&mut ids, "behaviorModelId", &item.behavior_model_id)?;
        }
        ids.clear();
        for item in &self.event_signal_needs {
            item.validate(&reqs, &maps)?;
            unique(&mut ids, "eventId", &item.event_id)?;
        }
        ids.clear();
        for item in &self.state_ability_plans {
            item.validate(&reqs, &maps)?;
            unique(&mut ids, "stateAbility planId", &item.plan_id)?;
        }
        ids.clear();
        for item in &self.expected_flags_events {
            item.validate(&reqs, &maps)?;
            unique(&mut ids, "expectationId", &item.expectation_id)?;
        }
        ids.clear();
        for item in &self.scenario_needs {
            item.validate(&reqs, &maps)?;
            unique(&mut ids, "scenarioId", &item.scenario_id)?;
        }
        ids.clear();
        for item in &self.unsupported_script_needs {
            item.validate(&reqs, &maps)?;
            unique(&mut ids, "scriptNeedId", &item.script_need_id)?;
        }
        ids.clear();
        for item in &self.target_refs {
            item.validate()?;
            unique(&mut ids, "targetRef", &item.target_ref)?;
        }
        ids.clear();
        for item in &self.expected_evidence {
            item.validate()?;
            unique(&mut ids, "evidenceId", &item.evidence_id)?;
        }
        self.validate_no_contradictory_core_loop()?;
        validate_text_list(
            "GDD gameplay behavior plan blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let stale = self.target_refs.iter().any(|target| target.stale_ref);
        let blocked = self.blocked_count() > 0;
        match self.status { GddGameplayBehaviorPlanStatus::Ready if stale || blocked => Err(anyhow!("ready GDD gameplay behavior plan must not include stale refs, unsupported scripts, or blocked elements"))?, GddGameplayBehaviorPlanStatus::Partial if !blocked => Err(anyhow!("partial GDD gameplay behavior plan requires visible missing behavior refs or blocked reasons"))?, GddGameplayBehaviorPlanStatus::Blocked if !blocked => Err(anyhow!("blocked GDD gameplay behavior plan requires visible blocked reasons"))?, GddGameplayBehaviorPlanStatus::Stale if !stale => Err(anyhow!("stale GDD gameplay behavior plan requires at least one staleRef"))?, _ => {} }
        require_text("GDD gameplay behavior plan boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "non-mutating",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no arbitrary script generation",
            "no arbitrary script execution",
            "no autonomous unrestricted game creation",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD gameplay behavior plan boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
    fn validate_no_contradictory_core_loop(&self) -> Result<()> {
        let mut seen = BTreeSet::new();
        for item in &self.expected_flags_events {
            let normalized = item.flag_or_event.to_ascii_lowercase();
            if normalized.starts_with("not:")
                && seen.contains(normalized.trim_start_matches("not:"))
            {
                return Err(anyhow!(
                    "GDD gameplay behavior plan has contradictory core loop behavior for `{}`",
                    item.flag_or_event
                ));
            }
            if seen.contains(&format!("not:{normalized}")) {
                return Err(anyhow!(
                    "GDD gameplay behavior plan has contradictory core loop behavior for `{}`",
                    item.flag_or_event
                ));
            }
            seen.insert(normalized);
        }
        Ok(())
    }
    fn blocked_count(&self) -> usize {
        self.blocked_reasons.len()
            + self.unsupported_script_needs.len()
            + self
                .behavior_models
                .iter()
                .filter(|x| {
                    x.capability_status != GddBehaviorCapabilityStatus::Supported
                        || !x.blocked_reasons.is_empty()
                })
                .count()
            + self
                .event_signal_needs
                .iter()
                .filter(|x| {
                    x.capability_status != GddBehaviorCapabilityStatus::Supported
                        || !x.blocked_reasons.is_empty()
                })
                .count()
            + self
                .state_ability_plans
                .iter()
                .filter(|x| {
                    x.capability_status != GddBehaviorCapabilityStatus::Supported
                        || !x.blocked_reasons.is_empty()
                })
                .count()
            + self
                .scenario_needs
                .iter()
                .filter(|x| !x.blocked_reasons.is_empty())
                .count()
            + self
                .target_refs
                .iter()
                .filter(|x| x.stale_ref || !x.blocked_reasons.is_empty())
                .count()
    }
}

impl GddBehaviorModelLink {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan behaviorModels.behaviorModelId",
            &self.behavior_model_id,
        )?;
        require_ref(
            "GDD gameplay behavior plan behaviorModels.behaviorModelRef",
            &self.behavior_model_ref,
        )?;
        validate_trace_links(
            "GDD gameplay behavior plan behaviorModels.traceLinks",
            &self.trace_links,
            reqs,
            maps,
        )?;
        validate_capability(
            "GDD gameplay behavior plan behaviorModels",
            &self.capability_status,
            &self.blocked_reasons,
        )
    }
}
impl GddEventSignalNeed {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan eventSignalNeeds.eventId",
            &self.event_id,
        )?;
        require_ref(
            "GDD gameplay behavior plan eventSignalNeeds.signalRef",
            &self.signal_ref,
        )?;
        validate_trace_links(
            "GDD gameplay behavior plan eventSignalNeeds.traceLinks",
            &self.trace_links,
            reqs,
            maps,
        )?;
        validate_capability(
            "GDD gameplay behavior plan eventSignalNeeds",
            &self.capability_status,
            &self.blocked_reasons,
        )
    }
}
impl GddStateAbilityPlan {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan stateAbilityPlans.planId",
            &self.plan_id,
        )?;
        require_ref(
            "GDD gameplay behavior plan stateAbilityPlans.stateMachineRef",
            &self.state_machine_ref,
        )?;
        require_ref(
            "GDD gameplay behavior plan stateAbilityPlans.abilityActionRef",
            &self.ability_action_ref,
        )?;
        validate_trace_links(
            "GDD gameplay behavior plan stateAbilityPlans.traceLinks",
            &self.trace_links,
            reqs,
            maps,
        )?;
        validate_capability(
            "GDD gameplay behavior plan stateAbilityPlans",
            &self.capability_status,
            &self.blocked_reasons,
        )
    }
}
impl GddExpectedFlagEvent {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan expectedFlagsEvents.expectationId",
            &self.expectation_id,
        )?;
        require_member(
            "GDD gameplay behavior plan expectedFlagsEvents.requirementId",
            &self.requirement_id,
            reqs,
        )?;
        require_member(
            "GDD gameplay behavior plan expectedFlagsEvents.mechanicsMappingId",
            &self.mechanics_mapping_id,
            maps,
        )?;
        require_id(
            "GDD gameplay behavior plan expectedFlagsEvents.flagOrEvent",
            self.flag_or_event.trim_start_matches("not:"),
        )?;
        require_ref(
            "GDD gameplay behavior plan expectedFlagsEvents.proofExpectationRef",
            &self.proof_expectation_ref,
        )?;
        if !self.proof_expectation_ref.contains("scenario")
            && !self.proof_expectation_ref.contains("evidence")
        {
            return Err(anyhow!("GDD gameplay behavior plan expected flag/event `{}` requires proof expectation refs", self.expectation_id));
        }
        Ok(())
    }
}
impl GddBehaviorScenarioNeed {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan scenarioNeeds.scenarioId",
            &self.scenario_id,
        )?;
        require_ref(
            "GDD gameplay behavior plan scenarioNeeds.scenarioRef",
            &self.scenario_ref,
        )?;
        validate_trace_links(
            "GDD gameplay behavior plan scenarioNeeds.traceLinks",
            &self.trace_links,
            reqs,
            maps,
        )?;
        validate_text_list(
            "GDD gameplay behavior plan scenarioNeeds.blockedReasons",
            &self.blocked_reasons,
            false,
        )
    }
}
impl GddUnsupportedScriptNeed {
    fn validate(&self, reqs: &BTreeSet<&str>, maps: &BTreeSet<&str>) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan unsupportedScriptNeeds.scriptNeedId",
            &self.script_need_id,
        )?;
        require_member(
            "GDD gameplay behavior plan unsupportedScriptNeeds.requirementId",
            &self.requirement_id,
            reqs,
        )?;
        require_member(
            "GDD gameplay behavior plan unsupportedScriptNeeds.mechanicsMappingId",
            &self.mechanics_mapping_id,
            maps,
        )?;
        require_text(
            "GDD gameplay behavior plan unsupportedScriptNeeds.description",
            &self.description,
        )?;
        validate_text_list(
            "GDD gameplay behavior plan unsupportedScriptNeeds.blockedReasons",
            &self.blocked_reasons,
            true,
        )
    }
}
impl GddBehaviorTargetRef {
    fn validate(&self) -> Result<()> {
        require_ref(
            "GDD gameplay behavior plan targetRefs.targetRef",
            &self.target_ref,
        )?;
        if let Some(ref value) = self.expected_hash {
            validate_hash("GDD gameplay behavior plan targetRefs.expectedHash", value)?;
        }
        validate_text_list(
            "GDD gameplay behavior plan targetRefs.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.stale_ref && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD gameplay behavior plan stale ref `{}` must include blockedReasons",
                self.target_ref
            ));
        }
        Ok(())
    }
}
impl GddBehaviorExpectedEvidence {
    fn validate(&self) -> Result<()> {
        require_id(
            "GDD gameplay behavior plan expectedEvidence.evidenceId",
            &self.evidence_id,
        )?;
        require_ref(
            "GDD gameplay behavior plan expectedEvidence.pathHint",
            &self.path_hint,
        )?;
        if !self.path_hint.contains("evidence") && !self.path_hint.contains("scenario") {
            return Err(anyhow!(
                "GDD gameplay behavior plan expectedEvidence must point to scenario/evidence refs"
            ));
        }
        require_text(
            "GDD gameplay behavior plan expectedEvidence.description",
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
    status: &GddBehaviorCapabilityStatus,
    blockers: &[String],
) -> Result<()> {
    validate_text_list(&format!("{field}.blockedReasons"), blockers, false)?;
    if *status != GddBehaviorCapabilityStatus::Supported && blockers.is_empty() {
        return Err(anyhow!("{field} with unsupported/deferred/script-needed behavior capability requires blockedReasons"));
    }
    Ok(())
}
fn status_label(status: &GddGameplayBehaviorPlanStatus) -> &'static str {
    match status {
        GddGameplayBehaviorPlanStatus::Ready => "ready",
        GddGameplayBehaviorPlanStatus::Partial => "partial",
        GddGameplayBehaviorPlanStatus::Blocked => "blocked",
        GddGameplayBehaviorPlanStatus::Stale => "stale",
    }
}
fn target_kind_label(kind: &GddBehaviorTargetKind) -> &'static str {
    match kind {
        GddBehaviorTargetKind::BehaviorModel => "behavior-model",
        GddBehaviorTargetKind::EventSignal => "event-signal",
        GddBehaviorTargetKind::StateMachine => "state-machine",
        GddBehaviorTargetKind::AbilityAction => "ability-action",
        GddBehaviorTargetKind::ScenarioPack => "scenario-pack",
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
fn unique<'a>(seen: &mut BTreeSet<&'a str>, field: &str, value: &'a str) -> Result<()> {
    if !seen.insert(value) {
        return Err(anyhow!(
            "GDD gameplay behavior plan {field} `{value}` is duplicated"
        ));
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
        "arbitrary script generation",
        "arbitrary script execution",
        "visual scripting implementation",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD gameplay behavior authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}
fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    const NEGATIONS: [&str; 7] = [
        "no ",
        "not ",
        "without ",
        "avoid ",
        "forbid ",
        "forbidden ",
        "unsupported ",
    ];
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
