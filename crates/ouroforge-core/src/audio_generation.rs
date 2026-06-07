//! Audio Generation Proposal Model v1 (#1642).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! This module turns a structured audio brief — whose natural-language
//! description is preserved for provenance — into an audio asset *proposal*,
//! validated by trusted Rust/local logic, carrying the license and generation
//! provenance that link the brief to the resulting proposal.
//!
//! Boundary: audio generation is the generative FRONT DOOR for audio. It emits a
//! proposal only. It reuses the existing [`crate::MutationProposal`] model (it is
//! not a new writer), it never performs a trusted write, auto-apply,
//! self-approval, or reviewer bypass, and it never promotes anything. Generated
//! audio is always unverified and pending review: it must route through the
//! existing review/apply/trust-gradient path (see [`AudioGenerationProposal::to_risk_tier_descriptor`])
//! and clear the audio-QA gate (#1643) before it could ever be promoted.
//! License and provenance are mandatory; a proposal with a missing license or a
//! malformed audio descriptor is rejected fail-closed. This module records
//! intent metadata only — it does not synthesize, mix, decode, or play audio.

use crate::export_hash::sha256_hex;
use crate::trust_gradient_risk_tier::{
    GateOutcome, MutationKind, MutationProposalDescriptor, TrustGradientGateVerdicts,
    TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION,
};
use crate::MutationProposal;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Schema version for the audio brief and the audio generation provenance.
pub const AUDIO_GENERATION_SCHEMA_VERSION: &str = "ouroforge.audio-generation.v1";
/// Schema version for the assembled audio asset descriptor (the proposal payload).
pub const AUDIO_ASSET_SCHEMA_VERSION: &str = "ouroforge.audio-asset.v1";
/// Identifier recorded as the generator of every audio proposal.
pub const AUDIO_GENERATION_GENERATOR: &str = "audio-generation-v1";
/// The intake source recorded in provenance (the proposal originated from a
/// brief routed through the audio generation front door).
pub const AUDIO_GENERATION_SOURCE: &str = "brief";
/// The mutation-proposal `target` for a generated audio asset. Matches the
/// existing `ProjectAssetType::Audio` classification.
pub const AUDIO_GENERATION_TARGET: &str = "audio";

/// Sentinel `from` value for a freshly generated proposal with no prior artifact.
/// The existing proposal model requires non-empty text; generation produces a new
/// artifact rather than mutating an existing one.
const GENERATIVE_FROM_NONE: &str = "(no prior artifact)";

/// Audio kinds supported by audio generation v1.
pub const SUPPORTED_AUDIO_KINDS: &[&str] = &["sfx", "music"];
/// Audio container formats supported by audio generation v1. Mirrors the existing
/// asset-manifest audio classification (`ogg`/`mp3`/`wav`).
pub const SUPPORTED_AUDIO_FORMATS: &[&str] = &["ogg", "mp3", "wav"];
/// Maximum channel count accepted (mono or stereo). Anything else is malformed.
pub const MAX_AUDIO_CHANNELS: u8 = 2;
/// Bounded maximum audio duration (10 minutes). Keeps generated audio bounded and
/// fails closed on absent/implausible durations.
pub const MAX_AUDIO_DURATION_MS: u64 = 600_000;

/// License and credit for a generated audio asset. All fields are mandatory;
/// unlicensed or uncredited audio can never be promoted, so a brief that omits or
/// blanks any of these is rejected fail-closed.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AudioLicense {
    /// License identifier (for example `CC0-1.0`, `CC-BY-4.0`, `MIT`).
    #[serde(rename = "licenseId")]
    pub license_id: String,
    /// Rights holder / author credited for the audio.
    pub holder: String,
    /// Origin of the audio (generator id, dataset, or source reference).
    pub source: String,
}

impl AudioLicense {
    /// Validate the license, failing closed on any missing field.
    pub fn validate(&self) -> Result<()> {
        crate::require_text("audio license licenseId", &self.license_id)?;
        crate::require_text("audio license holder", &self.holder)?;
        crate::require_text("audio license source", &self.source)?;
        Ok(())
    }
}

