//! Sub-100ms Responsiveness Verification v1 (#1821).
//!
//! Verifies input-to-feedback latency using existing fixed-step/probe-shaped
//! evidence. This is Rust/local validation over browser/runtime observations; it
//! does not add a new runtime, trusted browser write path, or automated fun/feel
//! verdict.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const RESPONSIVENESS_REPORT_SCHEMA_VERSION: &str = "ouroforge.responsiveness-report.v1";
pub const RESPONSIVENESS_EVENT_SCHEMA_VERSION: &str = "ouroforge.responsiveness-event.v1";
pub const DEFAULT_RESPONSIVENESS_BUDGET_MS: u32 = 100;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsivenessEvidence {
    pub schema_version: String,
    pub scenario_id: String,
    pub fixed_delta_ms: u32,
    #[serde(default = "default_budget")]
    pub budget_ms: u32,
    pub events: Vec<ResponsivenessEvent>,
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsivenessEvent {
    pub event_id: String,
    pub kind: ResponsivenessEventKind,
    pub tick: u32,
    #[serde(default)]
    pub input_id: Option<String>,
    #[serde(default)]
    pub feedback_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResponsivenessEventKind {
    Input,
    Feedback,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsivenessReport {
    pub schema_version: String,
    pub scenario_id: String,
    pub fixed_delta_ms: u32,
    pub budget_ms: u32,
    pub status: ResponsivenessStatus,
    pub max_latency_ms: u32,
    pub measurements: Vec<ResponsivenessMeasurement>,
    pub read_only_inspection: ResponsivenessReadOnlyInspection,
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResponsivenessStatus {
    Pass,
    Fail,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsivenessMeasurement {
    pub input_id: String,
    pub feedback_id: String,
    pub input_tick: u32,
    pub feedback_tick: u32,
    pub latency_ms: u32,
    pub within_budget: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponsivenessReadOnlyInspection {
    pub trusted_emitter: String,
    pub browser_studio_mode: String,
    pub disallowed_actions: Vec<String>,
}

fn default_budget() -> u32 {
    DEFAULT_RESPONSIVENESS_BUDGET_MS
}

pub fn verify_responsiveness(evidence: &ResponsivenessEvidence) -> Result<ResponsivenessReport> {
    if evidence.schema_version != RESPONSIVENESS_EVENT_SCHEMA_VERSION {
        return Err(anyhow!(
            "responsiveness evidence schemaVersion must be {RESPONSIVENESS_EVENT_SCHEMA_VERSION}"
        ));
    }
    if evidence.fixed_delta_ms == 0 || evidence.fixed_delta_ms > DEFAULT_RESPONSIVENESS_BUDGET_MS {
        return Err(anyhow!(
            "fixedDeltaMs must be between 1 and {DEFAULT_RESPONSIVENESS_BUDGET_MS}ms"
        ));
    }
    if evidence.budget_ms == 0 || evidence.budget_ms > DEFAULT_RESPONSIVENESS_BUDGET_MS {
        return Err(anyhow!(
            "budgetMs must be between 1 and {DEFAULT_RESPONSIVENESS_BUDGET_MS}ms"
        ));
    }

    let mut measurements = Vec::new();
    for input in evidence
        .events
        .iter()
        .filter(|event| matches!(event.kind, ResponsivenessEventKind::Input))
    {
        let input_id = input
            .input_id
            .as_ref()
            .ok_or_else(|| anyhow!("input event `{}` is missing inputId", input.event_id))?;
        let feedback = evidence
            .events
            .iter()
            .filter(|event| matches!(event.kind, ResponsivenessEventKind::Feedback))
            .filter(|event| event.input_id.as_ref() == Some(input_id))
            .min_by_key(|event| event.tick)
            .ok_or_else(|| anyhow!("input `{input_id}` has no matching feedback event"))?;
        if feedback.tick < input.tick {
            return Err(anyhow!(
                "feedback `{}` occurs before input `{input_id}`",
                feedback.event_id
            ));
        }
        let latency_ticks = feedback.tick - input.tick;
        let latency_ms = latency_ticks.saturating_mul(evidence.fixed_delta_ms);
        let feedback_id = feedback
            .feedback_id
            .clone()
            .unwrap_or_else(|| feedback.event_id.clone());
        measurements.push(ResponsivenessMeasurement {
            input_id: input_id.clone(),
            feedback_id,
            input_tick: input.tick,
            feedback_tick: feedback.tick,
            latency_ms,
            within_budget: latency_ms <= evidence.budget_ms,
        });
    }

    if measurements.is_empty() {
        return Err(anyhow!(
            "responsiveness evidence must include at least one input measurement"
        ));
    }
    measurements.sort_by(|left, right| left.input_id.cmp(&right.input_id));
    let max_latency_ms = measurements
        .iter()
        .map(|measurement| measurement.latency_ms)
        .max()
        .unwrap_or(0);
    let status = if measurements
        .iter()
        .all(|measurement| measurement.within_budget)
    {
        ResponsivenessStatus::Pass
    } else {
        ResponsivenessStatus::Fail
    };

    Ok(ResponsivenessReport {
        schema_version: RESPONSIVENESS_REPORT_SCHEMA_VERSION.to_string(),
        scenario_id: evidence.scenario_id.clone(),
        fixed_delta_ms: evidence.fixed_delta_ms,
        budget_ms: evidence.budget_ms,
        status,
        max_latency_ms,
        measurements,
        read_only_inspection: ResponsivenessReadOnlyInspection {
            trusted_emitter: "rust-responsiveness-verifier".to_string(),
            browser_studio_mode: "read-only input-to-feedback latency inspection".to_string(),
            disallowed_actions: vec![
                "trusted writes".to_string(),
                "command bridge".to_string(),
                "live mutation".to_string(),
                "new runtime".to_string(),
                "automated fun verdict".to_string(),
            ],
        },
        generated_state_policy: evidence.generated_state_policy.clone(),
        boundary: "Deterministic fixed-step responsiveness evidence only; browser/probe observations are read-only inputs, Rust/local owns verification, and feel/fun judgment remains human (Era J).".to_string(),
    })
}
