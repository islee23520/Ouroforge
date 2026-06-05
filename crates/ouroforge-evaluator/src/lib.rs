use anyhow::{anyhow, Context, Result};
use ouroforge_evidence::{
    add_evidence_artifact, read_evidence_index, validate_evidence_artifact_path, EvidenceArtifact,
    EvidenceIndex,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EvaluationVerdict {
    pub status: String,
    pub summary: String,
    pub failures: Vec<serde_json::Value>,
    pub evidence_refs: Vec<String>,
    pub metadata: serde_json::Value,
    #[serde(
        rename = "gateCategories",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gate_categories: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub visual: Vec<VisualGateVerdict>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic: Vec<SemanticGateVerdict>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct VisualGateVerdict {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "checkpointId")]
    pub checkpoint_id: String,
    pub state: VisualGateState,
    pub reason: String,
    #[serde(rename = "comparisonRef")]
    pub comparison_ref: String,
    #[serde(rename = "changedPixels")]
    pub changed_pixels: Option<u64>,
    #[serde(rename = "changedPercentX1000")]
    pub changed_percent_x1000: Option<u32>,
    #[serde(rename = "changedRegionCount")]
    pub changed_region_count: usize,
    #[serde(rename = "thresholdSummary")]
    pub threshold_summary: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "outputRoot")]
    pub output_root: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum VisualGateState {
    Pass,
    Fail,
    MissingBaseline,
    MissingScreenshot,
    UnsupportedFormat,
    ThresholdNotDeclared,
    StaleRef,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SemanticGateVerdict {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(rename = "invariantId")]
    pub invariant_id: String,
    #[serde(rename = "invariantType")]
    pub invariant_type: Option<RuntimeInvariantType>,
    pub state: SemanticGateState,
    pub reason: String,
    #[serde(rename = "modelRef")]
    pub model_ref: String,
    #[serde(rename = "worldStateRef")]
    pub world_state_ref: Option<String>,
    #[serde(rename = "targetPath")]
    pub target_path: Option<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticGateState {
    Pass,
    Fail,
    Unsupported,
    MissingTargetState,
    MalformedInvariant,
    UnsafeExpression,
    StaleRef,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeInvariantType {
    PlayerInBounds,
    EntityInBounds,
    FiniteTransform,
    HealthNonNegative,
    ObjectiveFlagsConsistent,
    SceneTransitionValid,
    NoImpossibleState,
    RequiredEntityPresent,
    BehaviorStateConsistent,
}

const RUNTIME_INVARIANT_MODEL_SCHEMA_VERSION: &str = "runtime-invariant-model-v1";
const RUNTIME_INVARIANT_EVIDENCE_SCHEMA_VERSION: &str = "runtime-invariant-evidence-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInvariantModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "scenarioId", skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<String>,
    #[serde(rename = "worldStatePath")]
    pub world_state_path: String,
    #[serde(rename = "scenarioResultPath", skip_serializing_if = "Option::is_none")]
    pub scenario_result_path: Option<String>,
    #[serde(rename = "evidenceIndexPath")]
    pub evidence_index_path: String,
    pub invariants: Vec<RuntimeInvariantSpec>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInvariantSpec {
    #[serde(rename = "invariantId")]
    pub invariant_id: String,
    #[serde(rename = "invariantType")]
    pub invariant_type: RuntimeInvariantType,
    #[serde(rename = "targetPath")]
    pub target_path: String,
    #[serde(rename = "evidencePath")]
    pub evidence_path: String,
    #[serde(rename = "requiredEntityId", skip_serializing_if = "Option::is_none")]
    pub required_entity_id: Option<String>,
    #[serde(rename = "boundsPath", skip_serializing_if = "Option::is_none")]
    pub bounds_path: Option<String>,
    #[serde(
        rename = "transitionTargetPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub transition_target_path: Option<String>,
    #[serde(rename = "behaviorStatePath", skip_serializing_if = "Option::is_none")]
    pub behavior_state_path: Option<String>,
    #[serde(
        rename = "allowedStates",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub allowed_states: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInvariantEvidence {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "scenarioId", skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<String>,
    #[serde(rename = "worldStatePath")]
    pub world_state_path: String,
    #[serde(rename = "scenarioResultPath", skip_serializing_if = "Option::is_none")]
    pub scenario_result_path: Option<String>,
    #[serde(rename = "recordedAtUnixMs")]
    pub recorded_at_unix_ms: u128,
    pub checks: Vec<RuntimeInvariantCheck>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInvariantCheck {
    #[serde(rename = "invariantId")]
    pub invariant_id: String,
    #[serde(rename = "invariantType")]
    pub invariant_type: RuntimeInvariantType,
    pub status: RuntimeInvariantStatus,
    #[serde(rename = "targetPath")]
    pub target_path: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeInvariantStatus {
    Passed,
    Failed,
    Unsupported,
    Missing,
    Malformed,
    Stale,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RuntimeInvariantEvidenceSummary {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "modelId")]
    pub model_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "scenarioId", skip_serializing_if = "Option::is_none")]
    pub scenario_id: Option<String>,
    #[serde(rename = "checkCount")]
    pub check_count: usize,
    #[serde(rename = "passedCount")]
    pub passed_count: usize,
    #[serde(rename = "failedCount")]
    pub failed_count: usize,
    #[serde(rename = "unsupportedCount")]
    pub unsupported_count: usize,
    #[serde(rename = "missingCount")]
    pub missing_count: usize,
    #[serde(rename = "malformedCount")]
    pub malformed_count: usize,
    #[serde(rename = "staleCount")]
    pub stale_count: usize,
}

impl RuntimeInvariantModel {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let model: RuntimeInvariantModel =
            serde_json::from_str(input).context("failed to parse Runtime Invariant Model JSON")?;
        model.validate()?;
        Ok(model)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_INVARIANT_MODEL_SCHEMA_VERSION {
            return Err(anyhow!("runtime invariant model schemaVersion must be {RUNTIME_INVARIANT_MODEL_SCHEMA_VERSION}"));
        }
        validate_path_component("runtime invariant model modelId", &self.model_id)?;
        validate_path_component("runtime invariant model runId", &self.run_id)?;
        if let Some(scenario_id) = &self.scenario_id {
            validate_path_component("runtime invariant model scenarioId", scenario_id)?;
        }
        validate_runtime_invariant_world_state_ref(&self.world_state_path)?;
        if let Some(path) = &self.scenario_result_path {
            validate_scenario_result_ref(path)?;
        }
        validate_runtime_invariant_evidence_index_ref(&self.evidence_index_path)?;
        if self.invariants.is_empty() {
            return Err(anyhow!(
                "runtime invariant model invariants must not be empty"
            ));
        }
        let mut ids = BTreeSet::new();
        for (index, invariant) in self.invariants.iter().enumerate() {
            invariant.validate(index)?;
            if !ids.insert(invariant.invariant_id.as_str()) {
                return Err(anyhow!(
                    "duplicate runtime invariant model invariantId: {}",
                    invariant.invariant_id
                ));
            }
        }
        Ok(())
    }
}

impl RuntimeInvariantSpec {
    fn validate(&self, index: usize) -> Result<()> {
        validate_path_component(
            &format!("runtime invariant model invariants[{index}].invariantId"),
            &self.invariant_id,
        )?;
        validate_scenario_path(&self.target_path).with_context(|| {
            format!("runtime invariant model invariants[{index}].targetPath is invalid")
        })?;
        validate_runtime_invariant_evidence_ref(&self.evidence_path)?;
        if let Some(entity_id) = &self.required_entity_id {
            validate_path_component(
                &format!("runtime invariant model invariants[{index}].requiredEntityId"),
                entity_id,
            )?;
        }
        if let Some(path) = &self.bounds_path {
            validate_scenario_path(path).with_context(|| {
                format!("runtime invariant model invariants[{index}].boundsPath is invalid")
            })?;
        }
        if let Some(path) = &self.transition_target_path {
            validate_scenario_path(path).with_context(|| {
                format!(
                    "runtime invariant model invariants[{index}].transitionTargetPath is invalid"
                )
            })?;
        }
        if let Some(path) = &self.behavior_state_path {
            validate_scenario_path(path).with_context(|| {
                format!("runtime invariant model invariants[{index}].behaviorStatePath is invalid")
            })?;
        }
        for state in &self.allowed_states {
            validate_path_component(
                &format!("runtime invariant model invariants[{index}].allowedStates"),
                state,
            )?;
        }
        match self.invariant_type {
            RuntimeInvariantType::PlayerInBounds | RuntimeInvariantType::EntityInBounds if self.bounds_path.is_none() => Err(anyhow!("runtime invariant model invariants[{index}] in-bounds invariants require boundsPath")),
            RuntimeInvariantType::SceneTransitionValid if self.transition_target_path.is_none() => Err(anyhow!("runtime invariant model invariants[{index}] scene transition invariants require transitionTargetPath")),
            RuntimeInvariantType::BehaviorStateConsistent if self.behavior_state_path.is_none() || self.allowed_states.is_empty() => Err(anyhow!("runtime invariant model invariants[{index}] behavior state invariants require behaviorStatePath and allowedStates")),
            RuntimeInvariantType::RequiredEntityPresent if self.required_entity_id.is_none() => Err(anyhow!("runtime invariant model invariants[{index}] required entity invariants require requiredEntityId")),
            _ => Ok(()),
        }
    }
}

impl RuntimeInvariantEvidence {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let evidence: RuntimeInvariantEvidence = serde_json::from_str(input)
            .context("failed to parse Runtime Invariant Evidence JSON")?;
        evidence.validate()?;
        Ok(evidence)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != RUNTIME_INVARIANT_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!("runtime invariant evidence schemaVersion must be {RUNTIME_INVARIANT_EVIDENCE_SCHEMA_VERSION}"));
        }
        validate_path_component("runtime invariant evidence modelId", &self.model_id)?;
        validate_path_component("runtime invariant evidence runId", &self.run_id)?;
        if let Some(scenario_id) = &self.scenario_id {
            validate_path_component("runtime invariant evidence scenarioId", scenario_id)?;
        }
        validate_runtime_invariant_world_state_ref(&self.world_state_path)?;
        if let Some(path) = &self.scenario_result_path {
            validate_scenario_result_ref(path)?;
        }
        if self.checks.is_empty() {
            return Err(anyhow!(
                "runtime invariant evidence checks must not be empty"
            ));
        }
        let mut ids = BTreeSet::new();
        for (index, check) in self.checks.iter().enumerate() {
            check.validate(index)?;
            if !ids.insert(check.invariant_id.as_str()) {
                return Err(anyhow!(
                    "duplicate runtime invariant evidence invariantId: {}",
                    check.invariant_id
                ));
            }
        }
        Ok(())
    }
    pub fn summary(&self) -> RuntimeInvariantEvidenceSummary {
        let count = |status: RuntimeInvariantStatus| -> usize {
            self.checks
                .iter()
                .filter(|check| check.status == status)
                .count()
        };
        RuntimeInvariantEvidenceSummary {
            schema_version: "runtime-invariant-evidence-summary-v1".to_string(),
            model_id: self.model_id.clone(),
            run_id: self.run_id.clone(),
            scenario_id: self.scenario_id.clone(),
            check_count: self.checks.len(),
            passed_count: count(RuntimeInvariantStatus::Passed),
            failed_count: count(RuntimeInvariantStatus::Failed),
            unsupported_count: count(RuntimeInvariantStatus::Unsupported),
            missing_count: count(RuntimeInvariantStatus::Missing),
            malformed_count: count(RuntimeInvariantStatus::Malformed),
            stale_count: count(RuntimeInvariantStatus::Stale),
        }
    }
}

