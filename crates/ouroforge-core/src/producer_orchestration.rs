//! Whole-Game Orchestration State v1 (#1684).
//!
//! Extends the Milestone 23 campaign-state pattern and the Milestone 42
//! production pipeline to a whole-game horizon by recording resumable Rust/local
//! state over an existing producer plan. This is inert orchestration evidence:
//! it dispatches proposal-only role/function work items and never performs a
//! trusted write, hidden worker run, browser command bridge, auto-apply,
//! auto-merge, self-approval, reviewer bypass, release, or quality/fun claim.

use crate::producer_plan::{ProducerPlanArtifact, ProducerPlanTask, PRODUCER_PLAN_SCHEMA_VERSION};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const PRODUCER_ORCHESTRATION_SCHEMA_VERSION: &str = "producer-orchestration-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProducerDispatchStatus {
    Dispatched,
    Completed,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProducerDispatchRecord {
    #[serde(rename = "dispatchId")]
    pub dispatch_id: String,
    #[serde(rename = "taskId")]
    pub task_id: String,
    #[serde(rename = "functionAgent")]
    pub function_agent: String,
    pub role: String,
    pub status: ProducerDispatchStatus,
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    #[serde(rename = "expectedVerification")]
    pub expected_verification: Vec<String>,
    #[serde(
        rename = "completionEvidenceRef",
        skip_serializing_if = "Option::is_none"
    )]
    pub completion_evidence_ref: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProducerOrchestrationState {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "orchestrationId")]
    pub orchestration_id: String,
    #[serde(rename = "planRef")]
    pub plan_ref: String,
    pub plan: ProducerPlanArtifact,
    #[serde(rename = "completedTaskIds")]
    pub completed_task_ids: Vec<String>,
    #[serde(rename = "currentDispatch")]
    pub current_dispatch: Option<ProducerDispatchRecord>,
    pub dispatches: Vec<ProducerDispatchRecord>,
    #[serde(rename = "resumeToken")]
    pub resume_token: String,
    #[serde(rename = "generatedStatePolicy")]
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProducerOrchestrationReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "orchestrationId")]
    pub orchestration_id: String,
    pub status: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "taskCount")]
    pub task_count: usize,
    #[serde(rename = "completedTaskCount")]
    pub completed_task_count: usize,
    #[serde(rename = "currentTaskId")]
    pub current_task_id: Option<String>,
    #[serde(rename = "currentRole")]
    pub current_role: Option<String>,
    #[serde(rename = "dispatchCount")]
    pub dispatch_count: usize,
    #[serde(rename = "resumeToken")]
    pub resume_token: String,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

pub fn start_producer_orchestration(
    orchestration_id: &str,
    plan_ref: &str,
    plan: ProducerPlanArtifact,
) -> Result<ProducerOrchestrationState> {
    require_local_id("producer orchestration orchestrationId", orchestration_id)?;
    require_local_ref("producer orchestration planRef", plan_ref)?;
    plan.validate()?;
    let first = plan
        .tasks
        .first()
        .ok_or_else(|| anyhow!("producer orchestration plan must include at least one task"))?;
    let dispatch = dispatch_for_task(
        orchestration_id,
        1,
        first,
        ProducerDispatchStatus::Dispatched,
        None,
    )?;
    let state = ProducerOrchestrationState {
        schema_version: PRODUCER_ORCHESTRATION_SCHEMA_VERSION.to_string(),
        orchestration_id: orchestration_id.to_string(),
        plan_ref: plan_ref.to_string(),
        plan,
        completed_task_ids: Vec::new(),
        current_dispatch: Some(dispatch.clone()),
        dispatches: vec![dispatch],
        resume_token: resume_token(orchestration_id, 0),
        generated_state_policy: "Generated orchestration state, runs, assets, content, coverage, and local artifacts remain untracked unless explicitly fixture-scoped".to_string(),
        boundary: "Extends Milestone 23 campaign state and Milestone 42 production pipeline to whole-game horizon; Rust/local resumable orchestration evidence; proposal-only role dispatch; browser/Studio read-only; no new orchestrator engine; no hidden worker runtime; no hosted orchestrator; no direct trusted writes; no auto-apply; no auto-merge; no self-approval; no reviewer bypass; no production-ready claim; no engine replacement claim".to_string(),
    };
    state.validate()?;
    Ok(state)
}

