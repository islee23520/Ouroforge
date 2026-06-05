use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

pub const BEHAVIOR_ARTIFACT_SCHEMA_VERSION: &str = "ouroforge.behavior-artifact.v1";

pub const BEHAVIOR_DRAFT_SCHEMA_VERSION: &str = "ouroforge.behavior-draft.v1";

pub const BEHAVIOR_APPLY_TRANSACTION_SCHEMA_VERSION: &str =
    "ouroforge.behavior-apply-transaction.v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    pub target: BehaviorDraftTarget,
    #[serde(rename = "proposedBehavior")]
    pub proposed_behavior: BehaviorArtifact,
    pub rationale: String,
    #[serde(rename = "linkedEvidence", default)]
    pub linked_evidence: Vec<BehaviorDraftEvidenceRef>,
    #[serde(rename = "expectedScenarioImpact", default)]
    pub expected_scenario_impact: Vec<BehaviorDraftScenarioImpact>,
    pub author: BehaviorDraftAuthor,
    #[serde(rename = "validationStatus")]
    pub validation_status: BehaviorDraftValidationStatus,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "untrustedBoundary")]
    pub untrusted_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftTarget {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    #[serde(rename = "sceneHash")]
    pub scene_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftEvidenceRef {
    pub id: String,
    pub kind: String,
    pub path: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftScenarioImpact {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    pub summary: String,
    #[serde(rename = "expectedVerdict")]
    pub expected_verdict: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftAuthor {
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorDraftValidationStatus {
    Drafted,
    Valid,
    Blocked,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorApplyTransactionArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    #[serde(rename = "reviewDecision")]
    pub review_decision: BehaviorApplyReviewDecision,
    pub target: BehaviorDraftTarget,
    #[serde(rename = "targetHashes")]
    pub target_hashes: BehaviorApplyTargetHashes,
    #[serde(rename = "proposedBehavior")]
    pub proposed_behavior: BehaviorArtifact,
    #[serde(rename = "transactionOutputRef")]
    pub transaction_output_ref: String,
    #[serde(rename = "rollbackMetadata")]
    pub rollback_metadata: BehaviorApplyRollbackMetadata,
    #[serde(rename = "rerunCommand")]
    pub rerun_command: BehaviorApplyRerunCommand,
    #[serde(rename = "evidenceRefs", default)]
    pub evidence_refs: Vec<BehaviorDraftEvidenceRef>,
    pub status: BehaviorApplyTransactionStatus,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "trustedBoundary")]
    pub trusted_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorApplyReviewDecision {
    #[serde(rename = "reviewDecisionId")]
    pub review_decision_id: String,
    pub status: BehaviorApplyReviewDecisionStatus,
    #[serde(rename = "reviewerId")]
    pub reviewer_id: String,
    #[serde(rename = "draftAuthorId")]
    pub draft_author_id: String,
    #[serde(rename = "decisionRef")]
    pub decision_ref: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorApplyReviewDecisionStatus {
    Accepted,
    Rejected,
    Missing,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorApplyTargetHashes {
    #[serde(rename = "expectedBeforeHash")]
    pub expected_before_hash: String,
    #[serde(rename = "observedBeforeHash")]
    pub observed_before_hash: String,
    #[serde(rename = "afterHash")]
    pub after_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorApplyRollbackMetadata {
    #[serde(rename = "beforeHash")]
    pub before_hash: String,
    #[serde(rename = "rollbackRef")]
    pub rollback_ref: String,
    pub strategy: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorApplyRerunCommand {
    pub command: String,
    pub argv: Vec<String>,
    #[serde(rename = "allowlistPolicyId")]
    pub allowlist_policy_id: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorApplyTransactionStatus {
    ReadyForTrustedApply,
    MissingReview,
    Rejected,
    Blocked,
    Stale,
}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dx: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dy: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct BehaviorExecutionInput {
    #[serde(rename = "triggerKind")]
    pub trigger_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(
        rename = "inputAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_action: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct BehaviorWorldState {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub flags: BTreeMap<String, bool>,
    #[serde(
        rename = "entityPositions",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub entity_positions: BTreeMap<String, BehaviorEntityPosition>,
    #[serde(
        rename = "entityHealth",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub entity_health: BTreeMap<String, i32>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub inventory: BTreeSet<String>,
    #[serde(
        rename = "activeStates",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub active_states: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(
        rename = "animationIntents",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub animation_intents: Vec<BehaviorIntent>,
    #[serde(
        rename = "audioIntents",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub audio_intents: Vec<BehaviorIntent>,
    #[serde(
        rename = "terminalState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub terminal_state: Option<BehaviorTerminalState>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(deny_unknown_fields)]
pub struct BehaviorEntityPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorIntent {
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "targetEntityId")]
    pub target_entity_id: String,
    pub intent: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorTerminalState {
    Win,
    Loss,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorExecutionReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    pub input: BehaviorExecutionInput,
    #[serde(rename = "initialWorldState", default)]
    pub initial_world_state: BehaviorWorldState,
    #[serde(rename = "appliedActions")]
    pub applied_actions: Vec<BehaviorAppliedAction>,
    pub diagnostics: Vec<BehaviorRuntimeDiagnostic>,
    #[serde(rename = "worldState")]
    pub world_state: BehaviorWorldState,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorRuntimeEvidenceBundle {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    pub summary: BehaviorRuntimeEvidenceSummary,
    pub reports: Vec<BehaviorExecutionEvidence>,
    #[serde(rename = "trustedBoundary")]
    pub trusted_boundary: BehaviorRuntimeBoundary,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioAssertionSuite {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "suiteId")]
    pub suite_id: String,
    #[serde(
        rename = "scenarioId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_id: Option<String>,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    pub assertions: Vec<BehaviorScenarioAssertion>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioAssertion {
    #[serde(rename = "assertionId")]
    pub assertion_id: String,
    pub kind: BehaviorScenarioAssertionKind,
    #[serde(
        rename = "reportIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_index: Option<u64>,
    #[serde(
        rename = "behaviorId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub behavior_id: Option<String>,
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flag: Option<String>,
    #[serde(
        rename = "expectedFlag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expected_flag: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(
        rename = "terminalState",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub terminal_state: Option<BehaviorTerminalState>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorScenarioAssertionKind {
    BehaviorExecuted,
    EventEmitted,
    FlagChanged,
    StateTransitionOccurred,
    AbilityUsed,
    CooldownActive,
    EntityAffected,
    ItemCollected,
    TerminalStateReached,
    UnsupportedBehaviorBlocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioAssertionVerdict {
    #[serde(rename = "assertionId")]
    pub assertion_id: String,
    pub status: BehaviorScenarioAssertionStatus,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioAssertionResultArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "suiteId")]
    pub suite_id: String,
    #[serde(
        rename = "scenarioId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_id: Option<String>,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    pub status: BehaviorScenarioAssertionStatus,
    pub assertions: Vec<BehaviorScenarioAssertionVerdict>,
    #[serde(rename = "trustedBoundary")]
    pub trusted_boundary: BehaviorRuntimeBoundary,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BehaviorScenarioAssertionStatus {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorRuntimeEvidenceSummary {
    #[serde(rename = "reportCount")]
    pub report_count: u64,
    #[serde(rename = "appliedActionCount")]
    pub applied_action_count: u64,
    #[serde(rename = "diagnosticCount")]
    pub diagnostic_count: u64,
    #[serde(rename = "terminalStateCount")]
    pub terminal_state_count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorExecutionEvidence {
    #[serde(rename = "reportIndex")]
    pub report_index: u64,
    #[serde(rename = "replayKey")]
    pub replay_key: String,
    #[serde(rename = "triggerKind")]
    pub trigger_kind: String,
    #[serde(rename = "event", default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(
        rename = "inputAction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_action: Option<String>,
    #[serde(rename = "appliedActionIds")]
    pub applied_action_ids: Vec<String>,
    #[serde(
        rename = "cooldownBehaviorIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cooldown_behavior_ids: Vec<String>,
    pub diagnostics: Vec<BehaviorRuntimeDiagnostic>,
    #[serde(rename = "initialWorldState", default)]
    pub initial_world_state: BehaviorWorldState,
    #[serde(rename = "worldState")]
    pub world_state: BehaviorWorldState,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAppliedAction {
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    #[serde(rename = "actionId")]
    pub action_id: String,
    pub kind: String,
    #[serde(rename = "targetEntityId")]
    pub target_entity_id: String,
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

impl BehaviorDraftArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse Behavior Draft Artifact JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "behavior draft schemaVersion must be {BEHAVIOR_DRAFT_SCHEMA_VERSION}"
            ));
        }
        require_local_id("behavior draft draftId", &self.draft_id)?;
        self.target.validate()?;
        self.proposed_behavior
            .validate()
            .context("behavior draft proposedBehavior is invalid")?;
        require_local_text("behavior draft rationale", &self.rationale)?;
        self.author.validate()?;
        require_local_text("behavior draft untrustedBoundary", &self.untrusted_boundary)?;
        let boundary = self.untrusted_boundary.to_ascii_lowercase();
        for required in ["untrusted", "does not apply", "no arbitrary"] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "behavior draft untrustedBoundary must state `{required}`"
                ));
            }
        }

        require_unique_ids(
            "behavior draft linkedEvidence.id",
            self.linked_evidence.iter().map(|evidence| &evidence.id),
        )?;
        for evidence in &self.linked_evidence {
            evidence.validate()?;
        }
        require_unique_ids(
            "behavior draft expectedScenarioImpact.scenarioId",
            self.expected_scenario_impact
                .iter()
                .map(|impact| &impact.scenario_id),
        )?;
        for impact in &self.expected_scenario_impact {
            impact.validate()?;
        }
        for reason in &self.blocked_reasons {
            require_local_text("behavior draft blockedReasons", reason)?;
        }

        let diagnostics = self.proposed_behavior.runtime_state().diagnostics;
        let has_unsupported = !diagnostics.is_empty();
        match self.validation_status {
            BehaviorDraftValidationStatus::Valid | BehaviorDraftValidationStatus::Drafted => {
                if self.linked_evidence.is_empty() {
                    return Err(anyhow!(
                        "behavior draft linkedEvidence is required before valid or drafted status"
                    ));
                }
                if has_unsupported {
                    return Err(anyhow!(
                        "behavior draft unsupported behavior must be blocked before validation"
                    ));
                }
                if !self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "behavior draft valid/drafted status must not include blockedReasons"
                    ));
                }
            }
            BehaviorDraftValidationStatus::Blocked => {
                if self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "behavior draft blocked status requires blockedReasons"
                    ));
                }
                if has_unsupported && !blocked_reasons_contain(&self.blocked_reasons, "unsupported")
                {
                    return Err(anyhow!(
                        "behavior draft unsupported behavior must be visible in blockedReasons"
                    ));
                }
                if self.linked_evidence.is_empty()
                    && !blocked_reasons_contain(&self.blocked_reasons, "evidence")
                {
                    return Err(anyhow!(
                        "behavior draft missing evidence must be visible in blockedReasons"
                    ));
                }
            }
            BehaviorDraftValidationStatus::Stale => {
                if self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "behavior draft stale status requires blockedReasons"
                    ));
                }
                if !blocked_reasons_contain(&self.blocked_reasons, "stale")
                    && !blocked_reasons_contain(&self.blocked_reasons, "hash")
                {
                    return Err(anyhow!(
                        "behavior draft stale status blockedReasons must mention stale target or hash drift"
                    ));
                }
            }
        }
        Ok(())
    }
}

