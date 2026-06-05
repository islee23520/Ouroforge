use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const GDD_REQUIREMENT_EXTRACTION_SCHEMA_VERSION: &str = "gdd-requirement-extraction-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddRequirementExtractionStatus {
    Ready,
    Partial,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddRequirementCategory {
    Mechanic,
    Control,
    Entity,
    Level,
    UiHud,
    WinLoss,
    Constraint,
    NonGoal,
    Acceptance,
    Evidence,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddRequirementPriority {
    Must,
    Should,
    Could,
    Wont,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddRequirementExtractionArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "extractionId")]
    pub extraction_id: String,
    pub status: GddRequirementExtractionStatus,
    #[serde(rename = "sourceArtifactRef")]
    pub source_artifact_ref: String,
    #[serde(rename = "sourceSections")]
    pub source_sections: Vec<GddRequirementSourceSection>,
    pub requirements: Vec<GddExtractedRequirement>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddRequirementSourceSection {
    pub id: String,
    pub title: String,
    #[serde(rename = "sourceRef")]
    pub source_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GddExtractedRequirement {
    pub id: String,
    pub category: GddRequirementCategory,
    #[serde(rename = "sourceSectionRef")]
    pub source_section_ref: String,
    #[serde(
        rename = "sourceExcerpt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_excerpt: Option<String>,
    pub priority: GddRequirementPriority,
    pub confidence: f32,
    #[serde(rename = "ambiguityFlags")]
    pub ambiguity_flags: Vec<String>,
    #[serde(rename = "dependencyLinks")]
    pub dependency_links: Vec<String>,
    #[serde(rename = "conflictsWith")]
    pub conflicts_with: Vec<String>,
    #[serde(rename = "blockedReasons")]
    pub blocked_reasons: Vec<String>,
    #[serde(rename = "evidenceBoundary")]
    pub evidence_boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddRequirementExtractionReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "extractionId")]
    pub extraction_id: String,
    pub status: String,
    #[serde(rename = "sourceSectionCount")]
    pub source_section_count: usize,
    #[serde(rename = "requirementCount")]
    pub requirement_count: usize,
    #[serde(rename = "blockedRequirementCount")]
    pub blocked_requirement_count: usize,
    #[serde(rename = "ambiguousRequirementCount")]
    pub ambiguous_requirement_count: usize,
    #[serde(rename = "dependencyLinkCount")]
    pub dependency_link_count: usize,
    #[serde(rename = "conflictLinkCount")]
    pub conflict_link_count: usize,
    #[serde(rename = "categoryCounts")]
    pub category_counts: BTreeMap<String, usize>,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

