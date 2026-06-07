//! Audio-QA Check v1 (#1643).
//!
//! Part of Audio Generation and Audio-QA v1 (#1641) under #1 Era G Milestone 37.
//! This is the audio ENGINE ROOM: a function-specific QA check that decides
//! whether a generated audio proposal (Audio Generation Proposal Model v1, #1642)
//! is fit to be promoted. It checks four things and fails closed on any problem:
//!
//! 1. **format validity** — supported container/codec, kind, channels, duration;
//! 2. **loudness validity** — measured integrated loudness and true peak within a
//!    declared bounded range;
//! 3. **license/provenance completeness** — a complete license and at least one
//!    generation-provenance reference;
//! 4. **regression vs baseline** — measured loudness within an allowed drift of a
//!    declared baseline.
//!
//! Boundary: this is an audio-QA *check*, not a new evaluator. It reuses the
//! existing audio-generation contract (#1642) for the format/license vocabulary
//! and composes into the existing evaluator aggregation by emitting a declared
//! gate verdict (`declared`/`status`/`resultCount`/`failureCount`) consumable
//! under the evaluator's `declared-gate-and` operator (see
//! [`AudioQaArtifact::gate_verdict`]). It performs no trusted write, no auto-apply,
//! and makes no quality/taste judgement — "sounds good" stays a human decision.
//! It only proves format/loudness validity, license/provenance completeness, and
//! non-regression. Unlicensed or invalid audio can never pass.

use crate::audio_generation::{
    AudioLicense, MAX_AUDIO_CHANNELS, MAX_AUDIO_DURATION_MS, SUPPORTED_AUDIO_FORMATS,
    SUPPORTED_AUDIO_KINDS,
};
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const AUDIO_QA_SCHEMA_VERSION: &str = "audio-qa-v1";

const BOUNDARY_REQUIRED_PHRASES: &[&str] = &["read-only", "no auto-apply"];

/// Classification of an audio-QA check. Fail-closed: anything that is not a
/// clean pass is a fail or a blocked (stale) outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AudioQaStatus {
    Pass,
    Fail,
    Stale,
}

impl AudioQaStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Stale => "stale",
        }
    }
}

/// Measured loudness for the candidate audio plus the declared acceptable range.
/// Loudness units: integrated loudness in LUFS, true peak in dBTP (both negative
/// in practice).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioLoudness {
    #[serde(rename = "integratedLufs")]
    pub integrated_lufs: f64,
    #[serde(rename = "truePeakDbtp")]
    pub true_peak_dbtp: f64,
    #[serde(rename = "minLufs")]
    pub min_lufs: f64,
    #[serde(rename = "maxLufs")]
    pub max_lufs: f64,
    #[serde(rename = "maxTruePeakDbtp")]
    pub max_true_peak_dbtp: f64,
}

/// Declared loudness baseline the candidate is compared against.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioBaseline {
    #[serde(rename = "baselineRef")]
    pub baseline_ref: String,
    #[serde(rename = "integratedLufs")]
    pub integrated_lufs: f64,
}

/// An audio-QA check artifact: the candidate audio descriptor, its measured
/// loudness, its license/provenance, and the regression baseline, together with a
/// declared status that must equal the computed classification.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AudioQaArtifact {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "checkId")]
    pub check_id: String,
    /// Declared status; must equal the computed classification.
    pub status: String,
    /// Reference to the audio proposal/asset under test.
    #[serde(rename = "assetRef")]
    pub asset_ref: String,
    pub format: String,
    pub kind: String,
    pub channels: u8,
    #[serde(rename = "durationMs")]
    pub duration_ms: u64,
    /// License/credit carried by the candidate audio. Mandatory and complete.
    pub license: AudioLicense,
    /// Generation-provenance references (proposal-only lineage). At least one is
    /// required; an empty list fails closed.
    #[serde(rename = "provenanceRefs", default)]
    pub provenance_refs: Vec<String>,
    pub loudness: AudioLoudness,
    #[serde(rename = "baselineRefs", default)]
    pub baseline_refs: Vec<String>,
    #[serde(rename = "staleBaselineRefs", default)]
    pub stale_baseline_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<AudioBaseline>,
    /// Maximum allowed loudness drift (in LU) from the baseline before it is a
    /// regression.
    #[serde(rename = "maxLoudnessDriftLu")]
    pub max_loudness_drift_lu: f64,
    pub verdict: String,
    #[serde(rename = "blockedReasons", default)]
    pub blocked_reasons: Vec<String>,
    pub boundary: String,
}

