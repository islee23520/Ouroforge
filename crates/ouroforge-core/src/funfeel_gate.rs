//! Human-in-the-loop Fun-Feel Evaluation Gate v1 (#1859).
//!
//! This module models the Era J fun/feel gate as a Rust-local release-readiness
//! precondition. It verifies that a scoped human verdict exists and is fresh for
//! the candidate/build/playtest evidence being evaluated. It never computes an
//! automated fun score, never grants browser/Studio write authority, and never
//! becomes a release button by itself.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::playtest_capture::PLAYTEST_CAPTURE_SCHEMA_VERSION;

/// Schema version for the fun-feel gate input evidence.
pub const FUNFEEL_GATE_SCHEMA_VERSION: &str = "ouroforge.funfeel-gate.v1";
/// Explicit non-scoring boundary for machine-readable gate output.
pub const FUNFEEL_GATE_BOUNDARY: &str = "human-funfeel-verdict-release-readiness-precondition";

/// Readiness states emitted by the gate. These describe human verdict validity,
/// not subjective quality or an automated fun score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunFeelReadiness {
    Blocked,
    ApprovedByHuman,
    NeedsHumanReview,
}

impl FunFeelReadiness {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::ApprovedByHuman => "approved-by-human",
            Self::NeedsHumanReview => "needs-human-review",
        }
    }
}

