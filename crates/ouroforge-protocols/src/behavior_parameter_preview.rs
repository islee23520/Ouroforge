//! Behavior Parameter Draft and Preview v1 (#2373, #1 M124).
//!
//! Data-only draft/preview contract for allowlisted hazard behavior parameters.
//! The preview records the deterministic replay API surface introduced by M119
//! (`runReplay`, `replayStateDigest`, and `compareReplayDigest`) as evidence
//! inputs, but this module does not run a browser, execute script, mutate source,
//! or apply drafts. It produces a bounded before/after evidence bundle that can
//! be reviewed before any Safe Source Apply handoff.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const BEHAVIOR_PARAMETER_DRAFT_SCHEMA_VERSION: &str = "behavior-parameter-draft-v1";
pub const BEHAVIOR_PARAMETER_PREVIEW_SCHEMA_VERSION: &str = "behavior-parameter-preview-v1";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorParameterPreviewDiagnosticCode {
    UnsupportedBehaviorModel,
    UnsupportedParameter,
    ParameterTypeMismatch,
    ParameterOutOfRange,
    MissingReplayApiEvidence,
    UnsafeArtifactPath,
    DuplicateParameterDraft,
}

impl BehaviorParameterPreviewDiagnosticCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnsupportedBehaviorModel => "unsupported_behavior_model",
            Self::UnsupportedParameter => "unsupported_parameter",
            Self::ParameterTypeMismatch => "parameter_type_mismatch",
            Self::ParameterOutOfRange => "parameter_out_of_range",
            Self::MissingReplayApiEvidence => "missing_replay_api_evidence",
            Self::UnsafeArtifactPath => "unsafe_artifact_path",
            Self::DuplicateParameterDraft => "duplicate_parameter_draft",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorParameterDiagnostic {
    pub code: BehaviorParameterPreviewDiagnosticCode,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorPreviewStatus {
    PreviewReady,
    Rejected,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorParameterDraftRequest {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    #[serde(rename = "behaviorModelId")]
    pub behavior_model_id: String,
    #[serde(rename = "behaviorKind")]
    pub behavior_kind: BehaviorKind,
    #[serde(rename = "parameterDrafts")]
    pub parameter_drafts: Vec<BehaviorParameterDraft>,
    #[serde(rename = "replayApi")]
    pub replay_api: ReplayApiEvidence,
    #[serde(rename = "sourceEvidenceRefs")]
    pub source_evidence_refs: Vec<BehaviorPreviewEvidenceRef>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BehaviorKind {
    HazardRouteTiming,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorParameterDraft {
    pub name: String,
    pub before: BehaviorParameterScalar,
    pub after: BehaviorParameterScalar,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", rename_all = "kebab-case")]
pub enum BehaviorParameterScalar {
    Integer(i64),
    TextId(String),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReplayApiEvidence {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "requiredMethods")]
    pub required_methods: Vec<String>,
    #[serde(rename = "scenePath")]
    pub scene_path: String,
    #[serde(rename = "replayPath")]
    pub replay_path: String,
    #[serde(rename = "deterministicSeed")]
    pub deterministic_seed: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorPreviewEvidenceRef {
    #[serde(rename = "runId")]
    pub run_id: String,
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorParameterPreviewBundle {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "draftId")]
    pub draft_id: String,
    pub status: BehaviorPreviewStatus,
    #[serde(rename = "behaviorModelId")]
    pub behavior_model_id: String,
    #[serde(rename = "replayApi")]
    pub replay_api: ReplayApiEvidence,
    pub before: HazardPreviewObservation,
    pub after: HazardPreviewObservation,
    #[serde(rename = "expectedScenarioImpact")]
    pub expected_scenario_impact: Vec<BehaviorScenarioImpactDraft>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<BehaviorPreviewEvidenceRef>,
    pub diagnostics: Vec<BehaviorParameterDiagnostic>,
    pub guardrails: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct HazardPreviewObservation {
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "contactFrameThreshold")]
    pub contact_frame_threshold: i64,
    pub outcome: HazardPreviewOutcome,
    #[serde(rename = "contactFrame")]
    pub contact_frame: i64,
    #[serde(rename = "finalStateDigest")]
    pub final_state_digest: String,
    #[serde(rename = "eventPath")]
    pub event_path: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HazardPreviewOutcome {
    ContactAllowed,
    ContactFails,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BehaviorScenarioImpactDraft {
    #[serde(rename = "assertionId")]
    pub assertion_id: String,
    #[serde(rename = "expectedPath")]
    pub expected_path: String,
    #[serde(rename = "beforeValue")]
    pub before_value: String,
    #[serde(rename = "afterValue")]
    pub after_value: String,
    #[serde(rename = "failureMessage")]
    pub failure_message: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<BehaviorPreviewEvidenceRef>,
}

impl BehaviorParameterDraftRequest {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let request: Self = serde_json::from_str(input)
            .context("failed to parse behavior parameter draft request JSON")?;
        request.validate()?;
        Ok(request)
    }

    pub fn validate(&self) -> Result<()> {
        let diagnostics = self.collect_diagnostics();
        if let Some(diagnostic) = diagnostics.first() {
            return Err(anyhow!(
                "{}: {}",
                diagnostic.code.as_str(),
                diagnostic.message
            ));
        }
        Ok(())
    }

    pub fn preview(&self) -> BehaviorParameterPreviewBundle {
        let diagnostics = self.collect_diagnostics();
        let evidence_refs = self.source_evidence_refs.clone();
        let fallback = HazardPreviewObservation {
            run_id: format!("{}-invalid", self.draft_id),
            contact_frame_threshold: 0,
            outcome: HazardPreviewOutcome::ContactFails,
            contact_frame: 0,
            final_state_digest: "sha256:invalid-preview".to_string(),
            event_path: "runtime-events.json".to_string(),
        };
        if !diagnostics.is_empty() {
            return BehaviorParameterPreviewBundle {
                schema_version: BEHAVIOR_PARAMETER_PREVIEW_SCHEMA_VERSION.to_string(),
                draft_id: self.draft_id.clone(),
                status: BehaviorPreviewStatus::Rejected,
                behavior_model_id: self.behavior_model_id.clone(),
                replay_api: self.replay_api.clone(),
                before: fallback.clone(),
                after: fallback,
                expected_scenario_impact: Vec::new(),
                evidence_refs,
                diagnostics,
                guardrails: self.guardrails.clone(),
            };
        }

        let draft = self
            .parameter_drafts
            .iter()
            .find(|draft| draft.name == "contactFrameThreshold")
            .expect("validated contactFrameThreshold draft");
        let before_threshold = integer_value(&draft.before).expect("validated integer before");
        let after_threshold = integer_value(&draft.after).expect("validated integer after");
        let contact_frame = deterministic_contact_frame(self.replay_api.deterministic_seed);
        let before = self.observation("before", before_threshold, contact_frame);
        let after = self.observation("after", after_threshold, contact_frame);
        let impact = BehaviorScenarioImpactDraft {
            assertion_id: "hazard-contact-outcome-changes".to_string(),
            expected_path: "worldState.hazards.demo-hazard.contactOutcome".to_string(),
            before_value: format!("{:?}", before.outcome).to_ascii_kebab(),
            after_value: format!("{:?}", after.outcome).to_ascii_kebab(),
            failure_message: "hazard contact outcome must reflect drafted contactFrameThreshold"
                .to_string(),
            evidence_refs: evidence_refs.clone(),
        };
        BehaviorParameterPreviewBundle {
            schema_version: BEHAVIOR_PARAMETER_PREVIEW_SCHEMA_VERSION.to_string(),
            draft_id: self.draft_id.clone(),
            status: BehaviorPreviewStatus::PreviewReady,
            behavior_model_id: self.behavior_model_id.clone(),
            replay_api: self.replay_api.clone(),
            before,
            after,
            expected_scenario_impact: vec![impact],
            evidence_refs,
            diagnostics,
            guardrails: self.guardrails.clone(),
        }
    }

    pub fn preview_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.preview())
            .context("failed to serialize behavior parameter preview JSON")
    }

    fn collect_diagnostics(&self) -> Vec<BehaviorParameterDiagnostic> {
        let mut diagnostics = Vec::new();
        if self.schema_version != BEHAVIOR_PARAMETER_DRAFT_SCHEMA_VERSION {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::UnsupportedBehaviorModel,
                format!(
                    "behavior parameter draft schemaVersion must be {BEHAVIOR_PARAMETER_DRAFT_SCHEMA_VERSION}"
                ),
                Some("schemaVersion"),
            ));
        }
        if require_local_id("draftId", &self.draft_id).is_err() {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::UnsupportedBehaviorModel,
                "draftId must be a bounded local id".to_string(),
                Some("draftId"),
            ));
        }
        if self.behavior_kind != BehaviorKind::HazardRouteTiming {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::UnsupportedBehaviorModel,
                "only hazard route/timing behavior models are allowlisted for M124.2".to_string(),
                Some("behaviorKind"),
            ));
        }
        if let Err(err) = self.replay_api.validate() {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::MissingReplayApiEvidence,
                err.to_string(),
                Some("replayApi"),
            ));
        }
        if self.source_evidence_refs.is_empty() {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::MissingReplayApiEvidence,
                "sourceEvidenceRefs must include replay API evidence refs".to_string(),
                Some("sourceEvidenceRefs"),
            ));
        }
        for reference in &self.source_evidence_refs {
            if let Err(err) = reference.validate("sourceEvidenceRef") {
                diagnostics.push(diagnostic(
                    BehaviorParameterPreviewDiagnosticCode::UnsafeArtifactPath,
                    err.to_string(),
                    Some("sourceEvidenceRefs"),
                ));
            }
        }
        let mut seen = BTreeSet::new();
        for draft in &self.parameter_drafts {
            if !seen.insert(draft.name.clone()) {
                diagnostics.push(diagnostic(
                    BehaviorParameterPreviewDiagnosticCode::DuplicateParameterDraft,
                    format!("parameter `{}` is drafted more than once", draft.name),
                    Some("parameterDrafts"),
                ));
            }
            diagnostics.extend(draft.validate());
        }
        if !self
            .parameter_drafts
            .iter()
            .any(|draft| draft.name == "contactFrameThreshold")
        {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::UnsupportedParameter,
                "M124.2 requires a contactFrameThreshold hazard parameter draft".to_string(),
                Some("parameterDrafts"),
            ));
        }
        if self.guardrails.is_empty() {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::UnsupportedBehaviorModel,
                "guardrails must not be empty".to_string(),
                Some("guardrails"),
            ));
        }
        diagnostics
    }

    fn observation(
        &self,
        phase: &str,
        threshold: i64,
        contact_frame: i64,
    ) -> HazardPreviewObservation {
        let outcome = if contact_frame <= threshold {
            HazardPreviewOutcome::ContactFails
        } else {
            HazardPreviewOutcome::ContactAllowed
        };
        HazardPreviewObservation {
            run_id: format!("{}-{phase}", self.draft_id),
            contact_frame_threshold: threshold,
            outcome,
            contact_frame,
            final_state_digest: stable_digest(&[
                &self.behavior_model_id,
                phase,
                &threshold.to_string(),
                &contact_frame.to_string(),
                &self.replay_api.deterministic_seed.to_string(),
            ]),
            event_path: format!("runs/{}/behavior-preview-{phase}.json", self.draft_id),
        }
    }
}