impl RuntimeInvariantCheck {
    fn validate(&self, index: usize) -> Result<()> {
        validate_path_component(
            &format!("runtime invariant evidence checks[{index}].invariantId"),
            &self.invariant_id,
        )?;
        validate_scenario_path(&self.target_path).with_context(|| {
            format!("runtime invariant evidence checks[{index}].targetPath is invalid")
        })?;
        if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "runtime invariant evidence checks[{index}].evidenceRefs must not be empty"
            ));
        }
        for evidence_ref in &self.evidence_refs {
            validate_runtime_invariant_evidence_ref(evidence_ref)?;
        }
        match self.status {
            RuntimeInvariantStatus::Passed if self.message.is_some() => Err(anyhow!(
                "runtime invariant evidence checks[{index}] passed status must not include message"
            )),
            RuntimeInvariantStatus::Failed
            | RuntimeInvariantStatus::Unsupported
            | RuntimeInvariantStatus::Missing
            | RuntimeInvariantStatus::Malformed
            | RuntimeInvariantStatus::Stale => {
                let Some(message) = &self.message else {
                    return Err(anyhow!(
                        "runtime invariant evidence checks[{index}] {:?} status requires message",
                        self.status
                    ));
                };
                require_bounded_display_text(
                    &format!("runtime invariant evidence checks[{index}].message"),
                    message,
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn validate_runtime_invariant_world_state_ref(reference: &str) -> Result<()> {
    validate_evidence_artifact_path(reference)?;
    if !reference.starts_with("evidence/scenarios/") || !reference.ends_with("/world-state.json") {
        return Err(anyhow!("runtime invariant worldStatePath must reference evidence/scenarios/<scenario-id>/world-state.json"));
    }
    Ok(())
}
fn validate_runtime_invariant_evidence_index_ref(reference: &str) -> Result<()> {
    validate_evidence_artifact_path(reference)?;
    if reference != "evidence/index.json" {
        return Err(anyhow!(
            "runtime invariant evidenceIndexPath must reference evidence/index.json"
        ));
    }
    Ok(())
}
pub fn validate_runtime_invariant_evidence_ref(reference: &str) -> Result<()> {
    require_text("runtime invariant evidence ref", reference)?;
    if reference.starts_with("evidence/") {
        return validate_evidence_artifact_path(reference);
    }
    if reference.starts_with("invariants/") {
        // Reject raw backslash separators and repeated path separators before the
        // component check. On Unix `Path::components()` treats a backslash as an
        // ordinary character, so a ref like `invariants/..\secret.json` would
        // otherwise pass as a single normal component yet escape the subtree in a
        // consumer that normalizes `\` to `/`. Fail closed on those raw shapes.
        if reference.contains('\\') {
            return Err(anyhow!(
                "runtime invariant evidence ref must not contain backslash separators"
            ));
        }
        if reference.contains("//") {
            return Err(anyhow!(
                "runtime invariant evidence ref must not contain repeated path separators"
            ));
        }
        let path = Path::new(reference);
        if path.is_absolute()
            || path.components().any(|component| {
                matches!(
                    component,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(anyhow!(
                "runtime invariant evidence ref must be run-relative and must not escape the run"
            ));
        }
        if !reference.ends_with(".json") {
            return Err(anyhow!(
                "runtime invariant evidence ref under invariants/ must point to a JSON artifact"
            ));
        }
        return Ok(());
    }
    Err(anyhow!(
        "runtime invariant evidence refs must point to evidence/ or invariants/"
    ))
}

pub fn write_runtime_invariant_evidence(
    run_dir: impl AsRef<Path>,
    evidence: &RuntimeInvariantEvidence,
) -> Result<EvidenceArtifact> {
    let run_dir = run_dir.as_ref();
    evidence.validate()?;
    let run = read_json_value(run_dir.join("run.json"))?;
    let current_run_id = json_string(&run, "id").unwrap_or_else(|| run_id_from_run_dir(run_dir));
    if evidence.run_id != current_run_id {
        return Err(anyhow!(
            "runtime invariant evidence runId {} is stale for current run {current_run_id}",
            evidence.run_id
        ));
    }
    let scenario_dir = evidence
        .scenario_id
        .as_deref()
        .map(|scenario_id| format!("scenarios/{scenario_id}"))
        .unwrap_or_else(|| "run".to_string());
    let artifact_path = format!(
        "evidence/{scenario_dir}/runtime-invariant-evidence-{}.json",
        evidence.model_id
    );
    if let Some(parent) = run_dir.join(&artifact_path).parent() {
        fs::create_dir_all(parent)?;
    }
    write_json(&run_dir.join(&artifact_path), &json!(evidence))?;
    let summary = evidence.summary();
    add_evidence_artifact(
        run_dir,
        &format!("runtime-invariant-evidence-{}", evidence.model_id),
        "application/json",
        &artifact_path,
        json!({
            "artifact": "runtime_invariant_evidence",
            "schemaVersion": evidence.schema_version,
            "modelId": evidence.model_id,
            "runId": evidence.run_id,
            "scenarioId": evidence.scenario_id,
            "checkCount": summary.check_count,
            "passedCount": summary.passed_count,
            "failedCount": summary.failed_count,
            "unsupportedCount": summary.unsupported_count,
            "missingCount": summary.missing_count,
            "malformedCount": summary.malformed_count,
            "staleCount": summary.stale_count,
            "boundary": "runtime invariant evidence is Rust-written validation evidence only; it does not mutate source, launch workers, execute browser commands, auto-fix, auto-apply, or auto-merge"
        }),
    )
}

pub fn evaluate_runtime_invariants(
    model: &RuntimeInvariantModel,
    world_state: &serde_json::Value,
    scenario_result: Option<&serde_json::Value>,
    recorded_at_unix_ms: u128,
) -> Result<RuntimeInvariantEvidence> {
    model.validate()?;
    let mut checks = Vec::new();
    for invariant in &model.invariants {
        checks.push(evaluate_runtime_invariant(
            model,
            invariant,
            world_state,
            scenario_result,
        ));
    }
    let evidence = RuntimeInvariantEvidence {
        schema_version: RUNTIME_INVARIANT_EVIDENCE_SCHEMA_VERSION.to_string(),
        model_id: model.model_id.clone(),
        run_id: model.run_id.clone(),
        scenario_id: model.scenario_id.clone(),
        world_state_path: model.world_state_path.clone(),
        scenario_result_path: model.scenario_result_path.clone(),
        recorded_at_unix_ms,
        checks,
    };
    evidence.validate()?;
    Ok(evidence)
}

fn evaluate_runtime_invariant(
    model: &RuntimeInvariantModel,
    invariant: &RuntimeInvariantSpec,
    world_state: &serde_json::Value,
    scenario_result: Option<&serde_json::Value>,
) -> RuntimeInvariantCheck {
    let Some(source) = runtime_invariant_source(model, invariant, world_state, scenario_result)
    else {
        return runtime_invariant_check(
            invariant,
            RuntimeInvariantStatus::Unsupported,
            None,
            Some("invariant evidencePath is not a supported world-state or scenario-result source"),
        );
    };

    let target = runtime_invariant_path_value(source, &invariant.target_path);
    if target.is_none() {
        return runtime_invariant_check(
            invariant,
            RuntimeInvariantStatus::Missing,
            None,
            Some("targetPath was not present in referenced runtime evidence"),
        );
    }
    let target = target.unwrap_or(&serde_json::Value::Null);
    let (status, observed, message) = match invariant.invariant_type {
        RuntimeInvariantType::PlayerInBounds | RuntimeInvariantType::EntityInBounds => {
            evaluate_in_bounds(source, target, invariant.bounds_path.as_deref())
        }
        RuntimeInvariantType::FiniteTransform => evaluate_finite_transform(target),
        RuntimeInvariantType::HealthNonNegative => evaluate_health_non_negative(target),
        RuntimeInvariantType::ObjectiveFlagsConsistent => {
            evaluate_objective_flags_consistent(target)
        }
        RuntimeInvariantType::SceneTransitionValid => evaluate_scene_transition_valid(
            source,
            target,
            invariant.transition_target_path.as_deref(),
        ),
        RuntimeInvariantType::NoImpossibleState => evaluate_no_impossible_state(target),
        RuntimeInvariantType::RequiredEntityPresent => {
            evaluate_required_entity_present(target, invariant.required_entity_id.as_deref())
        }
        RuntimeInvariantType::BehaviorStateConsistent => evaluate_behavior_state_consistent(
            source,
            invariant.behavior_state_path.as_deref(),
            &invariant.allowed_states,
        ),
    };
    runtime_invariant_check(invariant, status, observed, message.as_deref())
}

fn runtime_invariant_source<'a>(
    model: &RuntimeInvariantModel,
    invariant: &RuntimeInvariantSpec,
    world_state: &'a serde_json::Value,
    scenario_result: Option<&'a serde_json::Value>,
) -> Option<&'a serde_json::Value> {
    if invariant.evidence_path == model.world_state_path {
        return Some(world_state);
    }
    if invariant.evidence_path == model.scenario_result_path.as_deref().unwrap_or_default() {
        return scenario_result;
    }
    None
}

fn runtime_invariant_check(
    invariant: &RuntimeInvariantSpec,
    status: RuntimeInvariantStatus,
    observed: Option<serde_json::Value>,
    message: Option<&str>,
) -> RuntimeInvariantCheck {
    RuntimeInvariantCheck {
        invariant_id: invariant.invariant_id.clone(),
        invariant_type: invariant.invariant_type,
        status,
        target_path: invariant.target_path.clone(),
        evidence_refs: vec![invariant.evidence_path.clone()],
        observed,
        message: message.map(ToString::to_string),
    }
}

fn runtime_invariant_path_value<'a>(
    source: &'a serde_json::Value,
    path: &str,
) -> Option<&'a serde_json::Value> {
    let mut current = source;
    for segment in path.split('.') {
        current = match current {
            serde_json::Value::Object(map) => map.get(segment)?,
            serde_json::Value::Array(items) => {
                let index = segment.parse::<usize>().ok()?;
                items.get(index)?
            }
            _ => return None,
        };
    }
    Some(current)
}

fn evaluate_in_bounds(
    source: &serde_json::Value,
    target: &serde_json::Value,
    bounds_path: Option<&str>,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    let Some(bounds_path) = bounds_path else {
        return (
            RuntimeInvariantStatus::Unsupported,
            Some(target.clone()),
            Some("in-bounds invariant is missing boundsPath".to_string()),
        );
    };
    let Some(bounds) = runtime_invariant_path_value(source, bounds_path) else {
        return (
            RuntimeInvariantStatus::Missing,
            Some(target.clone()),
            Some("boundsPath was not present in referenced runtime evidence".to_string()),
        );
    };
    let Some((x, y)) = runtime_invariant_xy(target) else {
        return (
            RuntimeInvariantStatus::Malformed,
            Some(target.clone()),
            Some("targetPath did not contain finite numeric x/y values".to_string()),
        );
    };
    let Some((min_x, max_x, min_y, max_y)) = runtime_invariant_bounds(bounds) else {
        return (
            RuntimeInvariantStatus::Malformed,
            Some(bounds.clone()),
            Some(
                "boundsPath did not contain finite min/max or x/y/width/height bounds".to_string(),
            ),
        );
    };
    if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
        (
            RuntimeInvariantStatus::Passed,
            Some(json!({ "x": x, "y": y })),
            None,
        )
    } else {
        (
            RuntimeInvariantStatus::Failed,
            Some(
                json!({ "x": x, "y": y, "bounds": { "minX": min_x, "maxX": max_x, "minY": min_y, "maxY": max_y } }),
            ),
            Some("entity transform was outside configured bounds".to_string()),
        )
    }
}