impl GddRequirementExtractionArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self = serde_json::from_str(input)
            .context("failed to parse GDD Requirement Extraction JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddRequirementExtractionReadModel {
        let mut category_counts = BTreeMap::new();
        let mut blocked_requirement_count = 0;
        let mut ambiguous_requirement_count = 0;
        let mut dependency_link_count = 0;
        let mut conflict_link_count = 0;
        for requirement in &self.requirements {
            *category_counts
                .entry(category_label(&requirement.category).to_string())
                .or_insert(0) += 1;
            if !requirement.blocked_reasons.is_empty() {
                blocked_requirement_count += 1;
            }
            if !requirement.ambiguity_flags.is_empty() {
                ambiguous_requirement_count += 1;
            }
            dependency_link_count += requirement.dependency_links.len();
            conflict_link_count += requirement.conflicts_with.len();
        }

        let mut validation_summary = vec![
            "schemaVersion accepted by Rust/local validation".to_string(),
            "every requirement is linked to a declared GDD source section".to_string(),
            "LLM or agent text remains untrusted/advisory unless reviewed separately".to_string(),
            "GDD-derived output remains blocked from prototype generation until later gates"
                .to_string(),
        ];
        if blocked_requirement_count > 0 {
            validation_summary.push(format!(
                "{blocked_requirement_count} blocked requirement(s) remain visible for reviewers"
            ));
        }
        if ambiguous_requirement_count > 0 {
            validation_summary.push(format!(
                "{ambiguous_requirement_count} ambiguous requirement(s) require reviewer resolution"
            ));
        }

        GddRequirementExtractionReadModel {
            schema_version: self.schema_version.clone(),
            extraction_id: self.extraction_id.clone(),
            status: status_label(&self.status).to_string(),
            source_section_count: self.source_sections.len(),
            requirement_count: self.requirements.len(),
            blocked_requirement_count,
            ambiguous_requirement_count,
            dependency_link_count,
            conflict_link_count,
            category_counts,
            validation_summary,
            compatibility_notes: vec![
                "display-only read model; no trusted browser write authority".to_string(),
                "no prototype generation, source mutation, asset generation, command bridge, or apply authority".to_string(),
                "compatible with later mechanics, feasibility, planning, review, and evidence gates through stable local ids".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD requirement extraction read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_REQUIREMENT_EXTRACTION_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD requirement extraction schemaVersion must be {GDD_REQUIREMENT_EXTRACTION_SCHEMA_VERSION}"
            ));
        }
        require_local_id(
            "GDD requirement extraction extractionId",
            &self.extraction_id,
        )?;
        require_local_ref(
            "GDD requirement extraction sourceArtifactRef",
            &self.source_artifact_ref,
        )?;
        require_nonempty(
            "GDD requirement extraction sourceSections",
            self.source_sections.len(),
        )?;
        require_nonempty(
            "GDD requirement extraction requirements",
            self.requirements.len(),
        )?;
        let mut source_ids = BTreeSet::new();
        for section in &self.source_sections {
            section.validate()?;
            if !source_ids.insert(section.id.clone()) {
                return Err(anyhow!(
                    "GDD requirement extraction sourceSections.id `{}` is duplicated",
                    section.id
                ));
            }
        }

        let mut requirement_ids = BTreeSet::new();
        for requirement in &self.requirements {
            requirement.validate(&source_ids, &self.source_sections)?;
            if !requirement_ids.insert(requirement.id.clone()) {
                return Err(anyhow!(
                    "GDD requirement extraction requirements.id `{}` is duplicated",
                    requirement.id
                ));
            }
        }
        for requirement in &self.requirements {
            for linked in requirement
                .dependency_links
                .iter()
                .chain(requirement.conflicts_with.iter())
            {
                if !requirement_ids.contains(linked) {
                    return Err(anyhow!(
                        "GDD requirement extraction requirement `{}` links unknown requirement `{linked}`",
                        requirement.id
                    ));
                }
                if linked == &requirement.id {
                    return Err(anyhow!(
                        "GDD requirement extraction requirement `{}` must not link itself",
                        requirement.id
                    ));
                }
            }
            if !requirement.conflicts_with.is_empty() && requirement.blocked_reasons.is_empty() {
                return Err(anyhow!(
                    "GDD requirement extraction conflicting requirement `{}` must include blockedReasons",
                    requirement.id
                ));
            }
        }
        if self.status == GddRequirementExtractionStatus::Ready
            && self
                .requirements
                .iter()
                .any(|requirement| !requirement.blocked_reasons.is_empty())
        {
            return Err(anyhow!(
                "ready GDD requirement extraction must not contain blocked requirements"
            ));
        }
        if self.status == GddRequirementExtractionStatus::Blocked
            && !self
                .requirements
                .iter()
                .any(|requirement| !requirement.blocked_reasons.is_empty())
        {
            return Err(anyhow!(
                "blocked GDD requirement extraction requires at least one blocked requirement"
            ));
        }
        require_text("GDD requirement extraction boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "manual or structured extraction first",
            "llm extraction advisory only",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no autonomous unrestricted game creation",
            "not prototype generation authority",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD requirement extraction boundary must state `{required}`"
                ));
            }
        }
        Ok(())
    }
}