impl BehaviorParameterDraft {
    fn validate(&self) -> Vec<BehaviorParameterDiagnostic> {
        let mut diagnostics = Vec::new();
        if self.name != "contactFrameThreshold" {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::UnsupportedParameter,
                format!(
                    "parameter `{}` is not allowlisted for hazard preview",
                    self.name
                ),
                Some("parameterDrafts.name"),
            ));
            return diagnostics;
        }
        let (Some(before), Some(after)) = (integer_value(&self.before), integer_value(&self.after))
        else {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::ParameterTypeMismatch,
                "contactFrameThreshold must use integer before/after values".to_string(),
                Some("parameterDrafts.contactFrameThreshold"),
            ));
            return diagnostics;
        };
        for (label, value) in [("before", before), ("after", after)] {
            if !(0..=600).contains(&value) {
                diagnostics.push(diagnostic(
                    BehaviorParameterPreviewDiagnosticCode::ParameterOutOfRange,
                    format!(
                        "contactFrameThreshold {label} value {value} must be between 0 and 600"
                    ),
                    Some(format!("parameterDrafts.contactFrameThreshold.{label}")),
                ));
            }
        }
        if before == after {
            diagnostics.push(diagnostic(
                BehaviorParameterPreviewDiagnosticCode::ParameterOutOfRange,
                "contactFrameThreshold draft must change the previewed value".to_string(),
                Some("parameterDrafts.contactFrameThreshold"),
            ));
        }
        diagnostics
    }
}