/// A structured audio brief: the front-door intake for audio generation. The
/// `description` is the natural-language statement of intent (preserved for
/// provenance); the remaining fields describe the desired audio asset. `license`
/// is optional in the wire format only so an omitted license can be represented
/// and rejected with a clear message rather than failing to parse.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AudioGenerationBrief {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    pub title: String,
    /// Natural-language description of the desired audio. Preserved verbatim as
    /// part of provenance; never executed.
    pub description: String,
    #[serde(rename = "audioKind")]
    pub audio_kind: String,
    #[serde(rename = "audioId")]
    pub audio_id: String,
    pub format: String,
    #[serde(rename = "durationMs")]
    pub duration_ms: u64,
    pub channels: u8,
    /// License/provenance for the audio. Mandatory; absence is rejected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<AudioLicense>,
}

/// Generation provenance attached to an audio proposal: it links the brief to the
/// resulting proposal, carries the license, and records how the proposal was
/// produced. Read-only audit metadata; it confers no apply or promotion authority.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AudioGenerationProvenance {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "briefId")]
    pub brief_id: String,
    /// Deterministic digest of the canonical brief bytes; links proposal to the
    /// exact brief that produced it.
    #[serde(rename = "briefDigest")]
    pub brief_digest: String,
    pub generator: String,
    #[serde(rename = "audioKind")]
    pub audio_kind: String,
    pub source: String,
    /// Always true: audio generation emits proposals only, never a trusted write.
    #[serde(rename = "proposalOnly")]
    pub proposal_only: bool,
    /// License/credit carried with the proposal. Mandatory.
    pub license: AudioLicense,
    /// Deterministic digest of the canonical audio asset descriptor.
    #[serde(rename = "assetDigest")]
    pub asset_digest: String,
}

/// A generated audio proposal: the existing [`MutationProposal`] plus the audio
/// generation provenance (including license) that links it to its brief. This
/// wraps — it does not modify — the existing proposal model.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AudioGenerationProposal {
    pub proposal: MutationProposal,
    pub provenance: AudioGenerationProvenance,
}

impl AudioGenerationBrief {
    /// Parse a brief from JSON, failing closed on malformed JSON.
    pub fn from_json_str(text: &str) -> Result<Self> {
        let brief: AudioGenerationBrief = serde_json::from_str(text)
            .map_err(|err| anyhow!("audio brief is not valid JSON: {err}"))?;
        Ok(brief)
    }

    /// Validate the brief structurally, failing closed on any problem — including
    /// a missing or incomplete license. Does not validate the assembled audio
    /// asset descriptor (see [`validate_audio_asset`]).
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != AUDIO_GENERATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "audio brief schemaVersion must be \"{AUDIO_GENERATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("audio brief briefId", &self.brief_id)?;
        crate::require_text("audio brief title", &self.title)?;
        crate::require_text("audio brief description", &self.description)?;
        crate::require_text("audio brief audioId", &self.audio_id)?;
        match &self.license {
            None => {
                return Err(anyhow!(
                    "audio brief requires a license: unlicensed audio can never be promoted"
                ));
            }
            Some(license) => license.validate()?,
        }
        Ok(())
    }

    /// Deterministic digest over the canonical serialization of the brief.
    pub fn digest(&self) -> Result<String> {
        let canonical = serde_json::to_vec(self)
            .map_err(|err| anyhow!("failed to serialize audio brief: {err}"))?;
        Ok(sha256_hex(&canonical))
    }
}

impl AudioGenerationProvenance {
    /// Validate the provenance, failing closed on any problem.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != AUDIO_GENERATION_SCHEMA_VERSION {
            return Err(anyhow!(
                "audio generation provenance schemaVersion must be \"{AUDIO_GENERATION_SCHEMA_VERSION}\""
            ));
        }
        crate::require_text("audio generation provenance briefId", &self.brief_id)?;
        require_hex_digest(
            "audio generation provenance briefDigest",
            &self.brief_digest,
        )?;
        require_hex_digest(
            "audio generation provenance assetDigest",
            &self.asset_digest,
        )?;
        if self.generator != AUDIO_GENERATION_GENERATOR {
            return Err(anyhow!(
                "audio generation provenance generator must be \"{AUDIO_GENERATION_GENERATOR}\""
            ));
        }
        if !SUPPORTED_AUDIO_KINDS.contains(&self.audio_kind.as_str()) {
            return Err(anyhow!(
                "audio generation provenance audioKind \"{}\" is unsupported",
                self.audio_kind
            ));
        }
        if self.source != AUDIO_GENERATION_SOURCE {
            return Err(anyhow!(
                "audio generation provenance source must be \"{AUDIO_GENERATION_SOURCE}\""
            ));
        }
        if !self.proposal_only {
            return Err(anyhow!(
                "audio generation provenance proposalOnly must be true: generation emits proposals only"
            ));
        }
        self.license.validate()?;
        Ok(())
    }
}

