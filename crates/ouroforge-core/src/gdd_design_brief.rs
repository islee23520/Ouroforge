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
        require_nonempty("GDD design brief mechanics", self.mechanics.len())?;
        for mechanic in &self.mechanics {
            mechanic.validate()?;
        }
        require_nonempty("GDD design brief controls", self.controls.len())?;
        for control in &self.controls {
            control.validate()?;
        }
        self.win_loss_conditions.validate()?;
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
    fn validate(&self) -> Result<()> {
        require_string_list("GDD design brief winLossConditions.win", &self.win, true)?;
        require_string_list("GDD design brief winLossConditions.loss", &self.loss, true)
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
        require_local_ref("GDD design brief assetStyleRefs.source", &self.source)?;
        require_text("GDD design brief assetStyleRefs.license", &self.license)
    }
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
