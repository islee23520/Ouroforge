use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_MECHANICS_MAPPING_SCHEMA_VERSION: &str = "gdd-mechanics-mapping-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddMechanicsMappingStatus {
    Ready,
    Partial,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum GddMechanicSupportStatus {
    Supported,
    Partial,
    Unsupported,
    Deferred,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddMechanicCapabilityKind {
    #[serde(rename = "production-2d")]
    Production2d,
    #[serde(rename = "three-d-gate")]
    ThreeDGate,
    GameplayLogic,
    LevelDesigner,
    AssetManifest,
    ScenarioEvidence,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddMechanicsMappingArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "mappingId")]
    pub mapping_id: String,
    pub status: GddMechanicsMappingStatus,
    #[serde(rename = "sourceExtractionRef")]
    pub source_extraction_ref: String,
    #[serde(rename = "requirementRefs")]
    pub requirement_refs: Vec<GddMechanicRequirementRef>,
    #[serde(rename = "capabilityRefs")]
    pub capability_refs: Vec<GddMechanicCapabilityRef>,
    #[serde(rename = "coreLoops")]
    pub core_loops: Vec<GddCoreLoopMapping>,
    pub mappings: Vec<GddMechanicMapping>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddMechanicRequirementRef {
    pub id: String,
    pub category: String,
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddMechanicCapabilityRef {
    pub id: String,
    pub kind: GddMechanicCapabilityKind,
    #[serde(rename = "docsRef")]
    pub docs_ref: String,
    #[serde(rename = "supportStatus")]
    pub support_status: GddMechanicSupportStatus,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddCoreLoopMapping {
    pub id: String,
    pub title: String,
    #[serde(rename = "requirementRefs")]
    pub requirement_refs: Vec<String>,
    #[serde(rename = "mappingRefs")]
    pub mapping_refs: Vec<String>,
    #[serde(rename = "scopeBoundary")]
    pub scope_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddMechanicMapping {
    pub id: String,
    #[serde(rename = "requirementId")]
    pub requirement_id: String,
    #[serde(rename = "supportStatus")]
    pub support_status: GddMechanicSupportStatus,
    #[serde(rename = "engineCapabilityRefs")]
    pub engine_capability_refs: Vec<String>,
    #[serde(rename = "behaviorModelRefs")]
    pub behavior_model_refs: Vec<String>,
    #[serde(rename = "sceneLevelNeeds")]
    pub scene_level_needs: Vec<String>,
    #[serde(rename = "assetNeeds")]
    pub asset_needs: Vec<String>,
    #[serde(rename = "scenarioNeeds")]
    pub scenario_needs: Vec<String>,
    pub dependencies: Vec<String>,
    #[serde(rename = "conflictsWith")]
    pub conflicts_with: Vec<String>,
    #[serde(rename = "unsupportedGaps")]
    pub unsupported_gaps: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    pub recommendations: Vec<String>,
    #[serde(rename = "coreLoopRefs")]
    pub core_loop_refs: Vec<String>,
    #[serde(rename = "evidenceBoundary")]
    pub evidence_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddMechanicsMappingReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "mappingId")]
    pub mapping_id: String,
    pub status: String,
    #[serde(rename = "requirementCount")]
    pub requirement_count: usize,
    #[serde(rename = "capabilityCount")]
    pub capability_count: usize,
    #[serde(rename = "coreLoopCount")]
    pub core_loop_count: usize,
    #[serde(rename = "mappingCount")]
    pub mapping_count: usize,
    #[serde(rename = "blockedMappingCount")]
    pub blocked_mapping_count: usize,
    #[serde(rename = "unsupportedMappingCount")]
    pub unsupported_mapping_count: usize,
    #[serde(rename = "deferredMappingCount")]
    pub deferred_mapping_count: usize,
    #[serde(rename = "supportStatusCounts")]
    pub support_status_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddMechanicsMappingArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse GDD Mechanics Mapping JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddMechanicsMappingReadModel {
        let mut support_status_counts = BTreeMap::new();
        let mut blocked_mapping_count = 0;
        let mut unsupported_mapping_count = 0;
        let mut deferred_mapping_count = 0;
        for mapping in &self.mappings {
            *support_status_counts
                .entry(support_label(&mapping.support_status).to_string())
                .or_insert(0) += 1;
            if !mapping.blocked_reasons.is_empty() {
                blocked_mapping_count += 1;
            }
            if mapping.support_status == GddMechanicSupportStatus::Unsupported {
                unsupported_mapping_count += 1;
            }
            if mapping.support_status == GddMechanicSupportStatus::Deferred {
                deferred_mapping_count += 1;
            }
        }

        let mut validation_summary = vec![
            "schemaVersion accepted by Rust/local validation".to_string(),
            "each mechanics mapping is linked to a declared extracted requirement id".to_string(),
            "supported mappings declare engine, behavior, level, asset, and scenario needs"
                .to_string(),
            "unsupported or deferred mechanics stay visible as gap/defer recommendations"
                .to_string(),
        ];
        if blocked_mapping_count > 0 {
            validation_summary.push(format!(
                "{blocked_mapping_count} blocked mapping(s) require reviewer resolution before prototype planning"
            ));
        }
        if unsupported_mapping_count > 0 || deferred_mapping_count > 0 {
            validation_summary.push(format!(
                "{unsupported_mapping_count} unsupported and {deferred_mapping_count} deferred mapping(s) cannot be silently implemented"
            ));
        }

        GddMechanicsMappingReadModel {
            schema_version: self.schema_version.clone(),
            mapping_id: self.mapping_id.clone(),
            status: status_label(&self.status).to_string(),
            requirement_count: self.requirement_refs.len(),
            capability_count: self.capability_refs.len(),
            core_loop_count: self.core_loops.len(),
            mapping_count: self.mappings.len(),
            blocked_mapping_count,
            unsupported_mapping_count,
            deferred_mapping_count,
            support_status_counts,
            validation_summary,
            compatibility_notes: vec![
                "display-only/read-model compatible; browser and Studio surfaces receive no trusted write authority".to_string(),
                "mapping does not generate prototypes, mutate source, generate assets, execute scripts, or apply changes".to_string(),
                "links prior Production 2D, 3D gate, gameplay logic, level designer, asset, and scenario contracts by docs refs".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD mechanics mapping read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_MECHANICS_MAPPING_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD mechanics mapping schemaVersion must be {GDD_MECHANICS_MAPPING_SCHEMA_VERSION}"
            ));
        }
        require_local_id("GDD mechanics mapping mappingId", &self.mapping_id)?;
        require_local_ref(
            "GDD mechanics mapping sourceExtractionRef",
            &self.source_extraction_ref,
        )?;
        require_nonempty(
            "GDD mechanics mapping requirementRefs",
            self.requirement_refs.len(),
        )?;
        require_nonempty(
            "GDD mechanics mapping capabilityRefs",
            self.capability_refs.len(),
        )?;
        require_nonempty("GDD mechanics mapping coreLoops", self.core_loops.len())?;
        require_nonempty("GDD mechanics mapping mappings", self.mappings.len())?;

        let mut requirement_ids = BTreeSet::new();
        for requirement in &self.requirement_refs {
            requirement.validate()?;
            if !requirement_ids.insert(requirement.id.clone()) {
                return Err(anyhow!(
                    "GDD mechanics mapping requirementRefs.id `{}` is duplicated",
                    requirement.id
                ));
            }
        }

        let mut capability_ids = BTreeSet::new();
        let mut capability_statuses = BTreeMap::new();
        for capability in &self.capability_refs {
            capability.validate()?;
            if !capability_ids.insert(capability.id.clone()) {
                return Err(anyhow!(
                    "GDD mechanics mapping capabilityRefs.id `{}` is duplicated",
                    capability.id
                ));
            }
            capability_statuses.insert(capability.id.clone(), capability.support_status.clone());
        }

        let mut mapping_ids = BTreeSet::new();
        for mapping in &self.mappings {
            mapping.validate(&requirement_ids, &capability_ids, &capability_statuses)?;
            if !mapping_ids.insert(mapping.id.clone()) {
                return Err(anyhow!(
                    "GDD mechanics mapping mappings.id `{}` is duplicated",
                    mapping.id
                ));
            }
        }

        let mut core_loop_ids = BTreeSet::new();
        for core_loop in &self.core_loops {
            core_loop.validate(&requirement_ids, &mapping_ids)?;
            if !core_loop_ids.insert(core_loop.id.clone()) {
                return Err(anyhow!(
                    "GDD mechanics mapping coreLoops.id `{}` is duplicated",
                    core_loop.id
                ));
            }
        }

        for mapping in &self.mappings {
            for linked in mapping
                .dependencies
                .iter()
                .chain(mapping.conflicts_with.iter())
            {
                if !mapping_ids.contains(linked) {
                    return Err(anyhow!(
                        "GDD mechanics mapping `{}` links unknown mapping `{linked}`",
                        mapping.id
                    ));
                }
                if linked == &mapping.id {
                    return Err(anyhow!(
                        "GDD mechanics mapping `{}` must not link itself",
                        mapping.id
                    ));
                }
            }
            for core_loop_ref in &mapping.core_loop_refs {
                if !core_loop_ids.contains(core_loop_ref) {
                    return Err(anyhow!(
                        "GDD mechanics mapping `{}` links unknown coreLoop `{core_loop_ref}`",
                        mapping.id
                    ));
                }
            }
            if !mapping.conflicts_with.is_empty() && mapping.blocked_reasons.is_empty() {
                return Err(anyhow!(
                    "GDD mechanics mapping contradictory mapping `{}` must include blockedReasons",
                    mapping.id
                ));
            }
        }

        if self.status == GddMechanicsMappingStatus::Ready
            && self.mappings.iter().any(|mapping| {
                !mapping.blocked_reasons.is_empty()
                    || mapping.support_status != GddMechanicSupportStatus::Supported
            })
        {
            return Err(anyhow!(
                "ready GDD mechanics mapping must not contain blocked, partial, unsupported, or deferred mappings"
            ));
        }
        if self.status == GddMechanicsMappingStatus::Blocked
            && !self
                .mappings
                .iter()
                .any(|mapping| !mapping.blocked_reasons.is_empty())
        {
            return Err(anyhow!(
                "blocked GDD mechanics mapping requires at least one blocked mapping"
            ));
        }

        require_text("GDD mechanics mapping boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "evidence-gated prototype planning",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no autonomous unrestricted game creation",
            "no source or script mutation",
            "no asset generation",
            "browser read-only or draft-only",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD mechanics mapping boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl GddMechanicRequirementRef {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD mechanics mapping requirementRefs.id", &self.id)?;
        require_text(
            "GDD mechanics mapping requirementRefs.category",
            &self.category,
        )?;
        require_source_ref(
            "GDD mechanics mapping requirementRefs.sourceRef",
            &self.source_ref,
        )?;
        Ok(())
    }
}

impl GddMechanicCapabilityRef {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD mechanics mapping capabilityRefs.id", &self.id)?;
        require_docs_ref(
            "GDD mechanics mapping capabilityRefs.docsRef",
            &self.docs_ref,
        )?;
        require_text(
            "GDD mechanics mapping capabilityRefs.summary",
            &self.summary,
        )?;
        Ok(())
    }
}

impl GddCoreLoopMapping {
    fn validate(
        &self,
        requirement_ids: &BTreeSet<String>,
        mapping_ids: &BTreeSet<String>,
    ) -> Result<()> {
        require_local_id("GDD mechanics mapping coreLoops.id", &self.id)?;
        require_text("GDD mechanics mapping coreLoops.title", &self.title)?;
        require_nonempty(
            "GDD mechanics mapping coreLoops.requirementRefs",
            self.requirement_refs.len(),
        )?;
        require_nonempty(
            "GDD mechanics mapping coreLoops.mappingRefs",
            self.mapping_refs.len(),
        )?;
        if self.requirement_refs.len() > 6 || self.mapping_refs.len() > 6 {
            return Err(anyhow!(
                "GDD mechanics mapping coreLoop `{}` is overbroad; split loops above six requirements or mappings",
                self.id
            ));
        }
        validate_local_id_list(
            "GDD mechanics mapping coreLoops.requirementRefs",
            &self.requirement_refs,
        )?;
        validate_local_id_list(
            "GDD mechanics mapping coreLoops.mappingRefs",
            &self.mapping_refs,
        )?;
        for requirement_ref in &self.requirement_refs {
            if !requirement_ids.contains(requirement_ref) {
                return Err(anyhow!(
                    "GDD mechanics mapping coreLoop `{}` links unknown requirement `{requirement_ref}`",
                    self.id
                ));
            }
        }
        for mapping_ref in &self.mapping_refs {
            if !mapping_ids.contains(mapping_ref) {
                return Err(anyhow!(
                    "GDD mechanics mapping coreLoop `{}` links unknown mapping `{mapping_ref}`",
                    self.id
                ));
            }
        }
        require_text(
            "GDD mechanics mapping coreLoops.scopeBoundary",
            &self.scope_boundary,
        )?;
        let boundary = self.scope_boundary.to_ascii_lowercase();
        for required in ["bounded", "not a full-game generator", "review required"] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD mechanics mapping coreLoop `{}` scopeBoundary must state `{required}`",
                    self.id
                ));
            }
        }
        Ok(())
    }
}