fn evaluate_finite_transform(
    target: &serde_json::Value,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    let Some(object) = target.as_object() else {
        return (
            RuntimeInvariantStatus::Malformed,
            Some(target.clone()),
            Some("transform target must be an object".to_string()),
        );
    };
    let numeric_keys = ["x", "y", "z", "rotation", "scale", "scaleX", "scaleY"];
    let mut found = false;
    for key in numeric_keys {
        if let Some(value) = object.get(key) {
            found = true;
            if runtime_invariant_f64(value).is_none() {
                return (
                    RuntimeInvariantStatus::Malformed,
                    Some(target.clone()),
                    Some(format!("transform field {key} was not a finite number")),
                );
            }
        }
    }
    if found {
        (RuntimeInvariantStatus::Passed, Some(target.clone()), None)
    } else {
        (
            RuntimeInvariantStatus::Missing,
            Some(target.clone()),
            Some("transform target had no recognized numeric fields".to_string()),
        )
    }
}

fn evaluate_health_non_negative(
    target: &serde_json::Value,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    let Some(value) = runtime_invariant_f64(target) else {
        return (
            RuntimeInvariantStatus::Malformed,
            Some(target.clone()),
            Some("health target was not numeric".to_string()),
        );
    };
    if value >= 0.0 {
        (RuntimeInvariantStatus::Passed, Some(json!(value)), None)
    } else {
        (
            RuntimeInvariantStatus::Failed,
            Some(json!(value)),
            Some("health target was below zero".to_string()),
        )
    }
}

