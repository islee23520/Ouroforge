use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const GDD_DESIGN_BRIEF_SCHEMA_VERSION: &str = "gdd-design-brief-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddDesignBriefStatus {
    Ready,
    Partial,
    Blocked,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum GddTargetGameClass {
    Small2dPrototype,
    Compatibility3dPrototype,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddDesignBriefReadModel {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    pub status: String,
    #[serde(rename = "gameTitle")]
    pub game_title: String,
    #[serde(rename = "targetGameClass")]
    pub target_game_class: String,
    #[serde(rename = "coreLoopStepCount")]
    pub core_loop_step_count: usize,
    #[serde(rename = "mechanicCount")]
    pub mechanic_count: usize,
    #[serde(rename = "controlCount")]
    pub control_count: usize,
    #[serde(rename = "sceneLevelCount")]
    pub scene_level_count: usize,
    #[serde(rename = "entityCount")]
    pub entity_count: usize,
    #[serde(rename = "assetStyleRefCount")]
    pub asset_style_ref_count: usize,
    #[serde(rename = "acceptanceGoalCount")]
    pub acceptance_goal_count: usize,
    #[serde(rename = "blockedReasonCount")]
    pub blocked_reason_count: usize,
    #[serde(rename = "validationSummary")]
    pub validation_summary: Vec<String>,
    #[serde(rename = "compatibilityNotes")]
    pub compatibility_notes: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddDesignBriefArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    pub status: GddDesignBriefStatus,
    #[serde(rename = "gameTitle")]
    pub game_title: String,
    pub genre: String,
    #[serde(rename = "targetGameClass")]
    pub target_game_class: GddTargetGameClass,
    #[serde(rename = "playerFantasy")]
    pub player_fantasy: String,
    #[serde(rename = "coreLoop")]
    pub core_loop: GddCoreLoop,
    pub mechanics: Vec<GddMechanic>,
    pub controls: Vec<GddControl>,
    #[serde(rename = "winLossConditions")]
    pub win_loss_conditions: GddWinLossConditions,
    #[serde(rename = "scenesLevels")]
    pub scenes_levels: Vec<GddSceneLevel>,
    pub entities: Vec<GddEntity>,
    #[serde(rename = "assetStyleRefs")]
    pub asset_style_refs: Vec<GddAssetStyleRef>,
    pub constraints: Vec<GddTextItem>,
    #[serde(rename = "nonGoals")]
    pub non_goals: Vec<GddTextItem>,
    #[serde(rename = "acceptanceGoals")]
    pub acceptance_goals: Vec<GddTextItem>,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddCoreLoop {
    pub summary: String,
    pub steps: Vec<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddMechanic {
    pub id: String,
    pub summary: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddControl {
    pub action: String,
    pub input: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddWinLossConditions {
    pub win: Vec<String>,
    pub loss: Vec<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddSceneLevel {
    pub id: String,
    pub summary: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddEntity {
    pub id: String,
    pub role: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddAssetStyleRef {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub license: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct GddTextItem {
    pub id: String,
    pub text: String,
}

impl GddDesignBriefArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse GDD Design Brief JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    pub fn read_model(&self) -> GddDesignBriefReadModel {
        let status = status_label(&self.status).to_string();
        let blocked_reason_count = self.blocked_reasons.len();
        let mut validation_summary = vec![
            "schemaVersion accepted by Rust/local validation".to_string(),
            "targetGameClass remains bounded to v1 allowlist".to_string(),
            "asset/style refs are local fixture references with explicit license evidence"
                .to_string(),
            "GDD-derived output remains untrusted until review-gated apply".to_string(),
        ];
        if self.status == GddDesignBriefStatus::Ready {
            validation_summary
                .push("ready brief has concrete core loop and win/loss conditions".to_string());
        }
        if blocked_reason_count > 0 {
            validation_summary.push(format!(
                "{blocked_reason_count} blocked reason(s) remain visible for reviewers"
            ));
        }

        GddDesignBriefReadModel {
            schema_version: self.schema_version.clone(),
            brief_id: self.brief_id.clone(),
            status,
            game_title: self.game_title.clone(),
            target_game_class: target_game_class_label(&self.target_game_class).to_string(),
            core_loop_step_count: self.core_loop.steps.len(),
            mechanic_count: self.mechanics.len(),
            control_count: self.controls.len(),
            scene_level_count: self.scenes_levels.len(),
            entity_count: self.entities.len(),
            asset_style_ref_count: self.asset_style_refs.len(),
            acceptance_goal_count: self.acceptance_goals.len(),
            blocked_reason_count,
            validation_summary,
            compatibility_notes: vec![
                "display-only read model; no trusted browser write authority".to_string(),
                "no prototype generation, command bridge, source mutation, or asset generation authority".to_string(),
                "compatible with later extraction/planning surfaces through validated summary counts".to_string(),
            ],
            boundary: self.boundary.clone(),
        }
    }

    pub fn read_model_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.read_model())
            .context("failed to serialize GDD design brief read model JSON")
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != GDD_DESIGN_BRIEF_SCHEMA_VERSION {
            return Err(anyhow!(
                "GDD design brief schemaVersion must be {GDD_DESIGN_BRIEF_SCHEMA_VERSION}"
            ));
        }
        require_local_id("GDD design brief briefId", &self.brief_id)?;
        require_text("GDD design brief gameTitle", &self.game_title)?;
        require_text("GDD design brief genre", &self.genre)?;
        require_text("GDD design brief playerFantasy", &self.player_fantasy)?;
        self.core_loop.validate()?;
        if self.status == GddDesignBriefStatus::Ready && self.core_loop.steps.len() < 2 {
            return Err(anyhow!(
                "GDD design brief coreLoop.steps must include at least two concrete steps"
            ));
        }
        validate_scope_text("GDD design brief gameTitle", &self.game_title)?;
        validate_scope_text("GDD design brief genre", &self.genre)?;
        validate_scope_text("GDD design brief playerFantasy", &self.player_fantasy)?;
        require_nonempty("GDD design brief mechanics", self.mechanics.len())?;
        for mechanic in &self.mechanics {
            mechanic.validate()?;
        }
        require_nonempty("GDD design brief controls", self.controls.len())?;
        for control in &self.controls {
            control.validate()?;
        }
        self.win_loss_conditions
            .validate(self.status == GddDesignBriefStatus::Ready)?;
        require_nonempty("GDD design brief scenesLevels", self.scenes_levels.len())?;
        for scene in &self.scenes_levels {
            scene.validate()?;
        }
        require_nonempty("GDD design brief entities", self.entities.len())?;
        for entity in &self.entities {
            entity.validate()?;
        }
        for asset in &self.asset_style_refs {
            asset.validate()?;
        }
        validate_text_items("GDD design brief constraints", &self.constraints, true)?;
        validate_text_items("GDD design brief nonGoals", &self.non_goals, true)?;
        validate_text_items(
            "GDD design brief acceptanceGoals",
            &self.acceptance_goals,
            true,
        )?;
        validate_cross_field_scope(self)?;
        for reason in &self.blocked_reasons {
            require_text("GDD design brief blockedReasons", reason)?;
        }
        if self.status == GddDesignBriefStatus::Blocked && self.blocked_reasons.is_empty() {
            return Err(anyhow!("blocked GDD design brief requires blockedReasons"));
        }
        require_text("GDD design brief boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "input validation",
            "not generation authority",
            "untrusted until rust/local validation",
            "review-gated apply",
            "no autonomous unrestricted game creation",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!("GDD design brief boundary must state `{required}`"));
            }
        }
        Ok(())
    }
}
impl GddCoreLoop {
    fn validate(&self) -> Result<()> {
        require_text("GDD design brief coreLoop.summary", &self.summary)?;
        require_string_list("GDD design brief coreLoop.steps", &self.steps, true)
    }
}
impl GddMechanic {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD design brief mechanics.id", &self.id)?;
        require_text("GDD design brief mechanics.summary", &self.summary)
    }
}
impl GddControl {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD design brief controls.action", &self.action)?;
        require_text("GDD design brief controls.input", &self.input)
    }
}
impl GddWinLossConditions {
    fn validate(&self, ready: bool) -> Result<()> {
        require_string_list("GDD design brief winLossConditions.win", &self.win, true)?;
        require_string_list("GDD design brief winLossConditions.loss", &self.loss, true)?;
        if ready {
            for (field, values) in [
                ("GDD design brief winLossConditions.win", &self.win),
                ("GDD design brief winLossConditions.loss", &self.loss),
            ] {
                for value in values {
                    reject_unclear_goal_text(field, value)?;
                }
            }
        }
        Ok(())
    }
}
impl GddSceneLevel {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD design brief scenesLevels.id", &self.id)?;
        require_text("GDD design brief scenesLevels.summary", &self.summary)
    }
}
impl GddEntity {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD design brief entities.id", &self.id)?;
        require_text("GDD design brief entities.role", &self.role)
    }
}
impl GddAssetStyleRef {
    fn validate(&self) -> Result<()> {
        require_local_id("GDD design brief assetStyleRefs.id", &self.id)?;
        require_local_id("GDD design brief assetStyleRefs.kind", &self.kind)?;
        if !matches!(
            self.kind.as_str(),
            "placeholder"
                | "palette"
                | "tilemap"
                | "sprite-sheet"
                | "audio-placeholder"
                | "docs-ref"
        ) {
            return Err(anyhow!(
                "GDD design brief assetStyleRefs.kind `{}` is not supported in v1",
                self.kind
            ));
        }
        require_local_ref("GDD design brief assetStyleRefs.source", &self.source)?;
        require_text("GDD design brief assetStyleRefs.license", &self.license)?;
        let license = self.license.to_ascii_lowercase();
        if !(license.contains("repo-fixture")
            || license.contains("placeholder")
            || license.contains("public-domain")
            || license.contains("cc0")
            || license.contains("local-fixture"))
        {
            return Err(anyhow!(
                "GDD design brief assetStyleRefs.license must be explicit local fixture, placeholder, public-domain, or CC0 evidence"
            ));
        }
        Ok(())
    }
}

