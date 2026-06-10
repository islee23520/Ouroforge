//! Behavior Scenario Assertion Drafts v1 (#2374, #1 M124).
//!
//! Converts structured behavior authoring specs into draft-only scenario packs
//! using the existing `scenario-pack-v1` assertion vocabulary. Promotion to a
//! trusted suite is intentionally out of scope: reviewers must explicitly
//! promote drafts through the existing scenario validation pipeline.

use crate::behavior_authoring_spec::{
    BehaviorAuthoringDomain, BehaviorAuthoringSpec, BehaviorParameterValue,
    BehaviorScenarioAssertionKind,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const BEHAVIOR_SCENARIO_DRAFT_SCHEMA_VERSION: &str = "behavior-scenario-draft-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorScenarioDraftStatus {
    DraftReady,
    Rejected,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorScenarioDraftDiagnosticCode {
    UnsupportedBehaviorDomain,
    UnsupportedBehaviorAssertion,
    MissingEvidencePath,
    ScenarioValidationFailed,
}

impl BehaviorScenarioDraftDiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedBehaviorDomain => "unsupported_behavior_domain",
            Self::UnsupportedBehaviorAssertion => "unsupported_behavior_assertion",
            Self::MissingEvidencePath => "missing_evidence_path",
            Self::ScenarioValidationFailed => "scenario_validation_failed",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioDraftDiagnostic {
    pub code: BehaviorScenarioDraftDiagnosticCode,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assertion: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioDraftBundle {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "behaviorId")]
    pub behavior_id: String,
    pub status: BehaviorScenarioDraftStatus,
    #[serde(rename = "draftOnly")]
    pub draft_only: bool,
    #[serde(rename = "scenarioPack")]
    pub scenario_pack: Value,
    #[serde(rename = "assertionDrafts")]
    pub assertion_drafts: Vec<BehaviorAssertionDraftSummary>,
    pub diagnostics: Vec<BehaviorScenarioDraftDiagnostic>,
    #[serde(rename = "promotionBoundary")]
    pub promotion_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorAssertionDraftSummary {
    pub id: String,
    #[serde(rename = "evidencePath")]
    pub evidence_path: String,
    #[serde(rename = "failureMessage")]
    pub failure_message: String,
}

impl BehaviorScenarioDraftBundle {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_SCENARIO_DRAFT_SCHEMA_VERSION {
            return Err(anyhow!(
                "behavior scenario draft schemaVersion must be {BEHAVIOR_SCENARIO_DRAFT_SCHEMA_VERSION}"
            ));
        }
        if !self.draft_only {
            return Err(anyhow!(
                "behavior scenario assertions must remain draftOnly until reviewer promotion"
            ));
        }
        if self.status == BehaviorScenarioDraftStatus::DraftReady
            && self.assertion_drafts.is_empty()
        {
            return Err(anyhow!(
                "behavior scenario draft must include assertionDrafts when ready"
            ));
        }
        Ok(())
    }

    pub fn scenario_pack_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.scenario_pack)
            .context("failed to serialize behavior scenario pack JSON")
    }
}

pub fn generate_behavior_scenario_draft(
    spec: &BehaviorAuthoringSpec,
) -> BehaviorScenarioDraftBundle {
    let diagnostics = collect_diagnostics(spec);
    if !diagnostics.is_empty() {
        return rejected_bundle(spec, diagnostics);
    }

    let assertions: Vec<Value> = spec
        .scenario_assertions
        .iter()
        .map(|assertion| {
            let expected = parameter_value_json(&assertion.expected_value);
            match assertion.assertion_kind {
                BehaviorScenarioAssertionKind::WorldState => json!({
                    "world_state": { "path": assertion.expected_path, "equals": expected }
                }),
                BehaviorScenarioAssertionKind::WorldFlag => json!({
                    "world_flags": { "path": assertion.expected_path, "equals": expected }
                }),
                BehaviorScenarioAssertionKind::RuntimeEvent => json!({
                    "runtime_events": { "path": assertion.expected_path, "containsType": expected.as_str().unwrap_or("runtime.behavior.event") }
                }),
                BehaviorScenarioAssertionKind::FrameStats => json!({
                    "frame_stats": { "path": assertion.expected_path, "equals": expected }
                }),
            }
        })
        .collect();
    let assertion_drafts = spec
        .scenario_assertions
        .iter()
        .map(|assertion| BehaviorAssertionDraftSummary {
            id: assertion.id.clone(),
            evidence_path: assertion.expected_path.clone(),
            failure_message: format!(
                "behavior assertion `{}` from `{}` failed at `{}`",
                assertion.id, assertion.source, assertion.expected_path
            ),
        })
        .collect();
    let scenario_pack = json!({
        "schemaVersion": "scenario-pack-v1",
        "id": format!("{}-behavior-draft", spec.behavior_id),
        "description": format!("Draft-only behavior assertions generated from {}", spec.behavior_id),
        "seed": "seeds/behavior-authoring-draft.yaml",
        "scenes": ["scenes/behavior-authoring-draft.scene.json"],
        "scenarioGroups": [{
            "id": "behavior-assertion-drafts",
            "description": "Draft-only generated assertions; reviewer promotion required.",
            "scenarios": [{
                "id": format!("{}-hazard-pass-fail", spec.behavior_id),
                "description": "Hazard behavior pass/fail assertion draft generated from structured behavior data.",
                "steps": [{ "replayRef": { "id": "hazard-preview-replay", "path": "replays/hazard-preview.replay.json" }}],
                "assertions": assertions
            }]
        }]
    });
    BehaviorScenarioDraftBundle {
        schema_version: BEHAVIOR_SCENARIO_DRAFT_SCHEMA_VERSION.to_string(),
        behavior_id: spec.behavior_id.clone(),
        status: BehaviorScenarioDraftStatus::DraftReady,
        draft_only: true,
        scenario_pack,
        assertion_drafts,
        diagnostics: Vec::new(),
        promotion_boundary:
            "draft only; reviewer promotion is the only path to trusted scenario suites".to_string(),
    }
}