impl BehaviorDraftTarget {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior draft target.projectId", &self.project_id)?;
        require_relative_json_path(
            "behavior draft target.scenePath",
            &self.scene_path,
            ".scene.json",
        )?;
        require_hash_text("behavior draft target.sceneHash", &self.scene_hash)
    }
}

impl BehaviorDraftEvidenceRef {
    fn validate(&self) -> Result<()> {
        require_local_id("behavior draft linkedEvidence.id", &self.id)?;
        require_local_id("behavior draft linkedEvidence.kind", &self.kind)?;
        require_relative_json_path("behavior draft linkedEvidence.path", &self.path, ".json")?;
        require_local_text("behavior draft linkedEvidence.summary", &self.summary)
    }
}

impl BehaviorDraftScenarioImpact {
    fn validate(&self) -> Result<()> {
        require_local_id(
            "behavior draft expectedScenarioImpact.scenarioId",
            &self.scenario_id,
        )?;
        require_local_text(
            "behavior draft expectedScenarioImpact.summary",
            &self.summary,
        )?;
        match self.expected_verdict.as_str() {
            "passed" | "failed" | "blocked" | "unknown" => Ok(()),
            _ => Err(anyhow!(
                "behavior draft expectedScenarioImpact.expectedVerdict must be passed, failed, blocked, or unknown"
            )),
        }
    }
}

