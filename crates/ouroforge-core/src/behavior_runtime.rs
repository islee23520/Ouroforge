use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

pub const BEHAVIOR_ARTIFACT_SCHEMA_VERSION: &str = "ouroforge.behavior-artifact.v1";

const SUPPORTED_TRIGGERS: &[&str] = &[
    "onStart",
    "onTick",
    "onEvent",
    "onCollision",
    "onInputAction",
    "onStateEnter",
];
const SUPPORTED_CONDITIONS: &[&str] = &[
    "flagEquals",
    "stateEquals",
    "eventEquals",
    "hasItem",
    "cooldownReady",
    "always",
];
const SUPPORTED_ACTIONS: &[&str] = &[
    "setFlag",
    "moveEntity",
    "changeState",
    "emitEvent",
    "damage",
    "heal",
    "collectItem",
    "removeItem",
    "startAnimationIntent",
    "startAudioIntent",
    "markWinState",
    "markLossState",
];
const SUPPORTED_EFFECTS: &[&str] = &[
    "flag",
    "movement",
    "state",
    "event",
    "health",
    "inventory",
    "animationIntent",
    "audioIntent",
    "terminalState",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    #[serde(rename = "validatedBy")]
    pub validated_by: BehaviorValidationAuthority,
    #[serde(default)]
    pub behaviors: Vec<BehaviorDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorValidationAuthority {
    pub authority: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: String,
    #[serde(
        rename = "reviewDecisionId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_decision_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDefinition {
    pub id: String,
    #[serde(rename = "entityId")]
    pub entity_id: String,
    #[serde(default)]
    pub triggers: Vec<BehaviorTrigger>,
    #[serde(default)]
    pub conditions: Vec<BehaviorCondition>,
    #[serde(default)]
    pub actions: Vec<BehaviorAction>,
    #[serde(
        rename = "stateMachine",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub state_machine: Option<BehaviorStateMachine>,
    #[serde(default)]
    pub abilities: Vec<BehaviorAbility>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorTrigger {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub event: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorCondition {
    pub id: String,
    pub kind: String,
    #[serde(default)]
    pub field: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAction {
    pub id: String,
    pub kind: String,
    #[serde(rename = "effectKind")]
    pub effect_kind: String,
    #[serde(
        rename = "targetEntityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_entity_id: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorStateMachine {
    #[serde(rename = "initialState")]
    pub initial_state: String,
    pub states: Vec<BehaviorState>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorState {
    pub id: String,
    #[serde(default)]
    pub transitions: Vec<BehaviorStateTransition>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorStateTransition {
    pub id: String,
    pub to: String,
    #[serde(rename = "onEvent", default, skip_serializing_if = "Option::is_none")]
    pub on_event: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAbility {
    pub id: String,
    #[serde(default)]
    pub actions: Vec<BehaviorAction>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorRuntimeState {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    pub status: BehaviorRuntimeStatus,
    pub counts: BehaviorRuntimeCounts,
    #[serde(rename = "loadedBehaviorIds")]
    pub loaded_behavior_ids: Vec<String>,
    #[serde(rename = "entityBehaviorIds")]
    pub entity_behavior_ids: BTreeMap<String, Vec<String>>,
    pub diagnostics: Vec<BehaviorRuntimeDiagnostic>,
    #[serde(rename = "trustedBoundary")]
    pub trusted_boundary: BehaviorRuntimeBoundary,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorRuntimeStatus {
    Ready,
    ReadyWithWarnings,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorRuntimeCounts {
    #[serde(rename = "behaviorCount")]
    pub behavior_count: u64,
    #[serde(rename = "triggerCount")]
    pub trigger_count: u64,
    #[serde(rename = "conditionCount")]
    pub condition_count: u64,
    #[serde(rename = "actionCount")]
    pub action_count: u64,
    #[serde(rename = "stateMachineCount")]
    pub state_machine_count: u64,
    #[serde(rename = "abilityCount")]
    pub ability_count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorRuntimeDiagnostic {
    pub severity: BehaviorDiagnosticSeverity,
    pub code: String,
    pub message: String,
    #[serde(
        rename = "behaviorId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_id: Option<String>,
    #[serde(rename = "itemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorDiagnosticSeverity {
    Warning,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorRuntimeBoundary {
    #[serde(rename = "trustedLoader")]
    pub trusted_loader: String,
    #[serde(rename = "executionMode")]
    pub execution_mode: String,
    #[serde(rename = "disallowedActions")]
    pub disallowed_actions: Vec<String>,
}

impl BehaviorArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Behavior Artifact JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_ARTIFACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "schemaVersion must be {BEHAVIOR_ARTIFACT_SCHEMA_VERSION}"
            ));
        }
        require_local_id("artifactId", &self.artifact_id)?;
        require_local_id("sceneId", &self.scene_id)?;
        self.validated_by.validate()?;
        if self.behaviors.is_empty() {
            return Err(anyhow!("behaviors must not be empty"));
        }

        let mut behavior_ids = BTreeSet::new();
        for behavior in &self.behaviors {
            behavior.validate()?;
            if !behavior_ids.insert(behavior.id.clone()) {
                return Err(anyhow!("behavior id must be unique: {}", behavior.id));
            }
        }
        Ok(())
    }

    pub fn runtime_state(&self) -> BehaviorRuntimeState {
        let mut loaded_behavior_ids = Vec::new();
        let mut entity_behavior_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut diagnostics = Vec::new();
        let mut counts = BehaviorRuntimeCounts {
            behavior_count: self.behaviors.len() as u64,
            trigger_count: 0,
            condition_count: 0,
            action_count: 0,
            state_machine_count: 0,
            ability_count: 0,
        };

        for behavior in &self.behaviors {
            loaded_behavior_ids.push(behavior.id.clone());
            entity_behavior_ids
                .entry(behavior.entity_id.clone())
                .or_default()
                .push(behavior.id.clone());
            counts.trigger_count += behavior.triggers.len() as u64;
            counts.condition_count += behavior.conditions.len() as u64;
            counts.action_count += behavior.actions.len() as u64;
            counts.state_machine_count += u64::from(behavior.state_machine.is_some());
            counts.ability_count += behavior.abilities.len() as u64;

            for trigger in &behavior.triggers {
                if !SUPPORTED_TRIGGERS.contains(&trigger.kind.as_str()) {
                    diagnostics.push(unsupported_diagnostic(
                        "unsupportedTrigger",
                        format!("unsupported behavior trigger kind: {}", trigger.kind),
                        &behavior.id,
                        &trigger.id,
                    ));
                }
            }
            for condition in &behavior.conditions {
                if !SUPPORTED_CONDITIONS.contains(&condition.kind.as_str()) {
                    diagnostics.push(unsupported_diagnostic(
                        "unsupportedCondition",
                        format!("unsupported behavior condition kind: {}", condition.kind),
                        &behavior.id,
                        &condition.id,
                    ));
                }
            }
            collect_action_diagnostics(&mut diagnostics, &behavior.id, &behavior.actions);
            for ability in &behavior.abilities {
                counts.action_count += ability.actions.len() as u64;
                collect_action_diagnostics(&mut diagnostics, &behavior.id, &ability.actions);
            }
        }

        BehaviorRuntimeState {
            schema_version: "ouroforge.behavior-runtime-state.v1".to_string(),
            artifact_id: self.artifact_id.clone(),
            scene_id: self.scene_id.clone(),
            status: if diagnostics.is_empty() {
                BehaviorRuntimeStatus::Ready
            } else {
                BehaviorRuntimeStatus::ReadyWithWarnings
            },
            counts,
            loaded_behavior_ids,
            entity_behavior_ids,
            diagnostics,
            trusted_boundary: BehaviorRuntimeBoundary {
                trusted_loader: self.validated_by.authority.clone(),
                execution_mode:
                    "structured-data-only; no arbitrary scripts or dynamic code loading".to_string(),
                disallowed_actions: vec![
                    "eval".to_string(),
                    "dynamic import".to_string(),
                    "plugin loader".to_string(),
                    "command bridge".to_string(),
                    "browser trusted writes".to_string(),
                    "local server bridge".to_string(),
                ],
            },
        }
    }
}

impl BehaviorValidationAuthority {
    fn validate(&self) -> Result<()> {
        require_local_text("validatedBy.authority", &self.authority)?;
        require_local_text("validatedBy.validationStatus", &self.validation_status)?;
        if self.validation_status != "passed" {
            return Err(anyhow!("validatedBy.validationStatus must be passed"));
        }
        if let Some(review_decision_id) = &self.review_decision_id {
            require_local_id("validatedBy.reviewDecisionId", review_decision_id)?;
        }
        Ok(())
    }
}

impl BehaviorDefinition {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior.id", &self.id)?;
        require_local_id("behavior.entityId", &self.entity_id)?;
        require_unique_ids(
            "trigger id",
            self.triggers.iter().map(|trigger| &trigger.id),
        )?;
        require_unique_ids(
            "condition id",
            self.conditions.iter().map(|condition| &condition.id),
        )?;
        require_unique_ids("action id", self.actions.iter().map(|action| &action.id))?;
        for trigger in &self.triggers {
            trigger.validate()?;
        }
        for condition in &self.conditions {
            condition.validate()?;
        }
        for action in &self.actions {
            action.validate()?;
        }
        if let Some(state_machine) = &self.state_machine {
            state_machine.validate()?;
        }
        for ability in &self.abilities {
            ability.validate()?;
        }
        Ok(())
    }
}

impl BehaviorTrigger {
    fn validate(&self) -> Result<()> {
        require_local_id("trigger.id", &self.id)?;
        require_local_text("trigger.kind", &self.kind)?;
        if let Some(event) = &self.event {
            require_local_id("trigger.event", event)?;
        }
        Ok(())
    }
}

impl BehaviorCondition {
    fn validate(&self) -> Result<()> {
        require_local_id("condition.id", &self.id)?;
        require_local_text("condition.kind", &self.kind)?;
        if let Some(field) = &self.field {
            require_local_id("condition.field", field)?;
        }
        if let Some(value) = &self.value {
            require_local_text("condition.value", value)?;
        }
        Ok(())
    }
}

impl BehaviorAction {
    fn validate(&self) -> Result<()> {
        require_local_id("action.id", &self.id)?;
        require_local_text("action.kind", &self.kind)?;
        require_local_text("action.effectKind", &self.effect_kind)?;
        if let Some(target_entity_id) = &self.target_entity_id {
            require_local_id("action.targetEntityId", target_entity_id)?;
        }
        if let Some(value) = &self.value {
            require_local_text("action.value", value)?;
        }
        Ok(())
    }
}

impl BehaviorStateMachine {
    fn validate(&self) -> Result<()> {
        require_local_id("stateMachine.initialState", &self.initial_state)?;
        if self.states.is_empty() {
            return Err(anyhow!("stateMachine.states must not be empty"));
        }
        let mut states = BTreeSet::new();
        for state in &self.states {
            state.validate()?;
            if !states.insert(state.id.clone()) {
                return Err(anyhow!("state id must be unique: {}", state.id));
            }
        }
        if !states.contains(&self.initial_state) {
            return Err(anyhow!(
                "stateMachine.initialState must reference a declared state"
            ));
        }
        for state in &self.states {
            for transition in &state.transitions {
                if !states.contains(&transition.to) {
                    return Err(anyhow!(
                        "state transition target must reference a declared state: {}",
                        transition.to
                    ));
                }
            }
        }
        Ok(())
    }
}

impl BehaviorState {
    fn validate(&self) -> Result<()> {
        require_local_id("state.id", &self.id)?;
        require_unique_ids(
            "transition id",
            self.transitions.iter().map(|transition| &transition.id),
        )?;
        for transition in &self.transitions {
            transition.validate()?;
        }
        Ok(())
    }
}

impl BehaviorStateTransition {
    fn validate(&self) -> Result<()> {
        require_local_id("transition.id", &self.id)?;
        require_local_id("transition.to", &self.to)?;
        if let Some(on_event) = &self.on_event {
            require_local_id("transition.onEvent", on_event)?;
        }
        Ok(())
    }
}

impl BehaviorAbility {
    fn validate(&self) -> Result<()> {
        require_local_id("ability.id", &self.id)?;
        require_unique_ids(
            "ability action id",
            self.actions.iter().map(|action| &action.id),
        )?;
        for action in &self.actions {
            action.validate()?;
        }
        Ok(())
    }
}

pub fn read_behavior_artifact(path: impl AsRef<Path>) -> Result<BehaviorArtifact> {
    let path = path.as_ref();
    let input = fs::read_to_string(path)
        .with_context(|| format!("failed to read behavior artifact {}", path.display()))?;
    BehaviorArtifact::from_json_str(&input)
        .with_context(|| format!("failed to validate behavior artifact {}", path.display()))
}

fn collect_action_diagnostics(
    diagnostics: &mut Vec<BehaviorRuntimeDiagnostic>,
    behavior_id: &str,
    actions: &[BehaviorAction],
) {
    for action in actions {
        if !SUPPORTED_ACTIONS.contains(&action.kind.as_str()) {
            diagnostics.push(unsupported_diagnostic(
                "unsupportedAction",
                format!("unsupported behavior action kind: {}", action.kind),
                behavior_id,
                &action.id,
            ));
        }
        if !SUPPORTED_EFFECTS.contains(&action.effect_kind.as_str()) {
            diagnostics.push(unsupported_diagnostic(
                "unsupportedEffect",
                format!("unsupported behavior effect kind: {}", action.effect_kind),
                behavior_id,
                &action.id,
            ));
        }
    }
}

fn unsupported_diagnostic(
    code: &str,
    message: String,
    behavior_id: &str,
    item_id: &str,
) -> BehaviorRuntimeDiagnostic {
    BehaviorRuntimeDiagnostic {
        severity: BehaviorDiagnosticSeverity::Warning,
        code: code.to_string(),
        message,
        behavior_id: Some(behavior_id.to_string()),
        item_id: Some(item_id.to_string()),
    }
}

fn require_unique_ids<'a>(field: &str, ids: impl Iterator<Item = &'a String>) -> Result<()> {
    let mut seen = BTreeSet::new();
    for id in ids {
        require_local_id(field, id)?;
        if !seen.insert(id.as_str()) {
            return Err(anyhow!("{field} must be unique: {id}"));
        }
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    if value.contains('/') || value.contains('\\') || value == "." || value == ".." {
        return Err(anyhow!("{field} must be a local identifier"));
    }
    Ok(())
}

fn require_local_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    Ok(())
}
