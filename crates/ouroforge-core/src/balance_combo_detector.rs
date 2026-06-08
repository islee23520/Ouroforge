//! Combo-explosion / degenerate-build detector v1 (#1812).
//!
//! This module reuses the Milestone 32 synthetic-player balance telemetry shape
//! and the Milestone 28 over-solution idea of replayable counterexamples: a
//! detected build is reported with the seed/persona/action trace that reproduces
//! it. It is descriptive Rust/local validation only. It does not simulate a new
//! analyzer engine, write trusted state, auto-apply a balance change, or make a
//! fun/quality claim.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA_VERSION: &str = "ouroforge.balance-combo-telemetry.v1";
const MIN_WIN_RATE_BPS: u32 = 8000;
const MIN_COPLAY_WIN_SHARE_BPS: u32 = 8000;
const MIN_SCORE_RATIO_BPS: u32 = 15000;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboTelemetryFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub boundary: String,
    pub generated_state_policy: String,
    pub runs: Vec<SyntheticRunTelemetry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyntheticRunTelemetry {
    pub run_id: String,
    pub deck_seed: u32,
    pub persona_id: String,
    pub outcome: RunOutcome,
    pub turns: u32,
    pub score: u32,
    #[serde(default)]
    pub card_plays: BTreeMap<String, u32>,
    #[serde(default)]
    pub actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunOutcome {
    Won,
    Lost,
    Playing,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DegenerateBuildFinding {
    pub cards: Vec<String>,
    pub winning_runs_with_combo: u32,
    pub total_winning_runs: u32,
    pub combo_win_share_bps: u32,
    pub combo_win_rate_bps: u32,
    pub combo_avg_score: u32,
    pub baseline_avg_score: u32,
    pub score_ratio_bps: u32,
    pub replay_deck_seed: u32,
    pub replay_persona: String,
    pub replay_run_id: String,
    pub replay_actions: Vec<String>,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComboDetectorReport {
    pub schema_version: &'static str,
    pub fixture_id: String,
    pub total_runs: u32,
    pub total_winning_runs: u32,
    pub findings: Vec<DegenerateBuildFinding>,
    pub digest: String,
    pub boundary: String,
}

pub fn detect_degenerate_builds(
    fixture: &ComboTelemetryFixture,
) -> Result<ComboDetectorReport, String> {
    validate_fixture(fixture)?;

    let total_runs = fixture.runs.len() as u32;
    let total_winning_runs = fixture
        .runs
        .iter()
        .filter(|run| run.outcome == RunOutcome::Won)
        .count() as u32;
    let all_cards = card_vocabulary(&fixture.runs);
    let pairs = candidate_pairs(&all_cards);

    let mut findings = Vec::new();
    for pair in pairs {
        if let Some(finding) = evaluate_pair(&fixture.runs, &pair, total_winning_runs) {
            findings.push(finding);
        }
    }
    findings.sort_by(|a, b| a.cards.cmp(&b.cards));

    let digest = digest_report(
        &fixture.fixture_id,
        total_runs,
        total_winning_runs,
        &findings,
    );
    Ok(ComboDetectorReport {
        schema_version: SCHEMA_VERSION,
        fixture_id: fixture.fixture_id.clone(),
        total_runs,
        total_winning_runs,
        findings,
        digest,
        boundary: "descriptive Rust/local balance telemetry; browser/Studio read-only; no auto-apply, no auto-merge, no fun/quality/production/Godot claim; #1 and #23 remain open".to_string(),
    })
}

fn validate_fixture(fixture: &ComboTelemetryFixture) -> Result<(), String> {
    if fixture.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "schemaVersion must be {SCHEMA_VERSION}, got {}",
            fixture.schema_version
        ));
    }
    if fixture.fixture_id.trim().is_empty() {
        return Err("fixtureId must be non-empty".into());
    }
    let boundary = fixture.boundary.to_ascii_lowercase();
    for required in [
        "rust/local",
        "browser/studio read-only",
        "no auto-apply",
        "no auto-merge",
        "no fun",
        "#1 and #23 remain open",
    ] {
        if !boundary.contains(required) {
            return Err(format!("boundary missing required phrase: {required}"));
        }
    }
    if !fixture
        .generated_state_policy
        .to_ascii_lowercase()
        .contains("generated runs/artifacts remain untracked unless fixture-scoped")
    {
        return Err("generatedStatePolicy must preserve generated-state boundary".into());
    }
    if fixture.runs.is_empty() {
        return Err("runs must be non-empty".into());
    }
    let mut ids = BTreeSet::new();
    for run in &fixture.runs {
        if run.run_id.trim().is_empty() {
            return Err("runId must be non-empty".into());
        }
        if !ids.insert(run.run_id.as_str()) {
            return Err(format!("duplicate runId {}", run.run_id));
        }
        if run.persona_id.trim().is_empty() {
            return Err(format!("{} personaId must be non-empty", run.run_id));
        }
        if run.turns == 0 {
            return Err(format!("{} turns must be positive", run.run_id));
        }
        if run.card_plays.is_empty() {
            return Err(format!("{} cardPlays must be non-empty", run.run_id));
        }
        if run.card_plays.keys().any(|card| card.trim().is_empty()) {
            return Err(format!("{} card ids must be non-empty", run.run_id));
        }
        if run.actions.is_empty() {
            return Err(format!("{} replay actions must be present", run.run_id));
        }
    }
    Ok(())
}

