//! Contract tests for Save-Migration and Version-Compatibility v1 (#1846).

use std::path::{Path, PathBuf};

use ouroforge_core::save_migration::{
    incompatible_save_evidence, migrate_save_forward, SaveMigrationStatus, SAVE_MIGRATION_BOUNDARY,
};
use ouroforge_core::save_profile_scale::SAVE_PROFILE_SCALE_SCHEMA_VERSION;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(repo_root().join(rel))
        .unwrap_or_else(|error| panic!("read {rel}: {error}"))
}

#[test]
fn old_save_migrates_forward_with_replay_digest_integrity() {
    let old = read("examples/save-migration-v1/legacy-save.v0.fixture.json");
    let (store, evidence) =
        migrate_save_forward("migration-1846-forward", &old).expect("legacy save migrates forward");

    assert_eq!(store.schema_version, SAVE_PROFILE_SCALE_SCHEMA_VERSION);
    let profile = store.profile("legacy-hero").expect("profile migrated");
    assert_eq!(profile.run_history.len(), 2);
    assert_eq!(
        profile.run_history[0].replay_digest,
        "0000000000000000000000000000000000000000000000000000000000000001"
    );
    assert_eq!(
        profile.run_history[1].replay_digest,
        "0000000000000000000000000000000000000000000000000000000000000002"
    );
    store.verify_integrity().expect("migrated store is sealed");

    assert_eq!(evidence.status, SaveMigrationStatus::Migrated);
    assert_eq!(
        evidence.to_schema_version,
        SAVE_PROFILE_SCALE_SCHEMA_VERSION
    );
    assert!(evidence.replay_digests_preserved);
    assert!(evidence.migrated_hash.is_some());
    assert_eq!(evidence.boundary, SAVE_MIGRATION_BOUNDARY);
}

#[test]
fn incompatible_save_is_explicitly_handled_without_migrated_state() {
    let future = read("examples/save-migration-v1/invalid/incompatible-save.fixture.json");
    let error = migrate_save_forward("migration-1846-incompatible", &future)
        .expect_err("future save is incompatible");
    assert!(error.to_string().contains("incompatible"));

    let evidence = incompatible_save_evidence("migration-1846-incompatible", &future)
        .expect("incompatible evidence validates");
    assert_eq!(evidence.status, SaveMigrationStatus::Incompatible);
    assert!(evidence.migrated_hash.is_none());
    assert!(evidence
        .diagnostic
        .as_deref()
        .unwrap()
        .contains("incompatible"));
}

#[test]
fn tampered_migration_output_fails_digest_integrity() {
    let old = read("examples/save-migration-v1/legacy-save.v0.fixture.json");
    let (mut store, _) =
        migrate_save_forward("migration-1846-tamper", &old).expect("legacy save migrates forward");
    let profile = store
        .profiles
        .get_mut("legacy-hero")
        .expect("profile exists");
    profile.run_history[0].replay_digest = "tampered-replay-digest".to_string();

    let err = store
        .verify_integrity()
        .expect_err("tampered migration output fails closed");
    assert!(err.to_string().contains("integrity check failed"));
}

#[test]
fn migration_evidence_is_deterministic_and_governance_anchored() {
    let old = read("examples/save-migration-v1/legacy-save.v0.fixture.json");
    let (_, first) =
        migrate_save_forward("migration-1846-deterministic", &old).expect("first migration");
    let (_, second) =
        migrate_save_forward("migration-1846-deterministic", &old).expect("second migration");
    assert_eq!(first, second);
    assert_eq!(
        first.to_json().expect("serialize first"),
        second.to_json().expect("serialize second")
    );

    let doc = read("docs/post-launch-patch-v1.md");
    for required in [
        "Save schema version",
        "Migrations are forward-only, deterministic, and Rust/local-owned",
        "Unsupported, malformed, stale, missing, or ambiguous saves fail closed",
        "#1 remains open",
        "#23 remains open",
    ] {
        assert!(doc.contains(required), "missing governance text {required}");
    }
}