impl ReplayApiEvidence {
    fn validate(&self) -> Result<()> {
        require_boundary_text("replayApi apiVersion", &self.api_version)?;
        require_relative_path("replayApi scenePath", &self.scene_path)?;
        require_relative_path("replayApi replayPath", &self.replay_path)?;
        let required: BTreeSet<_> = ["runReplay", "replayStateDigest", "compareReplayDigest"]
            .into_iter()
            .collect();
        let actual: BTreeSet<_> = self.required_methods.iter().map(String::as_str).collect();
        if !required.is_subset(&actual) {
            return Err(anyhow!(
                "replayApi requiredMethods must include runReplay, replayStateDigest, and compareReplayDigest"
            ));
        }
        Ok(())
    }
}

impl BehaviorPreviewEvidenceRef {
    fn validate(&self, label: &str) -> Result<()> {
        require_local_id(&format!("{label} runId"), &self.run_id)?;
        require_relative_path(&format!("{label} path"), &self.path)?;
        require_digest(&format!("{label} digest"), &self.digest)
    }
}

impl BehaviorParameterPreviewBundle {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != BEHAVIOR_PARAMETER_PREVIEW_SCHEMA_VERSION {
            return Err(anyhow!(
                "behavior parameter preview schemaVersion must be {BEHAVIOR_PARAMETER_PREVIEW_SCHEMA_VERSION}"
            ));
        }
        require_local_id("behavior parameter preview draftId", &self.draft_id)?;
        require_local_id(
            "behavior parameter preview behaviorModelId",
            &self.behavior_model_id,
        )?;
        self.replay_api.validate()?;
        self.before.validate("before")?;
        self.after.validate("after")?;
        for reference in &self.evidence_refs {
            reference.validate("preview evidence ref")?;
        }
        if self.status == BehaviorPreviewStatus::PreviewReady {
            if self.before.final_state_digest == self.after.final_state_digest {
                return Err(anyhow!(
                    "behavior parameter preview before/after digests must differ"
                ));
            }
            if self.expected_scenario_impact.is_empty() {
                return Err(anyhow!(
                    "behavior parameter preview must include expected scenario impact drafts"
                ));
            }
        }
        Ok(())
    }
}