fn card_vocabulary(runs: &[SyntheticRunTelemetry]) -> Vec<String> {
    let mut cards = BTreeSet::new();
    for run in runs {
        for card in run.card_plays.keys() {
            cards.insert(card.clone());
        }
    }
    cards.into_iter().collect()
}

fn candidate_pairs(cards: &[String]) -> Vec<Vec<String>> {
    let mut pairs = Vec::new();
    for i in 0..cards.len() {
        for j in (i + 1)..cards.len() {
            pairs.push(vec![cards[i].clone(), cards[j].clone()]);
        }
    }
    pairs
}

fn evaluate_pair(
    runs: &[SyntheticRunTelemetry],
    pair: &[String],
    total_winning_runs: u32,
) -> Option<DegenerateBuildFinding> {
    if total_winning_runs == 0 {
        return None;
    }
    let with_combo: Vec<&SyntheticRunTelemetry> = runs
        .iter()
        .filter(|run| {
            pair.iter()
                .all(|card| run.card_plays.get(card).copied().unwrap_or(0) > 0)
        })
        .collect();
    let winning_with_combo: Vec<&SyntheticRunTelemetry> = with_combo
        .iter()
        .copied()
        .filter(|run| run.outcome == RunOutcome::Won)
        .collect();
    if winning_with_combo.is_empty() {
        return None;
    }

    let combo_win_share_bps = bps(winning_with_combo.len() as u32, total_winning_runs);
    let combo_win_rate_bps = bps(winning_with_combo.len() as u32, with_combo.len() as u32);
    let combo_avg_score = average_score(&with_combo);
    let baseline: Vec<&SyntheticRunTelemetry> = runs
        .iter()
        .filter(|run| {
            !pair
                .iter()
                .all(|card| run.card_plays.get(card).copied().unwrap_or(0) > 0)
        })
        .collect();
    let baseline_avg_score = average_score(&baseline);
    let score_ratio_bps = if baseline_avg_score == 0 {
        if combo_avg_score > 0 {
            u32::MAX
        } else {
            0
        }
    } else {
        ((combo_avg_score as u64 * 10_000) / baseline_avg_score as u64) as u32
    };

    if combo_win_share_bps < MIN_COPLAY_WIN_SHARE_BPS
        || combo_win_rate_bps < MIN_WIN_RATE_BPS
        || score_ratio_bps < MIN_SCORE_RATIO_BPS
    {
        return None;
    }

    let replay = winning_with_combo[0];
    Some(DegenerateBuildFinding {
        cards: pair.to_vec(),
        winning_runs_with_combo: winning_with_combo.len() as u32,
        total_winning_runs,
        combo_win_share_bps,
        combo_win_rate_bps,
        combo_avg_score,
        baseline_avg_score,
        score_ratio_bps,
        replay_deck_seed: replay.deck_seed,
        replay_persona: replay.persona_id.clone(),
        replay_run_id: replay.run_id.clone(),
        replay_actions: replay.actions.clone(),
        reason: format!(
            "combo {} behaves like an over-solution: it wins {}/{} winning runs and scores {} vs baseline {} with replayable seed {}",
            pair.join("+"),
            winning_with_combo.len(),
            total_winning_runs,
            combo_avg_score,
            baseline_avg_score,
            replay.deck_seed
        ),
    })
}

fn bps(numerator: u32, denominator: u32) -> u32 {
    if denominator == 0 {
        0
    } else {
        ((numerator as u64 * 10_000) / denominator as u64) as u32
    }
}

fn average_score(runs: &[&SyntheticRunTelemetry]) -> u32 {
    if runs.is_empty() {
        return 0;
    }
    (runs.iter().map(|run| run.score as u64).sum::<u64>() / runs.len() as u64) as u32
}

fn digest_report(
    fixture_id: &str,
    total_runs: u32,
    total_winning_runs: u32,
    findings: &[DegenerateBuildFinding],
) -> String {
    let findings = findings
        .iter()
        .map(|finding| {
            format!(
                "{}@{}/{}:{}:{}:{}:{}",
                finding.cards.join("+"),
                finding.winning_runs_with_combo,
                finding.total_winning_runs,
                finding.combo_win_share_bps,
                finding.combo_win_rate_bps,
                finding.score_ratio_bps,
                finding.replay_deck_seed
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "combo-detector|fixture={fixture_id}|runs={total_runs}|wins={total_winning_runs}|findings={findings}"
    )
}
