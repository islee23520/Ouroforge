//! Asset Generation Proposal Model v1 (#1635).
//!
//! Part of Asset Generation and Asset-QA v1 (#1634) under #1 Era G Milestone 36.
//! This module turns a structured asset brief — whose natural-language
//! description is preserved for provenance — into an asset *proposal* carrying
//! license/provenance, validated by trusted Rust/local logic.
//!
//! Boundary: asset generation emits a proposal only. It reuses the existing
//! [`crate::MutationProposal`] model and mirrors the Milestone 30 generation
//! pattern ([`crate::generative_intake`]); it is NOT a new writer. It never
//! performs a trusted write, auto-apply, self-approval, or reviewer bypass, and
//! it never promotes anything. A freshly generated asset proposal is always
//! unverified and pending review — promotion past the asset-QA gate (#1636) is
//! out of scope here. Every generated asset carries license/provenance: a
//! missing or unrecognized license, a missing required attribution, or an
//! off-list source fails closed. This module does not generate pixels; it
//! validates that an assembled asset descriptor is well-formed and licensed
//! before proposing it through the existing review/apply/trust-gradient path.

use crate::export_hash::sha256_hex;
use crate::MutationProposal;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Schema version for the asset brief, the asset proposal provenance, and the
/// attached license/provenance record.
pub const ASSET_GENERATION_SCHEMA_VERSION: &str = "ouroforge.asset-generation-proposal.v1";
/// Schema version for the assembled asset descriptor carried by the proposal.
pub const ASSET_ARTIFACT_SCHEMA_VERSION: &str = "ouroforge.asset.v1";
/// Identifier recorded as the generator of every asset proposal.
pub const ASSET_GENERATION_GENERATOR: &str = "asset-generation-proposal-v1";
/// The intake source recorded in provenance (the proposal originated from an
/// asset brief routed through the asset generation front door).
pub const ASSET_GENERATION_SOURCE: &str = "asset-brief";

/// Sentinel `from` value for a freshly generated proposal that has no prior
/// asset. The existing proposal model requires non-empty text; generation
/// produces a new asset rather than mutating an existing one.
const ASSET_FROM_NONE: &str = "(no prior asset)";

/// Asset kinds supported by the generation model v1.
const ALLOWED_ASSET_KINDS: &[&str] = &["sprite", "tileset", "ui-art"];
/// Asset formats supported by the generation model v1.
const ALLOWED_FORMATS: &[&str] = &["png"];
/// Maximum width/height (in pixels) accepted by the model v1. A larger
/// dimension fails closed as malformed; resolution validity is enforced here at
/// proposal time and again, with style/regression checks, by the asset-QA gate
/// (#1636).
const MAX_DIMENSION: u32 = 4096;

/// Recognized licenses and whether each requires attribution. An unrecognized
/// (or empty) license fails closed: no unlicensed asset is ever proposed.
const RECOGNIZED_LICENSES: &[(&str, bool)] = &[
    ("CC0-1.0", false),
    ("CC-BY-4.0", true),
    ("project-owned", false),
];

/// License and provenance attached to a generated asset. Records the rights and
/// the source chain (generator/model identity, prompt/input reference, upstream
/// references). Read-only audit metadata; it confers no apply or promotion
/// authority. A missing/unrecognized license, a missing required attribution,
/// or an off-list source fails closed.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetLicenseProvenance {
    /// An explicit, recognized license identifier (see [`RECOGNIZED_LICENSES`]).
    pub license: String,
    /// Attribution string, required when the license demands it (e.g. CC-BY).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,
    /// The provenance chain: generator/model identity, prompt or input
    /// reference, and any upstream-asset references.
    pub source: String,
    /// True iff the source is on the allowed-sources list. An off-list source
    /// (`false`) fails closed.
    #[serde(rename = "allowedSource")]
    pub allowed_source: bool,
}

impl AssetLicenseProvenance {
    /// True iff the license requires an attribution string.
    fn requires_attribution(license: &str) -> Option<bool> {
        RECOGNIZED_LICENSES
            .iter()
            .find(|(name, _)| *name == license)
            .map(|(_, requires)| *requires)
    }

