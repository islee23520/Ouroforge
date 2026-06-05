use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_PROTOTYPE_DRAFT_BUNDLE_SCHEMA_VERSION: &str = "gdd-prototype-draft-bundle-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeDraftBundleStatus {
    Ready,
    Incomplete,
    Stale,
    Blocked,
    Unsupported,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeBundleComponentKind {
    Requirements,
    Feasibility,
    Scaffold,
    SceneLevel,
    Behavior,
    Assets,
    Scenarios,
    TaskGraph,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddPrototypeBundleValidationStatus {
    Pass,
    Missing,
    Stale,
    Unsupported,
    Blocked,
    Contradictory,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeDraftBundleArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub status: GddPrototypeDraftBundleStatus,
    #[serde(rename = "gddRef")]
    pub gdd_ref: String,
    #[serde(rename = "requirementExtractionRef")]
    pub requirement_extraction_ref: String,
    #[serde(rename = "feasibilityGateRef")]
    pub feasibility_gate_ref: String,
    #[serde(rename = "scaffoldPlanRef")]
    pub scaffold_plan_ref: String,
    #[serde(rename = "sceneLevelPlanRef")]
    pub scene_level_plan_ref: String,
    #[serde(rename = "behaviorPlanRef")]
    pub behavior_plan_ref: String,
    #[serde(rename = "assetPlanRef")]
    pub asset_plan_ref: String,
    #[serde(rename = "scenarioDraftRefs")]
    pub scenario_draft_refs: Vec<String>,
    #[serde(rename = "taskGraphRef")]
    pub task_graph_ref: String,
    #[serde(rename = "expectedEvidenceRefs")]
    pub expected_evidence_refs: Vec<String>,
    #[serde(rename = "sourceNoteRefs")]
    pub source_note_refs: Vec<String>,
    #[serde(rename = "targetHashes")]
    pub target_hashes: Vec<GddPrototypeTargetHash>,
    pub components: Vec<GddPrototypeBundleComponent>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeTargetHash {
    #[serde(rename = "targetRef")]
    pub target_ref: String,
    pub hash: String,
    pub stale: bool,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeBundleComponent {
    pub kind: GddPrototypeBundleComponentKind,
    #[serde(rename = "artifactRef")]
    pub artifact_ref: String,
    #[serde(rename = "validationStatus")]
    pub validation_status: GddPrototypeBundleValidationStatus,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddPrototypeDraftBundleReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    pub status: String,
    #[serde(rename = "componentCount")]
    pub component_count: usize,
    #[serde(rename = "blockedComponentCount")]
    pub blocked_component_count: usize,
    #[serde(rename = "staleTargetCount")]
    pub stale_target_count: usize,
    #[serde(rename = "scenarioDraftCount")]
    pub scenario_draft_count: usize,
    #[serde(rename = "expectedEvidenceCount")]
    pub expected_evidence_count: usize,
    #[serde(rename = "componentStatusCounts")]
    pub component_status_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddPrototypeDraftBundleArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Prototype Draft Bundle JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddPrototypeDraftBundleReadModel {
        let mut component_status_counts = BTreeMap::new();
        let mut blocked_component_count = 0;
        for component in &self.components {
            *component_status_counts
                .entry(validation_status_label(&component.validation_status).to_string())
                .or_insert(0) += 1;
            if component.validation_status != GddPrototypeBundleValidationStatus::Pass
                || !component.blocked_reasons.is_empty()
            {
                blocked_component_count += 1;
            }
        }
        GddPrototypeDraftBundleReadModel {
            schema_version: self.schema_version.clone(),
            bundle_id: self.bundle_id.clone(),
            status: bundle_status_label(&self.status).to_string(),
            component_count: self.components.len(),
            blocked_component_count,
            stale_target_count: self.target_hashes.iter().filter(|target| target.stale).count(),
            scenario_draft_count: self.scenario_draft_refs.len(),
            expected_evidence_count: self.expected_evidence_refs.len(),
            component_status_counts,
            validation_summary: vec![
                "prototype draft bundle is a review surface and does not apply trusted writes".to_string(),
                "requirements, feasibility, scaffold, plans, scenarios, task graph, evidence, and hashes stay linked explicitly".to_string(),
                "missing, stale, unsupported, contradictory, unsafe, and source-note gaps fail closed".to_string(),
            ],
            compatibility_notes: vec![
                "composes existing GDD requirement, mechanics, feasibility, scaffold, scene, behavior, asset, scenario, task graph, and evidence contracts by reference".to_string(),
                "display-only read model; browser/dashboard/Studio consumers remain read-only or draft-only".to_string(),
                "generated prototype bundles remain untrusted and fixture-scoped unless later review-gated apply accepts them".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD prototype draft bundle read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_PROTOTYPE_DRAFT_BUNDLE_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD prototype draft bundle schemaVersion must be {GDD_PROTOTYPE_DRAFT_BUNDLE_SCHEMA_VERSION}"
            ));
        }
        require_local_id("GDD prototype draft bundle bundleId", &self.bundle_id)?;
        for (field, value) in [
            ("GDD prototype draft bundle gddRef", &self.gdd_ref),
            (
                "GDD prototype draft bundle requirementExtractionRef",
                &self.requirement_extraction_ref,
            ),
            (
                "GDD prototype draft bundle feasibilityGateRef",
                &self.feasibility_gate_ref,
            ),
            (
                "GDD prototype draft bundle scaffoldPlanRef",
                &self.scaffold_plan_ref,
            ),
            (
                "GDD prototype draft bundle sceneLevelPlanRef",
                &self.scene_level_plan_ref,
            ),
            (
                "GDD prototype draft bundle behaviorPlanRef",
                &self.behavior_plan_ref,
            ),
            (
                "GDD prototype draft bundle assetPlanRef",
                &self.asset_plan_ref,
            ),
            (
                "GDD prototype draft bundle taskGraphRef",
                &self.task_graph_ref,
            ),
        ] {
            require_local_ref(field, value)?;
        }
        validate_local_ref_list(
            "GDD prototype draft bundle scenarioDraftRefs",
            &self.scenario_draft_refs,
            true,
        )?;
        if self.scenario_draft_refs.len() > 8 {
            return Err(anyhow!(
                "GDD prototype draft bundle scenarioDraftRefs are overbroad for v1"
            ));
        }
        validate_local_ref_list(
            "GDD prototype draft bundle expectedEvidenceRefs",
            &self.expected_evidence_refs,
            true,
        )?;
        validate_local_ref_list(
            "GDD prototype draft bundle sourceNoteRefs",
            &self.source_note_refs,
            true,
        )?;
        require_nonempty(
            "GDD prototype draft bundle targetHashes",
            self.target_hashes.len(),
        )?;
        if self.target_hashes.len() > 12 {
            return Err(anyhow!(
                "GDD prototype draft bundle targetHashes are overbroad for v1"
            ));
        }
        let mut target_refs = BTreeSet::new();
        for target_hash in &self.target_hashes {
            target_hash.validate()?;
            if !target_refs.insert(target_hash.target_ref.clone()) {
                return Err(anyhow!(
                    "GDD prototype draft bundle targetRef `{}` is duplicated",
                    target_hash.target_ref
                ));
            }
        }
        self.validate_components()?;
        validate_string_list(
            "GDD prototype draft bundle blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        let has_blocked_component = self.components.iter().any(|component| {
            component.validation_status != GddPrototypeBundleValidationStatus::Pass
                || !component.blocked_reasons.is_empty()
        });
        let has_stale_target = self.target_hashes.iter().any(|target| target.stale);
        match self.status {
            GddPrototypeDraftBundleStatus::Ready => {
                if has_blocked_component || has_stale_target || !self.blocked_reasons.is_empty() {
                    return Err(anyhow!("ready GDD prototype draft bundle must not include blocked, stale, unsupported, missing, or contradictory plans"));
                }
            }
            GddPrototypeDraftBundleStatus::Incomplete
            | GddPrototypeDraftBundleStatus::Stale
            | GddPrototypeDraftBundleStatus::Blocked
            | GddPrototypeDraftBundleStatus::Unsupported => {
                if !has_blocked_component && !has_stale_target && self.blocked_reasons.is_empty() {
                    return Err(anyhow!("non-ready GDD prototype draft bundle requires visible blockedReasons, stale targets, or blocked components"));
                }
            }
        }
        require_text("GDD prototype draft bundle boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "review surface only",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no direct trusted writes",
            "no autonomous unrestricted game creation",
            "browser read-only or draft-only",
            "no asset generation",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD prototype draft bundle boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }

    fn validate_components(&self) -> Result<()> {
        require_nonempty(
            "GDD prototype draft bundle components",
            self.components.len(),
        )?;
        if self.components.len() > 12 {
            return Err(anyhow!(
                "GDD prototype draft bundle components are overbroad for v1"
            ));
        }
        let mut kinds = BTreeSet::new();
        for component in &self.components {
            component.validate()?;
            if !kinds.insert(component.kind.clone()) {
                return Err(anyhow!(
                    "GDD prototype draft bundle component `{}` is duplicated",
                    component_kind_label(&component.kind)
                ));
            }
        }
        for required in [
            GddPrototypeBundleComponentKind::Requirements,
            GddPrototypeBundleComponentKind::Feasibility,
            GddPrototypeBundleComponentKind::Scaffold,
            GddPrototypeBundleComponentKind::SceneLevel,
            GddPrototypeBundleComponentKind::Behavior,
            GddPrototypeBundleComponentKind::Assets,
            GddPrototypeBundleComponentKind::Scenarios,
            GddPrototypeBundleComponentKind::TaskGraph,
        ] {
            if !kinds.contains(&required) {
                return Err(anyhow!(
                    "GDD prototype draft bundle missing required component `{}`",
                    component_kind_label(&required)
                ));
            }
        }
        if self.source_note_refs.is_empty()
            && self.components.iter().any(|component| {
                component.kind == GddPrototypeBundleComponentKind::Assets
                    && component.validation_status == GddPrototypeBundleValidationStatus::Pass
            })
        {
            return Err(anyhow!(
                "GDD prototype draft bundle asset/source notes are required for asset plans"
            ));
        }
        Ok(())
    }
}

impl GddPrototypeTargetHash {
    fn validate(&self) -> Result<()> {
        require_local_ref(
            "GDD prototype draft bundle targetHashes.targetRef",
            &self.target_ref,
        )?;
        if self.hash.len() != 64 || !self.hash.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(anyhow!(
                "GDD prototype draft bundle targetHashes.hash must be a sha256 hex digest"
            ));
        }
        validate_string_list(
            "GDD prototype draft bundle targetHashes.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.stale && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD prototype draft bundle stale target `{}` must include blockedReasons",
                self.target_ref
            ));
        }
        Ok(())
    }
}