/// Evidence bundle evaluated by the human-owned fun-feel gate.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FunFeelGateInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "gateId")]
    pub gate_id: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "titleConfigId")]
    pub title_config_id: String,
    #[serde(rename = "candidateRefs")]
    pub candidate_refs: Vec<String>,
    #[serde(rename = "playtestCaptureRefs")]
    pub playtest_capture_refs: Vec<PlaytestCaptureRef>,
    #[serde(rename = "mechanicalGateRefs")]
    pub mechanical_gate_refs: Vec<String>,
    pub verdict: Option<HumanFunFeelVerdict>,
    #[serde(rename = "generatedStatePolicy")]
    pub generated_state_policy: String,
    #[serde(rename = "browserStudioMode")]
    pub browser_studio_mode: String,
    #[serde(rename = "trustedWriteRequested")]
    pub trusted_write_requested: bool,
    #[serde(rename = "releaseButtonRequested")]
    pub release_button_requested: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PlaytestCaptureRef {
    pub path: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "captureId")]
    pub capture_id: String,
    #[serde(rename = "candidateRefs")]
    pub candidate_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HumanFunFeelVerdict {
    #[serde(rename = "verdictId")]
    pub verdict_id: String,
    pub status: String,
    #[serde(rename = "decidedBy")]
    pub decided_by: HumanDecisionActor,
    #[serde(rename = "decidedAtUnixMs")]
    pub decided_at_unix_ms: u128,
    pub rationale: String,
    #[serde(rename = "candidateRefs")]
    pub candidate_refs: Vec<String>,
    #[serde(rename = "playtestCaptureRefs")]
    pub playtest_capture_refs: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HumanDecisionActor {
    #[serde(rename = "actorId")]
    pub actor_id: String,
    pub role: String,
    #[serde(rename = "humanConfirmed")]
    pub human_confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunFeelGateDecision {
    pub readiness: FunFeelReadiness,
    pub release_ready: bool,
    pub reason: String,
    pub decided_by: Option<String>,
    pub boundary: &'static str,
}

impl FunFeelGateInput {
    pub fn from_json_str(text: &str) -> Result<Self> {
        serde_json::from_str(text)
            .map_err(|err| anyhow!("fun-feel gate input is not valid JSON: {err}"))
    }

    pub fn evaluate(&self) -> FunFeelGateDecision {
        match self.validate_for_release_readiness() {
            Ok(verdict) => FunFeelGateDecision {
                readiness: FunFeelReadiness::ApprovedByHuman,
                release_ready: true,
                reason: "human fun/feel verdict approved the scoped candidate and evidence"
                    .to_string(),
                decided_by: Some(verdict.decided_by.actor_id.clone()),
                boundary: FUNFEEL_GATE_BOUNDARY,
            },
            Err(err) => {
                let reason = err.to_string();
                let readiness = if reason.contains("missing human fun/feel verdict") {
                    FunFeelReadiness::NeedsHumanReview
                } else {
                    FunFeelReadiness::Blocked
                };
                FunFeelGateDecision {
                    readiness,
                    release_ready: false,
                    reason,
                    decided_by: None,
                    boundary: FUNFEEL_GATE_BOUNDARY,
                }
            }
        }
    }

    pub fn validate_for_release_readiness(&self) -> Result<&HumanFunFeelVerdict> {
        if self.schema_version != FUNFEEL_GATE_SCHEMA_VERSION {
            return Err(anyhow!(
                "fun-feel gate schemaVersion must be \"{FUNFEEL_GATE_SCHEMA_VERSION}\""
            ));
        }
        require_text("fun-feel gate gateId", &self.gate_id)?;
        require_text("fun-feel gate projectId", &self.project_id)?;
        require_text("fun-feel gate titleConfigId", &self.title_config_id)?;
        require_non_empty_refs("fun-feel gate candidateRefs", &self.candidate_refs)?;
        if self.playtest_capture_refs.is_empty() {
            return Err(anyhow!(
                "fun-feel gate playtestCaptureRefs must include human playtest evidence"
            ));
        }
        for capture_ref in &self.playtest_capture_refs {
            capture_ref.validate(&self.candidate_refs)?;
        }
        require_non_empty_refs(
            "fun-feel gate mechanicalGateRefs",
            &self.mechanical_gate_refs,
        )?;
        require_text(
            "fun-feel gate generatedStatePolicy",
            &self.generated_state_policy,
        )?;
        if !self.generated_state_policy.contains("fixture-scoped")
            || !self.generated_state_policy.contains("untracked")
        {
            return Err(anyhow!(
                "fun-feel gate generatedStatePolicy must mention fixture-scoped and untracked generated state"
            ));
        }
        if !self.browser_studio_mode.contains("read-only") {
            return Err(anyhow!(
                "fun-feel gate browserStudioMode must keep browser/Studio surfaces read-only"
            ));
        }
        if self.trusted_write_requested || self.release_button_requested {
            return Err(anyhow!(
                "fun-feel gate cannot request trusted writes, release buttons, or browser/Studio authority"
            ));
        }
        if self.boundary != FUNFEEL_GATE_BOUNDARY {
            return Err(anyhow!(
                "fun-feel gate boundary must be \"{FUNFEEL_GATE_BOUNDARY}\""
            ));
        }

        let verdict = self
            .verdict
            .as_ref()
            .ok_or_else(|| anyhow!("missing human fun/feel verdict blocks release-readiness"))?;
        verdict.validate(&self.candidate_refs, &self.playtest_capture_refs)?;
        Ok(verdict)
    }
}

impl PlaytestCaptureRef {
    fn validate(&self, gate_candidate_refs: &[String]) -> Result<()> {
        validate_repo_ref("playtest capture ref path", &self.path)?;
        if self.schema_version != PLAYTEST_CAPTURE_SCHEMA_VERSION {
            return Err(anyhow!(
                "playtest capture ref schemaVersion must be \"{PLAYTEST_CAPTURE_SCHEMA_VERSION}\""
            ));
        }
        require_text("playtest capture ref captureId", &self.capture_id)?;
        require_non_empty_refs("playtest capture ref candidateRefs", &self.candidate_refs)?;
        if self.candidate_refs != gate_candidate_refs {
            return Err(anyhow!(
                "playtest capture ref candidateRefs must match the gate candidateRefs exactly"
            ));
        }
        Ok(())
    }
}

impl HumanFunFeelVerdict {
    fn validate(
        &self,
        gate_candidate_refs: &[String],
        playtest_capture_refs: &[PlaytestCaptureRef],
    ) -> Result<()> {
        require_text("human fun/feel verdict verdictId", &self.verdict_id)?;
        if self.status != "approved" {
            return Err(anyhow!(
                "human fun/feel verdict status must be approved for release-readiness; rejected, deferred, and needs-rework block"
            ));
        }
        self.decided_by.validate()?;
        if self.decided_at_unix_ms == 0 {
            return Err(anyhow!(
                "human fun/feel verdict decidedAtUnixMs must be greater than zero"
            ));
        }
        require_text("human fun/feel verdict rationale", &self.rationale)?;
        if self.candidate_refs != gate_candidate_refs {
            return Err(anyhow!(
                "human fun/feel verdict candidateRefs are stale or do not match the gate candidateRefs"
            ));
        }
        let expected_capture_ids: Vec<&str> = playtest_capture_refs
            .iter()
            .map(|capture_ref| capture_ref.capture_id.as_str())
            .collect();
        if self.playtest_capture_refs.len() != expected_capture_ids.len()
            || !expected_capture_ids.iter().all(|capture_id| {
                self.playtest_capture_refs
                    .iter()
                    .any(|actual| actual == capture_id)
            })
        {
            return Err(anyhow!(
                "human fun/feel verdict must cite each scoped playtest capture ref"
            ));
        }
        require_non_empty_refs("human fun/feel verdict evidenceRefs", &self.evidence_refs)?;
        Ok(())
    }
}

impl HumanDecisionActor {
    fn validate(&self) -> Result<()> {
        require_text("human fun/feel actor actorId", &self.actor_id)?;
        require_text("human fun/feel actor role", &self.role)?;
        if !matches!(
            self.role.as_str(),
            "human-reviewer" | "human-playtester" | "human-facilitator"
        ) {
            return Err(anyhow!(
                "human fun/feel actor role must be human-reviewer, human-playtester, or human-facilitator"
            ));
        }
        if !self.human_confirmed {
            return Err(anyhow!(
                "human fun/feel actor humanConfirmed must be true; automated metrics cannot decide fun"
            ));
        }
        Ok(())
    }
}

fn require_non_empty_refs(label: &str, refs: &[String]) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("{label} must contain at least one ref"));
    }
    for value in refs {
        validate_repo_ref(label, value)?;
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