pub fn resume_producer_orchestration(input: &str) -> Result<ProducerOrchestrationState> {
    let state: ProducerOrchestrationState =
        serde_json::from_str(input).context("failed to parse Producer Orchestration State JSON")?;
    state.validate()?;
    Ok(state)
}

pub fn complete_current_dispatch(
    state: &ProducerOrchestrationState,
    completion_evidence_ref: &str,
) -> Result<ProducerOrchestrationState> {
    state.validate()?;
    require_local_ref(
        "producer orchestration completionEvidenceRef",
        completion_evidence_ref,
    )?;
    let current = state
        .current_dispatch
        .as_ref()
        .ok_or_else(|| anyhow!("producer orchestration has no current dispatch to complete"))?;
    let mut next = state.clone();
    let completed_index = next.completed_task_ids.len();
    let expected =
        next.plan.tasks.get(completed_index).ok_or_else(|| {
            anyhow!("producer orchestration current dispatch is beyond plan tasks")
        })?;
    if current.task_id != expected.task_id {
        return Err(anyhow!(
            "producer orchestration current dispatch must match next incomplete plan task"
        ));
    }
    let mut completed = current.clone();
    completed.status = ProducerDispatchStatus::Completed;
    completed.completion_evidence_ref = Some(completion_evidence_ref.to_string());
    *next
        .dispatches
        .last_mut()
        .ok_or_else(|| anyhow!("producer orchestration dispatch history must not be empty"))? =
        completed;
    next.completed_task_ids.push(current.task_id.clone());
    if let Some(task) = next.plan.tasks.get(next.completed_task_ids.len()) {
        let dispatch = dispatch_for_task(
            &next.orchestration_id,
            next.dispatches.len() + 1,
            task,
            ProducerDispatchStatus::Dispatched,
            None,
        )?;
        next.current_dispatch = Some(dispatch.clone());
        next.dispatches.push(dispatch);
    } else {
        next.current_dispatch = None;
    }
    next.resume_token = resume_token(&next.orchestration_id, next.completed_task_ids.len());
    next.validate()?;
    Ok(next)
}