fn status_label(status: &GddDesignBriefStatus) -> &'static str {
    match status {
        GddDesignBriefStatus::Ready => "ready",
        GddDesignBriefStatus::Partial => "partial",
        GddDesignBriefStatus::Blocked => "blocked",
    }
}

fn target_game_class_label(target_game_class: &GddTargetGameClass) -> &'static str {
    match target_game_class {
        GddTargetGameClass::Small2dPrototype => "small2d-prototype",
        GddTargetGameClass::Compatibility3dPrototype => "compatibility3d-prototype",
    }
}

fn validate_cross_field_scope(artifact: &GddDesignBriefArtifact) -> Result<()> {
    for mechanic in &artifact.mechanics {
        validate_scope_text("GDD design brief mechanics.summary", &mechanic.summary)?;
    }
    for scene in &artifact.scenes_levels {
        validate_scope_text("GDD design brief scenesLevels.summary", &scene.summary)?;
    }
    for entity in &artifact.entities {
        validate_scope_text("GDD design brief entities.role", &entity.role)?;
    }
    for item in artifact
        .constraints
        .iter()
        .chain(artifact.acceptance_goals.iter())
    {
        validate_scope_text("GDD design brief scoped text", &item.text)?;
    }

    for phrase in [
        "native export",
        "asset generation",
        "autonomous unrestricted game creation",
        "source mutation",
        "remote assets",
        "production game",
        "commercial readiness",
    ] {
        let positive = artifact
            .constraints
            .iter()
            .chain(artifact.acceptance_goals.iter())
            .any(|item| contains_positive_phrase(&item.text, phrase));
        let rejected = artifact
            .non_goals
            .iter()
            .any(|item| item.text.to_ascii_lowercase().contains(phrase));
        if positive && rejected {
            return Err(anyhow!(
                "GDD design brief has contradictory requirements for `{phrase}`"
            ));
        }
    }
    Ok(())
}

