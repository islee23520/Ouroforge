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

pub const OPTIONAL_HUMAN_OVERSIGHT_INPUT_SCHEMA_VERSION: &str = "optional-human-oversight-input-v1";
pub const OPTIONAL_HUMAN_OVERSIGHT_VIEW_SCHEMA_VERSION: &str = "optional-human-oversight-view-v1";
pub const OPTIONAL_HUMAN_OVERRIDE_INPUT_SCHEMA_VERSION: &str = "optional-human-override-input-v1";
pub const OPTIONAL_HUMAN_OVERRIDE_RECORD_SCHEMA_VERSION: &str = "optional-human-override-record-v1";
pub const OPTIONAL_HUMAN_TASTE_FEEDBACK_INPUT_SCHEMA_VERSION: &str =
    "optional-human-taste-feedback-input-v1";
pub const OPTIONAL_HUMAN_TASTE_FEEDBACK_RECORD_SCHEMA_VERSION: &str =
    "optional-human-taste-feedback-record-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanOversightInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub contract: OptionalHumanChannelContract,
    #[serde(rename = "stageHealthRefs")]
    pub stage_health_refs: Vec<String>,
    #[serde(rename = "blockerRefs")]
    pub blocker_refs: Vec<String>,
    #[serde(rename = "diagnosisRefs")]
    pub diagnosis_refs: Vec<String>,
    #[serde(rename = "attributionRefs")]
    pub attribution_refs: Vec<String>,
    #[serde(rename = "loopCompletedWithoutHuman")]
    pub loop_completed_without_human: bool,
    #[serde(rename = "noTrustedWrites")]
    pub no_trusted_writes: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalHumanOversightView {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    pub optional: bool,
    #[serde(rename = "blocksAutonomousLoop")]
    pub blocks_autonomous_loop: bool,
    #[serde(rename = "trustedWritesPerformed")]
    pub trusted_writes_performed: bool,
    #[serde(rename = "loopCompletedWithoutHuman")]
    pub loop_completed_without_human: bool,
    #[serde(rename = "stageHealthRefs")]
    pub stage_health_refs: Vec<String>,
    #[serde(rename = "blockerRefs")]
    pub blocker_refs: Vec<String>,
    #[serde(rename = "diagnosisRefs")]
    pub diagnosis_refs: Vec<String>,
    #[serde(rename = "attributionRefs")]
    pub attribution_refs: Vec<String>,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "allowedActions")]
    pub allowed_actions: Vec<String>,
    #[serde(rename = "forbiddenActions")]
    pub forbidden_actions: Vec<String>,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanOverrideInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "stuckLoopRef")]
    pub stuck_loop_ref: String,
    #[serde(rename = "overrideReason")]
    pub override_reason: String,
    #[serde(rename = "operatorProvenanceRef")]
    pub operator_provenance_ref: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "blocksAutonomousLoop")]
    pub blocks_autonomous_loop: bool,
    #[serde(rename = "trustedWriteRequested")]
    pub trusted_write_requested: bool,
    #[serde(rename = "sourceApplyRequested")]
    pub source_apply_requested: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalHumanOverrideRecord {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "stuckLoopRef")]
    pub stuck_loop_ref: String,
    #[serde(rename = "overrideReason")]
    pub override_reason: String,
    #[serde(rename = "operatorProvenanceRef")]
    pub operator_provenance_ref: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "recordedAsProvenanceOnly")]
    pub recorded_as_provenance_only: bool,
    #[serde(rename = "blocksAutonomousLoop")]
    pub blocks_autonomous_loop: bool,
    #[serde(rename = "trustedWritesPerformed")]
    pub trusted_writes_performed: bool,
    #[serde(rename = "sourceApplyPerformed")]
    pub source_apply_performed: bool,
    pub boundary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanTasteFeedbackInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "feedbackId")]
    pub feedback_id: String,
    #[serde(rename = "feedbackText")]
    pub feedback_text: String,
    #[serde(rename = "m57CurationRef")]
    pub m57_curation_ref: String,
    #[serde(rename = "m58PlaytestRef")]
    pub m58_playtest_ref: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "autoApplyRequested")]
    pub auto_apply_requested: bool,
    #[serde(rename = "sourceApplyRequested")]
    pub source_apply_requested: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalHumanTasteFeedbackRecord {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "feedbackId")]
    pub feedback_id: String,
    #[serde(rename = "feedbackText")]
    pub feedback_text: String,
    #[serde(rename = "m57CurationRef")]
    pub m57_curation_ref: String,
    #[serde(rename = "m58PlaytestRef")]
    pub m58_playtest_ref: String,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
    #[serde(rename = "recordedAsProvenanceOnly")]
    pub recorded_as_provenance_only: bool,
    #[serde(rename = "autoApplied")]
    pub auto_applied: bool,
    #[serde(rename = "sourceApplyPerformed")]
    pub source_apply_performed: bool,
    #[serde(rename = "ring2HumanTasteVerdictRequired")]
    pub ring2_human_taste_verdict_required: bool,
    pub boundary: String,
}