impl AudioGenerationProposal {
    /// Validate the wrapped proposal and provenance together, failing closed.
    pub fn validate(&self) -> Result<()> {
        self.proposal.validate()?;
        self.provenance.validate()?;
        validate_audio_asset(&self.parse_asset()?)?;
        if !self
            .proposal
            .evidence_id
            .ends_with(self.provenance.brief_id.as_str())
        {
            return Err(anyhow!(
                "audio proposal evidence_id must reference provenance briefId"
            ));
        }
        // The provenance digest must match the proposal payload it describes.
        if sha256_hex(self.proposal.to.as_bytes()) != self.provenance.asset_digest {
            return Err(anyhow!(
                "audio proposal payload digest does not match provenance assetDigest"
            ));
        }
        Ok(())
    }

    /// Parse the audio asset descriptor carried in the proposal `to` payload.
    pub fn parse_asset(&self) -> Result<Value> {
        serde_json::from_str(&self.proposal.to)
            .map_err(|err| anyhow!("audio proposal payload is not a valid audio asset: {err}"))
    }

    /// True iff the provenance links this proposal to the given brief: the brief
    /// id matches and the recorded digest equals the brief's canonical digest.
    pub fn links_to(&self, brief: &AudioGenerationBrief) -> Result<bool> {
        Ok(self.provenance.brief_id == brief.brief_id
            && self.provenance.brief_digest == brief.digest()?)
    }

    /// Build the trust-gradient risk-tier descriptor for this proposal so it can
    /// be routed through the existing review/apply/trust-gradient path. A freshly
    /// generated audio proposal is unverified (no gates passed, no fresh refs, no
    /// confidence) and is an asset/manifest-affecting change, so it is never
    /// auto-apply eligible — it always resolves to manual-only review and must
    /// clear the audio-QA gate before promotion.
    pub fn to_risk_tier_descriptor(&self) -> MutationProposalDescriptor {
        MutationProposalDescriptor {
            schema_version: TRUST_GRADIENT_RISK_TIER_SCHEMA_VERSION.to_string(),
            proposal_ref: self.proposal.id.clone(),
            mutation_kind: MutationKind::Manifest,
            scope_paths: vec![self.proposal.path.clone()],
            confidence: None,
            gates: TrustGradientGateVerdicts {
                mechanical: GateOutcome::Missing,
                runtime: GateOutcome::Missing,
                visual: GateOutcome::Missing,
                semantic: GateOutcome::Missing,
            },
            refs_fresh: false,
        }
    }
}

/// Require a 64-character lowercase-or-mixed hex digest, failing closed.
fn require_hex_digest(field: &str, value: &str) -> Result<()> {
    if value.len() != 64 || !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!("{field} must be a 64-character hex digest"));
    }
    Ok(())
}

/// Assemble the canonical audio asset descriptor from the author's brief. The
/// license is embedded so the asset carries its provenance.
fn assemble_audio_asset(brief: &AudioGenerationBrief, license: &AudioLicense) -> Value {
    json!({
        "schemaVersion": AUDIO_ASSET_SCHEMA_VERSION,
        "audioId": brief.audio_id,
        "kind": brief.audio_kind,
        "format": brief.format,
        "durationMs": brief.duration_ms,
        "channels": brief.channels,
        "license": {
            "licenseId": license.license_id,
            "holder": license.holder,
            "source": license.source,
        },
    })
}