fn evaluate_objective_flags_consistent(
    target: &serde_json::Value,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    match target {
        serde_json::Value::Bool(_) => (RuntimeInvariantStatus::Passed, Some(target.clone()), None),
        serde_json::Value::Object(map) => {
            let contradiction = map.get("completed").and_then(|v| v.as_bool()) == Some(true)
                && map.get("failed").and_then(|v| v.as_bool()) == Some(true);
            if contradiction {
                (
                    RuntimeInvariantStatus::Failed,
                    Some(target.clone()),
                    Some("objective flags were both completed and failed".to_string()),
                )
            } else {
                (RuntimeInvariantStatus::Passed, Some(target.clone()), None)
            }
        }
        _ => (
            RuntimeInvariantStatus::Malformed,
            Some(target.clone()),
            Some("objective flag target must be boolean or object".to_string()),
        ),
    }
}

fn evaluate_scene_transition_valid(
    source: &serde_json::Value,
    target: &serde_json::Value,
    transition_target_path: Option<&str>,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    let Some(path) = transition_target_path else {
        return (
            RuntimeInvariantStatus::Unsupported,
            Some(target.clone()),
            Some("scene transition invariant is missing transitionTargetPath".to_string()),
        );
    };
    let Some(transition_target) = runtime_invariant_path_value(source, path) else {
        return (
            RuntimeInvariantStatus::Missing,
            Some(target.clone()),
            Some("transitionTargetPath was not present in referenced runtime evidence".to_string()),
        );
    };
    let current = target.as_str();
    let valid = match transition_target {
        serde_json::Value::String(next) => current.is_some_and(|current| current == next),
        serde_json::Value::Array(items) => {
            current.is_some_and(|current| items.iter().any(|item| item.as_str() == Some(current)))
        }
        serde_json::Value::Object(map) => current.is_some_and(|current| map.contains_key(current)),
        _ => false,
    };
    if valid {
        (
            RuntimeInvariantStatus::Passed,
            Some(json!({ "current": target, "declared": transition_target })),
            None,
        )
    } else {
        (
            RuntimeInvariantStatus::Failed,
            Some(json!({ "current": target, "declared": transition_target })),
            Some("scene transition target was not declared".to_string()),
        )
    }
}

fn evaluate_no_impossible_state(
    target: &serde_json::Value,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    match target.as_bool() {
        Some(false) => (RuntimeInvariantStatus::Passed, Some(target.clone()), None),
        Some(true) => (
            RuntimeInvariantStatus::Failed,
            Some(target.clone()),
            Some("impossible-state marker was true".to_string()),
        ),
        None => (
            RuntimeInvariantStatus::Malformed,
            Some(target.clone()),
            Some("impossible-state marker must be boolean".to_string()),
        ),
    }
}

fn evaluate_required_entity_present(
    target: &serde_json::Value,
    required_entity_id: Option<&str>,
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    let Some(required) = required_entity_id else {
        return (
            RuntimeInvariantStatus::Unsupported,
            Some(target.clone()),
            Some("required entity invariant is missing requiredEntityId".to_string()),
        );
    };
    let present = match target {
        serde_json::Value::Object(map) => {
            map.contains_key(required) || map.get("id").and_then(|v| v.as_str()) == Some(required)
        }
        serde_json::Value::Array(items) => items.iter().any(|item| {
            item.as_str() == Some(required)
                || item.get("id").and_then(|v| v.as_str()) == Some(required)
        }),
        serde_json::Value::String(value) => value == required,
        _ => false,
    };
    if present {
        (RuntimeInvariantStatus::Passed, Some(target.clone()), None)
    } else {
        (
            RuntimeInvariantStatus::Failed,
            Some(target.clone()),
            Some(format!("required entity {required} was not present")),
        )
    }
}

fn evaluate_behavior_state_consistent(
    source: &serde_json::Value,
    behavior_state_path: Option<&str>,
    allowed_states: &[String],
) -> (
    RuntimeInvariantStatus,
    Option<serde_json::Value>,
    Option<String>,
) {
    let Some(path) = behavior_state_path else {
        return (
            RuntimeInvariantStatus::Unsupported,
            None,
            Some("behavior invariant is missing behaviorStatePath".to_string()),
        );
    };
    if allowed_states.is_empty() {
        return (
            RuntimeInvariantStatus::Unsupported,
            None,
            Some("behavior invariant has no allowedStates".to_string()),
        );
    }
    let Some(state) = runtime_invariant_path_value(source, path) else {
        return (
            RuntimeInvariantStatus::Missing,
            None,
            Some("behaviorStatePath was not present in referenced runtime evidence".to_string()),
        );
    };
    let Some(state) = state.as_str() else {
        return (
            RuntimeInvariantStatus::Malformed,
            Some(state.clone()),
            Some("behavior state was not a string".to_string()),
        );
    };
    if allowed_states.iter().any(|allowed| allowed == state) {
        (RuntimeInvariantStatus::Passed, Some(json!(state)), None)
    } else {
        (
            RuntimeInvariantStatus::Failed,
            Some(json!(state)),
            Some("behavior state was outside allowedStates".to_string()),
        )
    }
}

