use serde::{Deserialize, Serialize};

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