impl BehaviorDraftAuthor {
    fn validate(&self) -> Result<()> {
        match self.source.as_str() {
            "agent" | "human" | "fixture" => {}
            _ => {
                return Err(anyhow!(
                    "behavior draft author.source must be agent, human, or fixture"
                ))
            }
        }
        if let Some(actor) = &self.actor {
            require_local_id("behavior draft author.actor", actor)?;
        }
        Ok(())
    }
}

impl BehaviorApplyTransactionArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse Behavior Apply Transaction Artifact JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_APPLY_TRANSACTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "behavior apply transaction schemaVersion must be {BEHAVIOR_APPLY_TRANSACTION_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "behavior apply transaction transactionId",
            &self.transaction_id,
        )?;
        require_local_id("behavior apply transaction draftId", &self.draft_id)?;
        self.review_decision.validate()?;
        self.target.validate()?;
        self.target_hashes.validate(&self.target.scene_hash)?;
        self.proposed_behavior
            .validate()
            .context("behavior apply transaction proposedBehavior is invalid")?;
        if !self
            .proposed_behavior
            .runtime_state()
            .diagnostics
            .is_empty()
        {
            return Err(anyhow!(
                "behavior apply transaction unsupported behavior must remain blocked before apply"
            ));
        }
        require_behavior_transaction_output_ref(
            "behavior apply transaction transactionOutputRef",
            &self.transaction_output_ref,
            &self.target.scene_path,
        )?;
        self.rollback_metadata.validate(&self.target_hashes)?;
        self.rerun_command.validate()?;
        require_unique_ids(
            "behavior apply transaction evidenceRefs.id",
            self.evidence_refs.iter().map(|evidence| &evidence.id),
        )?;
        for evidence in &self.evidence_refs {
            evidence.validate()?;
        }
        for reason in &self.blocked_reasons {
            require_local_text("behavior apply transaction blockedReasons", reason)?;
        }
        require_local_text(
            "behavior apply transaction trustedBoundary",
            &self.trusted_boundary,
        )?;
        let trusted_boundary = self.trusted_boundary.to_ascii_lowercase();
        for required in [
            "accepted review",
            "rollback",
            "no arbitrary",
            "no auto-apply",
            "no self-approval",
        ] {
            if !trusted_boundary.contains(required) {
                return Err(anyhow!(
                    "behavior apply transaction trustedBoundary must state `{required}`"
                ));
            }
        }

        match self.status {
            BehaviorApplyTransactionStatus::ReadyForTrustedApply => {
                if !self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "behavior apply transaction ready_for_trusted_apply status must not include blockedReasons"
                    ));
                }
                if self.review_decision.status != BehaviorApplyReviewDecisionStatus::Accepted {
                    return Err(anyhow!(
                        "behavior apply transaction ready_for_trusted_apply requires accepted review"
                    ));
                }
                if self.review_decision.reviewer_id == self.review_decision.draft_author_id {
                    return Err(anyhow!(
                        "behavior apply transaction ready_for_trusted_apply forbids self-approval"
                    ));
                }
                if self.target_hashes.expected_before_hash
                    != self.target_hashes.observed_before_hash
                {
                    return Err(anyhow!(
                        "behavior apply transaction ready_for_trusted_apply requires fresh target hashes"
                    ));
                }
            }
            BehaviorApplyTransactionStatus::MissingReview => {
                if self.review_decision.status != BehaviorApplyReviewDecisionStatus::Missing {
                    return Err(anyhow!(
                        "behavior apply transaction missing_review status requires missing review decision"
                    ));
                }
                require_blocked_reason(
                    "behavior apply transaction missing_review status",
                    &self.blocked_reasons,
                    "review",
                )?;
            }
            BehaviorApplyTransactionStatus::Rejected => {
                if self.review_decision.status != BehaviorApplyReviewDecisionStatus::Rejected {
                    return Err(anyhow!(
                        "behavior apply transaction rejected status requires rejected review decision"
                    ));
                }
                require_blocked_reason(
                    "behavior apply transaction rejected status",
                    &self.blocked_reasons,
                    "reject",
                )?;
            }
            BehaviorApplyTransactionStatus::Blocked => {
                if self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "behavior apply transaction blocked status requires blockedReasons"
                    ));
                }
            }
            BehaviorApplyTransactionStatus::Stale => {
                if self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "behavior apply transaction stale status requires blockedReasons"
                    ));
                }
                if !blocked_reasons_contain(&self.blocked_reasons, "stale")
                    && !blocked_reasons_contain(&self.blocked_reasons, "hash")
                {
                    return Err(anyhow!(
                        "behavior apply transaction stale status blockedReasons must mention stale target or hash drift"
                    ));
                }
            }
        }
        Ok(())
    }
}

