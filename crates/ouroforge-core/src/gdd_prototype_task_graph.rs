use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_PROTOTYPE_TASK_GRAPH_SCHEMA_VERSION: &str = "gdd-prototype-task-graph-v1";
const REQUIRED_TASK_KINDS: &[GddPrototypeTaskKind] = &[
    GddPrototypeTaskKind::Scaffold,
    GddPrototypeTaskKind::Assets,
    GddPrototypeTaskKind::SceneLevel,
    GddPrototypeTaskKind::Behavior,
    GddPrototypeTaskKind::Scenarios,
    GddPrototypeTaskKind::RunEvidence,
    GddPrototypeTaskKind::ReviewGate,
    GddPrototypeTaskKind::ApplyStep,
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeTaskGraphStatus {
    Ready,
    Blocked,
    Incomplete,
    Stale,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeTaskKind {
    Scaffold,
    Assets,
    SceneLevel,
    Behavior,
    Scenarios,
    RunEvidence,
    ReviewGate,
    ApplyStep,
    Deferred,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeTaskStatus {
    Planned,
    Ready,
    Blocked,
    Deferred,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeTaskGraphArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "graphId")]
    pub graph_id: String,
    pub status: GddPrototypeTaskGraphStatus,
    #[serde(rename = "gddRef")]
    pub gdd_ref: String,
    #[serde(rename = "requirementExtractionRef")]
    pub requirement_extraction_ref: String,
    #[serde(rename = "mechanicsMappingRef")]
    pub mechanics_mapping_ref: String,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "planRefs")]
    pub plan_refs: Vec<String>,
    pub tasks: Vec<GddPrototypeTask>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeTask {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub kind: GddPrototypeTaskKind,
    pub status: GddPrototypeTaskStatus,
    #[serde(rename = "dependsOn")]
    pub depends_on: Vec<String>,
    #[serde(rename = "producerArtifacts")]
    pub producer_artifacts: Vec<String>,
    #[serde(rename = "consumerArtifacts")]
    pub consumer_artifacts: Vec<String>,
    #[serde(rename = "fileOwnership")]
    pub file_ownership: Vec<String>,
    #[serde(rename = "expectedVerification")]
    pub expected_verification: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeTaskGraphReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "graphId")]
    pub graph_id: String,
    pub status: String,
    #[serde(rename = "taskCount")]
    pub task_count: usize,
    #[serde(rename = "blockedTaskCount")]
    pub blocked_task_count: usize,
    #[serde(rename = "kindCounts")]
    pub kind_counts: BTreeMap<String, usize>,
    #[serde(rename = "orderedTaskIds")]
    pub ordered_task_ids: Vec<String>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddPrototypeTaskGraphArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse GDD Prototype Task Graph JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddPrototypeTaskGraphReadModel {
        let mut kind_counts = BTreeMap::new();
        for task in &self.tasks {
            *kind_counts
                .entry(task_kind_label(task.kind).to_string())
                .or_insert(0) += 1;
        }
        GddPrototypeTaskGraphReadModel {
            schema_version: self.schema_version.clone(),
            graph_id: self.graph_id.clone(),
            status: graph_status_label(&self.status).to_string(),
            task_count: self.tasks.len(),
            blocked_task_count: self
                .tasks
                .iter()
                .filter(|task| task.status != GddPrototypeTaskStatus::Ready && task.status != GddPrototypeTaskStatus::Planned || !task.blocked_reasons.is_empty())
                .count(),
            kind_counts,
            ordered_task_ids: self.tasks.iter().map(|task| task.task_id.clone()).collect(),
            validation_summary: vec![
                "prototype implementation is represented as an ordered dependency-checked task graph before apply".to_string(),
                "cycles, missing dependencies, conflicting file ownership, missing producers, out-of-order apply, and unsupported scope fail closed".to_string(),
                "task graph data does not execute hidden commands or grant trusted writes".to_string(),
            ],
            compatibility_notes: vec![
                "GDD, requirements, mechanics, feasibility, plans, drafts, task graph, review, apply, run evidence, and journal artifacts remain separate".to_string(),
                "browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
                "generated task graphs remain untrusted until Rust/local validation and review-gated apply".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD prototype task graph read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_PROTOTYPE_TASK_GRAPH_SCHEMA_VERSION {
            return Err(anyhow!("GDD prototype task graph schemaVersion must be {GDD_PROTOTYPE_TASK_GRAPH_SCHEMA_VERSION}"));
        }
        require_local_id("GDD prototype task graph graphId", &self.graph_id)?;
        for (field, value) in [
            ("GDD prototype task graph gddRef", &self.gdd_ref),
            (
                "GDD prototype task graph requirementExtractionRef",
                &self.requirement_extraction_ref,
            ),
            (
                "GDD prototype task graph mechanicsMappingRef",
                &self.mechanics_mapping_ref,
            ),
            (
                "GDD prototype task graph feasibilityGateRef",
                &self.feasibility_gate_ref,
            ),
        ] {
            require_local_ref(field, value)?;
        }
        validate_local_ref_list("GDD prototype task graph planRefs", &self.plan_refs, true)?;
        self.validate_tasks()?;
        validate_string_list(
            "GDD prototype task graph blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let has_blocked = !self.blocked_reasons.is_empty()
            || self.tasks.iter().any(|task| {
                matches!(
                    task.status,
                    GddPrototypeTaskStatus::Blocked
                        | GddPrototypeTaskStatus::Deferred
                        | GddPrototypeTaskStatus::Stale
                ) || !task.blocked_reasons.is_empty()
            });
        match self.status {
            GddPrototypeTaskGraphStatus::Ready if has_blocked => {
                return Err(anyhow!("ready GDD prototype task graph must not include blocked, deferred, stale, or unsupported tasks"));
            }
            GddPrototypeTaskGraphStatus::Blocked
            | GddPrototypeTaskGraphStatus::Incomplete
            | GddPrototypeTaskGraphStatus::Stale
                if !has_blocked =>
            {
                return Err(anyhow!("non-ready GDD prototype task graph requires visible blockedReasons or blocked tasks"));
            }
            _ => {}
        }
        require_text("GDD prototype task graph boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "ordered dependency-checked task graph before apply",
            "untrusted until rust/local validation",
            "review-gated apply",
            "does not execute hidden commands",
            "no autonomous unrestricted game creation",
            "browser read-only or draft-only",
            "no direct trusted writes",
            "no asset generation",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD prototype task graph boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn validate_tasks(&self) -> Result<()> {
        require_nonempty("GDD prototype task graph tasks", self.tasks.len())?;
        if self.tasks.len() > 32 {
            return Err(anyhow!(
                "GDD prototype task graph tasks are overbroad for v1"
            ));
        }
        let mut ids = BTreeSet::new();
        for task in &self.tasks {
            task.validate()?;
            if !ids.insert(task.task_id.clone()) {
                return Err(anyhow!(
                    "GDD prototype task graph taskId `{}` is duplicated",
                    task.task_id
                ));
            }
        }
        for required in REQUIRED_TASK_KINDS {
            if !self.tasks.iter().any(|task| task.kind == *required) {
                return Err(anyhow!(
                    "GDD prototype task graph missing required task kind `{}`",
                    task_kind_label(*required)
                ));
            }
        }
        self.validate_dependencies(&ids)?;
        self.validate_file_ownership()?;
        self.validate_artifact_producers()?;
        self.validate_apply_order()?;
        Ok(())
    }

    fn validate_dependencies(&self, ids: &BTreeSet<String>) -> Result<()> {
        let mut positions = BTreeMap::new();
        for (index, task) in self.tasks.iter().enumerate() {
            positions.insert(task.task_id.as_str(), index);
        }
        for (index, task) in self.tasks.iter().enumerate() {
            for dep in &task.depends_on {
                if !ids.contains(dep) {
                    return Err(anyhow!(
                        "GDD prototype task graph task `{}` has missing dependency `{dep}`",
                        task.task_id
                    ));
                }
                if positions[dep.as_str()] >= index {
                    return Err(anyhow!("GDD prototype task graph has cycle or out-of-order dependency involving `{}` and `{dep}`", task.task_id));
                }
            }
        }
        Ok(())
    }

    fn validate_file_ownership(&self) -> Result<()> {
        let mut owners = BTreeMap::<String, String>::new();
        for task in &self.tasks {
            for path in &task.file_ownership {
                if let Some(previous) = owners.insert(path.clone(), task.task_id.clone()) {
                    return Err(anyhow!("GDD prototype task graph conflicting file ownership for `{path}` between `{previous}` and `{}`", task.task_id));
                }
            }
        }
        Ok(())
    }

    fn validate_artifact_producers(&self) -> Result<()> {
        let mut produced = BTreeSet::new();
        for task in &self.tasks {
            for consumer in &task.consumer_artifacts {
                if !produced.contains(consumer) {
                    return Err(anyhow!("GDD prototype task graph task `{}` consumes `{consumer}` before a producer artifact exists", task.task_id));
                }
            }
            for producer in &task.producer_artifacts {
                if !produced.insert(producer.clone()) {
                    return Err(anyhow!(
                        "GDD prototype task graph producer artifact `{producer}` is duplicated"
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_apply_order(&self) -> Result<()> {
        for task in &self.tasks {
            if task.kind == GddPrototypeTaskKind::ApplyStep {
                let has_review_dep = task.depends_on.iter().any(|dep| {
                    self.tasks.iter().any(|candidate| {
                        candidate.task_id == *dep
                            && candidate.kind == GddPrototypeTaskKind::ReviewGate
                    })
                });
                if !has_review_dep {
                    return Err(anyhow!(
                        "GDD prototype task graph apply steps must depend on a review-gate task"
                    ));
                }
            }
        }
        Ok(())
    }
}

impl GddPrototypeTask {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD prototype task graph taskId", &self.task_id)?;
        validate_local_id_list(
            "GDD prototype task graph dependsOn",
            &self.depends_on,
            false,
        )?;
        validate_local_ref_list(
            "GDD prototype task graph producerArtifacts",
            &self.producer_artifacts,
            true,
        )?;
        validate_local_ref_list(
            "GDD prototype task graph consumerArtifacts",
            &self.consumer_artifacts,
            false,
        )?;
        validate_local_ref_list(
            "GDD prototype task graph fileOwnership",
            &self.file_ownership,
            true,
        )?;
        validate_string_list(
            "GDD prototype task graph expectedVerification",
            &self.expected_verification,
            true,
        )?;
        validate_string_list(
            "GDD prototype task graph task.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let requires_blocker = matches!(
            self.kind,
            GddPrototypeTaskKind::Deferred | GddPrototypeTaskKind::Blocked
        ) || matches!(
            self.status,
            GddPrototypeTaskStatus::Blocked
                | GddPrototypeTaskStatus::Deferred
                | GddPrototypeTaskStatus::Stale
        );
        if requires_blocker && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD prototype task graph blocked/deferred/stale task `{}` requires blockedReasons",
                self.task_id
            ));
        }
        if self.kind == GddPrototypeTaskKind::ApplyStep
            && self
                .expected_verification
                .iter()
                .all(|value| !value.to_ascii_lowercase().contains("review"))
        {
            return Err(anyhow!("GDD prototype task graph apply-step expectedVerification must mention review evidence"));
        }
        Ok(())
    }
}

fn graph_status_label(status: &GddPrototypeTaskGraphStatus) -> &'static str {
    match status {
        GddPrototypeTaskGraphStatus::Ready => "ready",
        GddPrototypeTaskGraphStatus::Blocked => "blocked",
        GddPrototypeTaskGraphStatus::Incomplete => "incomplete",
        GddPrototypeTaskGraphStatus::Stale => "stale",
    }
}

fn task_kind_label(kind: GddPrototypeTaskKind) -> &'static str {
    match kind {
        GddPrototypeTaskKind::Scaffold => "scaffold",
        GddPrototypeTaskKind::Assets => "assets",
        GddPrototypeTaskKind::SceneLevel => "scene-level",
        GddPrototypeTaskKind::Behavior => "behavior",
        GddPrototypeTaskKind::Scenarios => "scenarios",
        GddPrototypeTaskKind::RunEvidence => "run-evidence",
        GddPrototypeTaskKind::ReviewGate => "review-gate",
        GddPrototypeTaskKind::ApplyStep => "apply-step",
        GddPrototypeTaskKind::Deferred => "deferred",
        GddPrototypeTaskKind::Blocked => "blocked",
    }
}

fn validate_local_id_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_id(field, value)?;
    }
    Ok(())
}

fn validate_local_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_ref(field, value)?;
    }
    Ok(())
}

fn validate_string_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_text(field, value)?;
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
        return Err(anyhow!("{field} contains forbidden traversal and must stay inside local fixture/reference roots"));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/")
        || value.starts_with("runs/"))
    {
        return Err(anyhow!(
            "{field} must use examples/, docs/, seeds/, or runs/ refs"
        ));
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
        "commercial readiness",
        "arbitrary source mutation",
        "arbitrary script execution",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD/prototype authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    if !value.contains(phrase) {
        return false;
    }
    !["no ", "not ", "without ", "avoid ", "forbid ", "forbidden "]
        .iter()
        .any(|prefix| value.contains(&format!("{prefix}{phrase}")))
}
