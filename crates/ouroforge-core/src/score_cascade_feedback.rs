//! Score-Cascade Payoff Feedback v1 (#1820).
//!
//! This module emits deterministic, resolution-ordered feedback evidence for the
//! existing card-roguelite scoring resolution. It does not recompute browser-side
//! score, add a new engine, or grant trusted write authority; it mirrors the
//! existing Rust/local substrate resolution into a read-only payoff trace so the
//! runtime/Studio/browser layer can inspect score cascades without becoming the
//! source of truth.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::card_roguelite_substrate::{
    resolve_card_roguelite_state, validate_card_roguelite_config, CardRogueliteConfig,
    CardRogueliteEffect,
};

pub const SCORE_CASCADE_FEEDBACK_SCHEMA_VERSION: &str = "ouroforge.score-cascade-feedback.v1";
pub const SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION: &str =
    "ouroforge.score-cascade-feedback-event.v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreCascadeFeedbackTrace {
    pub schema_version: String,
    pub config_id: String,
    pub variant: String,
    pub seed: u32,
    pub final_score: i32,
    pub authoritative_score: i32,
    pub status: String,
    pub events: Vec<ScoreCascadeFeedbackEvent>,
    pub read_only_inspection: ScoreCascadeReadOnlyInspection,
    pub generated_state_policy: String,
    pub boundary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreCascadeFeedbackEvent {
    pub schema_version: String,
    pub event_id: String,
    pub step_index: usize,
    pub card_id: Option<String>,
    pub phase: String,
    pub modifier_id: Option<String>,
    pub operation: String,
    pub before: i32,
    pub add_score: i32,
    pub multiply_score: i32,
    pub after: i32,
    pub cumulative_total: i32,
    pub feedback_kind: String,
    pub juice_trigger: String,
    pub read_only_evidence: bool,
    pub boundary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreCascadeReadOnlyInspection {
    pub trusted_emitter: String,
    pub browser_studio_mode: String,
    pub disallowed_actions: Vec<String>,
}

pub fn score_cascade_feedback_trace(
    config: &CardRogueliteConfig,
) -> Result<ScoreCascadeFeedbackTrace> {
    validate_card_roguelite_config(config)?;
    let state = resolve_card_roguelite_state(config)?;
    let mut events = Vec::new();
    let mut cumulative_total = 0;

    for card_id in &state.deck {
        let card = config
            .cards
            .get(card_id)
            .ok_or_else(|| anyhow!("resolved deck references missing card `{card_id}`"))?;
        let base = card
            .actions
            .iter()
            .map(|effect| match effect {
                CardRogueliteEffect::Damage { amount }
                | CardRogueliteEffect::Block { amount }
                | CardRogueliteEffect::Score { amount } => *amount,
            })
            .sum::<i32>();
        let mut local_score = base;
        events.push(ScoreCascadeFeedbackEvent {
            schema_version: SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION.to_string(),
            event_id: format!("cascade-{:04}", events.len() + 1),
            step_index: events.len(),
            card_id: Some(card_id.clone()),
            phase: "base".to_string(),
            modifier_id: None,
            operation: "base-score".to_string(),
            before: 0,
            add_score: base,
            multiply_score: 1,
            after: local_score,
            cumulative_total,
            feedback_kind: "number-pop".to_string(),
            juice_trigger: "score_cascade".to_string(),
            read_only_evidence: true,
            boundary: event_boundary(),
        });

        let mut ordered = card
            .modifier_refs
            .iter()
            .filter_map(|modifier_id| {
                config
                    .modifiers
                    .get(modifier_id)
                    .map(|modifier| (modifier_id, modifier))
            })
            .collect::<Vec<_>>();
        ordered.sort_by_key(|(_modifier_id, modifier)| modifier.order);
        for (modifier_id, modifier) in ordered {
            let before = local_score;
            local_score = (local_score + modifier.add_score) * modifier.multiply_score;
            events.push(ScoreCascadeFeedbackEvent {
                schema_version: SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION.to_string(),
                event_id: format!("cascade-{:04}", events.len() + 1),
                step_index: events.len(),
                card_id: Some(card_id.clone()),
                phase: "modifier".to_string(),
                modifier_id: Some(modifier_id.clone()),
                operation: "add-then-multiply".to_string(),
                before,
                add_score: modifier.add_score,
                multiply_score: modifier.multiply_score,
                after: local_score,
                cumulative_total,
                feedback_kind: "multiplier-pop".to_string(),
                juice_trigger: "score_cascade".to_string(),
                read_only_evidence: true,
                boundary: event_boundary(),
            });
        }

        cumulative_total += local_score;
        events.push(ScoreCascadeFeedbackEvent {
            schema_version: SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION.to_string(),
            event_id: format!("cascade-{:04}", events.len() + 1),
            step_index: events.len(),
            card_id: Some(card_id.clone()),
            phase: "card-total".to_string(),
            modifier_id: None,
            operation: "cumulative-total".to_string(),
            before: cumulative_total - local_score,
            add_score: local_score,
            multiply_score: 1,
            after: cumulative_total,
            cumulative_total,
            feedback_kind: "cascade-total".to_string(),
            juice_trigger: "score_cascade".to_string(),
            read_only_evidence: true,
            boundary: event_boundary(),
        });
    }

    if cumulative_total != state.score {
        return Err(anyhow!(
            "score cascade final total {cumulative_total} does not match authoritative score {}",
            state.score
        ));
    }

    events.push(ScoreCascadeFeedbackEvent {
        schema_version: SCORE_CASCADE_FEEDBACK_EVENT_SCHEMA_VERSION.to_string(),
        event_id: format!("cascade-{:04}", events.len() + 1),
        step_index: events.len(),
        card_id: None,
        phase: "cascade-complete".to_string(),
        modifier_id: None,
        operation: "authoritative-score-match".to_string(),
        before: cumulative_total,
        add_score: 0,
        multiply_score: 1,
        after: state.score,
        cumulative_total,
        feedback_kind: "payoff-complete".to_string(),
        juice_trigger: "score_cascade".to_string(),
        read_only_evidence: true,
        boundary: event_boundary(),
    });

    Ok(ScoreCascadeFeedbackTrace {
        schema_version: SCORE_CASCADE_FEEDBACK_SCHEMA_VERSION.to_string(),
        config_id: config.config_id.clone(),
        variant: config.variant.clone(),
        seed: config.seed,
        final_score: cumulative_total,
        authoritative_score: state.score,
        status: format!("{:?}", state.status).to_lowercase(),
        events,
        read_only_inspection: ScoreCascadeReadOnlyInspection {
            trusted_emitter: "rust-score-cascade-feedback".to_string(),
            browser_studio_mode: "read-only score cascade payoff feedback inspection".to_string(),
            disallowed_actions: vec![
                "trusted writes".to_string(),
                "score recomputation authority".to_string(),
                "command bridge".to_string(),
                "live mutation".to_string(),
                "automated fun verdict".to_string(),
            ],
        },
        generated_state_policy:
            "cascade traces are generated evidence and stay untracked unless fixture-scoped"
                .to_string(),
        boundary: "Deterministic mechanical payoff feedback only; authoritative score remains the existing Rust/local substrate resolution, browser/Studio surfaces are read-only, and feel/fun judgment remains human (Era J).".to_string(),
    })
}

fn event_boundary() -> String {
    "read-only score feedback evidence; not score authority, not a trusted write, not a fun/quality verdict".to_string()
}
