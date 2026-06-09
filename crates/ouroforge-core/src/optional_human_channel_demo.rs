//! Optional human channel demo v1 (#2044 / Era L M72).
//!
//! Demonstrates that the existing real-title dogfood loop completes without a
//! human and still completes when optional oversight, override provenance, and
//! taste feedback provenance are present. This is read-only composition over the
//! M72 surface contract; it does not run a verifier, apply source, or introduce
//! a data plane.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    record_optional_human_override, record_optional_taste_feedback,
    render_optional_human_oversight_view, OptionalHumanOverrideInput, OptionalHumanOverrideRecord,
    OptionalHumanOversightInput, OptionalHumanOversightView, OptionalHumanTasteFeedbackInput,
    OptionalHumanTasteFeedbackRecord,
};

pub const OPTIONAL_HUMAN_CHANNEL_DEMO_INPUT_SCHEMA_VERSION: &str =
    "optional-human-channel-demo-input-v1";
pub const OPTIONAL_HUMAN_CHANNEL_DEMO_REPORT_SCHEMA_VERSION: &str =
    "optional-human-channel-demo-report-v1";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanChannelDemoInput {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "noHumanRun")]
    pub no_human_run: OptionalHumanChannelDemoRun,
    #[serde(rename = "withHumanRun")]
    pub with_human_run: OptionalHumanChannelDemoRun,
    #[serde(rename = "oversightInput")]
    pub oversight_input: OptionalHumanOversightInput,
    #[serde(rename = "overrideInput")]
    pub override_input: OptionalHumanOverrideInput,
    #[serde(rename = "tasteFeedbackInput")]
    pub taste_feedback_input: OptionalHumanTasteFeedbackInput,
    #[serde(rename = "noNewVerificationEngine")]
    pub no_new_verification_engine: bool,
    #[serde(rename = "noNewDataPlane")]
    pub no_new_data_plane: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OptionalHumanChannelDemoRun {
    #[serde(rename = "runId")]
    pub run_id: String,
    #[serde(rename = "command")]
    pub command: String,
    #[serde(rename = "completed")]
    pub completed: bool,
    #[serde(rename = "waitedForHuman")]
    pub waited_for_human: bool,
    #[serde(rename = "humanInputObserved")]
    pub human_input_observed: bool,
    #[serde(rename = "trustedWritesFromSurface")]
    pub trusted_writes_from_surface: bool,
    #[serde(rename = "evidenceRefs")]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionalHumanChannelDemoReport {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "titleId")]
    pub title_id: String,
    #[serde(rename = "noHumanRunId")]
    pub no_human_run_id: String,
    #[serde(rename = "withHumanRunId")]
    pub with_human_run_id: String,
    #[serde(rename = "bothRunsCompleted")]
    pub both_runs_completed: bool,
    #[serde(rename = "loopNeverWaitedForHuman")]
    pub loop_never_waited_for_human: bool,
    #[serde(rename = "humanInputOptional")]
    pub human_input_optional: bool,
    #[serde(rename = "trustedWritesFromSurface")]
    pub trusted_writes_from_surface: bool,
    #[serde(rename = "oversightView")]
    pub oversight_view: OptionalHumanOversightView,
    #[serde(rename = "overrideRecord")]
    pub override_record: OptionalHumanOverrideRecord,
    #[serde(rename = "tasteFeedbackRecord")]
    pub taste_feedback_record: OptionalHumanTasteFeedbackRecord,
    #[serde(rename = "comparisonRefs")]
    pub comparison_refs: Vec<String>,
    pub boundary: String,
}

impl OptionalHumanChannelDemoInput {
    pub fn from_json_str(input: &str) -> Result<Self> {
        let demo: Self = serde_json::from_str(input)
            .map_err(|err| anyhow!("failed to parse optional human channel demo: {err}"))?;
        validate_demo_input(&demo)?;
        Ok(demo)
    }
}