    /// Validate the license/provenance, failing closed on any problem.
    pub fn validate(&self) -> Result<()> {
        crate::require_text("asset license", &self.license)?;
        let requires_attribution = Self::requires_attribution(&self.license).ok_or_else(|| {
            anyhow!(
                "asset license \"{}\" is not a recognized license; no unlicensed asset is proposed",
                self.license
            )
        })?;
        if requires_attribution {
            let attribution = self.attribution.as_deref().unwrap_or("");
            crate::require_text("asset license attribution", attribution).map_err(|_| {
                anyhow!(
                    "asset license \"{}\" requires a non-empty attribution",
                    self.license
                )
            })?;
        }
        crate::require_text("asset license source", &self.source)?;
        if !self.allowed_source {
            return Err(anyhow!(
                "asset license source is not on the allowed-sources list (allowedSource=false)"
            ));
        }
        Ok(())
    }
}

/// A structured asset brief: the generation front door. The `description` is the
/// natural-language statement of intent (preserved for provenance); the
/// remaining fields describe the asset to assemble and its license/provenance.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetGenerationBrief {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    pub title: String,
    /// Natural-language description of the desired asset. Preserved verbatim as
    /// the provenance source; never executed.
    pub description: String,
    #[serde(rename = "assetKind")]
    pub asset_kind: String,
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    /// License/provenance attached to the generated asset.
    pub license: AssetLicenseProvenance,
}

impl AssetGenerationBrief {
    /// Parse a brief from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> Result<Self> {
        let brief: AssetGenerationBrief = serde_json::from_str(text)
            .map_err(|err| anyhow!("asset generation brief is not valid JSON: {err}"))?;
        Ok(brief)
    }

    /// Validate the brief structurally, failing closed on any problem. Does not
    /// assemble or validate the resulting descriptor (see
    /// [`generate_asset_proposal`]).
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ASSET_GENERATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "asset generation brief schemaVersion must be \"{ASSET_GENERATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("asset generation brief briefId", &self.brief_id)?;
        crate::require_text("asset generation brief title", &self.title)?;
        crate::require_text("asset generation brief description", &self.description)?;
        crate::require_text("asset generation brief assetId", &self.asset_id)?;
        if !ALLOWED_ASSET_KINDS.contains(&self.asset_kind.as_str()) {
            return Err(anyhow!(
                "asset generation brief assetKind \"{}\" is unsupported; model v1 supports {ALLOWED_ASSET_KINDS:?}",
                self.asset_kind
            ));
        }
        if !ALLOWED_FORMATS.contains(&self.format.as_str()) {
            return Err(anyhow!(
                "asset generation brief format \"{}\" is unsupported; model v1 supports {ALLOWED_FORMATS:?}",
                self.format
            ));
        }
        if self.width == 0 || self.width > MAX_DIMENSION {
            return Err(anyhow!(
                "asset generation brief width must be in 1..={MAX_DIMENSION}, found {}",
                self.width
            ));
        }
        if self.height == 0 || self.height > MAX_DIMENSION {
            return Err(anyhow!(
                "asset generation brief height must be in 1..={MAX_DIMENSION}, found {}",
                self.height
            ));
        }
        self.license.validate()?;
        Ok(())
    }

    /// Deterministic digest over the canonical serialization of the brief.
    pub fn digest(&self) -> Result<String> {
        let canonical = serde_json::to_vec(self)
            .map_err(|err| anyhow!("failed to serialize asset generation brief: {err}"))?;
        Ok(sha256_hex(&canonical))
    }
}

/// Generation provenance attached to an asset proposal: it links the brief to
/// the resulting proposal, records how the proposal was produced, and carries
/// the asset's license/provenance. Read-only audit metadata; it confers no
/// apply or promotion authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetGenerationProvenance {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    /// Deterministic digest of the canonical brief bytes; links proposal to the
    /// exact brief that produced it.
    #[serde(rename = "briefDigest")]
    pub brief_digest: String,
    pub generator: String,
    #[serde(rename = "assetKind")]
    pub asset_kind: String,
    pub source: String,
    /// Always true: generation emits proposals only, never a trusted write.
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
    /// License/provenance carried by the generated asset.
    pub license: AssetLicenseProvenance,
}