impl BehaviorApplyReviewDecision {
    fn validate(&self) -> Result<()> {
        require_local_id(
            "behavior apply transaction reviewDecision.reviewDecisionId",
            &self.review_decision_id,
        )?;
        require_local_id(
            "behavior apply transaction reviewDecision.reviewerId",
            &self.reviewer_id,
        )?;
        require_local_id(
            "behavior apply transaction reviewDecision.draftAuthorId",
            &self.draft_author_id,
        )?;
        require_relative_json_path(
            "behavior apply transaction reviewDecision.decisionRef",
            &self.decision_ref,
            ".json",
        )
    }
}

impl BehaviorApplyTargetHashes {
    fn validate(&self, draft_target_hash: &str) -> Result<()> {
        require_hash_text(
            "behavior apply transaction targetHashes.expectedBeforeHash",
            &self.expected_before_hash,
        )?;
        require_hash_text(
            "behavior apply transaction targetHashes.observedBeforeHash",
            &self.observed_before_hash,
        )?;
        require_hash_text(
            "behavior apply transaction targetHashes.afterHash",
            &self.after_hash,
        )?;
        if self.expected_before_hash != draft_target_hash {
            return Err(anyhow!(
                "behavior apply transaction targetHashes.expectedBeforeHash must match draft target.sceneHash"
            ));
        }
        if self.after_hash == self.observed_before_hash {
            return Err(anyhow!(
                "behavior apply transaction targetHashes.afterHash must differ from observedBeforeHash"
            ));
        }
        Ok(())
    }
}

impl BehaviorApplyRollbackMetadata {
    fn validate(&self, target_hashes: &BehaviorApplyTargetHashes) -> Result<()> {
        require_hash_text(
            "behavior apply transaction rollbackMetadata.beforeHash",
            &self.before_hash,
        )?;
        if self.before_hash != target_hashes.observed_before_hash {
            return Err(anyhow!(
                "behavior apply transaction rollbackMetadata.beforeHash must match observedBeforeHash"
            ));
        }
        require_relative_json_path(
            "behavior apply transaction rollbackMetadata.rollbackRef",
            &self.rollback_ref,
            ".json",
        )?;
        require_local_text(
            "behavior apply transaction rollbackMetadata.strategy",
            &self.strategy,
        )?;
        let strategy = self.strategy.to_ascii_lowercase();
        if !strategy.contains("beforehash") && !strategy.contains("before hash") {
            return Err(anyhow!(
                "behavior apply transaction rollbackMetadata.strategy must reference beforeHash rollback"
            ));
        }
        Ok(())
    }
}

impl BehaviorApplyRerunCommand {
    fn validate(&self) -> Result<()> {
        require_local_text(
            "behavior apply transaction rerunCommand.command",
            &self.command,
        )?;
        require_local_id(
            "behavior apply transaction rerunCommand.allowlistPolicyId",
            &self.allowlist_policy_id,
        )?;
        if self.argv.is_empty() {
            return Err(anyhow!(
                "behavior apply transaction rerunCommand.argv must not be empty"
            ));
        }
        for arg in &self.argv {
            require_local_text("behavior apply transaction rerunCommand.argv", arg)?;
        }
        let command = self.command.to_ascii_lowercase();
        for forbidden in ["eval", "dynamic import", "npm install", "curl ", "bash -c"] {
            if command.contains(forbidden) {
                return Err(anyhow!(
                    "behavior apply transaction rerunCommand.command must stay on the local allowlist"
                ));
            }
        }
        Ok(())
    }
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

    pub fn execute(
        &self,
        input: BehaviorExecutionInput,
        world_state: BehaviorWorldState,
    ) -> BehaviorExecutionReport {
        // Snapshot the starting world before mutation so the replay key identifies
        // the input world state and the run can actually be replayed (two different
        // initial states that converge to the same final report must stay distinct).
        let initial_world_state = world_state.clone();
        let mut world_state = world_state;
        let mut applied_actions = Vec::new();
        let mut diagnostics = self.runtime_state().diagnostics;
        let mut queued_events = VecDeque::new();
        if let Some(event) = &input.event {
            queued_events.push_back(event.clone());
        }

        for behavior in &self.behaviors {
            initialize_behavior_state(behavior, &mut world_state);
            if !behavior_matches_input(behavior, &input) {
                continue;
            }
            if !conditions_match(behavior, &input, &world_state) {
                continue;
            }

            transition_state_machine(behavior, &input, &mut world_state, &mut applied_actions);
            apply_actions(
                &behavior.id,
                &behavior.entity_id,
                &behavior.actions,
                &mut world_state,
                &mut applied_actions,
                &mut diagnostics,
                &mut queued_events,
            );
            for ability in &behavior.abilities {
                apply_actions(
                    &behavior.id,
                    &behavior.entity_id,
                    &ability.actions,
                    &mut world_state,
                    &mut applied_actions,
                    &mut diagnostics,
                    &mut queued_events,
                );
            }
        }

        while let Some(event) = queued_events.pop_front() {
            world_state.events.push(event);
        }

        BehaviorExecutionReport {
            schema_version: "ouroforge.behavior-execution-report.v1".to_string(),
            artifact_id: self.artifact_id.clone(),
            scene_id: self.scene_id.clone(),
            input,
            initial_world_state,
            applied_actions,
            diagnostics,
            world_state,
        }
    }

