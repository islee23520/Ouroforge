use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const RUNTIME_FRAME_BUDGET_SCHEMA_VERSION: &str = "ouroforge.runtime-frame-budget.v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFrameBudgetEvidence {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "frameId")]
    pub frame_id: String,
    #[serde(rename = "sceneId")]
    pub scene_id: String,
    #[serde(
        default,
        rename = "scenarioId",
        skip_serializing_if = "Option::is_none"
    )]
    pub scenario_id: Option<String>,
    pub timings: RuntimeFrameTimings,
    pub budget: RuntimeFrameBudget,
    pub counts: RuntimeFrameDebugCounts,
    #[serde(rename = "readOnlyInspection")]
    pub read_only_inspection: RuntimeFrameBudgetInspectionBoundary,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFrameTimings {
    #[serde(rename = "updateMs")]
    pub update_ms: f64,
    #[serde(rename = "renderMs")]
    pub render_ms: f64,
    #[serde(rename = "evidenceMs")]
    pub evidence_ms: f64,
    #[serde(rename = "totalMs")]
    pub total_ms: f64,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFrameBudget {
    #[serde(rename = "updateMs")]
    pub update_ms: f64,
    #[serde(rename = "renderMs")]
    pub render_ms: f64,
    #[serde(rename = "evidenceMs")]
    pub evidence_ms: f64,
    #[serde(rename = "totalMs")]
    pub total_ms: f64,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFrameDebugCounts {
    #[serde(rename = "entityCount")]
    pub entity_count: u64,
    #[serde(rename = "drawCallCount")]
    pub draw_call_count: u64,
    #[serde(rename = "layerCount")]
    pub layer_count: u64,
    #[serde(rename = "collisionPairCount")]
    pub collision_pair_count: u64,
    #[serde(rename = "activeAnimationCount")]
    pub active_animation_count: u64,
    #[serde(rename = "activeVfxCount")]
    pub active_vfx_count: u64,
    #[serde(rename = "audioEventCount")]
    pub audio_event_count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeFrameBudgetInspectionBoundary {
    #[serde(rename = "trustedEmitter")]
    pub trusted_emitter: String,
    #[serde(rename = "browserStudioMode")]
    pub browser_studio_mode: String,
    #[serde(rename = "disallowedActions")]
    pub disallowed_actions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFrameBudgetStatus {
    WithinBudget,
    Violated,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeFrameBudgetViolation {
    pub field: &'static str,
    pub actual_ms: f64,
    pub budget_ms: f64,
}

impl RuntimeFrameBudgetEvidence {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let evidence: Self = serde_json::from_str(input)
            .context("failed to parse Runtime Frame Budget Evidence JSON")?;
        evidence.validate()?;
        Ok(evidence)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_FRAME_BUDGET_SCHEMA_VERSION {
            return Err(anyhow!(
                "schemaVersion must be {RUNTIME_FRAME_BUDGET_SCHEMA_VERSION}"
            ));
        }
        require_local_id("frameId", &self.frame_id)?;
        require_local_id("sceneId", &self.scene_id)?;
        if let Some(scenario_id) = &self.scenario_id {
            require_local_id("scenarioId", scenario_id)?;
        }
        self.timings.validate()?;
        self.budget.validate()?;
        self.read_only_inspection.validate()?;
        Ok(())
    }

    pub const fn debug_counts(&self) -> RuntimeFrameDebugCounts {
        self.counts
    }

    pub fn computed_violations(&self) -> Vec<RuntimeFrameBudgetViolation> {
        [
            ("updateMs", self.timings.update_ms, self.budget.update_ms),
            ("renderMs", self.timings.render_ms, self.budget.render_ms),
            (
                "evidenceMs",
                self.timings.evidence_ms,
                self.budget.evidence_ms,
            ),
            ("totalMs", self.timings.total_ms, self.budget.total_ms),
        ]
        .into_iter()
        .filter(|(_, actual_ms, budget_ms)| actual_ms > budget_ms)
        .map(
            |(field, actual_ms, budget_ms)| RuntimeFrameBudgetViolation {
                field,
                actual_ms,
                budget_ms,
            },
        )
        .collect()
    }

    pub fn status(&self) -> RuntimeFrameBudgetStatus {
        if self.computed_violations().is_empty() {
            RuntimeFrameBudgetStatus::WithinBudget
        } else {
            RuntimeFrameBudgetStatus::Violated
        }
    }
}

impl RuntimeFrameBudgetViolation {
    pub fn comparison_key(&self) -> String {
        format!("{}:{:.3}>{:.3}", self.field, self.actual_ms, self.budget_ms)
    }
}

impl RuntimeFrameTimings {
    fn validate(&self) -> Result<()> {
        validate_non_negative_ms("updateMs", self.update_ms)?;
        validate_non_negative_ms("renderMs", self.render_ms)?;
        validate_non_negative_ms("evidenceMs", self.evidence_ms)?;
        validate_non_negative_ms("totalMs", self.total_ms)
    }
}

impl RuntimeFrameBudget {
    fn validate(&self) -> Result<()> {
        validate_positive_ms("budget updateMs", self.update_ms)?;
        validate_positive_ms("budget renderMs", self.render_ms)?;
        validate_positive_ms("budget evidenceMs", self.evidence_ms)?;
        validate_positive_ms("budget totalMs", self.total_ms)
    }
}

impl RuntimeFrameBudgetInspectionBoundary {
    fn validate(&self) -> Result<()> {
        require_local_text("trustedEmitter", &self.trusted_emitter)?;
        require_local_text("browserStudioMode", &self.browser_studio_mode)?;
        if self.disallowed_actions.is_empty() {
            return Err(anyhow!("disallowedActions must not be empty"));
        }
        for action in &self.disallowed_actions {
            require_local_text("disallowedActions entry", action)?;
        }
        Ok(())
    }
}

pub fn read_runtime_frame_budget(path: impl AsRef<Path>) -> Result<RuntimeFrameBudgetEvidence> {
    let path = path.as_ref();
    let input = fs::read_to_string(path)
        .with_context(|| format!("failed to read runtime frame budget {}", path.display()))?;
    RuntimeFrameBudgetEvidence::from_json_str(&input)
        .with_context(|| format!("failed to validate runtime frame budget {}", path.display()))
}

fn validate_non_negative_ms(field: &str, value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(anyhow!("{field} must be finite"));
    }
    if value < 0.0 {
        return Err(anyhow!("{field} must be non-negative"));
    }
    Ok(())
}

fn validate_positive_ms(field: &str, value: f64) -> Result<()> {
    if !value.is_finite() {
        return Err(anyhow!("{field} must be finite"));
    }
    if value <= 0.0 {
        return Err(anyhow!("{field} must be positive"));
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