pub fn run_optional_human_channel_demo(
    input: &OptionalHumanChannelDemoInput,
) -> Result<OptionalHumanChannelDemoReport> {
    validate_demo_input(input)?;
    let oversight_view = render_optional_human_oversight_view(&input.oversight_input)?;
    let override_record = record_optional_human_override(&input.override_input)?;
    let taste_feedback_record = record_optional_taste_feedback(&input.taste_feedback_input)?;
    let mut comparison_refs = input.no_human_run.evidence_refs.clone();
    comparison_refs.extend(input.with_human_run.evidence_refs.clone());
    comparison_refs.push(input.override_input.operator_provenance_ref.clone());
    comparison_refs.push(input.taste_feedback_input.m57_curation_ref.clone());
    comparison_refs.push(input.taste_feedback_input.m58_playtest_ref.clone());

    let report = OptionalHumanChannelDemoReport {
        schema_version: OPTIONAL_HUMAN_CHANNEL_DEMO_REPORT_SCHEMA_VERSION.to_string(),
        title_id: input.title_id.clone(),
        no_human_run_id: input.no_human_run.run_id.clone(),
        with_human_run_id: input.with_human_run.run_id.clone(),
        both_runs_completed: true,
        loop_never_waited_for_human: true,
        human_input_optional: true,
        trusted_writes_from_surface: false,
        oversight_view,
        override_record,
        taste_feedback_record,
        comparison_refs,
        boundary: "Optional human channel demo over the existing Era I engine-builder deckbuilder: no-human and with-human spot-check runs both complete through openchrome, verdict, journal.md, ledger.jsonl, loop-coverage attribution, source-apply, and trust-gradient refs; optional oversight, override, and taste feedback are provenance-only, never trusted writes, never source-apply, never auto-apply, and never block the autonomous detect-explain-trace-attribute-propose-reverify-apply loop. No new verification engine, no new data plane, no new store; fun/taste and release go/no-go remain human Ring 2; #1 and #23 remain open.".to_string(),
    };
    validate_demo_report(&report)?;
    Ok(report)
}

fn validate_demo_input(input: &OptionalHumanChannelDemoInput) -> Result<()> {
    if input.schema_version != OPTIONAL_HUMAN_CHANNEL_DEMO_INPUT_SCHEMA_VERSION {
        return Err(anyhow!(
            "optional human channel demo input schemaVersion must be {OPTIONAL_HUMAN_CHANNEL_DEMO_INPUT_SCHEMA_VERSION}"
        ));
    }
    require_id("titleId", &input.title_id)?;
    input.no_human_run.validate("noHumanRun")?;
    input.with_human_run.validate("withHumanRun")?;
    if input.no_human_run.human_input_observed {
        return Err(anyhow!("no-human demo run must not observe human input"));
    }
    if !input.with_human_run.human_input_observed {
        return Err(anyhow!(
            "with-human demo run must include optional human provenance"
        ));
    }
    if input.no_human_run.waited_for_human || input.with_human_run.waited_for_human {
        return Err(anyhow!("demo loop must never wait for human input"));
    }
    if input.no_human_run.trusted_writes_from_surface
        || input.with_human_run.trusted_writes_from_surface
        || !input.no_new_verification_engine
        || !input.no_new_data_plane
    {
        return Err(anyhow!(
            "demo must not perform trusted writes or introduce verifier/data-plane drift"
        ));
    }
    Ok(())
}

impl OptionalHumanChannelDemoRun {
    fn validate(&self, label: &str) -> Result<()> {
        require_id(&format!("{label}.runId"), &self.run_id)?;
        if !self.command.contains(
            "cargo run -p ouroforge-cli -- run seeds/dogfood-deckbuilder.yaml --workers 2",
        ) {
            return Err(anyhow!(
                "{label}.command must be the real-title dogfood deckbuilder run command"
            ));
        }
        if !self.completed {
            return Err(anyhow!("{label} must complete"));
        }
        validate_pipeline_refs(&self.evidence_refs)?;
        Ok(())
    }
}

fn validate_demo_report(report: &OptionalHumanChannelDemoReport) -> Result<()> {
    if report.schema_version != OPTIONAL_HUMAN_CHANNEL_DEMO_REPORT_SCHEMA_VERSION {
        return Err(anyhow!(
            "optional human channel demo report schemaVersion must be {OPTIONAL_HUMAN_CHANNEL_DEMO_REPORT_SCHEMA_VERSION}"
        ));
    }
    if !(report.both_runs_completed
        && report.loop_never_waited_for_human
        && report.human_input_optional
        && !report.trusted_writes_from_surface)
    {
        return Err(anyhow!(
            "demo report must prove both runs complete without waiting or trusted writes"
        ));
    }
    validate_pipeline_refs(&report.comparison_refs)?;
    let boundary = report.boundary.to_ascii_lowercase();
    for required in [
        "no-human",
        "with-human",
        "both complete",
        "openchrome",
        "journal.md",
        "ledger.jsonl",
        "loop-coverage",
        "source-apply",
        "trust-gradient",
        "provenance-only",
        "never trusted writes",
        "never source-apply",
        "never auto-apply",
        "never block",
        "no new verification engine",
        "no new data plane",
        "no new store",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(required) {
            return Err(anyhow!("demo boundary must mention {required}"));
        }
    }
    Ok(())
}

fn validate_pipeline_refs(refs: &[String]) -> Result<()> {
    if refs.is_empty() {
        return Err(anyhow!("pipeline refs must not be empty"));
    }
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