    pub fn evidence_bundle(
        &self,
        reports: Vec<BehaviorExecutionReport>,
    ) -> BehaviorRuntimeEvidenceBundle {
        let runtime_state = self.runtime_state();
        let mut summary = BehaviorRuntimeEvidenceSummary {
            report_count: reports.len() as u64,
            applied_action_count: 0,
            diagnostic_count: 0,
            terminal_state_count: 0,
        };
        let evidence_reports = reports
            .into_iter()
            .enumerate()
            .map(|(index, report)| {
                summary.applied_action_count += report.applied_actions.len() as u64;
                summary.diagnostic_count += report.diagnostics.len() as u64;
                summary.terminal_state_count +=
                    u64::from(report.world_state.terminal_state.is_some());
                let mut cooldown_behavior_ids: Vec<String> = report
                    .applied_actions
                    .iter()
                    .map(|action| action.behavior_id.clone())
                    .collect();
                cooldown_behavior_ids.sort();
                cooldown_behavior_ids.dedup();
                BehaviorExecutionEvidence {
                    report_index: index as u64,
                    replay_key: replay_key_for_report(&report),
                    trigger_kind: report.input.trigger_kind,
                    event: report.input.event,
                    input_action: report.input.input_action,
                    applied_action_ids: report
                        .applied_actions
                        .into_iter()
                        .map(|action| action.action_id)
                        .collect(),
                    cooldown_behavior_ids,
                    diagnostics: report.diagnostics,
                    initial_world_state: report.initial_world_state,
                    world_state: report.world_state,
                }
            })
            .collect();

        BehaviorRuntimeEvidenceBundle {
            schema_version: "ouroforge.behavior-runtime-evidence.v1".to_string(),
            artifact_id: self.artifact_id.clone(),
            scene_id: self.scene_id.clone(),
            summary,
            reports: evidence_reports,
            trusted_boundary: runtime_state.trusted_boundary,
        }
    }
}

impl BehaviorScenarioAssertion {
    pub fn validate(&self) -> Result<()> {
        require_local_id("behavior assertion assertionId", &self.assertion_id)?;
        if let Some(behavior_id) = &self.behavior_id {
            require_local_id("behavior assertion behaviorId", behavior_id)?;
        }
        if let Some(action_id) = &self.action_id {
            require_local_id("behavior assertion actionId", action_id)?;
        }
        if let Some(event) = &self.event {
            require_local_id("behavior assertion event", event)?;
        }
        if let Some(flag) = &self.flag {
            require_local_id("behavior assertion flag", flag)?;
        }
        if let Some(state) = &self.state {
            require_local_id("behavior assertion state", state)?;
        }
        if let Some(entity_id) = &self.entity_id {
            require_local_id("behavior assertion entityId", entity_id)?;
        }
        if let Some(item) = &self.item {
            require_local_id("behavior assertion item", item)?;
        }

        match self.kind {
            BehaviorScenarioAssertionKind::BehaviorExecuted
            | BehaviorScenarioAssertionKind::AbilityUsed => require_some(
                "behavior assertion actionId is required for executed/ability assertions",
                self.action_id.as_ref(),
            ),
            BehaviorScenarioAssertionKind::EventEmitted => require_some(
                "behavior assertion event is required for event assertions",
                self.event.as_ref(),
            ),
            BehaviorScenarioAssertionKind::FlagChanged => {
                require_some(
                    "behavior assertion flag is required for flag assertions",
                    self.flag.as_ref(),
                )?;
                require_some(
                    "behavior assertion expectedFlag is required for flag assertions",
                    self.expected_flag.as_ref(),
                )
            }
            BehaviorScenarioAssertionKind::StateTransitionOccurred => {
                require_some(
                    "behavior assertion entityId is required for state assertions",
                    self.entity_id.as_ref(),
                )?;
                require_some(
                    "behavior assertion state is required for state assertions",
                    self.state.as_ref(),
                )
            }
            BehaviorScenarioAssertionKind::CooldownActive => require_some(
                "behavior assertion behaviorId is required for cooldown assertions",
                self.behavior_id.as_ref(),
            ),
            BehaviorScenarioAssertionKind::EntityAffected => require_some(
                "behavior assertion entityId is required for entity assertions",
                self.entity_id.as_ref(),
            ),
            BehaviorScenarioAssertionKind::ItemCollected => require_some(
                "behavior assertion item is required for item assertions",
                self.item.as_ref(),
            ),
            BehaviorScenarioAssertionKind::TerminalStateReached => require_some(
                "behavior assertion terminalState is required for terminal assertions",
                self.terminal_state.as_ref(),
            ),
            BehaviorScenarioAssertionKind::UnsupportedBehaviorBlocked => Ok(()),
        }
    }