fn validate_scope_text(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "full game",
        "open world",
        "massively multiplayer",
        "mmo",
        "every platform",
        "all platforms",
        "unlimited",
        "unrestricted",
        "ship-ready",
        "commercial release",
        "production game",
        "native export",
        "plugin runtime",
        "hosted cloud",
        "source mutation",
        "asset generation",
    ] {
        if contains_positive_phrase(&lower, forbidden) {
            return Err(anyhow!(
                "{field} contains overbroad or out-of-scope v1 requirement `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn contains_positive_phrase(value: &str, phrase: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    if !lower.contains(phrase) {
        return false;
    }
    !["no ", "not ", "without ", "avoid ", "forbid ", "forbidden "]
        .iter()
        .any(|prefix| lower.contains(&format!("{prefix}{phrase}")))
}

fn reject_unclear_goal_text(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for unclear in [
        "tbd",
        "todo",
        "unclear",
        "unknown",
        "clarify",
        "not decided",
    ] {
        if lower.contains(unclear) {
            return Err(anyhow!(
                "{field} must be concrete for ready design briefs, found `{unclear}`"
            ));
        }
    }
    Ok(())
}

fn validate_text_items(field: &str, items: &[GddTextItem], required: bool) -> Result<()> {
    if required {
        require_nonempty(field, items.len())?;
    }
    for item in items {
        require_local_id(&format!("{field}.id"), &item.id)?;
        require_text(&format!("{field}.text"), &item.text)?;
    }
    Ok(())
}
fn require_string_list(field: &str, values: &[String], required: bool) -> Result<()> {
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
        return Err(anyhow!(
            "{field} must stay inside local fixture/reference roots"
        ));
    }
    if !(value.starts_with("assets/")
        || value.starts_with("examples/")
        || value.starts_with("docs/"))
    {
        return Err(anyhow!(
            "{field} must use assets/, examples/, or docs/ refs"
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
        "godot replacement",
        "production-ready",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden GDD/prototype authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}
