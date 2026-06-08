//! Steamworks integration contract v1 (#1839).
//!
//! Validates the mockable JS-side `steamworks.js` integration surface over the
//! existing runtime. This module records feature wiring, graceful no-Steam
//! fallback behavior, and daily-seed leaderboard payload shape. It performs no
//! Steam SDK calls, no trusted browser writes, no credential handling, and no
//! release action.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Component, Path};

pub const STEAMWORKS_WIRING_SCHEMA_VERSION: &str = "steamworks-integration-wiring-v1";
pub const STEAMWORKS_FALLBACK_SCHEMA_VERSION: &str = "steamworks-no-steam-fallback-v1";
pub const STEAMWORKS_LEADERBOARD_PAYLOAD_SCHEMA_VERSION: &str =
    "steamworks-daily-seed-leaderboard-payload-v1";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamworksIntegrationWiring {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub issue: u32,
    #[serde(rename = "runtime")]
    pub runtime: String,
    #[serde(rename = "bridge")]
    pub bridge: String,
    #[serde(rename = "bridgeModule")]
    pub bridge_module: String,
    pub features: Vec<SteamworksFeature>,
    #[serde(rename = "fallbackRef")]
    pub fallback_ref: String,
    #[serde(rename = "leaderboardPayloadRef")]
    pub leaderboard_payload_ref: String,
    #[serde(rename = "trustedStatePolicy")]
    pub trusted_state_policy: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamworksFeature {
    pub name: String,
    #[serde(rename = "jsHook")]
    pub js_hook: String,
    #[serde(rename = "trustedSource")]
    pub trusted_source: String,
    #[serde(rename = "fallback")]
    pub fallback: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NoSteamFallback {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub mode: String,
    #[serde(rename = "steamAvailable")]
    pub steam_available: bool,
    #[serde(rename = "disabledFeatures")]
    pub disabled_features: Vec<String>,
    #[serde(rename = "localOnlyFeatures")]
    pub local_only_features: Vec<String>,
    #[serde(rename = "userMessage")]
    pub user_message: String,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DailySeedLeaderboardPayload {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "leaderboardId")]
    pub leaderboard_id: String,
    #[serde(rename = "dailySeed")]
    pub daily_seed: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    pub score: i64,
    #[serde(rename = "scoreSource")]
    pub score_source: String,
    #[serde(rename = "evidenceRef")]
    pub evidence_ref: String,
    #[serde(rename = "replayDigest")]
    pub replay_digest: String,
    #[serde(rename = "submittedBy")]
    pub submitted_by: String,
    pub boundary: String,
}

impl SteamworksIntegrationWiring {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let wiring: Self = serde_json::from_str(input)
            .context("failed to parse Steamworks integration wiring JSON")?;
        wiring.validate()?;
        Ok(wiring)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAMWORKS_WIRING_SCHEMA_VERSION {
            return Err(anyhow!(
                "steamworks wiring schemaVersion must be {STEAMWORKS_WIRING_SCHEMA_VERSION}"
            ));
        }
        if self.issue != 1839 {
            return Err(anyhow!("steamworks wiring issue must be 1839"));
        }
        if self.runtime != "existing-web-runtime" {
            return Err(anyhow!(
                "steamworks wiring runtime must be existing-web-runtime"
            ));
        }
        if self.bridge != "steamworks.js" {
            return Err(anyhow!("steamworks wiring bridge must be steamworks.js"));
        }
        validate_relative_path("steamworks wiring bridgeModule", &self.bridge_module)?;
        validate_relative_path("steamworks wiring fallbackRef", &self.fallback_ref)?;
        validate_relative_path(
            "steamworks wiring leaderboardPayloadRef",
            &self.leaderboard_payload_ref,
        )?;
        require_contains(
            "steamworks wiring trustedStatePolicy",
            &self.trusted_state_policy,
            &["Rust/local", "read-only", "no direct trusted writes"],
        )?;
        validate_boundary("steamworks wiring boundary", &self.boundary)?;
        let mut names = BTreeSet::new();
        for feature in &self.features {
            feature.validate()?;
            if !names.insert(feature.name.as_str()) {
                return Err(anyhow!(
                    "steamworks wiring contains duplicate feature `{}`",
                    feature.name
                ));
            }
        }
        for required in [
            "overlay",
            "achievements",
            "cloud-saves",
            "daily-seed-leaderboard",
        ] {
            if !names.contains(required) {
                return Err(anyhow!("steamworks wiring missing feature `{required}`"));
            }
        }
        Ok(())
    }
}

impl SteamworksFeature {
    fn validate(&self) -> Result<()> {
        match self.name.as_str() {
            "overlay" | "achievements" | "cloud-saves" | "daily-seed-leaderboard" => {}
            other => return Err(anyhow!("unsupported steamworks feature `{other}`")),
        }
        require_local_id("steamworks feature jsHook", &self.js_hook)?;
        require_contains(
            "steamworks feature trustedSource",
            &self.trusted_source,
            &["Rust/local", "trusted", "evidence"],
        )?;
        require_contains(
            "steamworks feature fallback",
            &self.fallback,
            &["no-Steam", "graceful"],
        )?;
        Ok(())
    }
}

