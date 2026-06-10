//! Behavior Authoring Model v1 (#2372, #1 M124).
//!
//! A data-only state-machine contract for authoring hazard, NPC, and system
//! behaviors without arbitrary scripting. The model enumerates triggers and the
//! allowed action vocabulary; serde plus validation fail closed on unknown or
//! authority-expanding fields. It produces deterministic preview/scenario
//! assertion data but does not execute code, load plugins, or apply drafts.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION: &str = "behavior-authoring-spec-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorAuthoringDomain {
    Hazard,
    Npc,
    System,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorTriggerKind {
    OnEnterState,
    OnFrameTick,
    OnPlayerContact,
    OnSensorEnter,
    OnTimerElapsed,
    OnHealthBelow,
    OnFlagEquals,
    OnDistanceLessThan,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorActionKind {
    SetState,
    SetFlag,
    EmitEvent,
    MoveByVector,
    MoveTowardEntity,
    ApplyDamage,
    SpawnEntityFromPrefab,
    DespawnSelf,
    StartTimer,
    StopTimer,
    PlayAnimation,
    PlaySoundCue,
    SetVelocity,
    ClampToPatrolRoute,
    OpenGate,
    CompleteObjective,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorDraftBoundary {
    DraftOnly,
    SafeSourceApplyReviewRequired,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAuthoringSpec {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    pub domain: BehaviorAuthoringDomain,
    #[serde(rename = "initialState")]
    pub initial_state: String,
    pub states: Vec<BehaviorState>,
    pub transitions: Vec<BehaviorTransition>,
    #[serde(rename = "parameterSchema")]
    pub parameter_schema: Vec<BehaviorParameter>,
    pub preview: BehaviorPreviewContract,
    #[serde(rename = "scenarioAssertions")]
    pub scenario_assertions: Vec<BehaviorScenarioAssertion>,
    #[serde(rename = "draftBoundary")]
    pub draft_boundary: BehaviorDraftBoundary,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorState {
    pub id: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorTransition {
    pub id: String,
    pub from: String,
    pub to: String,
    pub trigger: BehaviorTrigger,
    pub actions: Vec<BehaviorAction>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorTrigger {
    pub kind: BehaviorTriggerKind,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, BehaviorParameterValue>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAction {
    pub kind: BehaviorActionKind,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub parameters: BTreeMap<String, BehaviorParameterValue>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorParameter {
    pub name: String,
    #[serde(rename = "valueType")]
    pub value_type: BehaviorParameterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<BehaviorParameterValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorParameterType {
    Bool,
    Integer,
    Decimal,
    TextId,
    EntityRef,
    PrefabRef,
    Vector2,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", content = "value", rename_all = "kebab-case")]
pub enum BehaviorParameterValue {
    Bool(bool),
    Integer(i64),
    Decimal(f64),
    TextId(String),
    EntityRef(String),
    PrefabRef(String),
    Vector2 { x: i64, y: i64 },
}

impl Eq for BehaviorParameterValue {}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorPreviewContract {
    #[serde(rename = "previewMode")]
    pub preview_mode: BehaviorPreviewMode,
    #[serde(rename = "deterministicSeed")]
    pub deterministic_seed: u64,
    #[serde(rename = "expectedEvents")]
    pub expected_events: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<BehaviorEvidenceRef>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorPreviewMode {
    ReadOnlySimulation,
    ScenarioAssertionGeneration,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorEvidenceRef {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioAssertion {
    pub id: String,
    pub source: String,
    #[serde(rename = "assertionKind")]
    pub assertion_kind: BehaviorScenarioAssertionKind,
    #[serde(rename = "expectedPath")]
    pub expected_path: String,
    #[serde(rename = "expectedValue")]
    pub expected_value: BehaviorParameterValue,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorScenarioAssertionKind {
    WorldState,
    RuntimeEvent,
    WorldFlag,
    FrameStats,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAuthoringSpecEvaluation {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    pub status: BehaviorAuthoringSpecStatus,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<BehaviorActionKind>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "evidenceSummary")]
    pub evidence_summary: Vec<String>,
    #[serde(rename = "forbiddenSurfaces")]
    pub forbidden_surfaces: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorAuthoringSpecStatus {
    Valid,
    Blocked,
}

impl BehaviorAuthoringSpec {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let spec: Self =
            serde_json::from_str(input).context("failed to parse behavior authoring spec JSON")?;
        spec.validate()?;
        Ok(spec)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION {
            return Err(anyhow!(
                "behavior authoring spec schemaVersion must be {BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION}"
            ));
        }
        require_local_id("behavior authoring spec behaviorId", &self.behavior_id)?;
        require_local_id("behavior authoring spec initialState", &self.initial_state)?;
        if self.draft_boundary != BehaviorDraftBoundary::SafeSourceApplyReviewRequired {
            return Err(anyhow!(
                "behavior authoring spec draftBoundary must require Safe Source Apply review"
            ));
        }
        if self.states.is_empty() {
            return Err(anyhow!("behavior authoring spec states must not be empty"));
        }
        if self.transitions.is_empty() {
            return Err(anyhow!(
                "behavior authoring spec transitions must not be empty"
            ));
        }
        let mut state_ids = BTreeSet::new();
        for state in &self.states {
            state.validate()?;
            if !state_ids.insert(state.id.clone()) {
                return Err(anyhow!(
                    "behavior authoring spec duplicate state id `{}`",
                    state.id
                ));
            }
        }
        if !state_ids.contains(&self.initial_state) {
            return Err(anyhow!(
                "behavior authoring spec initialState must reference a declared state"
            ));
        }
        let mut transition_ids = BTreeSet::new();
        for transition in &self.transitions {
            transition.validate(&state_ids)?;
            if !transition_ids.insert(transition.id.clone()) {
                return Err(anyhow!(
                    "behavior authoring spec duplicate transition id `{}`",
                    transition.id
                ));
            }
        }
        let mut parameter_names = BTreeSet::new();
        for parameter in &self.parameter_schema {
            parameter.validate()?;
            if !parameter_names.insert(parameter.name.clone()) {
                return Err(anyhow!(
                    "behavior authoring spec duplicate parameter `{}`",
                    parameter.name
                ));
            }
        }
        self.preview.validate()?;
        if self.scenario_assertions.is_empty() {
            return Err(anyhow!(
                "behavior authoring spec scenarioAssertions must not be empty"
            ));
        }
        for assertion in &self.scenario_assertions {
            assertion.validate()?;
        }
        require_nonempty("behavior authoring spec guardrails", self.guardrails.len())?;
        for guardrail in &self.guardrails {
            require_boundary_text("behavior authoring spec guardrails", guardrail)?;
        }
        Ok(())
    }

    pub fn evaluate(&self) -> BehaviorAuthoringSpecEvaluation {
        let mut blocked = Vec::new();
        if self.validate().is_err() {
            blocked.push("schema validation failed".to_string());
        }
        if self.draft_boundary != BehaviorDraftBoundary::SafeSourceApplyReviewRequired {
            blocked
                .push("draft/apply boundary does not require Safe Source Apply review".to_string());
        }
        if self.preview.evidence_refs.is_empty() {
            blocked.push("deterministic behavior claims require evidence refs".to_string());
        }
        let status = if blocked.is_empty() {
            BehaviorAuthoringSpecStatus::Valid
        } else {
            BehaviorAuthoringSpecStatus::Blocked
        };
        BehaviorAuthoringSpecEvaluation {
            schema_version: BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION.to_string(),
            behavior_id: self.behavior_id.clone(),
            status,
            allowed_actions: allowed_action_vocabulary(),
            blocked_reasons: blocked,
            evidence_summary: vec![
                format!("states:{}", self.states.len()),
                format!("transitions:{}", self.transitions.len()),
                format!("assertions:{}", self.scenario_assertions.len()),
                format!("evidenceRefs:{}", self.preview.evidence_refs.len()),
            ],
            forbidden_surfaces: forbidden_surfaces(),
        }
    }

    pub fn evaluation_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.evaluate())
            .context("failed to serialize behavior authoring evaluation JSON")
    }
}

pub fn allowed_action_vocabulary() -> Vec<BehaviorActionKind> {
    vec![
        BehaviorActionKind::SetState,
        BehaviorActionKind::SetFlag,
        BehaviorActionKind::EmitEvent,
        BehaviorActionKind::MoveByVector,
        BehaviorActionKind::MoveTowardEntity,
        BehaviorActionKind::ApplyDamage,
        BehaviorActionKind::SpawnEntityFromPrefab,
        BehaviorActionKind::DespawnSelf,
        BehaviorActionKind::StartTimer,
        BehaviorActionKind::StopTimer,
        BehaviorActionKind::PlayAnimation,
        BehaviorActionKind::PlaySoundCue,
        BehaviorActionKind::SetVelocity,
        BehaviorActionKind::ClampToPatrolRoute,
        BehaviorActionKind::OpenGate,
        BehaviorActionKind::CompleteObjective,
    ]
}

pub fn forbidden_surfaces() -> Vec<String> {
    vec![
        "script_execution".to_string(),
        "eval".to_string(),
        "dynamic_import".to_string(),
        "plugin_loader".to_string(),
        "command_bridge".to_string(),
        "browser_trusted_write".to_string(),
        "auto_apply".to_string(),
        "self_approval".to_string(),
    ]
}

impl BehaviorState {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior state id", &self.id)?;
        require_boundary_text("behavior state label", &self.label)?;
        for tag in &self.tags {
            require_local_id("behavior state tag", tag)?;
        }
        Ok(())
    }
}

impl BehaviorTransition {
    fn validate(&self, state_ids: &BTreeSet<String>) -> Result<()> {
        require_local_id("behavior transition id", &self.id)?;
        if !state_ids.contains(&self.from) || !state_ids.contains(&self.to) {
            return Err(anyhow!(
                "behavior transition `{}` must reference declared from/to states",
                self.id
            ));
        }
        self.trigger.validate()?;
        if self.actions.is_empty() {
            return Err(anyhow!(
                "behavior transition `{}` actions must not be empty",
                self.id
            ));
        }
        for action in &self.actions {
            action.validate()?;
        }
        Ok(())
    }
}

impl BehaviorTrigger {
    fn validate(&self) -> Result<()> {
        for (key, value) in &self.parameters {
            require_local_id("behavior trigger parameter name", key)?;
            value.validate("behavior trigger parameter value")?;
        }
        Ok(())
    }
}

impl BehaviorAction {
    fn validate(&self) -> Result<()> {
        for (key, value) in &self.parameters {
            require_local_id("behavior action parameter name", key)?;
            value.validate("behavior action parameter value")?;
        }
        Ok(())
    }
}

impl BehaviorParameter {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior parameter name", &self.name)?;
        if let Some(default) = &self.default {
            default.validate("behavior parameter default")?;
        }
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if min > max {
                return Err(anyhow!(
                    "behavior parameter `{}` min must be <= max",
                    self.name
                ));
            }
        }
        Ok(())
    }
}

impl BehaviorParameterValue {
    fn validate(&self, label: &str) -> Result<()> {
        match self {
            BehaviorParameterValue::TextId(value)
            | BehaviorParameterValue::EntityRef(value)
            | BehaviorParameterValue::PrefabRef(value) => require_local_id(label, value),
            BehaviorParameterValue::Decimal(value) if !value.is_finite() => {
                Err(anyhow!("{label} decimal must be finite"))
            }
            _ => Ok(()),
        }
    }
}

impl BehaviorPreviewContract {
    fn validate(&self) -> Result<()> {
        if self.expected_events.is_empty() {
            return Err(anyhow!("behavior preview expectedEvents must not be empty"));
        }
        for event in &self.expected_events {
            require_local_id("behavior preview expected event", event)?;
        }
        if self.evidence_refs.is_empty() {
            return Err(anyhow!("behavior preview evidenceRefs must not be empty"));
        }
        for evidence in &self.evidence_refs {
            evidence.validate()?;
        }
        Ok(())
    }
}

impl BehaviorEvidenceRef {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior evidence runId", &self.run_id)?;
        require_relative_path("behavior evidence path", &self.path)?;
        require_digest("behavior evidence digest", &self.digest)
    }
}

impl BehaviorScenarioAssertion {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior scenario assertion id", &self.id)?;
        require_local_id("behavior scenario assertion source", &self.source)?;
        require_relative_path(
            "behavior scenario assertion expectedPath",
            &self.expected_path,
        )?;
        self.expected_value
            .validate("behavior scenario assertion expectedValue")
    }
}

fn require_nonempty(field: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.len() > 128
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, dot, or colon"
        ));
    }
    Ok(())
}

fn require_relative_path(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must stay inside the local artifact root"));
    }
    Ok(())
}

fn require_digest(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if !value.contains(':') || value.len() > 160 {
        return Err(anyhow!("{field} must include an algorithm prefix"));
    }
    Ok(())
}

fn require_boundary_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "eval(",
        "dynamic import",
        "plugin loader",
        "command bridge",
        "browser trusted write",
        "auto-apply",
        "self-approval",
        "arbitrary script",
        "execute script",
        "production-ready",
        "godot replacement",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden behavior-authoring authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hazard_behavior_spec_is_data_only_and_maps_to_evidence() {
        let spec = hazard_spec();
        spec.validate().unwrap();
        let evaluation = spec.evaluate();
        assert_eq!(evaluation.status, BehaviorAuthoringSpecStatus::Valid);
        assert!(evaluation
            .allowed_actions
            .contains(&BehaviorActionKind::ApplyDamage));
        assert!(evaluation
            .forbidden_surfaces
            .contains(&"script_execution".to_string()));
        assert_eq!(spec.preview.evidence_refs[0].run_id, "run-hazard-1");
    }

    #[test]
    fn npc_behavior_spec_supports_patrol_and_chase_without_scripts() {
        let mut spec = hazard_spec();
        spec.behavior_id = "npc-patrol-chase".to_string();
        spec.domain = BehaviorAuthoringDomain::Npc;
        spec.states = vec![
            BehaviorState {
                id: "patrol".to_string(),
                label: "Patrol".to_string(),
                tags: vec!["npc".to_string()],
            },
            BehaviorState {
                id: "chase".to_string(),
                label: "Chase".to_string(),
                tags: vec!["npc".to_string()],
            },
        ];
        spec.initial_state = "patrol".to_string();
        spec.transitions = vec![BehaviorTransition {
            id: "see-player".to_string(),
            from: "patrol".to_string(),
            to: "chase".to_string(),
            trigger: BehaviorTrigger {
                kind: BehaviorTriggerKind::OnDistanceLessThan,
                parameters: map([("distance".to_string(), BehaviorParameterValue::Integer(96))]),
            },
            actions: vec![
                BehaviorAction {
                    kind: BehaviorActionKind::MoveTowardEntity,
                    parameters: map([(
                        "target".to_string(),
                        BehaviorParameterValue::EntityRef("player".to_string()),
                    )]),
                },
                BehaviorAction {
                    kind: BehaviorActionKind::SetState,
                    parameters: map([(
                        "state".to_string(),
                        BehaviorParameterValue::TextId("chase".to_string()),
                    )]),
                },
            ],
        }];
        spec.validate().unwrap();
    }

    #[test]
    fn unknown_action_vocabulary_fails_closed_at_parse_time() {
        let mut value = serde_json::to_value(hazard_spec()).unwrap();
        value["transitions"][0]["actions"][0]["kind"] = serde_json::json!("run-script");
        let err = BehaviorAuthoringSpec::from_json_str(&value.to_string())
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("failed to parse behavior authoring spec JSON"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn script_or_plugin_authority_text_is_rejected() {
        let mut spec = hazard_spec();
        spec.guardrails = vec!["execute script through plugin loader".to_string()];
        let err = spec.validate().unwrap_err().to_string();
        assert!(
            err.contains("forbidden behavior-authoring authority text"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn draft_boundary_must_require_safe_source_apply_review() {
        let mut spec = hazard_spec();
        spec.draft_boundary = BehaviorDraftBoundary::DraftOnly;
        let err = spec.validate().unwrap_err().to_string();
        assert!(
            err.contains("Safe Source Apply review"),
            "unexpected error: {err}"
        );
    }

    fn hazard_spec() -> BehaviorAuthoringSpec {
        BehaviorAuthoringSpec {
            schema_version: BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION.to_string(),
            behavior_id: "hazard-spike-loop".to_string(),
            domain: BehaviorAuthoringDomain::Hazard,
            initial_state: "armed".to_string(),
            states: vec![
                BehaviorState {
                    id: "armed".to_string(),
                    label: "Armed".to_string(),
                    tags: vec!["hazard".to_string()],
                },
                BehaviorState {
                    id: "cooldown".to_string(),
                    label: "Cooldown".to_string(),
                    tags: vec!["hazard".to_string()],
                },
            ],
            transitions: vec![BehaviorTransition {
                id: "player-contact".to_string(),
                from: "armed".to_string(),
                to: "cooldown".to_string(),
                trigger: BehaviorTrigger {
                    kind: BehaviorTriggerKind::OnPlayerContact,
                    parameters: BTreeMap::new(),
                },
                actions: vec![
                    BehaviorAction {
                        kind: BehaviorActionKind::ApplyDamage,
                        parameters: map([(
                            "amount".to_string(),
                            BehaviorParameterValue::Integer(1),
                        )]),
                    },
                    BehaviorAction {
                        kind: BehaviorActionKind::EmitEvent,
                        parameters: map([(
                            "event".to_string(),
                            BehaviorParameterValue::TextId("hazard.damage".to_string()),
                        )]),
                    },
                ],
            }],
            parameter_schema: vec![BehaviorParameter {
                name: "amount".to_string(),
                value_type: BehaviorParameterType::Integer,
                default: Some(BehaviorParameterValue::Integer(1)),
                min: Some(1),
                max: Some(10),
            }],
            preview: BehaviorPreviewContract {
                preview_mode: BehaviorPreviewMode::ScenarioAssertionGeneration,
                deterministic_seed: 1241,
                expected_events: vec!["hazard.damage".to_string()],
                evidence_refs: vec![BehaviorEvidenceRef {
                    run_id: "run-hazard-1".to_string(),
                    path: "evidence/runtime-events.json".to_string(),
                    digest: "sha256:abc123".to_string(),
                }],
            },
            scenario_assertions: vec![BehaviorScenarioAssertion {
                id: "hazard-damage-event".to_string(),
                source: "player-contact".to_string(),
                assertion_kind: BehaviorScenarioAssertionKind::RuntimeEvent,
                expected_path: "runtimeEvents/hazard.damage".to_string(),
                expected_value: BehaviorParameterValue::Bool(true),
            }],
            draft_boundary: BehaviorDraftBoundary::SafeSourceApplyReviewRequired,
            guardrails: vec![
                "data-only states transitions triggers actions parameters".to_string(),
                "draft changes require Safe Source Apply review".to_string(),
            ],
        }
    }

    fn map<const N: usize>(
        items: [(String, BehaviorParameterValue); N],
    ) -> BTreeMap<String, BehaviorParameterValue> {
        items.into_iter().collect()
    }
}
