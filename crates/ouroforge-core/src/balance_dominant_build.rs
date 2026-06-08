//! Dominant-build analysis v1 (#1813).
//!
//! Extends Milestone 32 pick-rate/win-rate balance telemetry for engine-builder
//! builds. Metrics are computed from seeded run evidence, not asserted, and are
//! descriptive only: no new analyzer engine, no trusted browser write, no
//! auto-apply, no auto-merge, and no fun/quality/production/Godot claim.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const SCHEMA_VERSION: &str = "ouroforge.balance-dominant-build-telemetry.v1";
const DOMINANT_PICK_RATE_BPS: u32 = 6000;
const DOMINANT_WIN_RATE_BPS: u32 = 8000;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DominantBuildTelemetryFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub declared_modifiers: Vec<String>,
    pub boundary: String,
    pub generated_state_policy: String,
    pub runs: Vec<BuildRunTelemetry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildRunTelemetry {
    pub run_id: String,
    pub deck_seed: u32,
    pub persona_id: String,
    pub outcome: BuildRunOutcome,
    pub build_id: String,
    pub score: u32,
    #[serde(default)]
    pub modifiers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildRunOutcome {
    Won,
    Lost,
    Playing,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildMetric {
    pub build_id: String,
    pub picks: u32,
    pub wins: u32,
    pub pick_rate_bps: u32,
    pub win_rate_bps: u32,
    pub avg_score: u32,
    pub replay_deck_seed: u32,
    pub replay_persona: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeadModifierMetric {
    pub modifier: String,
    pub picks: u32,
    pub pick_rate_bps: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DominantBuildReport {
    pub schema_version: &'static str,
    pub fixture_id: String,
    pub total_runs: u32,
    pub total_wins: u32,
    pub build_metrics: Vec<BuildMetric>,
    pub dominant_builds: Vec<BuildMetric>,
    pub dead_modifiers: Vec<DeadModifierMetric>,
    pub digest: String,
    pub boundary: String,
}

pub fn analyze_dominant_builds(
    fixture: &DominantBuildTelemetryFixture,
) -> Result<DominantBuildReport, String> {
    validate_fixture(fixture)?;
    let total_runs = fixture.runs.len() as u32;
    let total_wins = fixture
        .runs
        .iter()
        .filter(|run| run.outcome == BuildRunOutcome::Won)
        .count() as u32;

    let mut by_build: BTreeMap<String, Vec<&BuildRunTelemetry>> = BTreeMap::new();
    for run in &fixture.runs {
        by_build.entry(run.build_id.clone()).or_default().push(run);
    }

    let mut build_metrics = Vec::new();
    for (build_id, runs) in by_build {
        let picks = runs.len() as u32;
        let wins = runs
            .iter()
            .filter(|run| run.outcome == BuildRunOutcome::Won)
            .count() as u32;
        let replay = runs[0];
        build_metrics.push(BuildMetric {
            build_id,
            picks,
            wins,
            pick_rate_bps: bps(picks, total_runs),
            win_rate_bps: bps(wins, picks),
            avg_score: average_score(&runs),
            replay_deck_seed: replay.deck_seed,
            replay_persona: replay.persona_id.clone(),
        });
    }

    let dominant_builds = build_metrics
        .iter()
        .filter(|metric| {
            metric.pick_rate_bps >= DOMINANT_PICK_RATE_BPS
                && metric.win_rate_bps >= DOMINANT_WIN_RATE_BPS
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut modifier_picks: BTreeMap<String, u32> = fixture
        .declared_modifiers
        .iter()
        .map(|modifier| (modifier.clone(), 0))
        .collect();
    for run in &fixture.runs {
        let unique = run.modifiers.iter().collect::<BTreeSet<_>>();
        for modifier in unique {
            if let Some(picks) = modifier_picks.get_mut(modifier) {
                *picks += 1;
            }
        }
    }
    let dead_modifiers = modifier_picks
        .into_iter()
        .filter(|(_, picks)| *picks == 0)
        .map(|(modifier, picks)| DeadModifierMetric {
            modifier,
            picks,
            pick_rate_bps: bps(picks, total_runs),
        })
        .collect::<Vec<_>>();

    let digest = digest_report(
        &fixture.fixture_id,
        total_runs,
        total_wins,
        &build_metrics,
        &dominant_builds,
        &dead_modifiers,
    );

    Ok(DominantBuildReport {
        schema_version: SCHEMA_VERSION,
        fixture_id: fixture.fixture_id.clone(),
        total_runs,
        total_wins,
        build_metrics,
        dominant_builds,
        dead_modifiers,
        digest,
        boundary: "descriptive Rust/local pick-rate and win-rate metrics; browser/Studio read-only; generated runs/artifacts untracked unless fixture-scoped; no auto-apply, no auto-merge, no fun/quality/production/Godot claim; #1 and #23 remain open".to_string(),
    })
}

fn validate_fixture(fixture: &DominantBuildTelemetryFixture) -> Result<(), String> {
    if fixture.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "schemaVersion must be {SCHEMA_VERSION}, got {}",
            fixture.schema_version
        ));
    }
    if fixture.fixture_id.trim().is_empty() {
        return Err("fixtureId must be non-empty".into());
    }
    if fixture.declared_modifiers.is_empty() {
        return Err("declaredModifiers must be non-empty".into());
    }
    let mut modifier_ids = BTreeSet::new();
    for modifier in &fixture.declared_modifiers {
        if modifier.trim().is_empty() {
            return Err("declared modifier ids must be non-empty".into());
        }
        if !modifier_ids.insert(modifier.as_str()) {
            return Err(format!("duplicate declared modifier {modifier}"));
        }
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
    let mut run_ids = BTreeSet::new();
    for run in &fixture.runs {
        if run.run_id.trim().is_empty() {
            return Err("runId must be non-empty".into());
        }
        if !run_ids.insert(run.run_id.as_str()) {
            return Err(format!("duplicate runId {}", run.run_id));
        }
        if run.persona_id.trim().is_empty() || run.build_id.trim().is_empty() {
            return Err(format!(
                "{} personaId and buildId must be non-empty",
                run.run_id
            ));
        }
        for modifier in &run.modifiers {
            if !modifier_ids.contains(modifier.as_str()) {
                return Err(format!(
                    "{} references undeclared modifier {}",
                    run.run_id, modifier
                ));
            }
        }
    }
    Ok(())
}

fn bps(numerator: u32, denominator: u32) -> u32 {
    if denominator == 0 {
        0
    } else {
        ((numerator as u64 * 10_000) / denominator as u64) as u32
    }
}

fn average_score(runs: &[&BuildRunTelemetry]) -> u32 {
    if runs.is_empty() {
        0
    } else {
        (runs.iter().map(|run| run.score as u64).sum::<u64>() / runs.len() as u64) as u32
    }
}

fn digest_report(
    fixture_id: &str,
    total_runs: u32,
    total_wins: u32,
    build_metrics: &[BuildMetric],
    dominant_builds: &[BuildMetric],
    dead_modifiers: &[DeadModifierMetric],
) -> String {
    let metrics = build_metrics
        .iter()
        .map(|metric| {
            format!(
                "{}:{}/{}:{}:{}:{}",
                metric.build_id,
                metric.picks,
                metric.wins,
                metric.pick_rate_bps,
                metric.win_rate_bps,
                metric.avg_score
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let dominant = dominant_builds
        .iter()
        .map(|metric| metric.build_id.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let dead = dead_modifiers
        .iter()
        .map(|metric| metric.modifier.as_str())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "dominant-build|fixture={fixture_id}|runs={total_runs}|wins={total_wins}|metrics={metrics}|dominant={dominant}|dead={dead}"
    )
}
