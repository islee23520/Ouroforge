//! Optional human channel contract v1 (#2042 / Era L M72).
//!
//! This contract describes read-only, optional human oversight/taste/escape-hatch
//! surfaces over existing evidence artifacts. It is not a verifier, runner,
//! source-apply authority, trusted write path, or data plane, and it must never
//! block the autonomous self-improvement loop.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

pub const OPTIONAL_HUMAN_CHANNEL_CONTRACT_SCHEMA_VERSION: &str =
    "optional-human-channel-contract-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanChannelContract {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "oversightSurface")]
    pub oversight_surface: OptionalHumanSurface,
    #[serde(rename = "escapeHatch")]
    pub escape_hatch: OptionalHumanSurface,
    #[serde(rename = "tasteFeedback")]
    pub taste_feedback: OptionalHumanSurface,
    #[serde(rename = "requiredPipelineRefs")]
    pub required_pipeline_refs: Vec<String>,
    #[serde(rename = "reuseMilestoneRefs")]
    pub reuse_milestone_refs: Vec<String>,
    #[serde(rename = "autonomousLoopBlocksOnChannel")]
    pub autonomous_loop_blocks_on_channel: bool,
    #[serde(rename = "noNewVerificationEngine")]
    pub no_new_verification_engine: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanSurface {
    pub name: String,
    pub purpose: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub optional: bool,
    #[serde(rename = "blocksAutonomousLoop")]
    pub blocks_autonomous_loop: bool,
    #[serde(rename = "allowedInputs")]
    pub allowed_inputs: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

impl OptionalHumanChannelContract {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let contract: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse optional human channel contract: {err}"))?;
        contract.validate()?;
        Ok(contract)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != OPTIONAL_HUMAN_CHANNEL_CONTRACT_SCHEMA_VERSION {
            return Err(anyhow!(
                "optional human channel schemaVersion must be {OPTIONAL_HUMAN_CHANNEL_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        require_id("titleId", &self.title_id)?;
        self.oversight_surface.validate("oversightSurface")?;
        self.escape_hatch.validate("escapeHatch")?;
        self.taste_feedback.validate("tasteFeedback")?;
        validate_pipeline_refs(&self.required_pipeline_refs)?;
        let reuse = self.reuse_milestone_refs.join("\n").to_ascii_lowercase();
        for required in ["m57", "m58", "playtest", "fun", "taste", "provenance"] {
            if !reuse.contains(required) {
                return Err(anyhow!("reuseMilestoneRefs must mention {required}"));
            }
        }
        if self.autonomous_loop_blocks_on_channel {
            return Err(anyhow!(
                "optional human channel must never block the autonomous loop"
            ));
        }
        if !(self.no_new_verification_engine && self.no_new_data_plane) {
            return Err(anyhow!(
                "optional human channel must not introduce a verifier or data plane"
            ));
        }
        let boundary = self.boundary.to_ascii_lowercase();
        for required in [
            "optional",
            "non-blocking",
            "read-only",
            "stage health",
            "blocker",
            "diagnosis",
            "attribution",
            "escape-hatch",
            "taste",
            "fun",
            "provenance",
            "openchrome",
            "verdict",
            "journal.md",
            "ledger.jsonl",
            "loop-coverage",
            "source-apply",
            "trust-gradient",
            "no new verification engine",
            "no new data plane",
            "#1 and #23 remain open",
        ] {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "optional human channel boundary must mention {required}"
                ));
            }
        }
        Ok(())
    }
}

impl OptionalHumanSurface {
    fn validate(&self, label: &str) -> Result<()> {
        require_text(&format!("{label}.name"), &self.name)?;
        require_text(&format!("{label}.purpose"), &self.purpose)?;
        if !(self.read_only && self.optional) {
            return Err(anyhow!("{label} must be read-only and optional"));
        }
        if self.blocks_autonomous_loop {
            return Err(anyhow!("{label} must not block the autonomous loop"));
        }
        require_nonempty(&format!("{label}.allowedInputs"), self.allowed_inputs.len())?;
        require_nonempty(
            &format!("{label}.forbiddenActions"),
            self.forbidden_actions.len(),
        )?;
        validate_pipeline_refs(&self.evidence_refs)?;
        let forbidden = self.forbidden_actions.join("\n").to_ascii_lowercase();
        for required in [
            "trusted_write",
            "source_apply",
            "auto_apply",
            "block_loop",
            "new_verifier",
            "new_data_plane",
        ] {
            if !forbidden.contains(required) {
                return Err(anyhow!("{label}.forbiddenActions must include {required}"));
            }
        }
        Ok(())
    }
}

fn validate_pipeline_refs(refs: &[String]) -> Result<()> {
    require_nonempty("pipeline refs", refs.len())?;
    let refs = refs.join("\n").to_ascii_lowercase();
    for required in [
        "openchrome",
        "verdict",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
    ] {
        if !refs.contains(required) {
            return Err(anyhow!("pipeline refs must include {required}"));
        }
    }
    Ok(())
}

fn require_id(label: &str, value: &str) -> Result<()> {
    let valid = !value.trim().is_empty()
        && value
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    if !valid {
        return Err(anyhow!("{label} must be a non-empty local id"));
    }
    Ok(())
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    Ok(())
}

fn require_nonempty(label: &str, len: usize) -> Result<()> {
    if len == 0 {
        return Err(anyhow!("{label} must not be empty"));
    }
    Ok(())
}