fn collect_diagnostics(spec: &BehaviorAuthoringSpec) -> Vec<BehaviorScenarioDraftDiagnostic> {
    let mut diagnostics = Vec::new();
    if let Err(err) = spec.validate() {
        diagnostics.push(diagnostic(
            BehaviorScenarioDraftDiagnosticCode::ScenarioValidationFailed,
            err.to_string(),
            None,
        ));
        return diagnostics;
    }
    if spec.domain != BehaviorAuthoringDomain::Hazard {
        diagnostics.push(diagnostic(
            BehaviorScenarioDraftDiagnosticCode::UnsupportedBehaviorDomain,
            "M124.3 only generates hazard behavior assertion drafts first".to_string(),
            None,
        ));
    }
    for assertion in &spec.scenario_assertions {
        if assertion.expected_path.trim().is_empty() {
            diagnostics.push(diagnostic(
                BehaviorScenarioDraftDiagnosticCode::MissingEvidencePath,
                "generated assertions require evidence paths".to_string(),
                Some(assertion.id.clone()),
            ));
        }
        if assertion.assertion_kind == BehaviorScenarioAssertionKind::RuntimeEvent
            && !matches!(assertion.expected_value, BehaviorParameterValue::TextId(_))
        {
            diagnostics.push(diagnostic(
                BehaviorScenarioDraftDiagnosticCode::UnsupportedBehaviorAssertion,
                "runtime event assertions require text-id expected values for containsType"
                    .to_string(),
                Some(assertion.id.clone()),
            ));
        }
    }
    diagnostics
}

fn rejected_bundle(
    spec: &BehaviorAuthoringSpec,
    diagnostics: Vec<BehaviorScenarioDraftDiagnostic>,
) -> BehaviorScenarioDraftBundle {
    BehaviorScenarioDraftBundle {
        schema_version: BEHAVIOR_SCENARIO_DRAFT_SCHEMA_VERSION.to_string(),
        behavior_id: spec.behavior_id.clone(),
        status: BehaviorScenarioDraftStatus::Rejected,
        draft_only: true,
        scenario_pack: json!({
            "schemaVersion": "scenario-pack-v1",
            "id": format!("{}-rejected-behavior-draft", spec.behavior_id),
            "description": "Rejected behavior scenario draft; diagnostics must be resolved before scenario generation.",
            "seed": "seeds/behavior-authoring-draft.yaml",
            "scenes": ["scenes/behavior-authoring-draft.scene.json"],
            "scenarioGroups": [{
                "id": "rejected",
                "description": "Rejected draft placeholder.",
                "scenarios": [{
                    "id": format!("{}-rejected", spec.behavior_id),
                    "description": "Rejected behavior scenario draft placeholder.",
                    "assertions": [{ "world_state": { "path": "sceneId", "exists": true }}]
                }]
            }]
        }),
        assertion_drafts: Vec::new(),
        diagnostics,
        promotion_boundary:
            "draft rejected; fail closed until a reviewer receives supported behavior assertions"
                .to_string(),
    }
}

fn parameter_value_json(value: &BehaviorParameterValue) -> Value {
    match value {
        BehaviorParameterValue::Bool(value) => json!(value),
        BehaviorParameterValue::Integer(value) => json!(value),
        BehaviorParameterValue::Decimal(value) => json!(value),
        BehaviorParameterValue::TextId(value)
        | BehaviorParameterValue::EntityRef(value)
        | BehaviorParameterValue::PrefabRef(value) => json!(value),
        BehaviorParameterValue::Vector2 { x, y } => json!({ "x": x, "y": y }),
    }
}