impl GddRequirementSourceSection {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD requirement extraction sourceSections.id", &self.id)?;
        require_text(
            "GDD requirement extraction sourceSections.title",
            &self.title,
        )?;
        require_source_ref(
            "GDD requirement extraction sourceSections.sourceRef",
            &self.source_ref,
        )?;
        if let Some(text) = &self.text {
            require_text("GDD requirement extraction sourceSections.text", text)?;
        }
        Ok(())
    }
}

impl GddExtractedRequirement {
    fn validate(
        &self,
        source_ids: &BTreeSet<String>,
        source_sections: &[GddRequirementSourceSection],
    ) -> Result<()> {
        require_local_id("GDD requirement extraction requirements.id", &self.id)?;
        if !source_ids.contains(&self.source_section_ref) {
            return Err(anyhow!(
                "GDD requirement extraction requirement `{}` has missing sourceSectionRef `{}`",
                self.id,
                self.source_section_ref
            ));
        }
        let source = source_sections
            .iter()
            .find(|section| section.id == self.source_section_ref)
            .expect("source id was checked");
        if let Some(excerpt) = &self.source_excerpt {
            require_text(
                "GDD requirement extraction requirements.sourceExcerpt",
                excerpt,
            )?;
            if let Some(source_text) = &source.text {
                if !source_text.contains(excerpt) {
                    return Err(anyhow!(
                        "GDD requirement extraction requirement `{}` sourceExcerpt is not present in source section `{}`",
                        self.id,
                        self.source_section_ref
                    ));
                }
            }
        } else {
            return Err(anyhow!(
                "GDD requirement extraction requirement `{}` must include sourceExcerpt to avoid invented/unlinked requirements",
                self.id
            ));
        }
        if !self.confidence.is_finite() || !(0.0..=1.0).contains(&self.confidence) {
            return Err(anyhow!(
                "GDD requirement extraction requirement `{}` confidence must be between 0.0 and 1.0",
                self.id
            ));
        }
        validate_string_list(
            "GDD requirement extraction requirements.ambiguityFlags",
            &self.ambiguity_flags,
            false,
        )?;
        validate_local_id_list(
            "GDD requirement extraction requirements.dependencyLinks",
            &self.dependency_links,
        )?;
        validate_local_id_list(
            "GDD requirement extraction requirements.conflictsWith",
            &self.conflicts_with,
        )?;
        validate_string_list(
            "GDD requirement extraction requirements.blockedReasons",
            &self.blocked_reasons,
            false,
        )?;
        if self.confidence < 0.6 && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD requirement extraction low-confidence requirement `{}` must include blockedReasons",
                self.id
            ));
        }
        if !self.ambiguity_flags.is_empty() && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "GDD requirement extraction ambiguous requirement `{}` must include blockedReasons",
                self.id
            ));
        }
        require_text(
            "GDD requirement extraction requirements.evidenceBoundary",
            &self.evidence_boundary,
        )?;
        let boundary = self.evidence_boundary.to_ascii_lowercase();
        for required in [
            "source-linked",
            "not generated authority",
            "review required",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "GDD requirement extraction requirement `{}` evidenceBoundary must state `{required}`",
                    self.id
                ));
            }
        }
        Ok(())
    }
}

fn status_label(status: &GddRequirementExtractionStatus) -> &'static str {
    match status {
        GddRequirementExtractionStatus::Ready => "ready",
        GddRequirementExtractionStatus::Partial => "partial",
        GddRequirementExtractionStatus::Blocked => "blocked",
    }
}

fn category_label(category: &GddRequirementCategory) -> &'static str {
    match category {
        GddRequirementCategory::Mechanic => "mechanic",
        GddRequirementCategory::Control => "control",
        GddRequirementCategory::Entity => "entity",
        GddRequirementCategory::Level => "level",
        GddRequirementCategory::UiHud => "ui-hud",
        GddRequirementCategory::WinLoss => "win-loss",
        GddRequirementCategory::Constraint => "constraint",
        GddRequirementCategory::NonGoal => "non-goal",
        GddRequirementCategory::Acceptance => "acceptance",
        GddRequirementCategory::Evidence => "evidence",
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
