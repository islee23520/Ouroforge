//! Structured Human-Playtest Capture v1 (#1858).
//!
//! Part of Human Playtest Harness and Fun-Feel Gate v1 (#1857) under #1 Era J
//! Milestone 58. This module records bounded local human-playtest observations
//! as evidence. It does not compute fun, approve release-readiness, apply
//! proposals, write trusted source/project state, or grant browser/Studio write
//! authority.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/// Schema version for playtest capture fixtures/evidence.
pub const PLAYTEST_CAPTURE_SCHEMA_VERSION: &str = "ouroforge.playtest-capture.v1";
/// Explicit boundary marker: captures are evidence for a later human verdict,
/// not verdicts or trusted mutations by themselves.
pub const PLAYTEST_CAPTURE_BOUNDARY: &str = "human-playtest-capture-evidence-only";

const ALLOWED_DURATION_BUCKETS: &[&str] = &[
    "under-5m", "5m-15m", "15m-30m", "30m-60m", "over-60m", "unknown",
];
const ALLOWED_END_REASONS: &[&str] = &[
    "completed",
    "abandoned",
    "timebox-ended",
    "stopped-by-facilitator",
];
const ALLOWED_ONE_MORE_RUN: &[&str] = &["yes", "no", "maybe", "not-asked"];
const ALLOWED_RETENTION_PROXY: &[&str] = &[
    "returned-within-local-window",
    "not-returned-within-local-window",
    "unknown",
];
const ALLOWED_SEVERITY: &[&str] = &["none", "low", "medium", "high"];

