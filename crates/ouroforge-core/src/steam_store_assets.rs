//! Steam Store Asset Generation v1 (#1840).
//!
//! Reuses the Milestone 36 [`crate::asset_generation_proposal`] front door to
//! produce Steam store asset proposals (capsules, screenshots, trailer frames)
//! at pinned Steam dimensions. This is not a new image pipeline and it does not
//! generate pixels, upload to Steam, submit store metadata, or perform trusted
//! writes. Rust/local validates specs and license/provenance; humans submit the
//! resulting proposals through the existing review/apply/trust-gradient path.

use crate::asset_generation_proposal::{
    generate_asset_proposal, AssetGenerationBrief, AssetGenerationProposal, AssetLicenseProvenance,
    ASSET_GENERATION_SCHEMA_VERSION,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const STEAM_STORE_ASSETS_SCHEMA_VERSION: &str = "ouroforge.steam-store-assets.v1";
pub const STEAM_STORE_ASSETS_GENERATOR: &str = "steam-store-assets-v1";
pub const STEAM_STORE_ASSETS_BOUNDARY: &str = "Rust/local Steam store asset proposals only; reuses Milestone 36 asset generation, human submits to Steam, no trusted write, no upload, no release authority, browser/Studio read-only.";

const FORMAT_PNG: &str = "png";
const PIPELINE_ASSET_KIND: &str = "ui-art";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SteamStoreAssetSpec {
    pub slot: &'static str,
    pub kind: &'static str,
    pub width: u32,
    pub height: u32,
}

pub const STEAM_STORE_ASSET_SPECS: &[SteamStoreAssetSpec] = &[
    SteamStoreAssetSpec {
        slot: "capsule_header",
        kind: "capsule",
        width: 460,
        height: 215,
    },
    SteamStoreAssetSpec {
        slot: "capsule_small",
        kind: "capsule",
        width: 231,
        height: 87,
    },
    SteamStoreAssetSpec {
        slot: "capsule_main",
        kind: "capsule",
        width: 616,
        height: 353,
    },
    SteamStoreAssetSpec {
        slot: "capsule_vertical",
        kind: "capsule",
        width: 374,
        height: 448,
    },
    SteamStoreAssetSpec {
        slot: "screenshot_1080p",
        kind: "screenshot",
        width: 1920,
        height: 1080,
    },
    SteamStoreAssetSpec {
        slot: "trailer_frame_1080p",
        kind: "trailer-frame",
        width: 1920,
        height: 1080,
    },
];

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamStoreAssetPlan {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "planId")]
    pub plan_id: String,
    pub title: String,
    #[serde(rename = "reusePipeline")]
    pub reuse_pipeline: String,
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
    #[serde(rename = "humanSubmissionRequired")]
    pub human_submission_required: bool,
    pub boundary: String,
    pub assets: Vec<SteamStoreAssetRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SteamStoreAssetRequest {
    pub slot: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub title: String,
    pub description: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub license: AssetLicenseProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSteamStoreAsset {
    pub slot: String,
    pub kind: String,
    pub width: u32,
    pub height: u32,
    pub proposal: AssetGenerationProposal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedSteamStoreAssetPlan {
    pub plan_id: String,
    pub proposal_only: bool,
    pub human_submission_required: bool,
    pub assets: Vec<GeneratedSteamStoreAsset>,
}

impl SteamStoreAssetPlan {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let plan: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("steam store asset plan is not valid JSON: {err}"))?;
        plan.validate()?;
        Ok(plan)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != STEAM_STORE_ASSETS_SCHEMA_VERSION {
            return Err(anyhow!(
                "steam store asset plan schemaVersion must be {STEAM_STORE_ASSETS_SCHEMA_VERSION}"
            ));
        }
        crate::require_text("steam store asset plan planId", &self.plan_id)?;
        crate::require_text("steam store asset plan title", &self.title)?;
        if self.reuse_pipeline != "milestone-36-asset-generation-proposal" {
            return Err(anyhow!(
                "steam store asset plan must reuse Milestone 36 asset generation proposal pipeline"
            ));
        }
        if !self.proposal_only || !self.human_submission_required {
            return Err(anyhow!(
                "steam store assets are proposal-only and require human Steam submission"
            ));
        }
        crate::require_text("steam store asset plan boundary", &self.boundary)?;
        for required in [
            "Rust/local",
            "Milestone 36",
            "human submits",
            "no trusted write",
            "browser/Studio read-only",
        ] {
            if !self.boundary.contains(required) {
                return Err(anyhow!(
                    "steam store asset boundary must state `{required}`"
                ));
            }
        }
        if self.assets.is_empty() {
            return Err(anyhow!("steam store asset plan assets must not be empty"));
        }
        for asset in &self.assets {
            asset.validate()?;
        }
        Ok(())
    }
}

impl SteamStoreAssetRequest {
    pub fn validate(&self) -> Result<()> {
        let spec = steam_store_asset_spec(&self.slot).ok_or_else(|| {
            anyhow!(
                "steam store asset slot `{}` is not a supported Steam spec",
                self.slot
            )
        })?;
        crate::require_text("steam store asset assetId", &self.asset_id)?;
        crate::require_text("steam store asset title", &self.title)?;
        crate::require_text("steam store asset description", &self.description)?;
        if self.format != FORMAT_PNG {
            return Err(anyhow!(
                "steam store asset format must be png for the reused Milestone 36 pipeline"
            ));
        }
        if self.width != spec.width || self.height != spec.height {
            return Err(anyhow!(
                "steam store asset `{}` must be {}x{}, found {}x{}",
                self.slot,
                spec.width,
                spec.height,
                self.width,
                self.height
            ));
        }
        self.license.validate()?;
        Ok(())
    }
}

pub fn steam_store_asset_spec(slot: &str) -> Option<SteamStoreAssetSpec> {
    STEAM_STORE_ASSET_SPECS
        .iter()
        .copied()
        .find(|spec| spec.slot == slot)
}

pub fn generate_steam_store_asset_plan(
    plan: &SteamStoreAssetPlan,
    now_unix_ms: u128,
) -> Result<GeneratedSteamStoreAssetPlan> {
    plan.validate()?;
    let mut assets = Vec::with_capacity(plan.assets.len());
    for asset in &plan.assets {
        let spec = steam_store_asset_spec(&asset.slot).expect("validated slot has spec");
        let brief = AssetGenerationBrief {
            schema_version: ASSET_GENERATION_SCHEMA_VERSION.to_string(),
            brief_id: format!("{}-{}", plan.plan_id, asset.slot),
            title: asset.title.clone(),
            description: format!(
                "Steam store {} proposal for {}. Reuses Milestone 36 asset generation; human submits to Steam.",
                asset.slot, asset.description
            ),
            asset_kind: PIPELINE_ASSET_KIND.to_string(),
            asset_id: asset.asset_id.clone(),
            format: asset.format.clone(),
            width: asset.width,
            height: asset.height,
            license: asset.license.clone(),
        };
        let proposal = generate_asset_proposal(&brief, now_unix_ms)?;
        assets.push(GeneratedSteamStoreAsset {
            slot: asset.slot.clone(),
            kind: spec.kind.to_string(),
            width: spec.width,
            height: spec.height,
            proposal,
        });
    }
    Ok(GeneratedSteamStoreAssetPlan {
        plan_id: plan.plan_id.clone(),
        proposal_only: true,
        human_submission_required: true,
        assets,
    })
}