impl HazardPreviewObservation {
    fn validate(&self, label: &str) -> Result<()> {
        require_local_id(&format!("hazard preview {label} runId"), &self.run_id)?;
        require_digest(
            &format!("hazard preview {label} finalStateDigest"),
            &self.final_state_digest,
        )?;
        require_relative_path(
            &format!("hazard preview {label} eventPath"),
            &self.event_path,
        )
    }
}

fn integer_value(value: &BehaviorParameterScalar) -> Option<i64> {
    match value {
        BehaviorParameterScalar::Integer(value) => Some(*value),
        BehaviorParameterScalar::TextId(_) => None,
    }
}

fn deterministic_contact_frame(seed: u64) -> i64 {
    30 + (seed % 90) as i64
}

fn stable_digest(parts: &[&str]) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    format!("sha256:{hash:016x}")
}

fn diagnostic(
    code: BehaviorParameterPreviewDiagnosticCode,
    message: String,
    field: Option<impl Into<String>>,
) -> BehaviorParameterDiagnostic {
    BehaviorParameterDiagnostic {
        code,
        message,
        field: field.map(Into::into),
    }
}

trait ToAsciiKebab {
    fn to_ascii_kebab(self) -> String;
}

impl ToAsciiKebab for String {
    fn to_ascii_kebab(self) -> String {
        let mut out = String::new();
        for (index, ch) in self.chars().enumerate() {
            if ch.is_ascii_uppercase() && index > 0 {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        }
        out
    }
}

fn require_local_id(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.len() > 160
        || !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | ':'))
    {
        return Err(anyhow!(
            "{field} must be a bounded local id using alphanumeric, dash, underscore, dot, or colon"
        ));
    }
    Ok(())
}