pub fn render_optional_human_oversight_view(
    input: &OptionalHumanOversightInput,
) -> Result<OptionalHumanOversightView> {
    validate_oversight_input(input)?;
    let mut evidence_refs = input.contract.required_pipeline_refs.clone();
    evidence_refs.extend(input.stage_health_refs.clone());
    evidence_refs.extend(input.blocker_refs.clone());
    evidence_refs.extend(input.diagnosis_refs.clone());
    evidence_refs.extend(input.attribution_refs.clone());
    let view = OptionalHumanOversightView {
        schema_version: OPTIONAL_HUMAN_OVERSIGHT_VIEW_SCHEMA_VERSION.to_string(),
        title_id: input.contract.title_id.clone(),
        read_only: true,
        optional: true,
        blocks_autonomous_loop: false,
        trusted_writes_performed: false,
        loop_completed_without_human: input.loop_completed_without_human,
        stage_health_refs: input.stage_health_refs.clone(),
        blocker_refs: input.blocker_refs.clone(),
        diagnosis_refs: input.diagnosis_refs.clone(),
        attribution_refs: input.attribution_refs.clone(),
        evidence_refs,
        allowed_actions: vec![
            "view_stage_health".to_string(),
            "view_blockers".to_string(),
            "view_diagnosis".to_string(),
            "view_attribution".to_string(),
            "record_optional_override_provenance".to_string(),
            "record_optional_taste_feedback_provenance".to_string(),
        ],
        forbidden_actions: vec![
            "trusted_write".to_string(),
            "source_apply".to_string(),
            "auto_apply".to_string(),
            "block_loop".to_string(),
            "new_verifier".to_string(),
            "new_data_plane".to_string(),
        ],
        boundary: "Optional read-only Era L M72 oversight surface over existing openchrome verdict, journal.md, ledger.jsonl, loop-coverage attribution, source-apply, and trust-gradient artifacts; shows stage health, blockers, diagnosis, and attribution only; the autonomous loop completes without human viewing; no trusted writes, no source-apply, no auto-apply, no new verification engine, no new data plane, no new store; taste/fun and release go/no-go remain human Ring 2; #1 and #23 remain open.".to_string(),
    };
    validate_oversight_view(&view)?;
    Ok(view)
}

pub fn record_optional_human_override(
    input: &OptionalHumanOverrideInput,
) -> Result<OptionalHumanOverrideRecord> {
    validate_override_input(input)?;
    let record = OptionalHumanOverrideRecord {
        schema_version: OPTIONAL_HUMAN_OVERRIDE_RECORD_SCHEMA_VERSION.to_string(),
        title_id: input.title_id.clone(),
        stuck_loop_ref: input.stuck_loop_ref.clone(),
        override_reason: input.override_reason.clone(),
        operator_provenance_ref: input.operator_provenance_ref.clone(),
        evidence_refs: input.evidence_refs.clone(),
        recorded_as_provenance_only: true,
        blocks_autonomous_loop: false,
        trusted_writes_performed: false,
        source_apply_performed: false,
        boundary: "Non-blocking optional stuck-loop escape-hatch record: provenance-only human nudge over existing verdict/journal.md/ledger.jsonl/loop-coverage/source-apply/trust-gradient evidence; records operator reason without trusted write, source-apply, auto-apply, verifier execution, data-plane changes, or blocking unrelated autonomous loop work.".to_string(),
    };
    validate_override_record(&record)?;
    Ok(record)
}