/// Structured evidence from one bounded local human playtest session.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestSessionCapture {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "captureId")]
    pub capture_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "titleConfigId")]
    pub title_config_id: String,
    #[serde(rename = "candidateRefs")]
    pub candidate_refs: Vec<String>,
    #[serde(rename = "playtestBuildRef")]
    pub playtest_build_ref: String,
    pub actor: PlaytestActor,
    #[serde(rename = "recordedAtUnixMs")]
    pub recorded_at_unix_ms: u128,
    pub signals: PlaytestSignals,
    pub feedback: PlaytestFeedback,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<PlaytestEvidenceRef>,
    pub boundary: String,
    #[serde(rename = "generatedStatePolicy")]
    pub generated_state_policy: String,
    #[serde(rename = "trustedWriteRequested")]
    pub trusted_write_requested: bool,
    #[serde(rename = "releaseAuthority")]
    pub release_authority: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestActor {
    #[serde(rename = "actorId")]
    pub actor_id: String,
    pub role: String,
    #[serde(rename = "humanConfirmed")]
    pub human_confirmed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestSignals {
    #[serde(rename = "durationBucket")]
    pub duration_bucket: String,
    #[serde(rename = "runCount")]
    pub run_count: u32,
    #[serde(rename = "endReason")]
    pub end_reason: String,
    #[serde(rename = "oneMoreRun")]
    pub one_more_run: String,
    #[serde(rename = "retentionProxy")]
    pub retention_proxy: String,
    #[serde(rename = "replayRequested")]
    pub replay_requested: bool,
    #[serde(rename = "confusionMarkers")]
    pub confusion_markers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestFeedback {
    pub notes: String,
    #[serde(rename = "likedMoments")]
    pub liked_moments: Vec<String>,
    #[serde(rename = "dislikedMoments")]
    pub disliked_moments: Vec<String>,
    #[serde(rename = "frictionTags")]
    pub friction_tags: Vec<String>,
    pub severity: String,
    #[serde(rename = "suggestedFollowUp")]
    pub suggested_follow_up: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestEvidenceRef {
    pub kind: String,
    pub path: String,
}

impl PlaytestSessionCapture {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("playtest session capture is not valid JSON: {err}"))
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema_version != PLAYTEST_CAPTURE_SCHEMA_VERSION {
            return Err(anyhow!(
                "playtest capture schemaVersion must be \"{PLAYTEST_CAPTURE_SCHEMA_VERSION}\""
            ));
        }
        require_text("playtest capture captureId", &self.capture_id)?;
        require_text("playtest capture projectId", &self.project_id)?;
        require_text("playtest capture runId", &self.run_id)?;
        require_text("playtest capture titleConfigId", &self.title_config_id)?;
        if self.candidate_refs.is_empty() {
            return Err(anyhow!(
                "playtest capture candidateRefs must contain at least one scoped candidate/version ref"
            ));
        }
        for candidate_ref in &self.candidate_refs {
            validate_repo_ref("playtest capture candidateRefs", candidate_ref)?;
        }
        validate_repo_ref(
            "playtest capture playtestBuildRef",
            &self.playtest_build_ref,
        )?;
        self.actor.validate()?;
        if self.recorded_at_unix_ms == 0 {
            return Err(anyhow!(
                "playtest capture recordedAtUnixMs must be greater than zero"
            ));
        }
        self.signals.validate()?;
        self.feedback.validate()?;
        if self.evidence_refs.is_empty() {
            return Err(anyhow!(
                "playtest capture evidenceRefs must cite at least one evidence artifact"
            ));
        }
        for evidence_ref in &self.evidence_refs {
            evidence_ref.validate()?;
        }
        if self.boundary != PLAYTEST_CAPTURE_BOUNDARY {
            return Err(anyhow!(
                "playtest capture boundary must be \"{PLAYTEST_CAPTURE_BOUNDARY}\""
            ));
        }
        if !self.generated_state_policy.contains("fixture-scoped")
            || !self.generated_state_policy.contains("untracked")
        {
            return Err(anyhow!(
                "playtest capture generatedStatePolicy must mention fixture-scoped and untracked generated state"
            ));
        }
        if self.trusted_write_requested || self.release_authority {
            return Err(anyhow!(
                "playtest capture is evidence only and cannot request trusted write or release authority"
            ));
        }
        Ok(())
    }
}

impl PlaytestActor {
    pub fn validate(&self) -> Result<()> {
        require_text("playtest actor actorId", &self.actor_id)?;
        require_text("playtest actor role", &self.role)?;
        if self.role != "human-playtester" && self.role != "human-facilitator" {
            return Err(anyhow!(
                "playtest actor role must be human-playtester or human-facilitator"
            ));
        }
        if !self.human_confirmed {
            return Err(anyhow!(
                "playtest actor humanConfirmed must be true; automated signals cannot replace a human"
            ));
        }
        Ok(())
    }
}

impl PlaytestSignals {
    pub fn validate(&self) -> Result<()> {
        require_allowed(
            "playtest signals durationBucket",
            &self.duration_bucket,
            ALLOWED_DURATION_BUCKETS,
        )?;
        if self.run_count == 0 {
            return Err(anyhow!(
                "playtest signals runCount must be greater than zero"
            ));
        }
        require_allowed(
            "playtest signals endReason",
            &self.end_reason,
            ALLOWED_END_REASONS,
        )?;
        require_allowed(
            "playtest signals oneMoreRun",
            &self.one_more_run,
            ALLOWED_ONE_MORE_RUN,
        )?;
        require_allowed(
            "playtest signals retentionProxy",
            &self.retention_proxy,
            ALLOWED_RETENTION_PROXY,
        )?;
        for marker in &self.confusion_markers {
            require_text("playtest signals confusionMarkers", marker)?;
        }
        Ok(())
    }
}

impl PlaytestFeedback {
    pub fn validate(&self) -> Result<()> {
        require_text("playtest feedback notes", &self.notes)?;
        require_allowed(
            "playtest feedback severity",
            &self.severity,
            ALLOWED_SEVERITY,
        )?;
        for liked in &self.liked_moments {
            require_text("playtest feedback likedMoments", liked)?;
        }
        for disliked in &self.disliked_moments {
            require_text("playtest feedback dislikedMoments", disliked)?;
        }
        for tag in &self.friction_tags {
            require_text("playtest feedback frictionTags", tag)?;
        }
        if let Some(follow_up) = &self.suggested_follow_up {
            require_text("playtest feedback suggestedFollowUp", follow_up)?;
        }
        Ok(())
    }
}

impl PlaytestEvidenceRef {
    pub fn validate(&self) -> Result<()> {
        require_text("playtest evidence ref kind", &self.kind)?;
        if !matches!(
            self.kind.as_str(),
            "runtime-probe" | "evaluator-verdict" | "provenance-bundle" | "playtest-log"
        ) {
            return Err(anyhow!("playtest evidence ref kind is unsupported"));
        }
        ouroforge_evidence::validate_evidence_artifact_path(&self.path)
            .map_err(|err| anyhow!("playtest evidence ref path is unsafe: {err}"))?;
        Ok(())
    }
}

fn require_allowed(label: &str, value: &str, allowed: &[&str]) -> Result<()> {
    require_text(label, value)?;
    if !allowed.contains(&value) {
        return Err(anyhow!("{label} \"{value}\" is unsupported"));
    }
    Ok(())
}

fn validate_repo_ref(label: &str, value: &str) -> Result<()> {
    require_text(label, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{label} must be a safe repo-relative ref"));
    }
    Ok(())
}

fn require_text(label: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must be non-empty"));
    }
    Ok(())
}
