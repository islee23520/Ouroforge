//! Design-Intent Decomposition and Production Plan v1 (#1683).
//!
//! Reuses Milestone 30 and GDD/design-brief surfaces to decompose a validated
//! human design intent into an inert, deterministic task graph of function-agent
//! work. This is proposal-only Rust/local validation; it adds no generator,
//! writer, runtime, command bridge, trusted write, auto-apply, auto-merge,
//! self-approval, reviewer bypass, or shipping authority.

use crate::gdd_design_brief::{GddDesignBriefArtifact, GddDesignBriefStatus};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const PRODUCER_PLAN_SCHEMA_VERSION: &str = "producer-plan-v1";

const SUPPORTED_FUNCTIONS: &[&str] = &[
    "design-brief",
    "requirements",
    "mechanics",
    "scaffold",
    "scene-level",
    "behavior",
    "assets",
    "scenarios",
    "qa",
    "review",
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProducerDesignIntent {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
    #[serde(rename = "designBrief")]
    pub design_brief: GddDesignBriefArtifact,
    #[serde(rename = "requestedFunctions")]
    pub requested_functions: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProducerPlanArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "gddRef")]
    pub gdd_ref: String,
    pub status: String,
    pub tasks: Vec<ProducerPlanTask>,
    #[serde(rename = "generatedStatePolicy")]
    pub generated_state_policy: String,
    #[serde(rename = "reviewPath")]
    pub review_path: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct ProducerPlanTask {
    #[serde(rename = "taskId")]
    pub task_id: String,
    #[serde(rename = "functionAgent")]
    pub function_agent: String,
    pub role: String,
    pub kind: String,
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
    #[serde(rename = "expectedVerification")]
    pub expected_verification: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProducerPlanReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[serde(rename = "intentId")]
    pub intent_id: String,
    #[serde(rename = "taskCount")]
    pub task_count: usize,
    #[serde(rename = "orderedTaskIds")]
    pub ordered_task_ids: Vec<String>,
    #[serde(rename = "roleCounts")]
    pub role_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl ProducerDesignIntent {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let intent: Self =
            serde_json::from_str(input).context("failed to parse Producer Design Intent JSON")?;
        intent.validate()?;
        Ok(intent)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCER_PLAN_SCHEMA_VERSION {
            return Err(anyhow!(
                "producer design intent schemaVersion must be {PRODUCER_PLAN_SCHEMA_VERSION}"
            ));
        }
        require_local_id("producer design intent intentId", &self.intent_id)?;
        require_local_ref("producer design intent sourceRef", &self.source_ref)?;
        self.design_brief.validate()?;
        if self.design_brief.status != GddDesignBriefStatus::Ready {
            return Err(anyhow!(
                "producer design intent requires a ready GDD design brief"
            ));
        }
        validate_functions(&self.requested_functions)?;
        require_text("producer design intent boundary", &self.boundary)?;
        require_boundary(
            "producer design intent boundary",
            &self.boundary,
            &[
                "reuses milestone 30",
                "gdd/design-brief surfaces",
                "decomposes intent into a plan",
                "proposal-only",
                "review/apply/trust-gradient path",
                "browser/studio read-only",
                "no direct trusted writes",
                "no auto-apply",
                "no auto-merge",
                "no production-ready",
                "no engine replacement",
            ],
        )?;
        Ok(())
    }
}