fn require_relative_path(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if value.starts_with('/') || value.contains("..") || value.contains('\\') {
        return Err(anyhow!("{field} must stay inside the local artifact root"));
    }
    Ok(())
}

fn require_digest(field: &str, value: &str) -> Result<()> {
    require_boundary_text(field, value)?;
    if !value.contains(':') || value.len() > 160 {
        return Err(anyhow!("{field} must include an algorithm prefix"));
    }
    Ok(())
}

fn require_boundary_text(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow!("{field} must not be empty"));
    }
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "<script",
        "javascript:",
        "command bridge",
        "hidden command",
        "browser trusted write",
        "self-apply",
        "self apply",
        "self-approval",
        "auto-merge",
        "auto-apply",
        "dependency install",
        "publish",
        "deploy",
        "upload",
        "production-ready",
        "godot replacement",
        "http://",
        "https://",
    ] {
        if lower.contains(forbidden) {
            return Err(anyhow!(
                "{field} contains forbidden behavior preview authority text `{forbidden}`"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hazard_parameter_preview_changes_before_after_outcome() {
        let request = hazard_request(20, 140);
        request.validate().unwrap();
        let preview = request.preview();
        preview.validate().unwrap();
        assert_eq!(preview.status, BehaviorPreviewStatus::PreviewReady);
        assert_eq!(preview.before.outcome, HazardPreviewOutcome::ContactAllowed);
        assert_eq!(preview.after.outcome, HazardPreviewOutcome::ContactFails);
        assert_ne!(
            preview.before.final_state_digest,
            preview.after.final_state_digest
        );
        assert_eq!(preview.replay_api.required_methods[0], "runReplay");
        assert_eq!(
            preview.expected_scenario_impact[0].failure_message,
            "hazard contact outcome must reflect drafted contactFrameThreshold"
        );
    }

    #[test]
    fn invalid_hazard_parameter_fails_with_named_diagnostic() {
        let request = hazard_request(20, 999);
        let err = request.validate().unwrap_err().to_string();
        assert!(err.contains("parameter_out_of_range"), "{err}");
        let preview = request.preview();
        assert_eq!(preview.status, BehaviorPreviewStatus::Rejected);
        assert!(preview
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.as_str() == "parameter_out_of_range"));
    }

    fn hazard_request(before: i64, after: i64) -> BehaviorParameterDraftRequest {
        BehaviorParameterDraftRequest {
            schema_version: BEHAVIOR_PARAMETER_DRAFT_SCHEMA_VERSION.to_string(),
            draft_id: "hazard-threshold-preview".to_string(),
            behavior_model_id: "hazard-route-timing-demo".to_string(),
            behavior_kind: BehaviorKind::HazardRouteTiming,
            parameter_drafts: vec![BehaviorParameterDraft {
                name: "contactFrameThreshold".to_string(),
                before: BehaviorParameterScalar::Integer(before),
                after: BehaviorParameterScalar::Integer(after),
            }],
            replay_api: ReplayApiEvidence {
                api_version: "119.2.0".to_string(),
                required_methods: vec![
                    "runReplay".to_string(),
                    "replayStateDigest".to_string(),
                    "compareReplayDigest".to_string(),
                ],
                scene_path: "examples/game-runtime/scene.json".to_string(),
                replay_path: "runs/session-h-2373/replay.json".to_string(),
                deterministic_seed: 42,
            },
            source_evidence_refs: vec![BehaviorPreviewEvidenceRef {
                run_id: "run-before".to_string(),
                path: "runs/session-h-2373/replay-digest.json".to_string(),
                digest: "sha256:replay".to_string(),
            }],
            guardrails: vec!["draft-only preview; Safe Source Apply review required".to_string()],
        }
    }
}