fn runtime_invariant_xy(value: &serde_json::Value) -> Option<(f64, f64)> {
    let object = value.as_object()?;
    Some((
        runtime_invariant_f64(object.get("x")?)?,
        runtime_invariant_f64(object.get("y")?)?,
    ))
}

fn runtime_invariant_bounds(value: &serde_json::Value) -> Option<(f64, f64, f64, f64)> {
    let object = value.as_object()?;
    if let (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) = (
        object.get("minX").and_then(runtime_invariant_f64),
        object.get("maxX").and_then(runtime_invariant_f64),
        object.get("minY").and_then(runtime_invariant_f64),
        object.get("maxY").and_then(runtime_invariant_f64),
    ) {
        return Some((min_x, max_x, min_y, max_y));
    }
    let x = object.get("x").and_then(runtime_invariant_f64)?;
    let y = object.get("y").and_then(runtime_invariant_f64)?;
    let width = object.get("width").and_then(runtime_invariant_f64)?;
    let height = object.get("height").and_then(runtime_invariant_f64)?;
    Some((x, x + width, y, y + height))
}

fn runtime_invariant_f64(value: &serde_json::Value) -> Option<f64> {
    let number = value.as_f64()?;
    number.is_finite().then_some(number)
}

fn validate_scenario_path(path: &str) -> Result<()> {
    for segment in path.split('.') {
        require_text("scenario assertion path segment", segment)?;
        if !segment
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
        {
            return Err(anyhow!(
                "scenario assertion paths may only contain ASCII letters, numbers, '_', '-' and '.'"
            ));
        }
    }
    Ok(())
}

fn validate_path_component(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if !value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err(anyhow!(
            "{field} may only contain ASCII letters, numbers, '-' or '_'"
        ));
    }
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}

fn require_bounded_display_text(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.chars().count() > 120 {
        return Err(anyhow!("{field} must be 120 characters or fewer"));
    }
    if value.chars().any(|ch| ch.is_control()) {
        return Err(anyhow!("{field} must not contain control characters"));
    }
    Ok(())
}

fn validate_scenario_result_ref(reference: &str) -> Result<()> {
    validate_evidence_artifact_path(reference)?;
    if !is_scenario_result_artifact_path(reference) {
        return Err(anyhow!(
            "regression promotion scenarioResultPath must reference a scenario result under evidence/scenarios/ (scenario-result.json or scenario-result-*.json)"
        ));
    }
    Ok(())
}

fn is_scenario_result_artifact_path(reference: &str) -> bool {
    reference.starts_with("evidence/scenarios/")
        && Path::new(reference)
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| {
                name == "scenario-result.json"
                    || (name.starts_with("scenario-result-") && name.ends_with(".json"))
            })
}

fn read_json_value(path: impl AsRef<Path>) -> Result<serde_json::Value> {
    let path = path.as_ref();
    let input = fs::read_to_string(path)
        .with_context(|| format!("failed to read JSON file {}", path.display()))?;
    serde_json::from_str(&input)
        .with_context(|| format!("failed to parse JSON file {}", path.display()))
}

fn json_string(value: &serde_json::Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

fn run_id_from_run_dir(run_dir: &Path) -> String {
    run_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown-run")
        .to_string()
}

fn write_json(path: &Path, value: &serde_json::Value) -> Result<()> {
    let body = serde_json::to_string_pretty(value).context("failed to serialize JSON")?;
    fs::write(path, format!("{body}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
}

pub const VISUAL_COMPARISON_EVIDENCE_SCHEMA_VERSION: &str = "visual-comparison-evidence-v1";
const MAX_VISUAL_COMPARISON_DIMENSION: u32 = 16_384;
const MAX_VISUAL_COMPARISON_CHANGED_REGIONS: usize = 256;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualComparisonEvidenceArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "comparisonId")]
    pub comparison_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    #[serde(rename = "checkpointId")]
    pub checkpoint_id: String,
    pub before: VisualComparisonScreenshotRef,
    pub after: VisualComparisonScreenshotRef,
    pub outcome: VisualComparisonOutcome,
    #[serde(rename = "pixelDiffSummary", skip_serializing_if = "Option::is_none")]
    pub pixel_diff_summary: Option<VisualComparisonPixelDiffSummary>,
    #[serde(
        rename = "changedRegions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub changed_regions: Vec<VisualComparisonChangedRegion>,
    pub thresholds: Vec<VisualComparisonThreshold>,
    #[serde(
        rename = "metadataRefs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_refs: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(
        rename = "blockedReasons",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "unsupportedReason", skip_serializing_if = "Option::is_none")]
    pub unsupported_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualComparisonScreenshotRef {
    #[serde(rename = "screenshotRef", skip_serializing_if = "Option::is_none")]
    pub screenshot_ref: Option<String>,
    #[serde(rename = "metadataRef", skip_serializing_if = "Option::is_none")]
    pub metadata_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<VisualComparisonImageFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(rename = "missingReason", skip_serializing_if = "Option::is_none")]
    pub missing_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VisualComparisonImageFormat {
    Png,
    Jpeg,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VisualComparisonOutcome {
    Unchanged,
    Changed,
    MissingScreenshot,
    MalformedScreenshot,
    MismatchedDimensions,
    Unsupported,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualComparisonPixelDiffSummary {
    #[serde(rename = "totalPixels")]
    pub total_pixels: u64,
    #[serde(rename = "changedPixels")]
    pub changed_pixels: u64,
    #[serde(rename = "changedPercentX1000")]
    pub changed_percent_x1000: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualComparisonChangedRegion {
    #[serde(rename = "regionId")]
    pub region_id: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "changedPixels")]
    pub changed_pixels: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VisualComparisonThreshold {
    #[serde(rename = "thresholdId")]
    pub threshold_id: String,
    #[serde(rename = "maxChangedPixels")]
    pub max_changed_pixels: u64,
    #[serde(rename = "maxChangedPercentX1000")]
    pub max_changed_percent_x1000: u32,
}

impl VisualComparisonEvidenceArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: VisualComparisonEvidenceArtifact = serde_json::from_str(input)
            .context("failed to parse Visual Comparison Evidence JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != VISUAL_COMPARISON_EVIDENCE_SCHEMA_VERSION {
            return Err(anyhow!(
                "visual comparison evidence schemaVersion must be {VISUAL_COMPARISON_EVIDENCE_SCHEMA_VERSION}"
            ));
        }
        validate_path_component("visual comparison comparisonId", &self.comparison_id)?;
        validate_path_component("visual comparison runId", &self.run_id)?;
        validate_path_component("visual comparison scenarioId", &self.scenario_id)?;
        validate_path_component("visual comparison checkpointId", &self.checkpoint_id)?;
        self.before.validate("visual comparison before")?;
        self.after.validate("visual comparison after")?;
        if self.evidence_refs.is_empty() {
            return Err(anyhow!("visual comparison evidenceRefs must not be empty"));
        }
        for reference in &self.evidence_refs {
            validate_evidence_artifact_path(reference)?;
        }
        for reference in &self.metadata_refs {
            validate_evidence_artifact_path(reference)?;
            if !reference.ends_with(".json") {
                return Err(anyhow!(
                    "visual comparison metadataRefs must be JSON evidence"
                ));
            }
        }
        if self.thresholds.is_empty()
            && !matches!(
                self.outcome,
                VisualComparisonOutcome::MissingScreenshot
                    | VisualComparisonOutcome::Unsupported
                    | VisualComparisonOutcome::Blocked
            )
        {
            return Err(anyhow!(
                "visual comparison thresholds are required for comparable outcomes"
            ));
        }
        let mut threshold_ids = BTreeSet::new();
        for threshold in &self.thresholds {
            threshold.validate()?;
            if !threshold_ids.insert(threshold.threshold_id.as_str()) {
                return Err(anyhow!(
                    "duplicate visual comparison thresholdId: {}",
                    threshold.threshold_id
                ));
            }
        }
        if self.changed_regions.len() > MAX_VISUAL_COMPARISON_CHANGED_REGIONS {
            return Err(anyhow!(
                "visual comparison changedRegions exceeds limit of {MAX_VISUAL_COMPARISON_CHANGED_REGIONS}"
            ));
        }
        let mut region_ids = BTreeSet::new();
        for region in &self.changed_regions {
            region.validate()?;
            if !region_ids.insert(region.region_id.as_str()) {
                return Err(anyhow!(
                    "duplicate visual comparison changed regionId: {}",
                    region.region_id
                ));
            }
        }
        if let Some(summary) = &self.pixel_diff_summary {
            summary.validate()?;
        }
        for reason in &self.blocked_reasons {
            require_bounded_display_text("visual comparison blockedReasons", reason)?;
        }
        if let Some(reason) = &self.unsupported_reason {
            require_bounded_display_text("visual comparison unsupportedReason", reason)?;
        }
        for guardrail in &self.guardrails {
            require_bounded_display_text("visual comparison guardrail", guardrail)?;
        }
        self.validate_outcome_consistency()
    }

    fn validate_threshold_result(&self) -> Result<()> {
        let Some(summary) = &self.pixel_diff_summary else {
            return Ok(());
        };
        if self.thresholds.is_empty() {
            return Ok(());
        }
        let threshold_passed = self.thresholds.iter().all(|threshold| {
            summary.changed_pixels <= threshold.max_changed_pixels
                && summary.changed_percent_x1000 <= threshold.max_changed_percent_x1000
        });
        match self.outcome {
            VisualComparisonOutcome::Unchanged if !threshold_passed => Err(anyhow!(
                "visual comparison unchanged outcome exceeds configured thresholds"
            )),
            VisualComparisonOutcome::Changed if threshold_passed => Err(anyhow!(
                "visual comparison changed outcome must exceed at least one configured threshold"
            )),
            _ => Ok(()),
        }
    }

    fn validate_outcome_consistency(&self) -> Result<()> {
        match self.outcome {
            VisualComparisonOutcome::Unchanged => {
                let summary = self.pixel_diff_summary.as_ref().ok_or_else(|| {
                    anyhow!("visual comparison unchanged outcome requires pixelDiffSummary")
                })?;
                if summary.changed_pixels != 0 || !self.changed_regions.is_empty() {
                    return Err(anyhow!(
                        "visual comparison unchanged outcome must not include changed pixels or regions"
                    ));
                }
            }
            VisualComparisonOutcome::Changed => {
                let summary = self.pixel_diff_summary.as_ref().ok_or_else(|| {
                    anyhow!("visual comparison changed outcome requires pixelDiffSummary")
                })?;
                if summary.changed_pixels == 0 || self.changed_regions.is_empty() {
                    return Err(anyhow!(
                        "visual comparison changed outcome requires changed pixels and changedRegions"
                    ));
                }
            }
            VisualComparisonOutcome::MissingScreenshot => {
                if !self.before.is_missing() && !self.after.is_missing() {
                    return Err(anyhow!(
                        "visual comparison missing_screenshot outcome requires a missing screenshot ref"
                    ));
                }
            }
            VisualComparisonOutcome::MismatchedDimensions => {
                if self.before.dimensions() == self.after.dimensions() {
                    return Err(anyhow!(
                        "visual comparison mismatched_dimensions outcome requires differing dimensions"
                    ));
                }
            }
            VisualComparisonOutcome::Unsupported => {
                if self.unsupported_reason.is_none() {
                    return Err(anyhow!(
                        "visual comparison unsupported outcome requires unsupportedReason"
                    ));
                }
            }
            VisualComparisonOutcome::Blocked => {
                if self.blocked_reasons.is_empty() {
                    return Err(anyhow!(
                        "visual comparison blocked outcome requires blockedReasons"
                    ));
                }
            }
            VisualComparisonOutcome::MalformedScreenshot => {}
        }
        Ok(())
    }
}

