use crate::behavior_draft_validation::{
    reject_forbidden_runtime_text, require_text, validate_path_component,
    validate_relative_artifact_path, validate_repo_relative_ref, validate_snapshot_hash,
    validate_status,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const BEHAVIOR_DRAFT_SCHEMA_VERSION: &str = "agent-generated-behavior-draft-v1";
const MAX_BEHAVIOR_DRAFT_OPERATIONS: usize = 64;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    pub target: BehaviorDraftTarget,
    #[serde(rename = "targetHashes")]
    pub target_hashes: Vec<BehaviorDraftTargetHash>,
    #[serde(rename = "proposedBehaviors")]
    pub proposed_behaviors: Vec<BehaviorDraftOperation>,
    pub rationale: String,
    #[serde(rename = "linkedEvidence")]
    pub linked_evidence: Vec<String>,
    pub author: BehaviorDraftAuthor,
    #[serde(rename = "validationStatus")]
    pub validation_status: BehaviorDraftValidationStatus,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftTarget {
    #[serde(rename = "projectManifestRef")]
    pub project_manifest_ref: String,
    #[serde(rename = "sceneRef")]
    pub scene_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftTargetHash {
    #[serde(rename = "targetRef")]
    pub target_ref: String,
    #[serde(rename = "expectedHash")]
    pub expected_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftOperation {
    #[serde(rename = "operationId")]
    pub operation_id: String,
    pub kind: BehaviorDraftOperationKind,
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    pub summary: String,
    #[serde(rename = "expectedScenarioImpact")]
    pub expected_scenario_impact: String,
    pub status: BehaviorDraftOperationStatus,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorDraftOperationKind {
    FlagChange,
    EventEmit,
    StateTransition,
    AbilityUse,
    CooldownRule,
    EntityEffect,
    ItemCollection,
    TerminalState,
    Unsupported,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorDraftOperationStatus {
    Proposed,
    MissingEvidence,
    Unsupported,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorDraftAuthor {
    #[serde(rename = "authorId")]
    pub author_id: String,
    pub source: BehaviorDraftAuthorSource,
    #[serde(rename = "generatedBy")]
    pub generated_by: String,
    pub untrusted: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorDraftAuthorSource {
    Agent,
    Tool,
    Human,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BehaviorDraftValidationStatus {
    Drafted,
    MissingEvidence,
    Unsupported,
    StaleTarget,
    Blocked,
}

impl BehaviorDraftArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: BehaviorDraftArtifact =
            serde_json::from_str(input).context("failed to parse Behavior Draft JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "behavior draft schemaVersion must be {BEHAVIOR_DRAFT_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("behavior draft draftId", &self.draft_id)?;
        self.target.validate()?;
        validate_target_hashes(&self.target, &self.target_hashes)?;
        validate_operations(&self.proposed_behaviors)?;
        require_text("behavior draft rationale", &self.rationale)?;
        validate_linked_evidence(&self.draft_id, &self.linked_evidence)?;
        self.author.validate()?;
        for reason in &self.blocked_reasons {
            require_text("behavior draft blockedReasons", reason)?;
        }
        for guardrail in &self.guardrails {
            require_text("behavior draft guardrails", guardrail)?;
        }
        validate_status(
            self.validation_status,
            &self.proposed_behaviors,
            &self.blocked_reasons,
        )
    }
}

impl BehaviorDraftTarget {
    fn validate(&self) -> Result<()> {
        validate_repo_relative_ref(
            "behavior draft target.projectManifestRef",
            &self.project_manifest_ref,
        )?;
        validate_repo_relative_ref("behavior draft target.sceneRef", &self.scene_ref)?;
        if !self.project_manifest_ref.ends_with(".json") {
            return Err(anyhow!(
                "behavior draft target.projectManifestRef must be a JSON file"
            ));
        }
        if !self.scene_ref.ends_with(".scene.json") {
            return Err(anyhow!(
                "behavior draft target.sceneRef must point to a .scene.json fixture"
            ));
        }
        Ok(())
    }
}

impl BehaviorDraftAuthor {
    fn validate(&self) -> Result<()> {
        validate_path_component("behavior draft author.authorId", &self.author_id)?;
        require_text("behavior draft author.generatedBy", &self.generated_by)?;
        if !self.untrusted {
            return Err(anyhow!(
                "behavior draft author.untrusted must be true until review-gated apply"
            ));
        }
        Ok(())
    }
}

fn validate_target_hashes(
    target: &BehaviorDraftTarget,
    hashes: &[BehaviorDraftTargetHash],
) -> Result<()> {
    if hashes.is_empty() {
        return Err(anyhow!("behavior draft targetHashes must not be empty"));
    }
    let mut refs = BTreeSet::new();
    for hash in hashes {
        validate_repo_relative_ref("behavior draft targetHashes.targetRef", &hash.target_ref)?;
        validate_snapshot_hash(
            "behavior draft targetHashes.expectedHash",
            &hash.expected_hash,
        )?;
        if !refs.insert(hash.target_ref.as_str()) {
            return Err(anyhow!(
                "duplicate behavior draft targetHashes.targetRef: {}",
                hash.target_ref
            ));
        }
    }
    if !refs.contains(target.scene_ref.as_str())
        && !refs.contains(target.project_manifest_ref.as_str())
    {
        return Err(anyhow!(
            "behavior draft targetHashes must include the scene or project target"
        ));
    }
    Ok(())
}

fn validate_operations(operations: &[BehaviorDraftOperation]) -> Result<()> {
    if operations.is_empty() || operations.len() > MAX_BEHAVIOR_DRAFT_OPERATIONS {
        return Err(anyhow!(
            "behavior draft proposedBehaviors must contain between 1 and {MAX_BEHAVIOR_DRAFT_OPERATIONS} entries"
        ));
    }
    let mut ids = BTreeSet::new();
    let mut behavior_ids = BTreeSet::new();
    for operation in operations {
        validate_path_component(
            "behavior draft proposedBehaviors.operationId",
            &operation.operation_id,
        )?;
        validate_path_component(
            "behavior draft proposedBehaviors.behaviorId",
            &operation.behavior_id,
        )?;
        require_text(
            "behavior draft proposedBehaviors.summary",
            &operation.summary,
        )?;
        require_text(
            "behavior draft proposedBehaviors.expectedScenarioImpact",
            &operation.expected_scenario_impact,
        )?;
        if operation.kind != BehaviorDraftOperationKind::Unsupported
            && operation.status == BehaviorDraftOperationStatus::Proposed
        {
            reject_forbidden_runtime_text(
                "behavior draft proposedBehaviors.behaviorId",
                &operation.behavior_id,
            )?;
            reject_forbidden_runtime_text(
                "behavior draft proposedBehaviors.summary",
                &operation.summary,
            )?;
            reject_forbidden_runtime_text(
                "behavior draft proposedBehaviors.expectedScenarioImpact",
                &operation.expected_scenario_impact,
            )?;
        }
        if !ids.insert(operation.operation_id.as_str()) {
            return Err(anyhow!(
                "duplicate behavior draft proposedBehaviors.operationId: {}",
                operation.operation_id
            ));
        }
        if !behavior_ids.insert(operation.behavior_id.as_str()) {
            return Err(anyhow!(
                "duplicate behavior draft proposedBehaviors.behaviorId: {}",
                operation.behavior_id
            ));
        }
    }
    Ok(())
}

fn validate_linked_evidence(draft_id: &str, refs: &[String]) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("behavior draft linkedEvidence must not be empty"));
    }
    let expected_prefix = format!("evidence/behavior-drafts/{draft_id}/");
    let mut paths = BTreeSet::new();
    for evidence_ref in refs {
        validate_relative_artifact_path("behavior draft linkedEvidence", evidence_ref)?;
        if !evidence_ref.starts_with(&expected_prefix) || !evidence_ref.ends_with(".json") {
            return Err(anyhow!(
                "behavior draft linkedEvidence must be JSON evidence under evidence/behavior-drafts/{draft_id}/"
            ));
        }
        if !paths.insert(evidence_ref.as_str()) {
            return Err(anyhow!(
                "duplicate behavior draft linkedEvidence: {evidence_ref}"
            ));
        }
    }
    Ok(())
}
