//! Seeded stochastic determinism contract (Era F Milestone 31, #1600).
//!
//! Owns the trusted seeded-RNG determinism contract for the deck-roguelike rung
//! (#1599). All randomness derives from an explicit seed through a `mulberry32`
//! stream whose full state is reproducible from the seed, mirroring the JS
//! runtime stream in `examples/game-runtime/runtime.js`. A replay digest over
//! the draw sequence reuses the existing `fnv1a64` canonical digest, so this is
//! not a parallel determinism mechanism: identical seeds yield identical runs
//! (digest-stable) and different seeds diverge detectably.
//!
//! No randomness is read from wall-clock time, host entropy, or any source that
//! is not reproducible from the declared seed.

use crate::fnv1a64;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

pub const SEEDED_RNG_RUN_SCHEMA_VERSION: &str = "ouroforge.seeded-rng-run.v1";
pub const SEEDED_RNG_CASES_SCHEMA_VERSION: &str = "ouroforge.seeded-rng-cases.v1";
pub const SEEDED_RNG_DIGEST_ALGORITHM: &str = "fnv1a64-canonical-json-v1";
pub const SEEDED_RNG_ALGORITHM: &str = "mulberry32";

const RNG_INCREMENT: u32 = 0x6d2b_79f5;

/// A captured RNG stream position. Plain integers so it round-trips through
/// snapshot/restore and save artifacts without hidden state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededRngState {
    pub seed: u32,
    pub state: u32,
    pub draw_count: u32,
}

/// Deterministic `mulberry32` PRNG. The arithmetic is bit-identical to the JS
/// runtime stream (32-bit wrapping add/multiply, logical shifts), so a draw made
/// here matches the same draw made in the browser runtime for the same seed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeededRng {
    seed: u32,
    state: u32,
    draw_count: u32,
}

impl SeededRng {
    /// Create a stream seeded by `seed`. The initial state equals the seed.
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            state: seed,
            draw_count: 0,
        }
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }

    pub fn state(&self) -> u32 {
        self.state
    }

    pub fn draw_count(&self) -> u32 {
        self.draw_count
    }

    /// Advance the stream and return the raw 32-bit draw.
    pub fn next_raw(&mut self) -> u32 {
        self.state = self.state.wrapping_add(RNG_INCREMENT);
        let mut t = self.state;
        t = (t ^ (t >> 15)).wrapping_mul(1 | t);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        self.draw_count = self.draw_count.wrapping_add(1);
        t ^ (t >> 14)
    }

    /// Advance the stream and return a draw in the half-open unit interval.
    pub fn next_unit(&mut self) -> f64 {
        f64::from(self.next_raw()) / 4_294_967_296.0
    }

    /// Capture the stream position for snapshot/restore.
    pub fn capture(&self) -> SeededRngState {
        SeededRngState {
            seed: self.seed,
            state: self.state,
            draw_count: self.draw_count,
        }
    }

    /// Restore a previously captured stream position so subsequent draws redraw
    /// the identical sequence the captured stream would have produced.
    pub fn restore(&mut self, captured: SeededRngState) {
        self.seed = captured.seed;
        self.state = captured.state;
        self.draw_count = captured.draw_count;
    }
}

/// The replay digest of a seeded run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededRunDigest {
    pub algorithm: String,
    pub value: String,
}

/// A reproducible, digest-stable record of a seeded run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededRunReadModel {
    pub schema_version: String,
    pub algorithm: String,
    pub seed: u32,
    pub draw_count: u32,
    pub draws: Vec<u32>,
    pub final_state: u32,
    pub digest: SeededRunDigest,
}

#[derive(Serialize)]
struct SeededRunDigestInput<'a> {
    seed: u32,
    draws: &'a [u32],
    final_state: u32,
}

/// Produce the reproducible read model for a run of `draw_count` draws from
/// `seed`. The same arguments always produce the same model and digest.
pub fn seeded_run(seed: u32, draw_count: u32) -> SeededRunReadModel {
    let mut rng = SeededRng::new(seed);
    let mut draws = Vec::with_capacity(draw_count as usize);
    for _ in 0..draw_count {
        draws.push(rng.next_raw());
    }
    let final_state = rng.state();
    let canonical = serde_json::to_vec(&SeededRunDigestInput {
        seed,
        draws: &draws,
        final_state,
    })
    .unwrap_or_default();
    SeededRunReadModel {
        schema_version: SEEDED_RNG_RUN_SCHEMA_VERSION.to_string(),
        algorithm: SEEDED_RNG_ALGORITHM.to_string(),
        seed,
        draw_count,
        draws,
        final_state,
        digest: SeededRunDigest {
            algorithm: SEEDED_RNG_DIGEST_ALGORITHM.to_string(),
            value: format!("{:016x}", fnv1a64(&canonical)),
        },
    }
}