pub fn validate_visual_comparison_evidence_refs(
    run_dir: impl AsRef<Path>,
    comparison: &VisualComparisonEvidenceArtifact,
) -> Result<()> {
    let run_dir = run_dir.as_ref();
    comparison.validate()?;
    comparison.validate_threshold_result()?;
    let index = read_evidence_index(run_dir)?;
    let indexed_paths = index
        .artifacts
        .iter()
        .map(|artifact| artifact.path.as_str())
        .collect::<BTreeSet<_>>();
    let mut references = Vec::new();
    references.extend(comparison.evidence_refs.iter().map(String::as_str));
    references.extend(comparison.metadata_refs.iter().map(String::as_str));
    references.extend(comparison.before.screenshot_ref.as_deref());
    references.extend(comparison.after.screenshot_ref.as_deref());
    references.extend(comparison.before.metadata_ref.as_deref());
    references.extend(comparison.after.metadata_ref.as_deref());
    references.sort();
    references.dedup();
    for reference in references {
        if !indexed_paths.contains(reference) {
            return Err(anyhow!(
                "visual comparison reference is missing from evidence index: {reference}"
            ));
        }
        validate_visual_comparison_indexed_reference(run_dir, comparison, reference)?;
    }
    Ok(())
}

fn validate_visual_comparison_indexed_reference(
    run_dir: &Path,
    comparison: &VisualComparisonEvidenceArtifact,
    reference: &str,
) -> Result<()> {
    if reference.ends_with(".json") {
        let value = read_json_value(run_dir.join(reference)).with_context(|| {
            format!("visual comparison metadata reference is unreadable: {reference}")
        })?;
        validate_visual_comparison_reference_freshness(comparison, reference, &value)?;
        if Some(reference) == comparison.before.metadata_ref.as_deref() {
            comparison.before.validate_metadata_dimensions(
                "visual comparison before",
                reference,
                &value,
            )?;
        }
        if Some(reference) == comparison.after.metadata_ref.as_deref() {
            comparison.after.validate_metadata_dimensions(
                "visual comparison after",
                reference,
                &value,
            )?;
        }
    }
    Ok(())
}

fn validate_visual_comparison_reference_freshness(
    comparison: &VisualComparisonEvidenceArtifact,
    reference: &str,
    value: &serde_json::Value,
) -> Result<()> {
    if let Some(run_id) = json_string(value, "runId").or_else(|| json_string(value, "run_id")) {
        if run_id != comparison.run_id {
            return Err(anyhow!(
                "visual comparison reference is stale for runId {run_id}; expected {} at {reference}",
                comparison.run_id
            ));
        }
    }
    if let Some(scenario_id) =
        json_string(value, "scenarioId").or_else(|| json_string(value, "scenario_id"))
    {
        if scenario_id != comparison.scenario_id {
            return Err(anyhow!(
                "visual comparison reference scenarioId drift at {reference}: {scenario_id} != {}",
                comparison.scenario_id
            ));
        }
    }
    if let Some(checkpoint_id) =
        json_string(value, "checkpointId").or_else(|| json_string(value, "checkpoint_id"))
    {
        if checkpoint_id != comparison.checkpoint_id {
            return Err(anyhow!(
                "visual comparison reference checkpointId drift at {reference}: {checkpoint_id} != {}",
                comparison.checkpoint_id
            ));
        }
    }
    Ok(())
}

