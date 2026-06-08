//! Fairness and daily-seed solvability verifier v1 (#1814).
//!
//! Reuses deterministic seeded-run evidence and solver-style witness traces to
//! verify whether seeds are skilled-winnable and whether declared daily seeds are
//! solvable. This is Rust/local descriptive validation over existing evidence,
//! not a new engine, browser write path, auto-apply, auto-merge, or fun score.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const SCHEMA_VERSION: &str = "ouroforge.balance-fairness.v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FairnessFixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub boundary: String,
    pub generated_state_policy: String,
    pub seeds: Vec<SeedFairnessEvidence>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedFairnessEvidence {
    pub seed: u32,
    #[serde(default)]
    pub daily_seed: bool,
    pub skilled_witness: Option<SkillWitness>,
    #[serde(default)]
    pub observed_losses: Vec<LossTrace>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillWitness {
    pub persona_id: String,
    pub actions: Vec<String>,
    pub outcome: WitnessOutcome,
    pub final_score: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum WitnessOutcome {
    Won,
    Lost,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LossTrace {
    pub persona_id: String,
    pub actions: Vec<String>,
    pub attribution: LossAttribution,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LossAttribution {
    Decisions,
    Luck,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SeedVerdict {
    Pass,
    Unfair,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedFairnessResult {
    pub seed: u32,
    pub daily_seed: bool,
    pub verdict: SeedVerdict,
    pub skilled_winnable: bool,
    pub daily_solvable: bool,
    pub loss_attribution_supported: bool,
    pub replay_persona: Option<String>,
    pub replay_actions: Vec<String>,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FairnessReport {
    pub schema_version: &'static str,
    pub fixture_id: String,
    pub total_seeds: u32,
    pub unfair_seeds: Vec<u32>,
    pub daily_seed_failures: Vec<u32>,
    pub results: Vec<SeedFairnessResult>,
    pub digest: String,
    pub boundary: String,
}

pub fn verify_fairness(fixture: &FairnessFixture) -> Result<FairnessReport, String> {
    validate_fixture(fixture)?;
    let mut results = fixture.seeds.iter().map(evaluate_seed).collect::<Vec<_>>();
    results.sort_by_key(|result| result.seed);
    let unfair_seeds = results
        .iter()
        .filter(|result| result.verdict == SeedVerdict::Unfair)
        .map(|result| result.seed)
        .collect::<Vec<_>>();
    let daily_seed_failures = results
        .iter()
        .filter(|result| result.daily_seed && !result.daily_solvable)
        .map(|result| result.seed)
        .collect::<Vec<_>>();
    let digest = digest_report(&fixture.fixture_id, &results);
    Ok(FairnessReport {
        schema_version: SCHEMA_VERSION,
        fixture_id: fixture.fixture_id.clone(),
        total_seeds: results.len() as u32,
        unfair_seeds,
        daily_seed_failures,
        results,
        digest,
        boundary: "descriptive Rust/local fairness and daily-seed solvability evidence; browser/Studio read-only; generated runs/artifacts untracked unless fixture-scoped; no auto-apply, no auto-merge, no fun/quality/production/Godot claim; #1 and #23 remain open".to_string(),
    })
}

fn evaluate_seed(seed: &SeedFairnessEvidence) -> SeedFairnessResult {
    let witness = seed
        .skilled_witness
        .as_ref()
        .filter(|w| w.outcome == WitnessOutcome::Won && !w.actions.is_empty());
    let skilled_winnable = witness.is_some();
    let loss_attribution_supported = seed
        .observed_losses
        .iter()
        .all(|loss| loss.attribution == LossAttribution::Decisions);
    let daily_solvable = !seed.daily_seed || skilled_winnable;
    let verdict = if skilled_winnable && loss_attribution_supported && daily_solvable {
        SeedVerdict::Pass
    } else {
        SeedVerdict::Unfair
    };
    let (replay_persona, replay_actions) = witness
        .map(|w| (Some(w.persona_id.clone()), w.actions.clone()))
        .unwrap_or_else(|| {
            seed.observed_losses
                .first()
                .map(|loss| (Some(loss.persona_id.clone()), loss.actions.clone()))
                .unwrap_or((None, Vec::new()))
        });
    let reason = match verdict {
        SeedVerdict::Pass => format!(
            "seed {} is skilled-winnable and losses are attributable to decisions",
            seed.seed
        ),
        SeedVerdict::Unfair => {
            if !skilled_winnable {
                format!("seed {} has no skilled winning witness", seed.seed)
            } else {
                format!("seed {} has loss attribution drift", seed.seed)
            }
        }
    };
    SeedFairnessResult {
        seed: seed.seed,
        daily_seed: seed.daily_seed,
        verdict,
        skilled_winnable,
        daily_solvable,
        loss_attribution_supported,
        replay_persona,
        replay_actions,
        reason,
    }
}

fn validate_fixture(fixture: &FairnessFixture) -> Result<(), String> {
    if fixture.schema_version != SCHEMA_VERSION {
        return Err(format!("schemaVersion must be {SCHEMA_VERSION}"));
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
    if fixture.seeds.is_empty() {
        return Err("seeds must be non-empty".into());
    }
    let mut seen = BTreeSet::new();
    for seed in &fixture.seeds {
        if !seen.insert(seed.seed) {
            return Err(format!("duplicate seed {}", seed.seed));
        }
        if let Some(witness) = &seed.skilled_witness {
            if witness.persona_id.trim().is_empty() || witness.actions.is_empty() {
                return Err(format!("seed {} witness must be replayable", seed.seed));
            }
        }
        for loss in &seed.observed_losses {
            if loss.persona_id.trim().is_empty() || loss.actions.is_empty() {
                return Err(format!("seed {} loss traces must be replayable", seed.seed));
            }
        }
    }
    Ok(())
}

fn digest_report(fixture_id: &str, results: &[SeedFairnessResult]) -> String {
    let rows = results
        .iter()
        .map(|r| {
            format!(
                "{}:{:?}:w{}:d{}:a{}",
                r.seed,
                r.verdict,
                r.skilled_winnable as u8,
                r.daily_solvable as u8,
                r.loss_attribution_supported as u8
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("fairness|fixture={fixture_id}|{rows}")
}