impl NoSteamFallback {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let fallback: Self =
            serde_json::from_str(input).context("failed to parse no-Steam fallback JSON")?;
        fallback.validate()?;
        Ok(fallback)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAMWORKS_FALLBACK_SCHEMA_VERSION {
            return Err(anyhow!(
                "no-Steam fallback schemaVersion must be {STEAMWORKS_FALLBACK_SCHEMA_VERSION}"
            ));
        }
        if self.mode != "no-steam" || self.steam_available {
            return Err(anyhow!(
                "no-Steam fallback must record mode no-steam and steamAvailable false"
            ));
        }
        require_feature_set(
            "no-Steam fallback disabledFeatures",
            &self.disabled_features,
        )?;
        if !self.local_only_features.iter().any(|f| f == "local-save") {
            return Err(anyhow!("no-Steam fallback must keep local-save available"));
        }
        require_contains(
            "no-Steam fallback userMessage",
            &self.user_message,
            &["Steam", "unavailable", "local"],
        )?;
        validate_boundary("no-Steam fallback boundary", &self.boundary)?;
        Ok(())
    }
}

impl DailySeedLeaderboardPayload {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let payload: Self = serde_json::from_str(input)
            .context("failed to parse daily-seed leaderboard payload JSON")?;
        payload.validate()?;
        Ok(payload)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAMWORKS_LEADERBOARD_PAYLOAD_SCHEMA_VERSION {
            return Err(anyhow!(
                "daily-seed leaderboard payload schemaVersion must be {STEAMWORKS_LEADERBOARD_PAYLOAD_SCHEMA_VERSION}"
            ));
        }
        require_local_id("leaderboardId", &self.leaderboard_id)?;
        require_local_id("dailySeed", &self.daily_seed)?;
        require_local_id("runId", &self.run_id)?;
        require_local_id("projectId", &self.project_id)?;
        if self.score < 0 || self.score > 1_000_000_000 {
            return Err(anyhow!(
                "daily-seed leaderboard score must be bounded and non-negative"
            ));
        }
        if self.score_source != "trusted-local-run-evidence" {
            return Err(anyhow!(
                "daily-seed leaderboard scoreSource must be trusted-local-run-evidence"
            ));
        }
        validate_relative_path("evidenceRef", &self.evidence_ref)?;
        require_sha256("replayDigest", &self.replay_digest)?;
        if self.submitted_by != "steam-user-id-pseudonymous" {
            return Err(anyhow!(
                "daily-seed leaderboard submittedBy must avoid real-player personal data"
            ));
        }
        validate_boundary("daily-seed leaderboard boundary", &self.boundary)?;
        Ok(())
    }
}

fn require_feature_set(field: &str, features: &[String]) -> Result<()> {
    let mut names = BTreeSet::new();
    for feature in features {
        match feature.as_str() {
            "overlay" | "achievements" | "cloud-saves" | "daily-seed-leaderboard" => {}
            other => return Err(anyhow!("{field} contains unsupported feature `{other}`")),
        }
        if !names.insert(feature.as_str()) {
            return Err(anyhow!("{field} contains duplicate feature `{feature}`"));
        }
    }
    Ok(())
}

fn validate_boundary(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for required in [
        "existing runtime",
        "rust/local",
        "read-only",
        "no direct trusted writes",
        "graceful no-steam fallback",
        "not layer-3",
        "human/ring-3",
    ] {
        if !lower.contains(&required.to_ascii_lowercase()) {
            return Err(anyhow!("{field} must state boundary `{required}`"));
        }
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_contains(field: &str, value: &str, required: &[&str]) -> Result<()> {
    require_text(field, value)?;
    let lower = value.to_ascii_lowercase();
    for needle in required {
        if !lower.contains(&needle.to_ascii_lowercase()) {
            return Err(anyhow!("{field} must contain `{needle}`"));
        }
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
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_text(field: &str, value: &str) -> Result<()> {
    let trimmed = value.trim();
    if trimmed.is_empty() || trimmed.len() > 768 {
        return Err(anyhow!("{field} must be non-empty text up to 768 bytes"));
    }
    reject_forbidden_wording(field, value)?;
    Ok(())
}

fn require_sha256(field: &str, value: &str) -> Result<()> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(anyhow!("{field} must start with sha256:"));
    };
    if hex.len() != 64 || !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must contain a sha256 digest"));
    }
    Ok(())
}

fn validate_relative_path(field: &str, value: &str) -> Result<()> {
    require_text(field, value)?;
    if value.contains('\\') {
        return Err(anyhow!("{field} must not contain backslashes"));
    }
    let path = Path::new(value);
    if path.is_absolute() || value.starts_with('/') {
        return Err(anyhow!("{field} must be relative"));
    }
    for component in path.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(anyhow!("{field} must be a normalized relative path"));
        }
    }
    Ok(())
}

fn reject_forbidden_wording(field: &str, value: &str) -> Result<()> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "production-ready",
        "godot replacement",
        "automated fun score",
        "auto-merge is authorized",
        "self-approval is authorized",
        "reviewer bypass is authorized",
        "trusted writes are authorized",
        "release button is automated",
        "market demand is automated",
        "layer-3 cloud/mobile is go",
        "password",
        "secret",
        "credential",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden Steamworks wording `{forbidden}`"
            ));
        }
    }
    Ok(())
}