pub fn record_optional_taste_feedback(
    input: &OptionalHumanTasteFeedbackInput,
) -> Result<OptionalHumanTasteFeedbackRecord> {
    validate_taste_feedback_input(input)?;
    let record = OptionalHumanTasteFeedbackRecord {
        schema_version: OPTIONAL_HUMAN_TASTE_FEEDBACK_RECORD_SCHEMA_VERSION.to_string(),
        title_id: input.title_id.clone(),
        feedback_id: input.feedback_id.clone(),
        feedback_text: input.feedback_text.clone(),
        m57_curation_ref: input.m57_curation_ref.clone(),
        m58_playtest_ref: input.m58_playtest_ref.clone(),
        evidence_refs: input.evidence_refs.clone(),
        recorded_as_provenance_only: true,
        auto_applied: false,
        source_apply_performed: false,
        ring2_human_taste_verdict_required: true,
        boundary: "Optional taste/fun-feedback capture reusing M57 curation and M58 playtest/fun-feel provenance; records feedback as provenance only, never auto-applies taste feedback, never performs source-apply, and does not automate fun/taste verdicts or release go/no-go.".to_string(),
    };
    validate_taste_feedback_record(&record)?;
    Ok(record)
}

fn validate_oversight_input(input: &OptionalHumanOversightInput) -> Result<()> {
    if input.schema_version != OPTIONAL_HUMAN_OVERSIGHT_INPUT_SCHEMA_VERSION {
        return Err(anyhow!(
            "optional human oversight input schemaVersion must be {OPTIONAL_HUMAN_OVERSIGHT_INPUT_SCHEMA_VERSION}"
        ));
    }
    input.contract.validate()?;
    validate_pipeline_refs(&input.stage_health_refs)?;
    validate_pipeline_refs(&input.blocker_refs)?;
    validate_pipeline_refs(&input.diagnosis_refs)?;
    validate_pipeline_refs(&input.attribution_refs)?;
    if !input.loop_completed_without_human {
        return Err(anyhow!(
            "oversight surface must prove the loop completed without human input"
        ));
    }
    if !(input.no_trusted_writes && input.no_new_data_plane) {
        return Err(anyhow!(
            "oversight surface must have no trusted writes and no new data plane"
        ));
    }
    Ok(())
}