/// A single audio-QA failure reason.
#[derive(Debug, Clone, PartialEq)]
pub struct AudioQaFailure {
    pub code: String,
    pub detail: String,
}

impl AudioQaArtifact {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let artifact: Self =
            serde_json::from_str(input).context("failed to parse audio-QA check JSON")?;
        artifact.validate()?;
        Ok(artifact)
    }

    /// All present failure reasons (loudness, provenance, regression). Independent
    /// of the stale/blocked precedence in [`Self::computed_status`].
    pub fn failures(&self) -> Vec<AudioQaFailure> {
        let mut failures = Vec::new();

        if self.provenance_refs.is_empty() {
            failures.push(AudioQaFailure {
                code: "missing_provenance".to_string(),
                detail: "no generation-provenance references; provenance is mandatory".to_string(),
            });
        }

        if self.loudness.integrated_lufs < self.loudness.min_lufs
            || self.loudness.integrated_lufs > self.loudness.max_lufs
        {
            failures.push(AudioQaFailure {
                code: "loudness_out_of_range".to_string(),
                detail: format!(
                    "integrated loudness {} LUFS outside [{}, {}]",
                    self.loudness.integrated_lufs, self.loudness.min_lufs, self.loudness.max_lufs
                ),
            });
        }
        if self.loudness.true_peak_dbtp > self.loudness.max_true_peak_dbtp {
            failures.push(AudioQaFailure {
                code: "true_peak_exceeded".to_string(),
                detail: format!(
                    "true peak {} dBTP exceeds max {} dBTP",
                    self.loudness.true_peak_dbtp, self.loudness.max_true_peak_dbtp
                ),
            });
        }

        // Regression vs baseline. A declared baseline ref with no baseline value
        // is a missing baseline (fail closed); a present baseline that drifts past
        // the allowance is a regression.
        if !self.baseline_refs.is_empty() {
            match &self.baseline {
                None => failures.push(AudioQaFailure {
                    code: "missing_baseline".to_string(),
                    detail: "baselineRefs declared but no baseline value supplied".to_string(),
                }),
                Some(baseline) => {
                    let drift = (self.loudness.integrated_lufs - baseline.integrated_lufs).abs();
                    if drift > self.max_loudness_drift_lu {
                        failures.push(AudioQaFailure {
                            code: "loudness_regression".to_string(),
                            detail: format!(
                                "loudness drift {drift} LU exceeds allowed {} LU",
                                self.max_loudness_drift_lu
                            ),
                        });
                    }
                }
            }
        }

        failures
    }

    /// Classify the check. Precedence: stale > fail > pass.
    pub fn computed_status(&self) -> AudioQaStatus {
        if !self.stale_baseline_refs.is_empty() {
            return AudioQaStatus::Stale;
        }
        if self.failures().is_empty() {
            AudioQaStatus::Pass
        } else {
            AudioQaStatus::Fail
        }
    }

    /// Emit a declared gate verdict that composes into the existing evaluator
    /// aggregation under the `declared-gate-and` operator. The audio-QA check is a
    /// declared gate; the verdict carries the native computed status so the
    /// `stale > fail > pass` precedence is preserved at the aggregation boundary,
    /// and any non-pass status contributes a failure so the aggregate fails
    /// closed.
    pub fn gate_verdict(&self) -> Value {
        let status = self.computed_status();
        let pass = matches!(status, AudioQaStatus::Pass);
        json!({
            "gate": "audio_qa",
            "declared": true,
            "status": status.as_str(),
            "pass": pass,
            "resultCount": 1,
            "failureCount": if pass { 0 } else { 1 },
            "aggregation": {
                "operator": "declared-gate-and",
                "undeclaredGatePolicy": "neutral"
            }
        })
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != AUDIO_QA_SCHEMA_VERSION {
            return Err(anyhow!(
                "audio-QA check schemaVersion must be {AUDIO_QA_SCHEMA_VERSION}"
            ));
        }
        require_text("audio-QA check checkId", &self.check_id)?;
        require_text("audio-QA check assetRef", &self.asset_ref)?;

        // Format validity (malformed audio fails closed).
        if !SUPPORTED_AUDIO_FORMATS.contains(&self.format.as_str()) {
            return Err(anyhow!(
                "audio-QA check format \"{}\" is unsupported; expected one of {SUPPORTED_AUDIO_FORMATS:?}",
                self.format
            ));
        }
        if !SUPPORTED_AUDIO_KINDS.contains(&self.kind.as_str()) {
            return Err(anyhow!(
                "audio-QA check kind \"{}\" is unsupported",
                self.kind
            ));
        }
        if self.channels < 1 || self.channels > MAX_AUDIO_CHANNELS {
            return Err(anyhow!(
                "audio-QA check channels {} is out of range (1..={MAX_AUDIO_CHANNELS})",
                self.channels
            ));
        }
        if self.duration_ms == 0 || self.duration_ms > MAX_AUDIO_DURATION_MS {
            return Err(anyhow!(
                "audio-QA check durationMs {} is out of range (1..={MAX_AUDIO_DURATION_MS})",
                self.duration_ms
            ));
        }

        // License completeness (unlicensed/uncredited audio fails closed).
        self.license.validate()?;

        // Provenance references must each be non-blank. An empty list is allowed
        // here (it computes to a fail via `failures()`), but a blank entry is
        // malformed and fails closed so it cannot masquerade as present provenance.
        for (index, reference) in self.provenance_refs.iter().enumerate() {
            require_text(
                &format!("audio-QA check provenanceRefs[{index}]"),
                reference,
            )?;
        }

        // Loudness range must be well-formed and finite.
        let l = &self.loudness;
        for (field, value) in [
            ("integratedLufs", l.integrated_lufs),
            ("truePeakDbtp", l.true_peak_dbtp),
            ("minLufs", l.min_lufs),
            ("maxLufs", l.max_lufs),
            ("maxTruePeakDbtp", l.max_true_peak_dbtp),
        ] {
            if !value.is_finite() {
                return Err(anyhow!("audio-QA check loudness {field} must be finite"));
            }
        }
        if l.min_lufs > l.max_lufs {
            return Err(anyhow!(
                "audio-QA check loudness minLufs must not exceed maxLufs"
            ));
        }
        if !self.max_loudness_drift_lu.is_finite() || self.max_loudness_drift_lu < 0.0 {
            return Err(anyhow!(
                "audio-QA check maxLoudnessDriftLu must be a non-negative finite number"
            ));
        }

        // Baseline references must be self-consistent.
        if let Some(baseline) = &self.baseline {
            if !baseline.integrated_lufs.is_finite() {
                return Err(anyhow!(
                    "audio-QA check baseline integratedLufs must be finite"
                ));
            }
            if !self.baseline_refs.contains(&baseline.baseline_ref) {
                return Err(anyhow!(
                    "audio-QA check baseline baselineRef must be one of the declared baselineRefs"
                ));
            }
        }
        for stale in &self.stale_baseline_refs {
            if !self.baseline_refs.contains(stale) {
                return Err(anyhow!(
                    "audio-QA check staleBaselineRefs must reference declared baselineRefs"
                ));
            }
        }

        require_text("audio-QA check verdict", &self.verdict)?;

        // Declared status must match the computed classification.
        let computed = self.computed_status();
        if self.status != computed.as_str() {
            return Err(anyhow!(
                "audio-QA check status `{}` does not match computed classification `{}`",
                self.status,
                computed.as_str()
            ));
        }
        if matches!(computed, AudioQaStatus::Stale) && self.blocked_reasons.is_empty() {
            return Err(anyhow!(
                "audio-QA check stale status requires visible blockedReasons"
            ));
        }

        require_text("audio-QA check boundary", &self.boundary)?;
        let boundary = self.boundary.to_ascii_lowercase();
        for required in BOUNDARY_REQUIRED_PHRASES {
            if !boundary.contains(required) {
                return Err(anyhow!(
                    "audio-QA check boundary must record \"{required}\""
                ));
            }
        }
        Ok(())
    }
}

fn require_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(anyhow!("{field} is required"))
    } else {
        Ok(())
    }
}
