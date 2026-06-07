//! Seeded stochastic determinism contract test (Era F Milestone 31, #1600).
//!
//! Validates the trusted seeded-RNG contract: identical seeds produce identical
//! runs (digest-stable), different seeds diverge detectably, and snapshot/restore
//! survives RNG draws. Drives the same shared fixture as the runtime test
//! (`examples/game-runtime/seeded-rng.test.cjs`).

use ouroforge_core::seeded_rng::{
    compare_runs, seeded_rng_cases_from_json_str, seeded_run, snapshot_restore_across_draw,
    SeededRng, SeededRunComparisonStatus, SEEDED_RNG_ALGORITHM, SEEDED_RNG_CASES_SCHEMA_VERSION,
    SEEDED_RNG_DIGEST_ALGORITHM, SEEDED_RNG_RUN_SCHEMA_VERSION,
};

fn cases_fixture() -> &'static str {
    include_str!("../../../examples/seeded-rng-v1/seeded-rng-cases.json")
}

#[test]
fn shared_cases_fixture_parses_with_expected_schema() {
    let cases = seeded_rng_cases_from_json_str(cases_fixture()).expect("cases fixture parses");
    assert_eq!(cases.schema_version, SEEDED_RNG_CASES_SCHEMA_VERSION);
    assert_eq!(cases.algorithm, SEEDED_RNG_ALGORITHM);
    assert!(cases.cases.same_seed.draws > 0);
    assert!(cases.cases.different_seed.draws > 0);
    assert!(cases.cases.snapshot_across_draw.draws_after > 0);
}

#[test]
fn rejects_fixture_with_wrong_schema_version() {
    let bad = r#"{"schemaVersion":"wrong","algorithm":"mulberry32","cases":{}}"#;
    assert!(seeded_rng_cases_from_json_str(bad).is_err());
}

#[test]
fn same_seed_yields_identical_digest_stable_run() {
    let cases = seeded_rng_cases_from_json_str(cases_fixture()).expect("cases fixture parses");
    let case = cases.cases.same_seed;
    let a = seeded_run(case.seed_a, case.draws);
    let b = seeded_run(case.seed_b, case.draws);

    assert_eq!(a.schema_version, SEEDED_RNG_RUN_SCHEMA_VERSION);
    assert_eq!(a.algorithm, SEEDED_RNG_ALGORITHM);
    assert_eq!(a.digest.algorithm, SEEDED_RNG_DIGEST_ALGORITHM);
    assert_eq!(a.digest.value.len(), 16);
    assert!(a.digest.value.chars().all(|c| c.is_ascii_hexdigit()));

    assert_eq!(a.draws, b.draws, "same seed yields identical draw sequence");
    assert_eq!(
        a.digest.value, b.digest.value,
        "same seed yields a digest-stable run"
    );

    let comparison = compare_runs(&a, &b);
    assert_eq!(comparison.status, SeededRunComparisonStatus::Matched);
    assert!(comparison.first_divergence.is_none());
}

#[test]
fn different_seed_diverges_detectably() {
    let cases = seeded_rng_cases_from_json_str(cases_fixture()).expect("cases fixture parses");
    let case = cases.cases.different_seed;
    assert_ne!(case.seed_a, case.seed_b);
    let a = seeded_run(case.seed_a, case.draws);
    let b = seeded_run(case.seed_b, case.draws);

    assert_ne!(a.draws, b.draws, "different seeds produce different draws");
    assert_ne!(
        a.digest.value, b.digest.value,
        "different seeds diverge in the digest"
    );

    let comparison = compare_runs(&a, &b);
    assert_eq!(comparison.status, SeededRunComparisonStatus::Diverged);
    let divergence = comparison
        .first_divergence
        .expect("divergent runs report a first divergence");
    // mulberry32 diverges on the very first draw for distinct seeds.
    assert_eq!(divergence.draw_index, 0);
    assert_ne!(divergence.expected, divergence.actual);
}

#[test]
fn snapshot_restore_survives_rng_draws() {
    let cases = seeded_rng_cases_from_json_str(cases_fixture()).expect("cases fixture parses");
    let case = cases.cases.snapshot_across_draw;
    let outcome =
        snapshot_restore_across_draw(case.seed, case.draws_before_snapshot, case.draws_after);

    assert_eq!(outcome.snapshot.seed, case.seed);
    assert_eq!(outcome.snapshot.draw_count, case.draws_before_snapshot);
    assert_eq!(outcome.continuation.len(), case.draws_after as usize);
    assert_eq!(
        outcome.continuation, outcome.replay,
        "restored stream redraws the identical sequence across a draw"
    );
}

#[test]
fn rng_stream_is_reproducible_from_seed_alone() {
    // No wall-clock, host entropy, or hidden state: a stream rebuilt from the
    // same seed reproduces the same sequence and capture position.
    let mut first = SeededRng::new(42);
    let mut second = SeededRng::new(42);
    let seq_first: Vec<u32> = (0..16).map(|_| first.next_raw()).collect();
    let seq_second: Vec<u32> = (0..16).map(|_| second.next_raw()).collect();
    assert_eq!(seq_first, seq_second);
    assert_eq!(first.capture(), second.capture());
    assert_eq!(first.draw_count(), 16);

    // next_unit stays in the half-open unit interval.
    let mut unit_rng = SeededRng::new(7);
    for _ in 0..64 {
        let value = unit_rng.next_unit();
        assert!((0.0..1.0).contains(&value));
    }
}