impl GddPrototypeBundleComponent {
    fn validate(&self) -> Result<()> {
        require_local_ref(
            "GDD prototype draft bundle components.artifactRef",
            &self.artifact_ref,
        )?;
        validate_string_list(
            "GDD prototype draft bundle components.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.validation_status != GddPrototypeBundleValidationStatus::Pass
            && self.blocked_reasons.is_empty()
        {
            return Err(anyhow!(
                "GDD prototype draft bundle component `{}` with status `{}` must include blockedReasons",
                component_kind_label(&self.kind),
                validation_status_label(&self.validation_status)
            ));
        }
        Ok(())
    }
}

fn bundle_status_label(status: &GddPrototypeDraftBundleStatus) -> &'static str {
    match status {
        GddPrototypeDraftBundleStatus::Ready => "ready",
        GddPrototypeDraftBundleStatus::Incomplete => "incomplete",
        GddPrototypeDraftBundleStatus::Stale => "stale",
        GddPrototypeDraftBundleStatus::Blocked => "blocked",
        GddPrototypeDraftBundleStatus::Unsupported => "unsupported",
    }
}

fn component_kind_label(kind: &GddPrototypeBundleComponentKind) -> &'static str {
    match kind {
        GddPrototypeBundleComponentKind::Requirements => "requirements",
        GddPrototypeBundleComponentKind::Feasibility => "feasibility",
        GddPrototypeBundleComponentKind::Scaffold => "scaffold",
        GddPrototypeBundleComponentKind::SceneLevel => "scene-level",
        GddPrototypeBundleComponentKind::Behavior => "behavior",
        GddPrototypeBundleComponentKind::Assets => "assets",
        GddPrototypeBundleComponentKind::Scenarios => "scenarios",
        GddPrototypeBundleComponentKind::TaskGraph => "task-graph",
    }
}