fn validate_oversight_view(view: &OptionalHumanOversightView) -> Result<()> {
    if view.schema_version != OPTIONAL_HUMAN_OVERSIGHT_VIEW_SCHEMA_VERSION {
        return Err(anyhow!("optional human oversight view schemaVersion must be {OPTIONAL_HUMAN_OVERSIGHT_VIEW_SCHEMA_VERSION}"));
    }
    require_id("titleId", &view.title_id)?;
    if !(view.read_only && view.optional && view.loop_completed_without_human) {
        return Err(anyhow!(
            "oversight view must be read-only, optional, and backed by a no-human loop completion"
        ));
    }
    if view.blocks_autonomous_loop || view.trusted_writes_performed {
        return Err(anyhow!(
            "oversight view must not block the autonomous loop or perform trusted writes"
        ));
    }
    validate_pipeline_refs(&view.evidence_refs)?;
    let forbidden = view.forbidden_actions.join("\n").to_ascii_lowercase();
    for required in [
        "trusted_write",
        "source_apply",
        "auto_apply",
        "block_loop",
        "new_verifier",
        "new_data_plane",
    ] {
        if !forbidden.contains(required) {
            return Err(anyhow!(
                "oversight view forbiddenActions must include {required}"
            ));
        }
    }
    let boundary = view.boundary.to_ascii_lowercase();
    for required in [
        "read-only",
        "stage health",
        "blockers",
        "diagnosis",
        "attribution",
        "completes without human",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "no new verification engine",
        "no new data plane",
        "no new store",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!("oversight view boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_override_input(input: &OptionalHumanOverrideInput) -> Result<()> {
    if input.schema_version != OPTIONAL_HUMAN_OVERRIDE_INPUT_SCHEMA_VERSION {
        return Err(anyhow!("optional human override input schemaVersion must be {OPTIONAL_HUMAN_OVERRIDE_INPUT_SCHEMA_VERSION}"));
    }
    require_id("titleId", &input.title_id)?;
    require_text("stuckLoopRef", &input.stuck_loop_ref)?;
    require_text("overrideReason", &input.override_reason)?;
    require_text("operatorProvenanceRef", &input.operator_provenance_ref)?;
    validate_pipeline_refs(&input.evidence_refs)?;
    if input.blocks_autonomous_loop
        || input.trusted_write_requested
        || input.source_apply_requested
        || !input.no_new_data_plane
    {
        return Err(anyhow!("override must be non-blocking provenance only: no trusted write, no source-apply, no new data plane"));
    }
    Ok(())
}

fn validate_override_record(record: &OptionalHumanOverrideRecord) -> Result<()> {
    if record.schema_version != OPTIONAL_HUMAN_OVERRIDE_RECORD_SCHEMA_VERSION {
        return Err(anyhow!("optional human override record schemaVersion must be {OPTIONAL_HUMAN_OVERRIDE_RECORD_SCHEMA_VERSION}"));
    }
    validate_pipeline_refs(&record.evidence_refs)?;
    if !(record.recorded_as_provenance_only
        && !record.blocks_autonomous_loop
        && !record.trusted_writes_performed
        && !record.source_apply_performed)
    {
        return Err(anyhow!(
            "override record must be provenance-only and non-blocking"
        ));
    }
    Ok(())
}

fn validate_taste_feedback_input(input: &OptionalHumanTasteFeedbackInput) -> Result<()> {
    if input.schema_version != OPTIONAL_HUMAN_TASTE_FEEDBACK_INPUT_SCHEMA_VERSION {
        return Err(anyhow!("optional human taste feedback input schemaVersion must be {OPTIONAL_HUMAN_TASTE_FEEDBACK_INPUT_SCHEMA_VERSION}"));
    }
    require_id("titleId", &input.title_id)?;
    require_id("feedbackId", &input.feedback_id)?;
    require_text("feedbackText", &input.feedback_text)?;
    let provenance =
        format!("{}\n{}", input.m57_curation_ref, input.m58_playtest_ref).to_ascii_lowercase();
    for required in ["m57", "m58", "playtest"] {
        if !provenance.contains(required) {
            return Err(anyhow!("taste feedback provenance must include {required}"));
        }
    }
    validate_pipeline_refs(&input.evidence_refs)?;
    if input.auto_apply_requested || input.source_apply_requested || !input.no_new_data_plane {
        return Err(anyhow!("taste feedback must be provenance-only: never auto-apply, no source-apply, no new data plane"));
    }
    Ok(())
}

fn validate_taste_feedback_record(record: &OptionalHumanTasteFeedbackRecord) -> Result<()> {
    if record.schema_version != OPTIONAL_HUMAN_TASTE_FEEDBACK_RECORD_SCHEMA_VERSION {
        return Err(anyhow!("optional human taste feedback record schemaVersion must be {OPTIONAL_HUMAN_TASTE_FEEDBACK_RECORD_SCHEMA_VERSION}"));
    }
    validate_pipeline_refs(&record.evidence_refs)?;
    if !record.recorded_as_provenance_only
        || record.auto_applied
        || record.source_apply_performed
        || !record.ring2_human_taste_verdict_required
    {
        return Err(anyhow!(
            "taste feedback record must remain Ring-2 provenance only and never auto-applied"
        ));
    }
    Ok(())
}