impl AssetGenerationProvenance {
    /// Validate the provenance, failing closed on any problem.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != ASSET_GENERATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "asset generation provenance schemaVersion must be \"{ASSET_GENERATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("asset generation provenance briefId", &self.brief_id)?;
        crate::require_text(
            "asset generation provenance briefDigest",
            &self.brief_digest,
        )?;
        if self.brief_digest.len() != 64
            || !self.brief_digest.chars().all(|c| c.is_ascii_hexdigit())
        {
            return Err(anyhow!(
                "asset generation provenance briefDigest must be a 64-character hex digest"
            ));
        }
        if self.generator != ASSET_GENERATION_GENERATOR {
            return Err(anyhow!(
                "asset generation provenance generator must be \"{ASSET_GENERATION_GENERATOR}\""
            ));
        }
        if !ALLOWED_ASSET_KINDS.contains(&self.asset_kind.as_str()) {
            return Err(anyhow!(
                "asset generation provenance assetKind \"{}\" is unsupported",
                self.asset_kind
            ));
        }
        if self.source != ASSET_GENERATION_SOURCE {
            return Err(anyhow!(
                "asset generation provenance source must be \"{ASSET_GENERATION_SOURCE}\""
            ));
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "asset generation provenance proposalOnly must be true: generation emits proposals only"
            ));
        }
        self.license.validate()?;
        Ok(())
    }
}

/// A generated asset proposal: the existing [`MutationProposal`] plus the
/// generation provenance (with license/provenance) that links it to its brief.
/// This wraps — it does not modify — the existing proposal model.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetGenerationProposal {
    pub proposal: MutationProposal,
    pub provenance: AssetGenerationProvenance,
}

impl AssetGenerationProposal {
    /// Validate the wrapped proposal and provenance together, failing closed.
    pub fn validate(&self) -> Result<()> {
        self.proposal.validate()?;
        self.provenance.validate()?;
        if !self
            .proposal
            .evidence_id
            .ends_with(self.provenance.brief_id.as_str())
        {
            return Err(anyhow!(
                "asset generation proposal evidence_id must reference provenance briefId"
            ));
        }
        // The `to` payload is the assembled asset descriptor; it must be
        // well-formed and its embedded license must match the provenance.
        let artifact: Value = serde_json::from_str(&self.proposal.to).map_err(|err| {
            anyhow!("asset generation proposal payload is not a valid asset descriptor: {err}")
        })?;
        validate_asset_artifact(&artifact)?;
        let artifact_license: AssetLicenseProvenance =
            serde_json::from_value(artifact["license"].clone()).map_err(|err| {
                anyhow!("asset descriptor license is not a valid license/provenance record: {err}")
            })?;
        if artifact_license != self.provenance.license {
            return Err(anyhow!(
                "asset descriptor license must match the proposal provenance license"
            ));
        }
        Ok(())
    }

    /// True iff the provenance links this proposal to the given brief: the brief
    /// id matches and the recorded digest equals the brief's canonical digest.
    pub fn links_to(&self, brief: &AssetGenerationBrief) -> Result<bool> {
        Ok(self.provenance.brief_id == brief.brief_id
            && self.provenance.brief_digest == brief.digest()?)
    }
}

/// Assemble the canonical asset descriptor from the author's brief. The
/// descriptor is a manifest-entry-shaped record the existing asset
/// manifest/loader already understands; this module does not generate pixels.
fn assemble_asset_artifact(brief: &AssetGenerationBrief) -> Value {
    json!({
        "schemaVersion": ASSET_ARTIFACT_SCHEMA_VERSION,
        "id": brief.asset_id,
        "kind": brief.asset_kind,
        "format": brief.format,
        "width": brief.width,
        "height": brief.height,
        "license": brief.license,
    })
}