impl ProducerPlanArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let plan: Self =
            serde_json::from_str(input).context("failed to parse Producer Plan JSON")?;
        plan.validate()?;
        Ok(plan)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PRODUCER_PLAN_SCHEMA_VERSION {
            return Err(anyhow!(
                "producer plan schemaVersion must be {PRODUCER_PLAN_SCHEMA_VERSION}"
            ));
        }
        require_local_id("producer plan planId", &self.plan_id)?;
        require_local_id("producer plan intentId", &self.intent_id)?;
        require_local_ref("producer plan gddRef", &self.gdd_ref)?;
        if self.status != "planned" {
            return Err(anyhow!("producer plan status must be planned"));
        }
        validate_tasks(&self.tasks)?;
        require_text(
            "producer plan generatedStatePolicy",
            &self.generated_state_policy,
        )?;
        let generated_policy = self.generated_state_policy.to_ascii_lowercase();
        for required in ["untracked", "fixture-scoped"] {
            if !generated_policy.contains(required) {
                return Err(anyhow!(
                    "producer plan generatedStatePolicy must mention `{required}`"
                ));
            }
        }
        require_text("producer plan reviewPath", &self.review_path)?;
        require_boundary(
            "producer plan reviewPath",
            &self.review_path,
            &["proposal", "review/apply/trust-gradient", "human"],
        )?;
        require_text("producer plan boundary", &self.boundary)?;
        require_boundary(
            "producer plan boundary",
            &self.boundary,
            &[
                "task graph of function-agent work",
                "proposal-only",
                "review/apply/trust-gradient path",
                "browser/studio read-only",
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

    pub fn read_model(&self) -> ProducerPlanReadModel {
        let mut role_counts = BTreeMap::new();
        for task in &self.tasks {
            *role_counts.entry(task.role.clone()).or_insert(0) += 1;
        }
        ProducerPlanReadModel {
            schema_version: self.schema_version.clone(),
            plan_id: self.plan_id.clone(),
            intent_id: self.intent_id.clone(),
            task_count: self.tasks.len(),
            ordered_task_ids: self.tasks.iter().map(|task| task.task_id.clone()).collect(),
            role_counts,
            validation_summary: vec![
                "valid design intent decomposes into a deterministic production plan".to_string(),
                "plan is a task graph of function-agent work and performs no trusted write".to_string(),
                "malformed intent, unsupported functions, cycles, duplicate ids, and non-proposal tasks fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "Rust/local owns validation and decomposition; browser/Studio surfaces remain read-only".to_string(),
                "Milestone 30 and GDD/design-brief surfaces are reused; this is not a new generator".to_string(),
                "promotion remains through the existing review/apply/trust-gradient path with human gates".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize Producer Plan read model JSON")
    }
}

pub fn derive_producer_plan(intent: &ProducerDesignIntent) -> Result<ProducerPlanArtifact> {
    intent.validate()?;
    let requested = canonical_functions(&intent.requested_functions);
    let mut tasks = Vec::with_capacity(requested.len());
    let mut previous: Option<String> = None;
    for (index, function) in requested.iter().enumerate() {
        let task_id = format!("{}-{:02}-{}", intent.intent_id, index + 1, function);
        let depends_on = previous.iter().cloned().collect::<Vec<_>>();
        let task = task_for_function(intent, &task_id, function, depends_on)?;
        previous = Some(task.task_id.clone());
        tasks.push(task);
    }
    let plan = ProducerPlanArtifact {
        schema_version: PRODUCER_PLAN_SCHEMA_VERSION.to_string(),
        plan_id: format!("{}-production-plan", intent.intent_id),
        intent_id: intent.intent_id.clone(),
        gdd_ref: intent.source_ref.clone(),
        status: "planned".to_string(),
        tasks,
        generated_state_policy: "Generated runs/assets/content/artifacts remain untracked unless explicitly fixture-scoped".to_string(),
        review_path: "Every proposal routes through the existing review/apply/trust-gradient path with human approval gates".to_string(),
        boundary: "Rust/local validated task graph of function-agent work; proposal-only; review/apply/trust-gradient path; browser/Studio read-only; no direct trusted writes; no auto-apply; no auto-merge; no self-approval; no reviewer bypass; no production-ready claim; no engine replacement claim".to_string(),
    };
    plan.validate()?;
    Ok(plan)
}

fn task_for_function(
    intent: &ProducerDesignIntent,
    task_id: &str,
    function: &str,
    depends_on: Vec<String>,
) -> Result<ProducerPlanTask> {
    let (role, kind, inputs, outputs, verification) = match function {
        "design-brief" => (
            "designer",
            "intent-review",
            vec![intent.source_ref.clone()],
            vec![format!("design-brief:{}", intent.design_brief.brief_id)],
            vec!["validate GDD design brief read model"],
        ),
        "requirements" => (
            "designer",
            "requirement-extraction",
            vec![format!("design-brief:{}", intent.design_brief.brief_id)],
            vec!["gdd-requirement-extraction".to_string()],
            vec!["reject unsupported/ambiguous requirements"],
        ),
        "mechanics" => (
            "gameplay-engineer",
            "mechanics-mapping",
            vec!["gdd-requirement-extraction".to_string()],
            vec!["gdd-mechanics-mapping".to_string()],
            vec!["map mechanics to existing bounded contracts"],
        ),
        "scaffold" => (
            "gameplay-engineer",
            "project-scaffold-plan",
            vec!["gdd-mechanics-mapping".to_string()],
            vec!["gdd-project-scaffold-plan".to_string()],
            vec!["validate scaffold plan remains proposal-only"],
        ),
        "scene-level" => (
            "level-designer",
            "scene-level-plan",
            vec!["gdd-mechanics-mapping".to_string()],
            vec!["gdd-scene-level-plan".to_string()],
            vec!["validate scene/level plan refs and boundaries"],
        ),
        "behavior" => (
            "gameplay-engineer",
            "behavior-plan",
            vec!["gdd-mechanics-mapping".to_string()],
            vec!["gdd-gameplay-behavior-plan".to_string()],
            vec!["validate behavior plan without script generation authority"],
        ),
        "assets" => (
            "asset-import-planner",
            "asset-placeholder-plan",
            vec!["design-brief".to_string()],
            vec!["gdd-asset-placeholder-plan".to_string()],
            vec!["validate license/provenance placeholders"],
        ),
        "scenarios" => (
            "qa-agent",
            "scenario-acceptance-plan",
            vec!["gdd-requirement-extraction".to_string()],
            vec!["gdd-scenario-acceptance-plan".to_string()],
            vec!["validate scenario acceptance criteria refs"],
        ),
        "qa" => (
            "qa-agent",
            "function-qa-gates",
            vec!["gdd-scenario-acceptance-plan".to_string()],
            vec!["qa-evidence-bundle".to_string()],
            vec!["run targeted Rust/local contract checks"],
        ),
        "review" => (
            "reviewer",
            "review-gate",
            vec!["qa-evidence-bundle".to_string()],
            vec!["production-review-gate".to_string()],
            vec!["block until independent review/critic approval when required"],
        ),
        _ => return Err(anyhow!("unsupported producer plan function `{function}`")),
    };
    Ok(ProducerPlanTask {
        task_id: task_id.to_string(),
        function_agent: function.to_string(),
        role: role.to_string(),
        kind: kind.to_string(),
        depends_on,
        inputs,
        outputs,
        proposal_only: true,
        expected_verification: verification.into_iter().map(str::to_string).collect(),
    })
}

fn validate_functions(functions: &[String]) -> Result<()> {
    if functions.is_empty() {
        return Err(anyhow!(
            "producer design intent requestedFunctions must not be empty"
        ));
    }
    let mut seen = BTreeSet::new();
    for function in functions {
        require_text("producer design intent requestedFunctions", function)?;
        if !SUPPORTED_FUNCTIONS.contains(&function.as_str()) {
            return Err(anyhow!(
                "producer design intent requestedFunctions contains unsupported function `{function}`"
            ));
        }
        if !seen.insert(function.as_str()) {
            return Err(anyhow!(
                "producer design intent requestedFunctions contains duplicate function `{function}`"
            ));
        }
    }
    Ok(())
}

fn canonical_functions(functions: &[String]) -> Vec<String> {
    // `review` is a mandatory gate: every derived plan must terminate in a review
    // task (enforced by `validate_tasks`). Include it even when the intent's
    // `requestedFunctions` omits it, so any valid intent decomposes into a plan
    // that preserves the review/apply/human gate rather than failing derivation.
    SUPPORTED_FUNCTIONS
        .iter()
        .filter(|function| {
            **function == "review" || functions.iter().any(|requested| requested == **function)
        })
        .map(|function| (*function).to_string())
        .collect()
}

fn validate_tasks(tasks: &[ProducerPlanTask]) -> Result<()> {
    if tasks.is_empty() {
        return Err(anyhow!("producer plan tasks must not be empty"));
    }
    let mut ids = BTreeSet::new();
    let mut completed = BTreeSet::new();
    for task in tasks {
        task.validate()?;
        if !ids.insert(task.task_id.clone()) {
            return Err(anyhow!(
                "producer plan taskId `{}` is duplicated",
                task.task_id
            ));
        }
        for dependency in &task.depends_on {
            if !completed.contains(dependency) {
                return Err(anyhow!(
                    "producer plan task `{}` has missing or out-of-order dependency `{dependency}`",
                    task.task_id
                ));
            }
        }
        completed.insert(task.task_id.clone());
    }
    if !tasks.iter().any(|task| task.function_agent == "review") {
        return Err(anyhow!("producer plan must include a review function task"));
    }
    Ok(())
}

impl ProducerPlanTask {
    fn validate(&self) -> Result<()> {
        require_local_id("producer plan taskId", &self.task_id)?;
        if !SUPPORTED_FUNCTIONS.contains(&self.function_agent.as_str()) {
            return Err(anyhow!(
                "producer plan task functionAgent `{}` is unsupported",
                self.function_agent
            ));
        }
        require_text("producer plan task role", &self.role)?;
        require_text("producer plan task kind", &self.kind)?;
        for dependency in &self.depends_on {
            require_local_id("producer plan task dependsOn", dependency)?;
        }
        validate_text_list("producer plan task inputs", &self.inputs, true)?;
        validate_text_list("producer plan task outputs", &self.outputs, true)?;
        if !self.proposal_only {
            return Err(anyhow!(
                "producer plan task proposalOnly must be true; producer tasks cannot perform trusted writes"
            ));
        }
        validate_text_list(
            "producer plan task expectedVerification",
            &self.expected_verification,
            true,
        )?;
        Ok(())
    }
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
    reject_forbidden_wording(field, value)?;
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
                "{field} contains forbidden producer-plan wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}