    pub fn evaluate(
        &self,
        evidence: &BehaviorRuntimeEvidenceBundle,
    ) -> Result<BehaviorScenarioAssertionVerdict> {
        self.validate()?;
        let reports = matching_reports(self.report_index, evidence);
        let passed = match self.kind {
            BehaviorScenarioAssertionKind::BehaviorExecuted
            | BehaviorScenarioAssertionKind::AbilityUsed => reports.iter().any(|report| {
                self.action_id
                    .as_ref()
                    .is_some_and(|action_id| report.applied_action_ids.contains(action_id))
            }),
            BehaviorScenarioAssertionKind::EventEmitted => reports.iter().any(|report| {
                self.event
                    .as_ref()
                    .is_some_and(|event| report.world_state.events.contains(event))
            }),
            BehaviorScenarioAssertionKind::FlagChanged => reports.iter().any(|report| {
                self.flag.as_ref().is_some_and(|flag| {
                    report.world_state.flags.get(flag) == self.expected_flag.as_ref()
                })
            }),
            BehaviorScenarioAssertionKind::StateTransitionOccurred => {
                reports.iter().any(|report| {
                    self.entity_id.as_ref().is_some_and(|entity_id| {
                        report.world_state.active_states.get(entity_id) == self.state.as_ref()
                    })
                })
            }
            BehaviorScenarioAssertionKind::CooldownActive => reports.iter().any(|report| {
                self.behavior_id
                    .as_ref()
                    .is_some_and(|behavior_id| report.cooldown_behavior_ids.contains(behavior_id))
            }),
            BehaviorScenarioAssertionKind::EntityAffected => reports.iter().any(|report| {
                self.entity_id.as_ref().is_some_and(|entity_id| {
                    report.world_state.entity_positions.contains_key(entity_id)
                        || report.world_state.entity_health.contains_key(entity_id)
                        || report
                            .world_state
                            .animation_intents
                            .iter()
                            .any(|intent| &intent.target_entity_id == entity_id)
                        || report
                            .world_state
                            .audio_intents
                            .iter()
                            .any(|intent| &intent.target_entity_id == entity_id)
                })
            }),
            BehaviorScenarioAssertionKind::ItemCollected => reports.iter().any(|report| {
                self.item
                    .as_ref()
                    .is_some_and(|item| report.world_state.inventory.contains(item))
            }),
            BehaviorScenarioAssertionKind::TerminalStateReached => reports
                .iter()
                .any(|report| report.world_state.terminal_state == self.terminal_state),
            BehaviorScenarioAssertionKind::UnsupportedBehaviorBlocked => {
                reports.iter().any(|report| {
                    report.diagnostics.iter().any(|diagnostic| {
                        matches!(
                            diagnostic.code.as_str(),
                            "unsupportedAction"
                                | "unsupportedEffect"
                                | "unsupportedTrigger"
                                | "unsupportedCondition"
                        )
                    }) && report.applied_action_ids.is_empty()
                })
            }
        };

        Ok(BehaviorScenarioAssertionVerdict {
            assertion_id: self.assertion_id.clone(),
            status: if passed {
                BehaviorScenarioAssertionStatus::Passed
            } else {
                BehaviorScenarioAssertionStatus::Failed
            },
            message: if passed {
                format!("behavior assertion `{}` passed", self.assertion_id)
            } else {
                format!("behavior assertion `{}` failed", self.assertion_id)
            },
        })
    }
}

impl BehaviorScenarioAssertionSuite {
    pub fn validate(&self) -> Result<()> {
        require_local_text(
            "behavior assertion suite schemaVersion",
            &self.schema_version,
        )?;
        if self.schema_version != "ouroforge.behavior-scenario-assertion-suite.v1" {
            return Err(anyhow!(
                "behavior assertion suite schemaVersion is unsupported: {}",
                self.schema_version
            ));
        }
        require_local_id("behavior assertion suite suiteId", &self.suite_id)?;
        if let Some(scenario_id) = &self.scenario_id {
            require_local_id("behavior assertion suite scenarioId", scenario_id)?;
        }
        require_local_text("behavior assertion suite evidenceRef", &self.evidence_ref)?;
        if self.assertions.is_empty() {
            return Err(anyhow!(
                "behavior assertion suite assertions must not be empty"
            ));
        }
        for assertion in &self.assertions {
            assertion.validate()?;
        }
        Ok(())
    }