fn validation_status_label(status: &GddPrototypeBundleValidationStatus) -> &'static str {
    match status {
        GddPrototypeBundleValidationStatus::Pass => "pass",
        GddPrototypeBundleValidationStatus::Missing => "missing",
        GddPrototypeBundleValidationStatus::Stale => "stale",
        GddPrototypeBundleValidationStatus::Unsupported => "unsupported",
        GddPrototypeBundleValidationStatus::Blocked => "blocked",
        GddPrototypeBundleValidationStatus::Contradictory => "contradictory",
    }
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

fn validate_local_ref_list(field: &str, values: &[String], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, values.len())?;
    }
    for value in values {
        require_local_ref(field, value)?;
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
        return Err(anyhow!(
            "{field} contains forbidden traversal and must stay inside local fixture/reference roots"
        ));
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
    const NEGATIONS: [&str; 6] = [
        "no ",
        "not ",
        "without ",
        "avoid ",
        "forbid ",
        "forbidden ",
    ];
    let hay = value;
    // Scope negation to the clause/sentence containing each occurrence so a
    // negated mention in one sentence cannot whitelist a positive mention in
    // another (fail-closed), while a single leading negation still covers a
    // list such as `no auto-apply or self-approval`.
    let mut search_start = 0;
    while let Some(rel) = hay[search_start..].find(phrase) {
        let idx = search_start + rel;
        let clause_start = hay[..idx]
            .rfind(['.', ';', '!', '\n', '\r'])
            .map(|p| p + 1)
            .unwrap_or(0);
        let preceding = &hay[clause_start..idx];
        let negated = NEGATIONS.iter().any(|n| preceding.contains(n));
        if !negated {
            return true;
        }
        search_start = idx + phrase.len();
    }
    false
}