impl ProducerOrchestrationState {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCER_ORCHESTRATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "producer orchestration schemaVersion must be {PRODUCER_ORCHESTRATION_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "producer orchestration orchestrationId",
            &self.orchestration_id,
        )?;
        require_local_ref("producer orchestration planRef", &self.plan_ref)?;
        if self.plan.schema_version != PRODUCER_PLAN_SCHEMA_VERSION {
            return Err(anyhow!("producer orchestration plan schemaVersion drift"));
        }
        self.plan.validate()?;
        validate_completed_prefix(&self.plan.tasks, &self.completed_task_ids)?;
        validate_dispatch_history(self)?;
        require_text("producer orchestration resumeToken", &self.resume_token)?;
        if self.resume_token != resume_token(&self.orchestration_id, self.completed_task_ids.len())
        {
            return Err(anyhow!(
                "producer orchestration resumeToken must match resumable long-horizon state"
            ));
        }
        require_text(
            "producer orchestration generatedStatePolicy",
            &self.generated_state_policy,
        )?;
        let policy = self.generated_state_policy.to_ascii_lowercase();
        for required in ["untracked", "fixture-scoped"] {
            if !policy.contains(required) {
                return Err(anyhow!(
                    "producer orchestration generatedStatePolicy must mention `{required}`"
                ));
            }
        }
        require_boundary(
            "producer orchestration boundary",
            &self.boundary,
            &[
                "extends milestone 23 campaign state",
                "milestone 42 production pipeline",
                "whole-game horizon",
                "rust/local resumable orchestration evidence",
                "proposal-only role dispatch",
                "browser/studio read-only",
                "no new orchestrator engine",
                "no direct trusted writes",
                "no auto-apply",
                "no auto-merge",
                "no self-approval",
                "no reviewer bypass",
                "no production-ready",
                "no engine replacement",
            ],
        )?;
        Ok(())
    }

    pub fn read_model(&self) -> ProducerOrchestrationReadModel {
        let status = if self.completed_task_ids.len() == self.plan.tasks.len() {
            "complete"
        } else {
            "in-progress"
        };
        ProducerOrchestrationReadModel {
            schema_version: self.schema_version.clone(),
            orchestration_id: self.orchestration_id.clone(),
            status: status.to_string(),
            plan_id: self.plan.plan_id.clone(),
            task_count: self.plan.tasks.len(),
            completed_task_count: self.completed_task_ids.len(),
            current_task_id: self.current_dispatch.as_ref().map(|d| d.task_id.clone()),
            current_role: self.current_dispatch.as_ref().map(|d| d.role.clone()),
            dispatch_count: self.dispatches.len(),
            resume_token: self.resume_token.clone(),
            validation_summary: vec![
                "producer drives role agents through the plan in deterministic order".to_string(),
                "dispatches are proposal-only records and never trusted writes".to_string(),
                "completed prefix and resume token make long-horizon state resumable".to_string(),
            ],
            compatibility_notes: vec![
                "Extends Milestone 23 campaign state and Milestone 42 production pipeline; no new orchestrator engine".to_string(),
                "browser/Studio surfaces remain read-only display consumers".to_string(),
                "generated orchestration state remains untracked unless fixture-scoped".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }
}

fn validate_completed_prefix(tasks: &[ProducerPlanTask], completed: &[String]) -> Result<()> {
    if completed.len() > tasks.len() {
        return Err(anyhow!(
            "producer orchestration completedTaskIds cannot exceed plan task count"
        ));
    }
    let mut seen = BTreeSet::new();
    for (index, task_id) in completed.iter().enumerate() {
        require_local_id("producer orchestration completedTaskIds", task_id)?;
        if !seen.insert(task_id) {
            return Err(anyhow!(
                "producer orchestration completedTaskIds must be unique"
            ));
        }
        if tasks[index].task_id != *task_id {
            return Err(anyhow!(
                "producer orchestration completedTaskIds must be a prefix of the plan task order"
            ));
        }
    }
    Ok(())
}

fn validate_dispatch_history(state: &ProducerOrchestrationState) -> Result<()> {
    if state.dispatches.is_empty() {
        return Err(anyhow!(
            "producer orchestration dispatches must contain history"
        ));
    }
    if state.dispatches.len() > state.plan.tasks.len() {
        return Err(anyhow!(
            "producer orchestration dispatch history cannot exceed plan task count"
        ));
    }
    for (index, dispatch) in state.dispatches.iter().enumerate() {
        dispatch.validate()?;
        let expected =
            state.plan.tasks.get(index).ok_or_else(|| {
                anyhow!("producer orchestration dispatch has no matching plan task")
            })?;
        validate_dispatch_matches_task(dispatch, expected)?;
        let should_be_completed = index < state.completed_task_ids.len();
        match (&dispatch.status, should_be_completed) {
            (ProducerDispatchStatus::Completed, true) => {
                if dispatch.completion_evidence_ref.is_none() {
                    return Err(anyhow!(
                        "completed producer dispatch requires completionEvidenceRef"
                    ));
                }
            }
            (ProducerDispatchStatus::Dispatched, false)
                if index == state.completed_task_ids.len() => {}
            _ => {
                return Err(anyhow!(
                    "producer orchestration dispatch statuses must match completed prefix and current dispatch"
                ));
            }
        }
    }
    match &state.current_dispatch {
        Some(current) => {
            if state.completed_task_ids.len() >= state.plan.tasks.len() {
                return Err(anyhow!(
                    "complete producer orchestration must not have a current dispatch"
                ));
            }
            let last = state.dispatches.last().ok_or_else(|| {
                anyhow!("producer orchestration dispatch history must not be empty")
            })?;
            if current != last || current.status != ProducerDispatchStatus::Dispatched {
                return Err(anyhow!(
                    "producer orchestration current dispatch must equal the latest dispatched history item"
                ));
            }
        }
        None => {
            if state.completed_task_ids.len() != state.plan.tasks.len() {
                return Err(anyhow!(
                    "incomplete producer orchestration requires a current dispatch"
                ));
            }
        }
    }
    Ok(())
}

impl ProducerDispatchRecord {
    fn validate(&self) -> Result<()> {
        require_local_id("producer dispatch dispatchId", &self.dispatch_id)?;
        require_local_id("producer dispatch taskId", &self.task_id)?;
        require_text("producer dispatch functionAgent", &self.function_agent)?;
        require_text("producer dispatch role", &self.role)?;
        if !self.proposal_only {
            return Err(anyhow!(
                "producer dispatch proposalOnly must be true; producer dispatch cannot perform trusted writes"
            ));
        }
        validate_text_list("producer dispatch inputs", &self.inputs, true)?;
        validate_text_list("producer dispatch outputs", &self.outputs, true)?;
        validate_text_list(
            "producer dispatch expectedVerification",
            &self.expected_verification,
            true,
        )?;
        if let Some(evidence) = &self.completion_evidence_ref {
            require_local_ref("producer dispatch completionEvidenceRef", evidence)?;
        }
        Ok(())
    }
}

fn dispatch_for_task(
    orchestration_id: &str,
    sequence: usize,
    task: &ProducerPlanTask,
    status: ProducerDispatchStatus,
    completion_evidence_ref: Option<String>,
) -> Result<ProducerDispatchRecord> {
    let dispatch = ProducerDispatchRecord {
        dispatch_id: format!("{orchestration_id}-dispatch-{sequence:02}"),
        task_id: task.task_id.clone(),
        function_agent: task.function_agent.clone(),
        role: task.role.clone(),
        status,
        proposal_only: task.proposal_only,
        inputs: task.inputs.clone(),
        outputs: task.outputs.clone(),
        expected_verification: task.expected_verification.clone(),
        completion_evidence_ref,
    };
    dispatch.validate()?;
    Ok(dispatch)
}

fn validate_dispatch_matches_task(
    dispatch: &ProducerDispatchRecord,
    task: &ProducerPlanTask,
) -> Result<()> {
    if dispatch.task_id != task.task_id
        || dispatch.function_agent != task.function_agent
        || dispatch.role != task.role
        || dispatch.inputs != task.inputs
        || dispatch.outputs != task.outputs
        || dispatch.expected_verification != task.expected_verification
        || dispatch.proposal_only != task.proposal_only
    {
        return Err(anyhow!(
            "producer orchestration dispatch must match its current plan task"
        ));
    }
    Ok(())
}

fn resume_token(orchestration_id: &str, completed_count: usize) -> String {
    format!("{orchestration_id}:completed-{completed_count}")
}

fn validate_text_list(field: &str, values: &[String], require_items: bool) -> Result<()> {
    if require_items && values.is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    for value in values {
        require_text(field, value)?;
    }
    Ok(())
}

fn require_boundary(field: &str, value: &str, required: &[&str]) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for phrase in required {
        if !lower.contains(phrase) {
            return Err(anyhow!("{field} must state `{phrase}`"));
        }
    }
    Ok(())
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('.')
        || value.starts_with('/')
        || value.contains("..")
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
    {
        return Err(anyhow!(
            "{field} must be a local stable id using alphanumeric, dash, underscore, or dot characters"
        ));
    }
    Ok(())
}

fn require_local_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with('/')
        || value.contains("..")
    {
        return Err(anyhow!(
            "{field} must be a local fixture/reference path or stable ref"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn reject_forbidden_wording(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "auto-apply enabled",
        "auto-merge enabled",
        "self-approval enabled",
        "reviewer bypass enabled",
        "direct trusted write enabled",
        "browser trusted write enabled",
        "production-ready engine",
        "godot replacement enabled",
        "godot parity enabled",
        "finished game shipping",
        "quality/fun guaranteed",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden producer-orchestration wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}