impl GddMechanicMapping {
    fn validate(
        &self,
        requirement_ids: &BTreeSet<String>,
        capability_ids: &BTreeSet<String>,
        capability_statuses: &BTreeMap<String, GddMechanicSupportStatus>,
    ) -> Result<()> {
        require_local_id("GDD mechanics mapping mappings.id", &self.id)?;
        require_local_id(
            "GDD mechanics mapping mappings.requirementId",
            &self.requirement_id,
        )?;
        if !requirement_ids.contains(&self.requirement_id) {
            return Err(anyhow!(
                "GDD mechanics mapping `{}` has missing requirementId `{}`",
                self.id,
                self.requirement_id
            ));
        }
        validate_capability_list(
            &self.id,
            "GDD mechanics mapping mappings.engineCapabilityRefs",
            &self.engine_capability_refs,
            capability_ids,
        )?;
        validate_capability_list(
            &self.id,
            "GDD mechanics mapping mappings.behaviorModelRefs",
            &self.behavior_model_refs,
            capability_ids,
        )?;
        validate_string_list(
            "GDD mechanics mapping mappings.sceneLevelNeeds",
            &self.scene_level_needs,
            false,
        )?;
        validate_string_list(
            "GDD mechanics mapping mappings.assetNeeds",
            &self.asset_needs,
            false,
        )?;
        validate_string_list(
            "GDD mechanics mapping mappings.scenarioNeeds",
            &self.scenario_needs,
            false,
        )?;
        validate_local_id_list(
            "GDD mechanics mapping mappings.dependencies",
            &self.dependencies,
        )?;
        validate_local_id_list(
            "GDD mechanics mapping mappings.conflictsWith",
            &self.conflicts_with,
        )?;
        validate_string_list(
            "GDD mechanics mapping mappings.unsupportedGaps",
            &self.unsupported_gaps,
            false,
        )?;
        validate_string_list(
            "GDD mechanics mapping mappings.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        validate_string_list(
            "GDD mechanics mapping mappings.recommendations",
            &self.recommendations,
            false,
        )?;
        validate_local_id_list(
            "GDD mechanics mapping mappings.coreLoopRefs",
            &self.core_loop_refs,
        )?;
        require_nonempty(
            "GDD mechanics mapping mappings.coreLoopRefs",
            self.core_loop_refs.len(),
        )?;

        if self.support_status == GddMechanicSupportStatus::Supported {
            for (field, len) in [
                (
                    "GDD mechanics mapping supported engineCapabilityRefs",
                    self.engine_capability_refs.len(),
                ),
                (
                    "GDD mechanics mapping supported behaviorModelRefs",
                    self.behavior_model_refs.len(),
                ),
                (
                    "GDD mechanics mapping supported sceneLevelNeeds",
                    self.scene_level_needs.len(),
                ),
                (
                    "GDD mechanics mapping supported assetNeeds",
                    self.asset_needs.len(),
                ),
                (
                    "GDD mechanics mapping supported scenarioNeeds",
                    self.scenario_needs.len(),
                ),
            ] {
                require_nonempty(field, len)?;
            }
            if !self.unsupported_gaps.is_empty() || !self.blocked_reasons.is_empty() {
                return Err(anyhow!(
                    "GDD mechanics mapping supported mapping `{}` must not carry unsupportedGaps or blockedReasons",
                    self.id
                ));
            }
            for capability_ref in self
                .engine_capability_refs
                .iter()
                .chain(self.behavior_model_refs.iter())
            {
                if capability_statuses.get(capability_ref)
                    != Some(&GddMechanicSupportStatus::Supported)
                {
                    return Err(anyhow!(
                        "GDD mechanics mapping supported mapping `{}` references non-supported capability `{capability_ref}`",
                        self.id
                    ));
                }
            }
        }

        if matches!(
            self.support_status,
            GddMechanicSupportStatus::Unsupported | GddMechanicSupportStatus::Deferred
        ) {
            require_nonempty(
                "GDD mechanics mapping unsupported/deferred unsupportedGaps",
                self.unsupported_gaps.len(),
            )?;
            require_nonempty(
                "GDD mechanics mapping unsupported/deferred blockedReasons",
                self.blocked_reasons.len(),
            )?;
            require_nonempty(
                "GDD mechanics mapping unsupported/deferred recommendations",
                self.recommendations.len(),
            )?;
            if !self.recommendations.iter().any(|recommendation| {
                let lower = recommendation.to_ascii_lowercase();
                lower.contains("downgrade")
                    || lower.contains("defer")
                    || lower.contains("placeholder")
            }) {
                return Err(anyhow!(
                    "GDD mechanics mapping unsupported/deferred mapping `{}` must include downgrade, defer, or placeholder recommendation",
                    self.id
                ));
            }
        }

        if self.support_status == GddMechanicSupportStatus::Partial {
            require_nonempty(
                "GDD mechanics mapping partial unsupportedGaps",
                self.unsupported_gaps.len(),
            )?;
            require_nonempty(
                "GDD mechanics mapping partial recommendations",
                self.recommendations.len(),
            )?;
        }

        require_text(
            "GDD mechanics mapping mappings.evidenceBoundary",
            &self.evidence_boundary,
        )?;
        let boundary = self.evidence_boundary.to_ascii_lowercase();
        for required in [
            "requirement-linked",
            "not generation authority",
            "review required",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD mechanics mapping `{}` evidenceBoundary must state `{required}`",
                    self.id
                ));
            }
        }
        Ok(())
    }
}