/// Whether two seeded runs are digest-identical, and where they first diverge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededRunComparison {
    pub schema_version: String,
    pub status: SeededRunComparisonStatus,
    pub expected: String,
    pub actual: String,
    pub first_divergence: Option<SeededRunDivergence>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeededRunComparisonStatus {
    Matched,
    Diverged,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeededRunDivergence {
    pub draw_index: usize,
    pub expected: u32,
    pub actual: u32,
}

/// Compare two runs by their draw sequence and digest, reporting the first
/// diverging draw index (mirroring the runtime replay-divergence surface).
pub fn compare_runs(
    expected: &SeededRunReadModel,
    actual: &SeededRunReadModel,
) -> SeededRunComparison {
    let first_divergence = expected
        .draws
        .iter()
        .zip(actual.draws.iter())
        .enumerate()
        .find(|(_, (lhs, rhs))| lhs != rhs)
        .map(|(draw_index, (lhs, rhs))| SeededRunDivergence {
            draw_index,
            expected: *lhs,
            actual: *rhs,
        })
        .or_else(|| {
            if expected.draws.len() != actual.draws.len() {
                let draw_index = expected.draws.len().min(actual.draws.len());
                Some(SeededRunDivergence {
                    draw_index,
                    expected: expected.draws.get(draw_index).copied().unwrap_or_default(),
                    actual: actual.draws.get(draw_index).copied().unwrap_or_default(),
                })
            } else {
                None
            }
        });
    let digest_matches = expected.digest.value == actual.digest.value;
    let status = if first_divergence.is_none() && digest_matches {
        SeededRunComparisonStatus::Matched
    } else {
        SeededRunComparisonStatus::Diverged
    };
    SeededRunComparison {
        schema_version: "ouroforge.seeded-rng-divergence.v1".to_string(),
        status,
        expected: expected.digest.value.clone(),
        actual: actual.digest.value.clone(),
        first_divergence,
    }
}

/// The outcome of a snapshot/restore-across-a-draw exercise: the draws taken
/// after the snapshot on the uninterrupted stream, and the draws taken after
/// restoring the snapshot. The contract holds when the two are identical.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotAcrossDrawOutcome {
    pub snapshot: SeededRngState,
    pub continuation: Vec<u32>,
    pub replay: Vec<u32>,
}

/// Draw `before` times, snapshot, draw `after` times (the continuation), then
/// restore the snapshot and draw `after` times again (the replay).
pub fn snapshot_restore_across_draw(
    seed: u32,
    before: u32,
    after: u32,
) -> SnapshotAcrossDrawOutcome {
    let mut rng = SeededRng::new(seed);
    for _ in 0..before {
        rng.next_raw();
    }
    let snapshot = rng.capture();
    let continuation: Vec<u32> = (0..after).map(|_| rng.next_raw()).collect();
    rng.restore(snapshot);
    let replay: Vec<u32> = (0..after).map(|_| rng.next_raw()).collect();
    SnapshotAcrossDrawOutcome {
        snapshot,
        continuation,
        replay,
    }
}

/// Shared seeded-determinism contract cases (the fixture consumed by both this
/// crate and the runtime test).
#[derive(Clone, Debug, Deserialize)]
pub struct SeededRngCases {
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub algorithm: String,
    pub cases: SeededRngCaseSet,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeededRngCaseSet {
    pub same_seed: SameSeedCase,
    pub different_seed: DifferentSeedCase,
    pub snapshot_across_draw: SnapshotAcrossDrawCase,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SameSeedCase {
    pub seed_a: u32,
    pub seed_b: u32,
    pub draws: u32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferentSeedCase {
    pub seed_a: u32,
    pub seed_b: u32,
    pub draws: u32,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotAcrossDrawCase {
    pub seed: u32,
    pub draws_before_snapshot: u32,
    pub draws_after: u32,
}

/// Parse the shared seeded-determinism cases fixture, validating its schema.
pub fn seeded_rng_cases_from_json_str(json: &str) -> Result<SeededRngCases> {
    let cases: SeededRngCases =
        serde_json::from_str(json).context("seeded rng cases fixture must be valid JSON")?;
    if cases.schema_version != SEEDED_RNG_CASES_SCHEMA_VERSION {
        return Err(anyhow!(
            "seeded rng cases schemaVersion must be {SEEDED_RNG_CASES_SCHEMA_VERSION}, got {}",
            cases.schema_version
        ));
    }
    if cases.algorithm != SEEDED_RNG_ALGORITHM {
        return Err(anyhow!(
            "seeded rng cases algorithm must be {SEEDED_RNG_ALGORITHM}, got {}",
            cases.algorithm
        ));
    }
    Ok(cases)
}