fn diagnostic(
    code: BehaviorScenarioDraftDiagnosticCode,
    message: String,
    assertion: Option<String>,
) -> BehaviorScenarioDraftDiagnostic {
    BehaviorScenarioDraftDiagnostic {
        code,
        message,
        assertion,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behavior_authoring_spec::{
        BehaviorAction, BehaviorActionKind, BehaviorDraftBoundary, BehaviorEvidenceRef,
        BehaviorParameter, BehaviorParameterType, BehaviorPreviewContract, BehaviorPreviewMode,
        BehaviorState, BehaviorTransition, BehaviorTrigger, BehaviorTriggerKind,
    };
    use std::collections::BTreeMap;

    #[test]
    fn hazard_behavior_maps_to_draft_scenario_assertions() {
        let spec = hazard_spec(BehaviorAuthoringDomain::Hazard);
        let draft = generate_behavior_scenario_draft(&spec);
        draft.validate().unwrap();
        assert_eq!(draft.status, BehaviorScenarioDraftStatus::DraftReady);
        assert!(draft.draft_only);
        assert_eq!(draft.assertion_drafts.len(), 2);
        let pack = draft.scenario_pack_json().unwrap();
        assert!(pack.contains("scenario-pack-v1"));
        assert!(pack.contains("runtime_events"));
    }

    #[test]
    fn unsupported_behavior_domain_fails_closed() {
        let spec = hazard_spec(BehaviorAuthoringDomain::Npc);
        let draft = generate_behavior_scenario_draft(&spec);
        assert_eq!(draft.status, BehaviorScenarioDraftStatus::Rejected);
        assert!(draft
            .diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.code.as_str() == "unsupported_behavior_domain" }));
    }

    pub(crate) fn hazard_spec(domain: BehaviorAuthoringDomain) -> BehaviorAuthoringSpec {
        BehaviorAuthoringSpec {
            schema_version: crate::behavior_authoring_spec::BEHAVIOR_AUTHORING_SPEC_SCHEMA_VERSION
                .to_string(),
            behavior_id: "hazard-route-timing-demo".to_string(),
            domain,
            initial_state: "patrolling".to_string(),
            states: vec![
                BehaviorState {
                    id: "patrolling".to_string(),
                    label: "Patrolling".to_string(),
                    tags: vec!["hazard".to_string()],
                },
                BehaviorState {
                    id: "contact-failed".to_string(),
                    label: "Contact failed".to_string(),
                    tags: vec!["hazard".to_string()],
                },
            ],
            transitions: vec![BehaviorTransition {
                id: "contact-fails".to_string(),
                from: "patrolling".to_string(),
                to: "contact-failed".to_string(),
                trigger: BehaviorTrigger {
                    kind: BehaviorTriggerKind::OnPlayerContact,
                    parameters: BTreeMap::new(),
                },
                actions: vec![BehaviorAction {
                    kind: BehaviorActionKind::EmitEvent,
                    parameters: BTreeMap::from([(
                        "event".to_string(),
                        BehaviorParameterValue::TextId("hazard.contact.failed".to_string()),
                    )]),
                }],
            }],
            parameter_schema: vec![BehaviorParameter {
                name: "contactFrameThreshold".to_string(),
                value_type: BehaviorParameterType::Integer,
                default: Some(BehaviorParameterValue::Integer(60)),
                min: Some(0),
                max: Some(600),
            }],
            preview: BehaviorPreviewContract {
                preview_mode: BehaviorPreviewMode::ScenarioAssertionGeneration,
                deterministic_seed: 42,
                expected_events: vec!["hazard.contact.failed".to_string()],
                evidence_refs: vec![BehaviorEvidenceRef {
                    run_id: "run-after".to_string(),
                    path: "runs/session-h-2374/behavior-preview-after.json".to_string(),
                    digest: "sha256:after".to_string(),
                }],
            },
            scenario_assertions: vec![
                crate::behavior_authoring_spec::BehaviorScenarioAssertion {
                    id: "hazard-fail-flag".to_string(),
                    source: "contact-fails".to_string(),
                    assertion_kind: BehaviorScenarioAssertionKind::WorldFlag,
                    expected_path: "hazard_failed".to_string(),
                    expected_value: BehaviorParameterValue::Bool(true),
                },
                crate::behavior_authoring_spec::BehaviorScenarioAssertion {
                    id: "hazard-fail-event".to_string(),
                    source: "contact-fails".to_string(),
                    assertion_kind: BehaviorScenarioAssertionKind::RuntimeEvent,
                    expected_path: "events".to_string(),
                    expected_value: BehaviorParameterValue::TextId(
                        "hazard.contact.failed".to_string(),
                    ),
                },
            ],
            draft_boundary: BehaviorDraftBoundary::SafeSourceApplyReviewRequired,
            guardrails: vec![
                "draft-only generated assertions; reviewer promotion required".to_string(),
            ],
        }
    }
}