/// Trusted structural validator for an assembled asset descriptor. A fail-closed
/// well-formedness check over the descriptor shape; it is not the asset-QA gate
/// (#1636) and it does not generate or render pixels.
fn validate_asset_artifact(artifact: &Value) -> Result<()> {
    if artifact["schemaVersion"] != ASSET_ARTIFACT_SCHEMA_VERSION {
        return Err(anyhow!(
            "asset descriptor schemaVersion must be \"{ASSET_ARTIFACT_SCHEMA_VERSION}\""
        ));
    }
    let id = artifact["id"]
        .as_str()
        .ok_or_else(|| anyhow!("asset descriptor id must be a string"))?;
    crate::require_text("asset descriptor id", id)?;
    let kind = artifact["kind"]
        .as_str()
        .ok_or_else(|| anyhow!("asset descriptor kind must be a string"))?;
    if !ALLOWED_ASSET_KINDS.contains(&kind) {
        return Err(anyhow!("asset descriptor kind \"{kind}\" is unsupported"));
    }
    let format = artifact["format"]
        .as_str()
        .ok_or_else(|| anyhow!("asset descriptor format must be a string"))?;
    if !ALLOWED_FORMATS.contains(&format) {
        return Err(anyhow!(
            "asset descriptor format \"{format}\" is unsupported"
        ));
    }
    let width = artifact["width"]
        .as_u64()
        .filter(|w| *w > 0 && *w <= u64::from(MAX_DIMENSION))
        .ok_or_else(|| anyhow!("asset descriptor width must be in 1..={MAX_DIMENSION}"))?;
    let height = artifact["height"]
        .as_u64()
        .filter(|h| *h > 0 && *h <= u64::from(MAX_DIMENSION))
        .ok_or_else(|| anyhow!("asset descriptor height must be in 1..={MAX_DIMENSION}"))?;
    let _ = (width, height);
    if !artifact["license"].is_object() {
        return Err(anyhow!(
            "asset descriptor must carry a license/provenance object"
        ));
    }
    Ok(())
}

/// Front-door asset generation: turn a brief into a validated asset proposal
/// with generation provenance and attached license/provenance. Fails closed on
/// a malformed brief, a malformed assembled descriptor, or a missing/invalid
/// license. `now_unix_ms` is supplied by the caller so the result is
/// deterministic and testable; this function never reads the clock, the
/// filesystem, the network, or performs any trusted write.
pub fn generate_asset_proposal(
    brief: &AssetGenerationBrief,
    now_unix_ms: u128,
) -> Result<AssetGenerationProposal> {
    brief.validate()?;

    let artifact = assemble_asset_artifact(brief);
    validate_asset_artifact(&artifact)?;
    let artifact_json = serde_json::to_string(&artifact)
        .map_err(|err| anyhow!("failed to serialize asset descriptor: {err}"))?;

    let brief_digest = brief.digest()?;
    let evidence_id = format!("asset-generation/{}", brief.brief_id);

    // Build the proposal directly via the existing model. Generation is
    // proposal-only: it does not bind to a run directory, read evidence from
    // disk, or perform any trusted write. A freshly generated proposal is
    // proposed/pending — it has not passed the asset-QA gate (#1636).
    let proposal = MutationProposal {
        id: format!("asset-{}", brief.asset_id),
        reason: format!(
            "Generated {} asset proposal from brief: {}",
            brief.asset_kind, brief.title
        ),
        evidence_id: evidence_id.clone(),
        target: brief.asset_kind.clone(),
        path: format!(
            "assets/{}/{}.{}",
            brief.asset_kind, brief.asset_id, brief.format
        ),
        from: ASSET_FROM_NONE.to_string(),
        to: artifact_json,
        confidence: "unverified".to_string(),
        status: "proposed".to_string(),
        verdict_status: "pending".to_string(),
        created_at_unix_ms: now_unix_ms,
        rationale: None,
    };

    let provenance = AssetGenerationProvenance {
        schema_version: ASSET_GENERATION_SCHEMA_VERSION.to_string(),
        brief_id: brief.brief_id.clone(),
        brief_digest,
        generator: ASSET_GENERATION_GENERATOR.to_string(),
        asset_kind: brief.asset_kind.clone(),
        source: ASSET_GENERATION_SOURCE.to_string(),
        proposal_only: true,
        license: brief.license.clone(),
    };

    let generated = AssetGenerationProposal {
        proposal,
        provenance,
    };
    generated.validate()?;
    Ok(generated)
}