/// Trusted structural validator for an assembled audio asset descriptor. This is
/// a fail-closed well-formedness check over the descriptor shape; it is not the
/// audio-QA gate (#1643). Rejects malformed audio: unsupported kind/format,
/// out-of-range duration, or an invalid channel count.
pub fn validate_audio_asset(asset: &Value) -> Result<()> {
    if asset["schemaVersion"] != AUDIO_ASSET_SCHEMA_VERSION {
        return Err(anyhow!(
            "audio asset schemaVersion must be \"{AUDIO_ASSET_SCHEMA_VERSION}\""
        ));
    }
    let kind = asset["kind"]
        .as_str()
        .ok_or_else(|| anyhow!("audio asset kind must be a string"))?;
    if !SUPPORTED_AUDIO_KINDS.contains(&kind) {
        return Err(anyhow!("audio asset kind \"{kind}\" is unsupported"));
    }
    let format = asset["format"]
        .as_str()
        .ok_or_else(|| anyhow!("audio asset format must be a string"))?;
    if !SUPPORTED_AUDIO_FORMATS.contains(&format) {
        return Err(anyhow!(
            "audio asset format \"{format}\" is unsupported; expected one of {SUPPORTED_AUDIO_FORMATS:?}"
        ));
    }
    let duration_ms = asset["durationMs"]
        .as_u64()
        .ok_or_else(|| anyhow!("audio asset durationMs must be a non-negative integer"))?;
    if duration_ms == 0 || duration_ms > MAX_AUDIO_DURATION_MS {
        return Err(anyhow!(
            "audio asset durationMs {duration_ms} is out of range (1..={MAX_AUDIO_DURATION_MS})"
        ));
    }
    let channels = asset["channels"]
        .as_u64()
        .ok_or_else(|| anyhow!("audio asset channels must be a non-negative integer"))?;
    if channels < 1 || channels > MAX_AUDIO_CHANNELS as u64 {
        return Err(anyhow!(
            "audio asset channels {channels} is out of range (1..={MAX_AUDIO_CHANNELS})"
        ));
    }
    Ok(())
}

/// Front-door audio generation: turn a brief into a validated audio asset
/// proposal carrying license and generation provenance. Fails closed on a
/// malformed brief, a missing license, or a malformed audio descriptor.
/// `now_unix_ms` is supplied by the caller so the result is deterministic and
/// testable; this function never reads the clock, the filesystem, the network, or
/// performs any trusted write.
pub fn generate_audio(
    brief: &AudioGenerationBrief,
    now_unix_ms: u128,
) -> Result<AudioGenerationProposal> {
    brief.validate()?;
    // `validate()` guarantees the license is present.
    let license = brief
        .license
        .clone()
        .expect("brief.validate guarantees a license");

    let asset = assemble_audio_asset(brief, &license);
    validate_audio_asset(&asset)?;
    let asset_json = serde_json::to_string(&asset)
        .map_err(|err| anyhow!("failed to serialize audio asset: {err}"))?;

    let brief_digest = brief.digest()?;
    let asset_digest = sha256_hex(asset_json.as_bytes());
    let evidence_id = format!("audio-generation/{}", brief.brief_id);

    // Build the proposal directly via the existing model. Audio generation is
    // proposal-only: a freshly generated proposal is proposed/pending — it has
    // not cleared the audio-QA gate (#1643) and is never auto-applied.
    let proposal = MutationProposal {
        id: format!("audio-generation-{}", brief.audio_id),
        reason: format!(
            "Generated {} audio proposal from brief: {}",
            brief.audio_kind, brief.title
        ),
        evidence_id: evidence_id.clone(),
        target: AUDIO_GENERATION_TARGET.to_string(),
        path: format!("audio/{}.{}", brief.audio_id, brief.format),
        from: GENERATIVE_FROM_NONE.to_string(),
        to: asset_json,
        confidence: "unverified".to_string(),
        status: "proposed".to_string(),
        verdict_status: "pending".to_string(),
        created_at_unix_ms: now_unix_ms,
        rationale: None,
    };

    let provenance = AudioGenerationProvenance {
        schema_version: AUDIO_GENERATION_SCHEMA_VERSION.to_string(),
        brief_id: brief.brief_id.clone(),
        brief_digest,
        generator: AUDIO_GENERATION_GENERATOR.to_string(),
        audio_kind: brief.audio_kind.clone(),
        source: AUDIO_GENERATION_SOURCE.to_string(),
        proposal_only: true,
        license,
        asset_digest,
    };

    let generated = AudioGenerationProposal {
        proposal,
        provenance,
    };
    generated.validate()?;
    Ok(generated)
}