    pub fn evaluate(
        &self,
        evidence: &BehaviorRuntimeEvidenceBundle,
    ) -> Result<BehaviorScenarioAssertionResultArtifact> {
        self.validate()?;
        let assertions = self
            .assertions
            .iter()
            .map(|assertion| assertion.evaluate(evidence))
            .collect::<Result<Vec<_>>>()?;
        let status = if assertions
            .iter()
            .all(|verdict| verdict.status == BehaviorScenarioAssertionStatus::Passed)
        {
            BehaviorScenarioAssertionStatus::Passed
        } else {
            BehaviorScenarioAssertionStatus::Failed
        };
        Ok(BehaviorScenarioAssertionResultArtifact {
            schema_version: "ouroforge.behavior-scenario-assertion-result.v1".to_string(),
            suite_id: self.suite_id.clone(),
            scenario_id: self.scenario_id.clone(),
            evidence_ref: self.evidence_ref.clone(),
            status,
            assertions,
            trusted_boundary: evidence.trusted_boundary.clone(),
        })
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
        if let Some(flag) = &self.flag {
            require_local_id("action.flag", flag)?;
        }
        if let Some(state) = &self.state {
            require_local_id("action.state", state)?;
        }
        if let Some(event) = &self.event {
            require_local_id("action.event", event)?;
        }
        if let Some(item) = &self.item {
            require_local_id("action.item", item)?;
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

impl BehaviorExecutionInput {
    pub fn new(trigger_kind: impl Into<String>) -> Self {
        Self {
            trigger_kind: trigger_kind.into(),
            event: None,
            input_action: None,
        }
    }

    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }

    pub fn with_input_action(mut self, input_action: impl Into<String>) -> Self {
        self.input_action = Some(input_action.into());
        self
    }
}

impl BehaviorWorldState {
    pub fn with_flag(mut self, flag: impl Into<String>, value: bool) -> Self {
        self.flags.insert(flag.into(), value);
        self
    }

    pub fn with_position(mut self, entity_id: impl Into<String>, x: i32, y: i32) -> Self {
        self.entity_positions
            .insert(entity_id.into(), BehaviorEntityPosition { x, y });
        self
    }

    pub fn with_health(mut self, entity_id: impl Into<String>, health: i32) -> Self {
        self.entity_health.insert(entity_id.into(), health);
        self
    }

    pub fn with_item(mut self, item: impl Into<String>) -> Self {
        self.inventory.insert(item.into());
        self
    }
}

fn behavior_matches_input(behavior: &BehaviorDefinition, input: &BehaviorExecutionInput) -> bool {
    behavior.triggers.iter().any(|trigger| {
        if trigger.kind != input.trigger_kind {
            return false;
        }
        match trigger.kind.as_str() {
            "onEvent" | "onCollision" | "onStateEnter" => {
                trigger.event.as_ref() == input.event.as_ref()
            }
            "onInputAction" => trigger.event.as_ref() == input.input_action.as_ref(),
            _ => true,
        }
    })
}

fn conditions_match(
    behavior: &BehaviorDefinition,
    input: &BehaviorExecutionInput,
    world_state: &BehaviorWorldState,
) -> bool {
    behavior
        .conditions
        .iter()
        .all(|condition| condition_matches(condition, behavior, input, world_state))
}

fn condition_matches(
    condition: &BehaviorCondition,
    behavior: &BehaviorDefinition,
    input: &BehaviorExecutionInput,
    world_state: &BehaviorWorldState,
) -> bool {
    match condition.kind.as_str() {
        "always" => true,
        "flagEquals" => condition
            .field
            .as_ref()
            .and_then(|field| world_state.flags.get(field))
            .is_some_and(|actual| {
                condition
                    .value
                    .as_deref()
                    .is_some_and(|value| bool_string(*actual) == value)
            }),
        "stateEquals" => world_state
            .active_states
            .get(&behavior.entity_id)
            .is_some_and(|actual| condition.value.as_deref() == Some(actual.as_str())),
        "eventEquals" => condition.value.as_ref() == input.event.as_ref(),
        "hasItem" => condition
            .value
            .as_ref()
            .is_some_and(|item| world_state.inventory.contains(item)),
        "cooldownReady" => true,
        _ => false,
    }
}

fn initialize_behavior_state(behavior: &BehaviorDefinition, world_state: &mut BehaviorWorldState) {
    if let Some(state_machine) = &behavior.state_machine {
        world_state
            .active_states
            .entry(behavior.entity_id.clone())
            .or_insert_with(|| state_machine.initial_state.clone());
    }
}

fn transition_state_machine(
    behavior: &BehaviorDefinition,
    input: &BehaviorExecutionInput,
    world_state: &mut BehaviorWorldState,
    applied_actions: &mut Vec<BehaviorAppliedAction>,
) {
    let Some(state_machine) = &behavior.state_machine else {
        return;
    };
    let Some(event) = input.event.as_ref() else {
        return;
    };
    let Some(current_state_id) = world_state.active_states.get(&behavior.entity_id).cloned() else {
        return;
    };
    let Some(current_state) = state_machine
        .states
        .iter()
        .find(|state| state.id == current_state_id)
    else {
        return;
    };
    let Some(transition) = current_state
        .transitions
        .iter()
        .find(|transition| transition.on_event.as_deref() == Some(event.as_str()))
    else {
        return;
    };

    world_state
        .active_states
        .insert(behavior.entity_id.clone(), transition.to.clone());
    applied_actions.push(BehaviorAppliedAction {
        behavior_id: behavior.id.clone(),
        action_id: transition.id.clone(),
        kind: "stateTransition".to_string(),
        target_entity_id: behavior.entity_id.clone(),
    });
}

fn apply_actions(
    behavior_id: &str,
    default_entity_id: &str,
    actions: &[BehaviorAction],
    world_state: &mut BehaviorWorldState,
    applied_actions: &mut Vec<BehaviorAppliedAction>,
    diagnostics: &mut Vec<BehaviorRuntimeDiagnostic>,
    queued_events: &mut VecDeque<String>,
) {
    for action in actions {
        if !SUPPORTED_ACTIONS.contains(&action.kind.as_str())
            || !SUPPORTED_EFFECTS.contains(&action.effect_kind.as_str())
        {
            continue;
        }
        let target_entity_id = action
            .target_entity_id
            .as_deref()
            .unwrap_or(default_entity_id)
            .to_string();
        let applied = apply_action(
            action,
            &target_entity_id,
            behavior_id,
            world_state,
            queued_events,
        );
        if applied {
            applied_actions.push(BehaviorAppliedAction {
                behavior_id: behavior_id.to_string(),
                action_id: action.id.clone(),
                kind: action.kind.clone(),
                target_entity_id,
            });
        } else {
            diagnostics.push(BehaviorRuntimeDiagnostic {
                severity: BehaviorDiagnosticSeverity::Warning,
                code: "actionMissingRequiredField".to_string(),
                message: format!(
                    "behavior action `{}` missing required structured field",
                    action.id
                ),
                behavior_id: Some(behavior_id.to_string()),
                item_id: Some(action.id.clone()),
            });
        }
    }
}

fn apply_action(
    action: &BehaviorAction,
    target_entity_id: &str,
    behavior_id: &str,
    world_state: &mut BehaviorWorldState,
    queued_events: &mut VecDeque<String>,
) -> bool {
    match action.kind.as_str() {
        "setFlag" => {
            let Some(flag) = action.flag.as_ref().or(action.value.as_ref()) else {
                return false;
            };
            world_state.flags.insert(flag.clone(), true);
            true
        }
        "moveEntity" => {
            let (dx, dy) = action_delta(action);
            let position = world_state
                .entity_positions
                .entry(target_entity_id.to_string())
                .or_default();
            position.x += dx;
            position.y += dy;
            true
        }
        "changeState" => {
            let Some(state) = action.state.as_ref().or(action.value.as_ref()) else {
                return false;
            };
            world_state
                .active_states
                .insert(target_entity_id.to_string(), state.clone());
            true
        }
        "emitEvent" => {
            let Some(event) = action.event.as_ref().or(action.value.as_ref()) else {
                return false;
            };
            queued_events.push_back(event.clone());
            true
        }
        "damage" => {
            let amount = action.amount.unwrap_or(1).max(0);
            *world_state
                .entity_health
                .entry(target_entity_id.to_string())
                .or_insert(0) -= amount;
            true
        }
        "heal" => {
            let amount = action.amount.unwrap_or(1).max(0);
            *world_state
                .entity_health
                .entry(target_entity_id.to_string())
                .or_insert(0) += amount;
            true
        }
        "collectItem" => {
            let Some(item) = action.item.as_ref().or(action.value.as_ref()) else {
                return false;
            };
            world_state.inventory.insert(item.clone());
            true
        }
        "removeItem" => {
            let Some(item) = action.item.as_ref().or(action.value.as_ref()) else {
                return false;
            };
            world_state.inventory.remove(item);
            true
        }
        "startAnimationIntent" => {
            let Some(intent) = action.value.as_ref() else {
                return false;
            };
            world_state.animation_intents.push(BehaviorIntent {
                behavior_id: behavior_id.to_string(),
                action_id: action.id.clone(),
                target_entity_id: target_entity_id.to_string(),
                intent: intent.clone(),
            });
            true
        }
        "startAudioIntent" => {
            let Some(intent) = action.value.as_ref() else {
                return false;
            };
            world_state.audio_intents.push(BehaviorIntent {
                behavior_id: behavior_id.to_string(),
                action_id: action.id.clone(),
                target_entity_id: target_entity_id.to_string(),
                intent: intent.clone(),
            });
            true
        }
        "markWinState" => {
            world_state.terminal_state = Some(BehaviorTerminalState::Win);
            true
        }
        "markLossState" => {
            world_state.terminal_state = Some(BehaviorTerminalState::Loss);
            true
        }
        _ => false,
    }
}

fn action_delta(action: &BehaviorAction) -> (i32, i32) {
    if action.dx.is_some() || action.dy.is_some() {
        return (action.dx.unwrap_or(0), action.dy.unwrap_or(0));
    }
    match action.value.as_deref() {
        Some("up-1") => (0, -1),
        Some("down-1") => (0, 1),
        Some("left-1") => (-1, 0),
        Some("right-1") => (1, 0),
        _ => (0, 0),
    }
}

fn bool_string(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn replay_key_for_report(report: &BehaviorExecutionReport) -> String {
    let canonical = serde_json::to_vec(report).unwrap_or_else(|_| Vec::new());
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in canonical {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("behavior-replay-{hash:016x}")
}

fn matching_reports(
    report_index: Option<u64>,
    evidence: &BehaviorRuntimeEvidenceBundle,
) -> Vec<&BehaviorExecutionEvidence> {
    evidence
        .reports
        .iter()
        .filter(|report| report_index.is_none_or(|index| report.report_index == index))
        .collect()
}

fn require_some<T>(message: &str, value: Option<&T>) -> Result<()> {
    if value.is_some() {
        Ok(())
    } else {
        Err(anyhow!(message.to_string()))
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

fn require_blocked_reason(field: &str, reasons: &[String], needle: &str) -> Result<()> {
    if reasons.is_empty() {
        return Err(anyhow!("{field} requires blockedReasons"));
    }
    if !blocked_reasons_contain(reasons, needle) {
        return Err(anyhow!("{field} blockedReasons must mention {needle}"));
    }
    Ok(())
}

fn blocked_reasons_contain(reasons: &[String], needle: &str) -> bool {
    reasons
        .iter()
        .any(|reason| reason.to_ascii_lowercase().contains(needle))
}

fn require_hash_text(field: &str, value: &str) -> Result<()> {
    require_local_text(field, value)?;
    let Some((algorithm, digest)) = value.split_once(':') else {
        return Err(anyhow!("{field} must include algorithm:digest"));
    };
    require_local_id(&format!("{field}.algorithm"), algorithm)?;
    if digest.trim().is_empty()
        || !digest
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() || ch == '-' || ch == '_')
    {
        return Err(anyhow!("{field}.digest must be a bounded local digest"));
    }
    Ok(())
}

fn require_behavior_transaction_output_ref(
    field: &str,
    value: &str,
    target_scene_path: &str,
) -> Result<()> {
    require_relative_json_path(field, value, ".json")?;
    if !value.starts_with("runs/") && !value.starts_with(".omx/") {
        return Err(anyhow!("{field} must be under runs/ or .omx/"));
    }
    if value == target_scene_path {
        return Err(anyhow!("{field} must not collide with target scenePath"));
    }
    if value.ends_with(".scene.json") {
        return Err(anyhow!(
            "{field} must be generated transaction evidence, not a scene source path"
        ));
    }
    if value.contains("/seeds/")
        || value.contains("/examples/")
        || value.ends_with("Cargo.toml")
        || value.ends_with("Cargo.lock")
    {
        return Err(anyhow!(
            "{field} must not target source fixtures, dependency manifests, or trusted project files"
        ));
    }
    if value.contains("/../") || value.starts_with("../") {
        return Err(anyhow!(
            "{field} must stay inside generated transaction output roots"
        ));
    }
    Ok(())
}

fn require_relative_json_path(field: &str, value: &str, suffix: &str) -> Result<()> {
    require_local_text(field, value)?;
    let path = Path::new(value);
    if path.is_absolute() || value.contains('\\') || value.contains("://") {
        return Err(anyhow!("{field} must be a relative local path"));
    }
    for component in path.components() {
        match component {
            std::path::Component::Normal(_) | std::path::Component::CurDir => {}
            _ => return Err(anyhow!("{field} must stay inside the local project tree")),
        }
    }
    if !value.ends_with(suffix) {
        return Err(anyhow!("{field} must end with {suffix}"));
    }
    Ok(())
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