fn status_label(status: &GddMechanicsMappingStatus) -> &'static str {
    match status {
        GddMechanicsMappingStatus::Ready => "ready",
        GddMechanicsMappingStatus::Partial => "partial",
        GddMechanicsMappingStatus::Blocked => "blocked",
    }
}

fn support_label(status: &GddMechanicSupportStatus) -> &'static str {
    match status {
        GddMechanicSupportStatus::Supported => "supported",
        GddMechanicSupportStatus::Partial => "partial",
        GddMechanicSupportStatus::Unsupported => "unsupported",
        GddMechanicSupportStatus::Deferred => "deferred",
    }
}

fn validate_capability_list(
    mapping_id: &str,
    field: &str,
    values: &[String],
    capability_ids: &BTreeSet<String>,
) -> Result<()> {
    validate_local_id_list(field, values)?;
    for value in values {
        if !capability_ids.contains(value) {
            return Err(anyhow!(
                "GDD mechanics mapping `{mapping_id}` links unknown capability `{value}`"
            ));
        }
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

fn validate_local_id_list(field: &str, values: &[String]) -> Result<()> {
    for value in values {
        require_local_id(field, value)?;
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
            "{field} must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("examples/")
        || value.starts_with("docs/")
        || value.starts_with("seeds/"))
    {
        return Err(anyhow!("{field} must use examples/, docs/, or seeds/ refs"));
    }
    Ok(())
}

fn require_source_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must stay inside local GDD refs"));
    }
    if !(value.starts_with("gdd:") || value.starts_with("docs/") || value.starts_with("examples/"))
    {
        return Err(anyhow!("{field} must use gdd:, docs/, or examples/ refs"));
    }
    Ok(())
}

fn require_docs_ref(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must stay inside local docs refs"));
    }
    if !value.starts_with("docs/") {
        return Err(anyhow!("{field} must use docs/ refs"));
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
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD/mechanics authority text `{forbidden}`"
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