impl VisualComparisonScreenshotRef {
    fn validate(&self, field: &str) -> Result<()> {
        match (&self.screenshot_ref, &self.missing_reason) {
            (Some(reference), None) => {
                validate_evidence_artifact_path(reference)?;
                if !(reference.ends_with(".png")
                    || reference.ends_with(".jpg")
                    || reference.ends_with(".jpeg"))
                {
                    return Err(anyhow!(
                        "{field}.screenshotRef must be PNG or JPEG evidence"
                    ));
                }
                if self.format.is_none() || self.width.is_none() || self.height.is_none() {
                    return Err(anyhow!(
                        "{field} present screenshot requires format, width, and height"
                    ));
                }
                let (width, height) = self.dimensions().unwrap_or_default();
                if width == 0
                    || height == 0
                    || width > MAX_VISUAL_COMPARISON_DIMENSION
                    || height > MAX_VISUAL_COMPARISON_DIMENSION
                {
                    return Err(anyhow!(
                        "{field} dimensions must be between 1 and {MAX_VISUAL_COMPARISON_DIMENSION}"
                    ));
                }
            }
            (None, Some(reason)) => {
                require_bounded_display_text(&format!("{field}.missingReason"), reason)?;
            }
            (Some(_), Some(_)) => {
                return Err(anyhow!(
                    "{field} must not include missingReason when screenshotRef is present"
                ));
            }
            (None, None) => {
                return Err(anyhow!("{field} requires screenshotRef or missingReason"));
            }
        }
        if let Some(reference) = &self.metadata_ref {
            validate_evidence_artifact_path(reference)?;
            if !reference.ends_with(".json") {
                return Err(anyhow!("{field}.metadataRef must be JSON evidence"));
            }
        }
        Ok(())
    }

    fn validate_metadata_dimensions(
        &self,
        field: &str,
        reference: &str,
        value: &serde_json::Value,
    ) -> Result<()> {
        let width = value
            .get("width")
            .or_else(|| value.get("imageWidth"))
            .or_else(|| value.get("image_width"))
            .and_then(|value| value.as_u64())
            .and_then(|value| u32::try_from(value).ok());
        let height = value
            .get("height")
            .or_else(|| value.get("imageHeight"))
            .or_else(|| value.get("image_height"))
            .and_then(|value| value.as_u64())
            .and_then(|value| u32::try_from(value).ok());
        if let Some((expected_width, expected_height)) = self.dimensions() {
            if width != Some(expected_width) || height != Some(expected_height) {
                return Err(anyhow!(
                    "{field}.metadataRef dimensions are stale at {reference}"
                ));
            }
        }
        if let Some(format) = value
            .get("format")
            .or_else(|| value.get("imageFormat"))
            .or_else(|| value.get("image_format"))
            .and_then(|value| value.as_str())
        {
            let expected = match self.format {
                Some(VisualComparisonImageFormat::Png) => "png",
                Some(VisualComparisonImageFormat::Jpeg) => "jpeg",
                None => format,
            };
            let normalized = format.trim().to_ascii_lowercase();
            if normalized != expected && !(expected == "jpeg" && normalized == "jpg") {
                return Err(anyhow!(
                    "{field}.metadataRef format is stale at {reference}: {format} != {expected}"
                ));
            }
        }
        Ok(())
    }

    pub fn is_missing(&self) -> bool {
        self.screenshot_ref.is_none() && self.missing_reason.is_some()
    }

    pub fn dimensions(&self) -> Option<(u32, u32)> {
        Some((self.width?, self.height?))
    }
}

impl VisualComparisonPixelDiffSummary {
    fn validate(&self) -> Result<()> {
        if self.total_pixels == 0 || self.changed_pixels > self.total_pixels {
            return Err(anyhow!(
                "visual comparison pixelDiffSummary changedPixels must be <= totalPixels"
            ));
        }
        if self.changed_percent_x1000 > 100_000 {
            return Err(anyhow!(
                "visual comparison pixelDiffSummary changedPercentX1000 must be <= 100000"
            ));
        }
        Ok(())
    }
}

impl VisualComparisonChangedRegion {
    fn validate(&self) -> Result<()> {
        validate_path_component("visual comparison changedRegions.regionId", &self.region_id)?;
        if self.width == 0 || self.height == 0 || self.changed_pixels == 0 {
            return Err(anyhow!(
                "visual comparison changedRegions must have nonzero width, height, and changedPixels"
            ));
        }
        Ok(())
    }
}

impl VisualComparisonThreshold {
    fn validate(&self) -> Result<()> {
        validate_path_component(
            "visual comparison thresholds.thresholdId",
            &self.threshold_id,
        )?;
        if self.max_changed_percent_x1000 > 100_000 {
            return Err(anyhow!(
                "visual comparison thresholds.maxChangedPercentX1000 must be <= 100000"
            ));
        }
        Ok(())
    }
}

pub fn evaluate_visual_gate(
    run_dir: &Path,
    evidence: &EvidenceIndex,
    failures: &mut Vec<serde_json::Value>,
    evidence_refs: &mut Vec<String>,
) -> Result<Vec<VisualGateVerdict>> {
    let mut verdicts = Vec::new();
    for artifact in evidence.artifacts.iter().filter(|artifact| {
        artifact
            .metadata
            .get("artifact")
            .and_then(|value| value.as_str())
            == Some("visual_comparison_evidence")
            && (artifact
                .metadata
                .get("declaredAcceptance")
                .or_else(|| artifact.metadata.get("declared_acceptance"))
                .and_then(|value| value.as_bool())
                == Some(true)
                || artifact
                    .metadata
                    .get("gate")
                    .and_then(|value| value.as_str())
                    == Some("visual"))
    }) {
        let verdict = evaluate_declared_visual_comparison(run_dir, artifact)?;
        if !matches!(verdict.state, VisualGateState::Pass) {
            failures.push(json!({
                "kind": "visual_gate_failed",
                "scenario_id": verdict.scenario_id,
                "checkpoint_id": verdict.checkpoint_id,
                "state": verdict.state,
                "path": verdict.comparison_ref,
                "reason": verdict.reason,
                "changed_pixels": verdict.changed_pixels,
                "changed_percent_x1000": verdict.changed_percent_x1000,
                "changed_region_count": verdict.changed_region_count,
                "threshold_summary": verdict.threshold_summary
            }));
        }
        if !evidence_refs
            .iter()
            .any(|reference| reference == &verdict.comparison_ref)
        {
            evidence_refs.push(verdict.comparison_ref.clone());
        }
        verdicts.push(verdict);
    }
    Ok(verdicts)
}

pub fn evaluate_declared_visual_comparison(
    run_dir: &Path,
    artifact: &EvidenceArtifact,
) -> Result<VisualGateVerdict> {
    let path = artifact.path.clone();
    let value = read_json_value(run_dir.join(&path)).with_context(|| {
        format!(
            "failed to read declared visual comparison {}",
            artifact.path
        )
    })?;
    let scenario_id = json_string(&value, "scenarioId")
        .or_else(|| json_string(&value, "scenario_id"))
        .unwrap_or_else(|| {
            artifact
                .metadata
                .get("scenarioId")
                .or_else(|| artifact.metadata.get("scenario_id"))
                .and_then(|value| value.as_str())
                .unwrap_or("unknown-scenario")
                .to_string()
        });
    let checkpoint_id = json_string(&value, "checkpointId")
        .or_else(|| json_string(&value, "checkpoint_id"))
        .unwrap_or_else(|| {
            artifact
                .metadata
                .get("checkpointId")
                .or_else(|| artifact.metadata.get("checkpoint_id"))
                .and_then(|value| value.as_str())
                .unwrap_or("unknown-checkpoint")
                .to_string()
        });

    let comparison = match serde_json::from_value::<VisualComparisonEvidenceArtifact>(value.clone())
        .context("failed to parse Visual Comparison Evidence JSON")
        .and_then(|comparison| {
            comparison.validate()?;
            Ok(comparison)
        }) {
        Ok(comparison) => comparison,
        Err(error) => {
            let reason = error.to_string();
            let state = classify_visual_gate_parse_error(&reason);
            return Ok(VisualGateVerdictParts {
                scenario_id,
                checkpoint_id,
                state,
                reason,
                comparison_ref: path,
                changed_pixels: None,
                changed_percent_x1000: None,
                changed_region_count: 0,
                threshold_summary: Vec::new(),
                evidence_refs: Vec::new(),
            }
            .into_verdict());
        }
    };

    let summary = comparison.pixel_diff_summary.as_ref();
    let changed_pixels = summary.map(|summary| summary.changed_pixels);
    let changed_percent_x1000 = summary.map(|summary| summary.changed_percent_x1000);
    let threshold_summary = comparison
        .thresholds
        .iter()
        .map(|threshold| {
            format!(
                "{} <= {} px and {} x1000",
                threshold.threshold_id,
                threshold.max_changed_pixels,
                threshold.max_changed_percent_x1000
            )
        })
        .collect::<Vec<_>>();
    let evidence_refs = visual_gate_evidence_refs(&comparison);
    let changed_region_count = comparison.changed_regions.len();

    let (state, reason) = if comparison.before.is_missing() {
        (
            VisualGateState::MissingBaseline,
            format!("baseline screenshot missing for declared visual gate at {path}"),
        )
    } else if comparison.after.is_missing() {
        (
            VisualGateState::MissingScreenshot,
            format!("actual screenshot missing for declared visual gate at {path}"),
        )
    } else if comparison.thresholds.is_empty() {
        (
            VisualGateState::ThresholdNotDeclared,
            format!("declared visual gate at {path} does not declare a threshold"),
        )
    } else if let Err(error) = validate_visual_comparison_evidence_refs(run_dir, &comparison) {
        let reason = error.to_string();
        if reason.contains("changed outcome must exceed") {
            (
                VisualGateState::Pass,
                visual_gate_threshold_reason("visual gate passed", &path, &comparison),
            )
        } else {
            let state = if reason.contains("threshold") {
                VisualGateState::ThresholdNotDeclared
            } else if reason.contains("format") || reason.contains("PNG") || reason.contains("JPEG")
            {
                VisualGateState::UnsupportedFormat
            } else {
                VisualGateState::StaleRef
            };
            (state, reason)
        }
    } else {
        match comparison.outcome {
            VisualComparisonOutcome::Unchanged => (
                VisualGateState::Pass,
                visual_gate_threshold_reason("visual gate passed", &path, &comparison),
            ),
            VisualComparisonOutcome::Changed | VisualComparisonOutcome::MismatchedDimensions => (
                VisualGateState::Fail,
                visual_gate_threshold_reason("visual gate failed", &path, &comparison),
            ),
            VisualComparisonOutcome::MissingScreenshot => {
                if comparison.before.is_missing() {
                    (
                        VisualGateState::MissingBaseline,
                        format!("baseline screenshot missing for declared visual gate at {path}"),
                    )
                } else {
                    (
                        VisualGateState::MissingScreenshot,
                        format!("actual screenshot missing for declared visual gate at {path}"),
                    )
                }
            }
            VisualComparisonOutcome::MalformedScreenshot | VisualComparisonOutcome::Unsupported => (
                VisualGateState::UnsupportedFormat,
                format!("declared visual gate at {path} has unsupported or malformed screenshot evidence"),
            ),
            VisualComparisonOutcome::Blocked => (
                VisualGateState::Fail,
                format!("declared visual gate at {path} is blocked: {}", comparison.blocked_reasons.join("; ")),
            ),
        }
    };

    Ok(VisualGateVerdictParts {
        scenario_id: comparison.scenario_id.clone(),
        checkpoint_id: comparison.checkpoint_id.clone(),
        state,
        reason,
        comparison_ref: path,
        changed_pixels,
        changed_percent_x1000,
        changed_region_count,
        threshold_summary,
        evidence_refs,
    }
    .into_verdict())
}

fn classify_visual_gate_parse_error(reason: &str) -> VisualGateState {
    if reason.contains("threshold") {
        VisualGateState::ThresholdNotDeclared
    } else if reason.contains("screenshot")
        || reason.contains("PNG")
        || reason.contains("JPEG")
        || reason.contains("format")
        || reason.contains("dimensions")
    {
        VisualGateState::UnsupportedFormat
    } else {
        VisualGateState::StaleRef
    }
}

struct VisualGateVerdictParts {
    scenario_id: String,
    checkpoint_id: String,
    reason: String,
    state: VisualGateState,
    comparison_ref: String,
    changed_pixels: Option<u64>,
    changed_percent_x1000: Option<u32>,
    changed_region_count: usize,
    threshold_summary: Vec<String>,
    evidence_refs: Vec<String>,
}

impl VisualGateVerdictParts {
    fn into_verdict(self) -> VisualGateVerdict {
        VisualGateVerdict {
            scenario_id: self.scenario_id,
            checkpoint_id: self.checkpoint_id,
            state: self.state,
            reason: self.reason,
            output_root: visual_gate_output_root(&self.comparison_ref),
            comparison_ref: self.comparison_ref,
            changed_pixels: self.changed_pixels,
            changed_percent_x1000: self.changed_percent_x1000,
            changed_region_count: self.changed_region_count,
            threshold_summary: self.threshold_summary,
            evidence_refs: self.evidence_refs,
        }
    }
}

fn visual_gate_output_root(path: &str) -> String {
    let parent = Path::new(path)
        .parent()
        .map(|parent| parent.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| "evidence/visual".to_string());
    if parent.starts_with("evidence/") {
        parent
    } else {
        "evidence/visual".to_string()
    }
}

fn visual_gate_threshold_reason(
    prefix: &str,
    path: &str,
    comparison: &VisualComparisonEvidenceArtifact,
) -> String {
    let summary = comparison
        .pixel_diff_summary
        .as_ref()
        .map(|summary| {
            format!(
                "{} changed pixels ({} x1000)",
                summary.changed_pixels, summary.changed_percent_x1000
            )
        })
        .unwrap_or_else(|| "no pixel diff summary".to_string());
    let threshold = comparison
        .thresholds
        .iter()
        .map(|threshold| {
            format!(
                "{} <= {} px/{} x1000",
                threshold.threshold_id,
                threshold.max_changed_pixels,
                threshold.max_changed_percent_x1000
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{prefix}: {path}; {summary}; {} changed region(s); threshold(s): {threshold}",
        comparison.changed_regions.len()
    )
}

fn visual_gate_evidence_refs(comparison: &VisualComparisonEvidenceArtifact) -> Vec<String> {
    let mut refs = Vec::new();
    refs.extend(comparison.evidence_refs.iter().cloned());
    refs.extend(comparison.metadata_refs.iter().cloned());
    refs.extend(comparison.before.screenshot_ref.iter().cloned());
    refs.extend(comparison.after.screenshot_ref.iter().cloned());
    refs.extend(comparison.before.metadata_ref.iter().cloned());
    refs.extend(comparison.after.metadata_ref.iter().cloned());
    refs.sort();
    refs.dedup();
    refs
}
